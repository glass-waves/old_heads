use bevy::prelude::*;

/// Curve type for charge-to-power mapping
#[derive(Clone, Copy, Debug, Default)]
pub enum ChargeCurve {
    /// Direct 1:1 mapping (charge = power)
    Linear,
    /// Slow start, fast end (charge²) - current default
    #[default]
    Quadratic,
    /// Fast start, slow end (√charge) - more control at high power
    SquareRoot,
    /// Slow start, fast middle, slow end - best precision at both extremes
    SCurve,
}

impl ChargeCurve {
    /// Apply the curve to a charge value (0.0 to 1.0)
    pub fn apply(&self, charge: f32) -> f32 {
        match self {
            ChargeCurve::Linear => charge,
            ChargeCurve::Quadratic => charge * charge,
            ChargeCurve::SquareRoot => charge.sqrt(),
            ChargeCurve::SCurve => charge * charge * (3.0 - 2.0 * charge),
        }
    }
}

/// Centralized physics configuration for easy tuning
/// All physics-related constants are collected here for experimentation
#[derive(Resource)]
pub struct PhysicsConfig {
    // === Ball Properties ===
    /// Basketball radius in meters
    pub ball_radius: f32,
    /// Bounciness of the ball (0.0 = no bounce, 1.0 = perfect bounce)
    pub ball_restitution: f32,
    /// Friction coefficient of the ball
    pub ball_friction: f32,

    // === Surface Properties ===
    /// Friction coefficient of the floor
    pub floor_friction: f32,
    /// Bounciness of the floor (0.0 = no bounce, 1.0 = perfect bounce)
    pub floor_restitution: f32,

    // === Throw Mechanics ===
    /// Minimum throw power for quick taps (m/s)
    pub min_throw_power: f32,
    /// Maximum throw power at full charge (m/s)
    pub max_throw_power: f32,
    /// Time in seconds to reach full charge
    pub charge_time: f32,
    /// Curve type for charge-to-power mapping
    pub charge_curve: ChargeCurve,
    /// Minimum upward arc angle in degrees (quick tap)
    pub min_arc_angle: f32,
    /// Maximum upward arc angle in degrees (full charge)
    pub max_arc_angle: f32,
    /// Minimum backspin (quick tap)
    pub min_backspin: f32,
    /// Maximum backspin (full charge)
    pub max_backspin: f32,

    // === Dribble Mechanics ===
    /// Time before auto-pickup if no dribble input (seconds)
    pub dribble_timeout: f32,
    /// Minimum time between dribbles (seconds)
    pub dribble_cooldown: f32,
    /// Height of dribble bounce (meters)
    pub dribble_bounce_height: f32,
    /// Horizontal offset for dribble (left/right of player)
    pub dribble_side_offset: f32,
    /// Forward offset for dribble (in front of player)
    pub dribble_forward_offset: f32,
    /// Time to complete a crossover (seconds)
    pub crossover_time: f32,

    // === Pickup/Hold ===
    /// Maximum distance to pick up the ball (meters)
    pub pickup_range: f32,
    /// Offset from camera when holding ball (x=right, y=down, z=forward)
    pub hold_offset: Vec3,

    // === Pass Mechanics ===
    /// Speed of chest pass (m/s)
    pub chest_pass_speed: f32,
    /// Speed of bounce pass (m/s)
    pub bounce_pass_speed: f32,
    /// Speed of lob pass (m/s)
    pub lob_pass_speed: f32,
    /// Upward angle for chest pass (degrees) - slight tilt up
    pub chest_pass_angle: f32,
    /// Downward angle for bounce pass (degrees)
    pub bounce_pass_angle: f32,
    /// Upward angle for lob pass (degrees)
    pub lob_pass_angle: f32,
    /// Camera pitch threshold for bounce pass (forward.y below this = bounce)
    /// Negative value = looking down. -0.25 ≈ looking down 15°
    pub bounce_pass_pitch_threshold: f32,
    /// Camera pitch threshold for lob pass (forward.y above this = lob)
    /// Positive value = looking up. 0.20 ≈ looking up 12°
    pub lob_pass_pitch_threshold: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            // Ball
            ball_radius: 0.12,
            ball_restitution: 0.7,
            ball_friction: 0.8,

            // Surfaces
            floor_friction: 1.0,
            floor_restitution: 0.75,

            // Throw
            min_throw_power: 4.0,
            max_throw_power: 12.0,
            charge_time: 1.0,
            charge_curve: ChargeCurve::Linear,
            min_arc_angle: 20.0,
            max_arc_angle: 50.0,
            min_backspin: 2.0,
            max_backspin: 8.0,

            // Dribble
            dribble_timeout: 0.8,
            dribble_cooldown: 0.25,
            dribble_bounce_height: 0.25,
            dribble_side_offset: 0.35,
            dribble_forward_offset: 0.5,
            crossover_time: 0.3,

            // Pickup/Hold
            pickup_range: 1.5,
            hold_offset: Vec3::new(0.0, -0.3, -0.5),

            // Pass
            chest_pass_speed: 12.0,
            bounce_pass_speed: 10.0,
            lob_pass_speed: 9.0,
            chest_pass_angle: 12.0,   // Upward tilt
            bounce_pass_angle: 30.0,  // Degrees downward
            lob_pass_angle: 45.0,     // Degrees upward
            bounce_pass_pitch_threshold: -0.25, // ~15° down
            lob_pass_pitch_threshold: 0.20,     // ~12° up
        }
    }
}
