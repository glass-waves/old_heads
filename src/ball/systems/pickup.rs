use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::{BallState, Basketball, HeldBy};
use crate::physics_config::PhysicsConfig;
use crate::player::{CameraMount, Player};

/// System to pick up the ball when F is pressed and ball is within range
pub fn ball_pickup(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<PhysicsConfig>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    camera_query: Query<&GlobalTransform, With<CameraMount>>,
    mut ball_query: Query<
        (Entity, &GlobalTransform, &mut BallState),
        (With<Basketball>, Without<HeldBy>),
    >,
) {
    // Only process on F key press (moved from E since Q/E are now dribble keys)
    if !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    let Ok(_camera_transform) = camera_query.single() else {
        return;
    };

    let player_pos = player_transform.translation();

    // Find the closest free ball within pickup range
    let mut closest_ball: Option<(Entity, f32)> = None;

    for (ball_entity, ball_transform, ball_state) in ball_query.iter() {
        // Skip balls that are already held
        if *ball_state == BallState::Held {
            continue;
        }

        let ball_pos = ball_transform.translation();
        let distance = player_pos.distance(ball_pos);

        if distance <= config.pickup_range {
            if closest_ball.is_none() || distance < closest_ball.unwrap().1 {
                closest_ball = Some((ball_entity, distance));
            }
        }
    }

    // Pick up the closest ball
    if let Some((ball_entity, _)) = closest_ball {
        if let Ok((_, _, mut ball_state)) = ball_query.get_mut(ball_entity) {
            *ball_state = BallState::Held;

            commands.entity(ball_entity).insert((
                HeldBy(player_entity),
                RigidBody::Kinematic,
                LinearVelocity::ZERO,
                AngularVelocity::ZERO,
            ));

            info!("Picked up basketball");
        }
    }
}
