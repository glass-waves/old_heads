use bevy::prelude::*;

use crate::ball::{BallState, Basketball, DribblePhase, DribbleState, Hand, HasDribbled, HeldBy};
use crate::physics_config::PhysicsConfig;
use crate::player::{CameraMount, CursorGrabbed, Player};

/// System to start dribbling when Q or E is pressed while holding the ball.
/// Requires the ball to not have HasDribbled marker (can only dribble once per possession).
pub fn dribble_start(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    cursor_grabbed: Res<CursorGrabbed>,
    time: Res<Time>,
    mut ball_query: Query<
        (Entity, &mut BallState, Option<&HasDribbled>),
        (With<Basketball>, With<HeldBy>),
    >,
) {
    // Only process when cursor is grabbed
    if !cursor_grabbed.0 {
        return;
    }

    // Check for Q (left) or E (right) key held
    let left_pressed = keyboard.pressed(KeyCode::KeyQ);
    let right_pressed = keyboard.pressed(KeyCode::KeyE);

    if !left_pressed && !right_pressed {
        return;
    }

    // Find a ball that is currently Held (not already dribbling)
    for (ball_entity, mut ball_state, has_dribbled) in ball_query.iter_mut() {
        if *ball_state != BallState::Held {
            continue;
        }

        // Cannot dribble if already used dribble this possession
        if has_dribbled.is_some() {
            continue;
        }

        // Determine which hand to start dribbling with (prefer right if both pressed)
        let hand = if right_pressed { Hand::Right } else { Hand::Left };

        // Transition to dribbling state
        *ball_state = BallState::Dribbling;

        // Add dribble state component
        commands.entity(ball_entity).insert(DribbleState {
            current_hand: hand,
            last_dribble_time: time.elapsed_secs(),
            phase: DribblePhase::GoingDown,
            phase_progress: 0.0,
            crossover_in_progress: false,
            crossover_target: hand,
            crossover_progress: 0.0,
        });

        info!("Started dribbling with {:?} hand", hand);
        break;
    }
}

/// System to handle switching hands while dribbling (crossover).
/// Also detects when dribble keys are released to stop dribbling.
pub fn dribble_update(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    cursor_grabbed: Res<CursorGrabbed>,
    time: Res<Time>,
    config: Res<PhysicsConfig>,
    mut ball_query: Query<(Entity, &mut BallState, &mut DribbleState), With<Basketball>>,
) {
    // Check for Q (left) or E (right) key held
    let left_pressed = keyboard.pressed(KeyCode::KeyQ);
    let right_pressed = keyboard.pressed(KeyCode::KeyE);

    for (ball_entity, mut ball_state, mut dribble_state) in ball_query.iter_mut() {
        if *ball_state != BallState::Dribbling {
            continue;
        }

        // If neither key is pressed, or cursor released, stop dribbling
        if !left_pressed && !right_pressed || !cursor_grabbed.0 {
            // Stop dribbling - transition to Held and mark as having dribbled
            *ball_state = BallState::Held;
            commands
                .entity(ball_entity)
                .remove::<DribbleState>()
                .insert(HasDribbled);
            info!("Stopped dribbling - picked up dribble");
            continue;
        }

        // Determine which hand is being pressed
        let pressed_hand = if right_pressed { Hand::Right } else { Hand::Left };

        // If different hand, initiate crossover
        if pressed_hand != dribble_state.current_hand && !dribble_state.crossover_in_progress {
            let elapsed_since_dribble = time.elapsed_secs() - dribble_state.last_dribble_time;
            if elapsed_since_dribble >= config.dribble_cooldown {
                dribble_state.crossover_in_progress = true;
                dribble_state.crossover_target = pressed_hand;
                dribble_state.crossover_progress = 0.0;
                dribble_state.phase = DribblePhase::GoingDown;
                dribble_state.phase_progress = 0.0;
                dribble_state.last_dribble_time = time.elapsed_secs();
                info!("Crossover to {:?} hand", pressed_hand);
            }
        }
    }
}

/// System to update dribble physics/animation (continuous bouncing while key held)
pub fn dribble_physics_update(
    time: Res<Time>,
    config: Res<PhysicsConfig>,
    player_query: Query<&GlobalTransform, With<Player>>,
    camera_query: Query<&GlobalTransform, With<CameraMount>>,
    mut ball_query: Query<
        (&mut Transform, &BallState, &mut DribbleState),
        (With<Basketball>, With<HeldBy>),
    >,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let delta = time.delta_secs();

    for (mut ball_transform, ball_state, mut dribble_state) in ball_query.iter_mut() {
        if *ball_state != BallState::Dribbling {
            continue;
        }

        // Update phase progress - continuous bouncing
        let phase_duration = match dribble_state.phase {
            DribblePhase::GoingDown => 0.15,
            DribblePhase::ComingUp => 0.2,
            DribblePhase::AtPeak => 0.05, // Brief pause at peak, then auto-continue
        };

        dribble_state.phase_progress += delta / phase_duration;

        if dribble_state.phase_progress >= 1.0 {
            // Transition to next phase (continuous loop)
            match dribble_state.phase {
                DribblePhase::GoingDown => {
                    dribble_state.phase = DribblePhase::ComingUp;
                    dribble_state.phase_progress = 0.0;
                }
                DribblePhase::ComingUp => {
                    dribble_state.phase = DribblePhase::AtPeak;
                    dribble_state.phase_progress = 0.0;

                    // Complete crossover if in progress
                    if dribble_state.crossover_in_progress {
                        dribble_state.current_hand = dribble_state.crossover_target;
                        dribble_state.crossover_in_progress = false;
                    }
                }
                DribblePhase::AtPeak => {
                    // Auto-continue to next bounce
                    dribble_state.phase = DribblePhase::GoingDown;
                    dribble_state.phase_progress = 0.0;
                    dribble_state.last_dribble_time = time.elapsed_secs();
                }
            }
        }

        // Update crossover progress
        if dribble_state.crossover_in_progress {
            dribble_state.crossover_progress += delta / config.crossover_time;
            dribble_state.crossover_progress = dribble_state.crossover_progress.min(1.0);
        }

        // Calculate ball position
        let player_pos = player_transform.translation();
        let player_forward = camera_transform.forward().as_vec3();
        let player_forward_flat =
            Vec3::new(player_forward.x, 0.0, player_forward.z).normalize_or_zero();
        let player_right = player_forward_flat.cross(Vec3::Y).normalize_or_zero();

        // Determine horizontal offset based on current hand (or interpolate during crossover)
        let side_multiplier = if dribble_state.crossover_in_progress {
            let current = dribble_state.current_hand.side_multiplier();
            let target = dribble_state.crossover_target.side_multiplier();
            let t = dribble_state.crossover_progress;
            let smooth_t = t * t * (3.0 - 2.0 * t);
            current + (target - current) * smooth_t
        } else {
            dribble_state.current_hand.side_multiplier()
        };

        let side_offset = player_right * (side_multiplier * config.dribble_side_offset);
        let forward_offset = player_forward_flat * config.dribble_forward_offset;

        // Calculate vertical position based on phase
        let floor_height = config.ball_radius;
        let peak_height = config.dribble_bounce_height + config.ball_radius;

        let height = match dribble_state.phase {
            DribblePhase::GoingDown => {
                let t = dribble_state.phase_progress;
                let smooth_t = t * t;
                peak_height + (floor_height - peak_height) * smooth_t
            }
            DribblePhase::ComingUp => {
                let t = dribble_state.phase_progress;
                let smooth_t = 1.0 - (1.0 - t) * (1.0 - t);
                floor_height + (peak_height - floor_height) * smooth_t
            }
            DribblePhase::AtPeak => peak_height,
        };

        // Set ball position
        ball_transform.translation = player_pos + side_offset + forward_offset + Vec3::Y * height;
    }
}

/// System to clear HasDribbled when ball becomes Free
pub fn clear_has_dribbled_on_free(
    mut commands: Commands,
    ball_query: Query<(Entity, &BallState, Option<&HasDribbled>), With<Basketball>>,
) {
    for (entity, ball_state, has_dribbled) in ball_query.iter() {
        if *ball_state == BallState::Free && has_dribbled.is_some() {
            commands.entity(entity).remove::<HasDribbled>();
        }
    }
}
