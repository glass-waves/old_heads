use bevy::prelude::*;

use crate::ball::{BallSettings, BallState, Basketball, HeldBy, ShootingChargeState};
use crate::player::CursorGrabbed;

/// Base FOV in degrees
const BASE_FOV: f32 = 60.0;
/// FOV at full charge (zoomed in for aiming)
const CHARGED_FOV: f32 = 50.0;

/// System to detect when charging starts (mouse press while holding ball)
pub fn shot_charge_start(
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_grabbed: Res<CursorGrabbed>,
    time: Res<Time>,
    mut charge_state: ResMut<ShootingChargeState>,
    ball_query: Query<&BallState, (With<Basketball>, With<HeldBy>)>,
) {
    // Only start charge when cursor is grabbed and left mouse just pressed
    if !cursor_grabbed.0 || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // Check if player is holding a ball
    let is_holding_ball = ball_query.iter().any(|state| *state == BallState::Held);
    if !is_holding_ball {
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
    settings: Res<BallSettings>,
    mut charge_state: ResMut<ShootingChargeState>,
) {
    if !charge_state.is_charging {
        return;
    }

    // If mouse button released, stop updating (throw system will handle it)
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    // Calculate elapsed time since charge started
    if let Some(start_time) = charge_state.charge_start {
        let elapsed = time.elapsed_secs() - start_time;
        // Clamp charge level to 0.0-1.0
        charge_state.charge_level = (elapsed / settings.charge_time).min(1.0);
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

    // Cancel if ball is no longer held
    let is_holding_ball = ball_query.iter().any(|state| *state == BallState::Held);
    if !is_holding_ball {
        reset_charge_state(&mut charge_state);
    }
}

/// System to provide visual feedback via camera FOV zoom while charging
pub fn shot_charge_camera_feedback(
    charge_state: Res<ShootingChargeState>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
) {
    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };

    let Projection::Perspective(ref mut persp) = *projection else {
        return;
    };

    // Calculate target FOV based on charge state
    let target_fov = if charge_state.is_charging {
        // Interpolate from base to charged FOV based on charge level
        BASE_FOV - (BASE_FOV - CHARGED_FOV) * charge_state.charge_level
    } else {
        BASE_FOV
    };

    // Smoothly interpolate current FOV towards target (about 10% per frame for smooth feel)
    let current_fov = persp.fov.to_degrees();
    let new_fov = current_fov + (target_fov - current_fov) * 0.15;
    persp.fov = new_fov.to_radians();
}

/// Helper to reset charge state
fn reset_charge_state(charge_state: &mut ShootingChargeState) {
    charge_state.charge_start = None;
    charge_state.charge_level = 0.0;
    charge_state.is_charging = false;
}
