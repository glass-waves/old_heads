use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use std::f32::consts::FRAC_PI_2;

use crate::player::{CameraMount, CursorGrabbed, Player, PlayerConfig, PlayerSettings, PlayerYaw};

/// Handles cursor grab state - left click to grab, Escape to release
pub fn handle_cursor_grab(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cursor_grabbed: ResMut<CursorGrabbed>,
    mut cursor_query: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let Ok(mut cursor) = cursor_query.single_mut() else {
        return;
    };

    // Click to grab cursor
    if mouse_button.just_pressed(MouseButton::Left) && !cursor_grabbed.0 {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        cursor_grabbed.0 = true;
    }

    // Escape to release cursor
    if keyboard.just_pressed(KeyCode::Escape) && cursor_grabbed.0 {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        cursor_grabbed.0 = false;
    }
}

/// Processes mouse movement for look controls
/// Applies yaw to PlayerYaw and pitch to CameraMount
pub fn mouse_look(
    cursor_grabbed: Res<CursorGrabbed>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player_settings: Res<PlayerSettings>,
    mut player_query: Query<(&PlayerConfig, &mut PlayerYaw), With<Player>>,
    mut camera_mount_query: Query<&mut CameraMount>,
) {
    // Only process mouse look when cursor is grabbed
    if !cursor_grabbed.0 {
        return;
    }

    let Ok((config, mut yaw)) = player_query.single_mut() else {
        return;
    };

    let Ok(mut camera_mount) = camera_mount_query.single_mut() else {
        return;
    };

    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = config.mouse_sensitivity * player_settings.sensitivity_multiplier;

    // Apply yaw (horizontal rotation)
    // Mouse moving right (positive X) should rotate right (negative yaw in Bevy)
    yaw.0 -= delta.x * sensitivity;

    // Apply pitch (vertical rotation)
    // Mouse moving down (positive Y) should look down (negative pitch)
    let pitch_delta = if player_settings.invert_y {
        delta.y * sensitivity
    } else {
        -delta.y * sensitivity
    };

    // Clamp pitch to prevent over-rotation (±89 degrees)
    const MAX_PITCH: f32 = FRAC_PI_2 - 0.01; // ~89 degrees
    camera_mount.pitch = (camera_mount.pitch + pitch_delta).clamp(-MAX_PITCH, MAX_PITCH);
}

/// Applies pitch rotation to the camera mount's transform
pub fn apply_camera_pitch(
    mut query: Query<(&CameraMount, &mut Transform)>,
) {
    for (mount, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_x(mount.pitch);
    }
}
