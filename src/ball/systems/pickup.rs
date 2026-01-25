use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::{BallSettings, BallState, Basketball, HeldBy};
use crate::player::{CameraMount, Player};

/// System to pick up the ball when E is pressed and ball is within range
pub fn ball_pickup(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<BallSettings>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    camera_query: Query<&GlobalTransform, With<CameraMount>>,
    mut ball_query: Query<
        (Entity, &GlobalTransform, &mut BallState),
        (With<Basketball>, Without<HeldBy>),
    >,
) {
    // Only process on E key press
    if !keyboard.just_pressed(KeyCode::KeyE) {
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

        if distance <= settings.pickup_range {
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
