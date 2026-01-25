use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::{BallSettings, BallState, Basketball, HeldBy, ShootingChargeState};
use crate::player::{CameraMount, CursorGrabbed};

/// Minimum upward arc angle in degrees (quick tap)
const MIN_ARC_ANGLE: f32 = 5.0;
/// Maximum upward arc angle in degrees (full charge)
const MAX_ARC_ANGLE: f32 = 35.0;

/// Minimum backspin (quick tap)
const MIN_BACKSPIN: f32 = 2.0;
/// Maximum backspin (full charge)
const MAX_BACKSPIN: f32 = 8.0;

/// System to throw the ball when left mouse button is released after charging
pub fn ball_throw(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_grabbed: Res<CursorGrabbed>,
    settings: Res<BallSettings>,
    mut charge_state: ResMut<ShootingChargeState>,
    camera_query: Query<&GlobalTransform, With<CameraMount>>,
    mut ball_query: Query<(Entity, &mut BallState), (With<Basketball>, With<HeldBy>)>,
) {
    // Only throw when cursor is grabbed, was charging, and left mouse just released
    if !cursor_grabbed.0 || !charge_state.is_charging || !mouse.just_released(MouseButton::Left) {
        return;
    }

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for (ball_entity, mut ball_state) in ball_query.iter_mut() {
        if *ball_state != BallState::Held {
            continue;
        }

        // Get charge level and apply quadratic scaling for better feel
        // (small charges feel distinct from medium charges)
        let charge = charge_state.charge_level;
        let scaled_charge = charge * charge; // Quadratic curve

        // Calculate throw power based on charge
        let throw_power = settings.min_throw_power
            + (settings.max_throw_power - settings.min_throw_power) * scaled_charge;

        // Get throw direction from camera (as Vec3)
        let forward = camera_transform.forward().as_vec3();

        // Calculate arc angle based on charge (more charge = higher arc)
        let arc_angle_deg = MIN_ARC_ANGLE + (MAX_ARC_ANGLE - MIN_ARC_ANGLE) * scaled_charge;
        let arc_angle_rad = arc_angle_deg.to_radians();

        // Calculate throw velocity with upward arc
        // Project forward onto horizontal plane, then add vertical component
        let horizontal_dir = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let horizontal_speed = throw_power * arc_angle_rad.cos();
        let vertical_speed = throw_power * arc_angle_rad.sin();

        let throw_velocity = horizontal_dir * horizontal_speed + Vec3::Y * vertical_speed;

        // Calculate backspin based on charge
        let backspin = MIN_BACKSPIN + (MAX_BACKSPIN - MIN_BACKSPIN) * scaled_charge;

        // Release the ball
        *ball_state = BallState::Free;

        commands
            .entity(ball_entity)
            .remove::<HeldBy>()
            .insert((
                RigidBody::Dynamic,
                LinearVelocity(throw_velocity),
                // Add backspin for more realistic basketball throw
                AngularVelocity(Vec3::new(-backspin, 0.0, 0.0)),
            ));

        info!(
            "Threw basketball - charge: {:.0}%, power: {:.1} m/s, arc: {:.0}°",
            charge * 100.0,
            throw_power,
            arc_angle_deg
        );
    }

    // Reset charge state after throw
    charge_state.charge_start = None;
    charge_state.charge_level = 0.0;
    charge_state.is_charging = false;
}
