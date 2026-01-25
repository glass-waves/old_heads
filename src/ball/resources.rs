use bevy::prelude::*;

/// Settings for ball pickup and throw mechanics
#[derive(Resource)]
pub struct BallSettings {
    /// Maximum distance to pick up the ball (meters)
    pub pickup_range: f32,
    /// Offset from camera when holding ball (forward, down, right)
    pub hold_offset: Vec3,
    /// Minimum throw power for quick taps (meters/second)
    pub min_throw_power: f32,
    /// Maximum throw power at full charge (meters/second)
    pub max_throw_power: f32,
    /// Time in seconds to reach full charge
    pub charge_time: f32,
}

impl Default for BallSettings {
    fn default() -> Self {
        Self {
            pickup_range: 1.5,
            hold_offset: Vec3::new(0.0, -0.3, -0.5), // In front and below camera
            min_throw_power: 3.0,  // Weak toss for quick taps
            max_throw_power: 12.0, // Full power shot
            charge_time: 1.5,      // 1.5 seconds to max charge
        }
    }
}

/// Tracks the current shooting charge state
#[derive(Resource, Default)]
pub struct ShootingChargeState {
    /// Time when charging started (in seconds since app start)
    pub charge_start: Option<f32>,
    /// Current charge level from 0.0 to 1.0
    pub charge_level: f32,
    /// Whether currently charging a shot
    pub is_charging: bool,
}
