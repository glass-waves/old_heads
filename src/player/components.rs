use bevy::prelude::*;

/// Marker component for the main player entity with physics
#[derive(Component, Default)]
pub struct Player;

/// Tracks horizontal rotation (yaw) for the player
#[derive(Component, Default)]
pub struct PlayerYaw(pub f32);

/// Child entity that handles vertical look (pitch)
/// Attached to camera mount, which holds Camera3d
#[derive(Component, Default)]
pub struct CameraMount {
    pub pitch: f32,
}

/// Accumulated WASD input for the current frame
#[derive(Component, Default)]
pub struct MovementInput {
    pub direction: Vec2,
}

/// Configuration for player movement and controls
#[derive(Component)]
pub struct PlayerConfig {
    pub walk_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            walk_speed: 2.5,       // Slow "old heads" walk speed
            mouse_sensitivity: 0.003,
        }
    }
}
