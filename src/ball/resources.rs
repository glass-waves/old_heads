use bevy::prelude::*;

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
