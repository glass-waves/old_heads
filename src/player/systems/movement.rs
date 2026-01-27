use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::{BallState, Basketball, HeldBy};
use crate::player::{MovementInput, Player, PlayerConfig, PlayerYaw};

/// Gathers WASD input and stores it in MovementInput component
pub fn gather_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInput, With<Player>>,
) {
    let Ok(mut movement_input) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // Normalize to prevent faster diagonal movement
    movement_input.direction = direction.normalize_or_zero();
}

/// Applies movement input as velocity, taking into account player's yaw rotation.
/// Movement is blocked when holding a ball (not dribbling).
pub fn apply_movement(
    mut player_query: Query<
        (Entity, &MovementInput, &PlayerConfig, &PlayerYaw, &mut LinearVelocity),
        With<Player>,
    >,
    ball_query: Query<(&HeldBy, &BallState), With<Basketball>>,
) {
    let Ok((player_entity, input, config, yaw, mut velocity)) = player_query.single_mut() else {
        return;
    };

    // Check if player is holding a ball in Held state (not Dribbling)
    let is_planted = ball_query.iter().any(|(held_by, ball_state)| {
        held_by.0 == player_entity && *ball_state == BallState::Held
    });

    if input.direction == Vec2::ZERO || is_planted {
        // Stop horizontal movement when no input OR when planted with ball
        velocity.x = 0.0;
        velocity.z = 0.0;
        return;
    }

    // Calculate movement direction in world space based on yaw
    let yaw_rotation = Quat::from_rotation_y(yaw.0);

    // Forward is -Z in Bevy's coordinate system
    let forward = yaw_rotation * Vec3::NEG_Z;
    let right = yaw_rotation * Vec3::X;

    // Combine forward/back and left/right movement
    let move_direction =
        (forward * input.direction.y + right * input.direction.x).normalize_or_zero();

    // Apply velocity
    let move_velocity = move_direction * config.walk_speed;
    velocity.x = move_velocity.x;
    velocity.z = move_velocity.z;
    // Don't modify Y velocity (preserve gravity effects if any)
}

/// Applies yaw rotation to the player's transform
pub fn apply_player_rotation(
    mut query: Query<(&PlayerYaw, &mut Transform), With<Player>>,
) {
    let Ok((yaw, mut transform)) = query.single_mut() else {
        return;
    };

    transform.rotation = Quat::from_rotation_y(yaw.0);
}
