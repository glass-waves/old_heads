use bevy::prelude::*;

use crate::ball::{BallState, Basketball, HeldBy, ShootingChargeState};
use crate::physics_config::PhysicsConfig;
use crate::player::CursorGrabbed;

/// System to detect when charging starts (mouse press while holding or dribbling ball)
pub fn shot_charge_start(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    cursor_grabbed: Res<CursorGrabbed>,
    time: Res<Time>,
    mut charge_state: ResMut<ShootingChargeState>,
    ball_query: Query<&BallState, (With<Basketball>, With<HeldBy>)>,
) {
    // Only start charge when cursor is grabbed and left mouse just pressed
    // Don't start charge if Shift is held (Shift+click = pass)
    if !cursor_grabbed.0 || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let shift_held = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    if shift_held {
        return;
    }

    // Check if player is holding or dribbling a ball
    let can_shoot = ball_query
        .iter()
        .any(|state| *state == BallState::Held || *state == BallState::Dribbling);
    if !can_shoot {
        return;
    }

    // Start charging
    charge_state.charge_start = Some(time.elapsed_secs());
    charge_state.is_charging = true;
    charge_state.charge_level = 0.0;
}

/// System to update charge level while button is held
pub fn shot_charge_update(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    config: Res<PhysicsConfig>,
    mut charge_state: ResMut<ShootingChargeState>,
) {
    if !charge_state.is_charging {
        return;
    }

    // If mouse button released, reset charge state as fallback
    // (ball_throw handles the actual throw via just_released, but that can miss
    // when other keys like WASD are held due to Bevy input ordering issues)
    if !mouse.pressed(MouseButton::Left) {
        reset_charge_state(&mut charge_state);
        return;
    }

    // Calculate elapsed time since charge started
    if let Some(start_time) = charge_state.charge_start {
        let elapsed = time.elapsed_secs() - start_time;
        // Clamp charge level to 0.0-1.0
        charge_state.charge_level = (elapsed / config.charge_time).min(1.0);
    }
}

/// System to cancel charge if ball is lost or cursor is released
pub fn shot_charge_cancel(
    cursor_grabbed: Res<CursorGrabbed>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut charge_state: ResMut<ShootingChargeState>,
    ball_query: Query<&BallState, (With<Basketball>, With<HeldBy>)>,
) {
    if !charge_state.is_charging {
        return;
    }

    // Cancel if cursor is no longer grabbed (Escape pressed)
    if !cursor_grabbed.0 {
        reset_charge_state(&mut charge_state);
        return;
    }

    // Cancel if Escape key just pressed
    if keyboard.just_pressed(KeyCode::Escape) {
        reset_charge_state(&mut charge_state);
        return;
    }

    // Cancel if ball is no longer held or dribbling
    let can_shoot = ball_query
        .iter()
        .any(|state| *state == BallState::Held || *state == BallState::Dribbling);
    if !can_shoot {
        reset_charge_state(&mut charge_state);
    }
}

/// System to draw charge indicator using gizmos
pub fn shot_charge_indicator(
    charge_state: Res<ShootingChargeState>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    mut gizmos: Gizmos,
) {
    if !charge_state.is_charging {
        return;
    }

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    // Position the indicator in world space, in front of the camera
    let camera_pos = camera_transform.translation();
    let camera_forward = camera_transform.forward().as_vec3();
    let camera_right = camera_transform.right().as_vec3();
    let camera_up = camera_transform.up().as_vec3();

    // Place bar 1.5m in front, slightly below center of view
    let bar_center = camera_pos + camera_forward * 1.5 + camera_up * -0.3;

    // Bar dimensions
    let bar_width = 0.3;
    let bar_height = 0.02;

    // Calculate bar endpoints
    let bar_left = bar_center - camera_right * (bar_width / 2.0);
    let bar_right = bar_center + camera_right * (bar_width / 2.0);

    // Draw background bar (dark gray)
    gizmos.line(bar_left, bar_right, Color::srgba(0.3, 0.3, 0.3, 0.8));

    // Draw filled portion based on charge level
    let filled_width = bar_width * charge_state.charge_level;
    let filled_right = bar_left + camera_right * filled_width;

    // Color transitions: green -> yellow -> red as charge increases
    let charge = charge_state.charge_level;
    let color = if charge < 0.5 {
        // Green to yellow
        let t = charge * 2.0;
        Color::srgb(t, 1.0, 0.0)
    } else {
        // Yellow to red
        let t = (charge - 0.5) * 2.0;
        Color::srgb(1.0, 1.0 - t, 0.0)
    };

    gizmos.line(bar_left, filled_right, color);

    // Draw top and bottom lines for the bar outline
    let top_left = bar_left + camera_up * bar_height;
    let top_right = bar_right + camera_up * bar_height;
    let bottom_left = bar_left - camera_up * bar_height;
    let bottom_right = bar_right - camera_up * bar_height;

    let outline_color = Color::srgba(0.5, 0.5, 0.5, 0.6);
    gizmos.line(top_left, top_right, outline_color);
    gizmos.line(bottom_left, bottom_right, outline_color);
    gizmos.line(top_left, bottom_left, outline_color);
    gizmos.line(top_right, bottom_right, outline_color);
}

/// Helper to reset charge state
fn reset_charge_state(charge_state: &mut ShootingChargeState) {
    charge_state.charge_start = None;
    charge_state.charge_level = 0.0;
    charge_state.is_charging = false;
}
