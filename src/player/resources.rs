use bevy::prelude::*;

/// Runtime player settings that can be modified
#[derive(Resource)]
pub struct PlayerSettings {
    pub invert_y: bool,
    pub sensitivity_multiplier: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            invert_y: false,
            sensitivity_multiplier: 1.0,
        }
    }
}

/// Tracks whether cursor is currently grabbed
#[derive(Resource, Default)]
pub struct CursorGrabbed(pub bool);
