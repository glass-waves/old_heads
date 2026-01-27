use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::{BallState, Basketball, DribbleState, HeldBy, ShootingChargeState};
use crate::physics_config::PhysicsConfig;
use crate::player::{CameraMount, CursorGrabbed};

/// System to throw the ball when left mouse button is released after charging
/// Works from both Held and Dribbling states
pub fn ball_throw(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_grabbed: Res<CursorGrabbed>,
    config: Res<PhysicsConfig>,
    mut charge_state: ResMut<ShootingChargeState>,
    camera_query: Query<(&GlobalTransform, &CameraMount)>,
    mut ball_query: Query<(Entity, &mut BallState), (With<Basketball>, With<HeldBy>)>,
) {
    // Only throw when cursor is grabbed, was charging, and left mouse is no longer pressed
    // Note: We use !pressed() instead of just_released() because just_released() is unreliable
    // when other keys (like WASD) are held simultaneously (known Bevy input ordering issue)
    if !cursor_grabbed.0 || !charge_state.is_charging || mouse.pressed(MouseButton::Left) {
        return;
    }

    let Ok((camera_transform, camera_mount)) = camera_query.single() else {
        return;
    };

    for (ball_entity, mut ball_state) in ball_query.iter_mut() {
        // Allow throwing from both Held and Dribbling states
        if *ball_state != BallState::Held && *ball_state != BallState::Dribbling {
            continue;
        }

        // Get charge level and apply curve scaling for better feel
        let charge = charge_state.charge_level;
        let scaled_charge = config.charge_curve.apply(charge);

        // Calculate throw power based on charge
        let throw_power = config.min_throw_power
            + (config.max_throw_power - config.min_throw_power) * scaled_charge;

        // Get camera pitch angle (negative when looking down, positive when looking up)
        let camera_pitch = camera_mount.pitch;

        // Calculate additional arc angle based on charge (more charge = higher arc)
        let arc_angle_deg =
            config.min_arc_angle + (config.max_arc_angle - config.min_arc_angle) * scaled_charge;
        let arc_angle_rad = arc_angle_deg.to_radians();

        // Total throw angle = camera pitch + arc angle
        // This makes the ball go where you're looking, plus the arc
        let total_angle_rad = camera_pitch + arc_angle_rad;

        // Get horizontal direction from camera forward (projected onto horizontal plane)
        let forward = camera_transform.forward().as_vec3();
        let horizontal_dir = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

        // Calculate velocity components
        // The throw goes in the horizontal direction you're facing
        // with vertical component determined by camera pitch + arc
        let horizontal_speed = throw_power * total_angle_rad.cos();
        let vertical_speed = throw_power * total_angle_rad.sin();

        let throw_velocity = horizontal_dir * horizontal_speed + Vec3::Y * vertical_speed;

        // Calculate backspin based on charge
        let backspin =
            config.min_backspin + (config.max_backspin - config.min_backspin) * scaled_charge;

        // Release the ball
        *ball_state = BallState::Free;

        // Remove both HeldBy and DribbleState (if present)
        commands
            .entity(ball_entity)
            .remove::<HeldBy>()
            .remove::<DribbleState>()
            .insert((
                RigidBody::Dynamic,
                LinearVelocity(throw_velocity),
                // Add backspin for more realistic basketball throw
                AngularVelocity(Vec3::new(-backspin, 0.0, 0.0)),
            ));

        info!(
            "Threw basketball - charge: {:.0}%, power: {:.1} m/s, pitch: {:.0}°, arc: {:.0}°, total: {:.0}°",
            charge * 100.0,
            throw_power,
            camera_pitch.to_degrees(),
            arc_angle_deg,
            total_angle_rad.to_degrees()
        );
    }

    // Reset charge state after throw
    charge_state.charge_start = None;
    charge_state.charge_level = 0.0;
    charge_state.is_charging = false;
}
