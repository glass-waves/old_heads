use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::{BallState, Basketball, DribbleState, HasDribbled, HeldBy};
use crate::physics_config::PhysicsConfig;
use crate::player::{CameraMount, CursorGrabbed, Player};

/// Type of pass being performed
#[derive(Clone, Copy, Debug)]
pub enum PassType {
    /// Direct pass toward reticle (looking level)
    Chest,
    /// Bounces off ground toward target (looking down)
    Bounce,
    /// Arcing pass over defenders (looking up)
    Lob,
}

impl PassType {
    /// Base speed for this pass type
    pub fn base_speed(&self, config: &PhysicsConfig) -> f32 {
        match self {
            PassType::Chest => config.chest_pass_speed,
            PassType::Bounce => config.bounce_pass_speed,
            PassType::Lob => config.lob_pass_speed,
        }
    }

    /// Determine pass type from camera pitch (Y component of forward vector)
    /// - Looking down (y < threshold) = bounce pass
    /// - Looking up (y > threshold) = lob pass
    /// - Otherwise = chest pass
    pub fn from_camera_pitch(forward_y: f32, config: &PhysicsConfig) -> Self {
        if forward_y < config.bounce_pass_pitch_threshold {
            PassType::Bounce
        } else if forward_y > config.lob_pass_pitch_threshold {
            PassType::Lob
        } else {
            PassType::Chest
        }
    }
}

/// System to handle passing input (Shift + left-click to pass)
/// Pass type is determined by camera angle:
/// - Looking down → bounce pass
/// - Looking level → chest pass
/// - Looking up → lob pass
/// Works from both Held and Dribbling states
pub fn pass_input(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    cursor_grabbed: Res<CursorGrabbed>,
    config: Res<PhysicsConfig>,
    camera_query: Query<&GlobalTransform, With<CameraMount>>,
    _player_query: Query<Entity, With<Player>>,
    mut ball_query: Query<
        (
            Entity,
            &mut Transform,
            &mut BallState,
            Option<&DribbleState>,
        ),
        (With<Basketball>, With<HeldBy>),
    >,
) {
    // Only process when cursor is grabbed
    if !cursor_grabbed.0 {
        return;
    }

    // Shift + Left-click to pass
    let shift_held = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    if !shift_held || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_forward = camera_transform.forward().as_vec3();

    // Determine pass type from camera pitch
    let pass_type = PassType::from_camera_pitch(camera_forward.y, &config);

    // Find a ball that is Held or Dribbling by this player
    for (ball_entity, _ball_transform, mut ball_state, dribble_state) in ball_query.iter_mut() {
        // Must be held or dribbling
        if *ball_state != BallState::Held && *ball_state != BallState::Dribbling {
            continue;
        }

        // Calculate pass velocity based on type
        let speed = pass_type.base_speed(&config);

        let velocity = match pass_type {
            PassType::Chest => {
                // Direct pass toward where camera is looking, with slight upward tilt
                let forward_flat =
                    Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize_or_zero();
                let up_angle = config.chest_pass_angle.to_radians();
                let pass_dir = (forward_flat * up_angle.cos() + Vec3::Y * up_angle.sin()).normalize();
                pass_dir * speed
            }
            PassType::Bounce => {
                // Calculate trajectory to hit ground and bounce up
                // Aim forward and down, the bounce will redirect upward
                let forward_flat =
                    Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize_or_zero();

                // Ball should hit ground about 1/3 of the way to a typical pass distance
                // Then bounce up toward the target
                let down_angle = -config.bounce_pass_angle.to_radians();
                let pass_dir = (forward_flat * down_angle.cos() + Vec3::Y * down_angle.sin()).normalize();

                pass_dir * speed
            }
            PassType::Lob => {
                // Arcing pass - add upward component
                let forward_flat =
                    Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize_or_zero();

                let up_angle = config.lob_pass_angle.to_radians();
                let pass_dir = (forward_flat * up_angle.cos() + Vec3::Y * up_angle.sin()).normalize();

                pass_dir * speed
            }
        };

        // Transition ball to Free state
        *ball_state = BallState::Free;

        // Remove held/dribble components and enable physics
        let mut entity_commands = commands.entity(ball_entity);
        entity_commands
            .remove::<HeldBy>()
            .remove::<HasDribbled>()
            .insert(RigidBody::Dynamic)
            .insert(LinearVelocity(velocity));

        if dribble_state.is_some() {
            entity_commands.remove::<DribbleState>();
        }

        info!("Executed {:?} pass (pitch: {:.2}) with velocity {:?}", pass_type, camera_forward.y, velocity);
        break;
    }
}
