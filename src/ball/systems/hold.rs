use bevy::prelude::*;

use crate::ball::{BallState, Basketball, HeldBy};
use crate::physics_config::PhysicsConfig;
use crate::player::CameraMount;

/// System to position the held ball relative to the camera
/// Only positions when in Held state (not Dribbling - that's handled by dribble_physics_update)
pub fn ball_hold_position(
    config: Res<PhysicsConfig>,
    camera_query: Query<&GlobalTransform, With<CameraMount>>,
    mut ball_query: Query<(&mut Transform, &BallState, &HeldBy), With<Basketball>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for (mut ball_transform, ball_state, _held_by) in ball_query.iter_mut() {
        // Only position when Held, not Dribbling
        if *ball_state != BallState::Held {
            continue;
        }

        // Position the ball in front of and below the camera
        let camera_forward = camera_transform.forward();
        let camera_right = camera_transform.right();
        let camera_up = camera_transform.up();

        let hold_position = camera_transform.translation()
            + camera_forward * config.hold_offset.z.abs()
            + camera_up * config.hold_offset.y
            + camera_right * config.hold_offset.x;

        ball_transform.translation = hold_position;
    }
}
