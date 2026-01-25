use bevy::prelude::*;

use crate::player::resources::{CursorGrabbed, PlayerSettings};
use crate::player::systems::{
    apply_camera_pitch, apply_movement, apply_player_rotation, gather_movement_input,
    handle_cursor_grab, mouse_look, spawn_player,
};

/// Plugin that sets up the first-person player with:
/// - Physics-based kinematic character controller
/// - First-person camera with mouse look
/// - WASD movement
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<PlayerSettings>()
            .init_resource::<CursorGrabbed>()
            // Startup systems
            .add_systems(Startup, spawn_player)
            // Runtime systems
            .add_systems(
                Update,
                (
                    // Input gathering (runs first)
                    handle_cursor_grab,
                    gather_movement_input,
                    mouse_look,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    // Apply transforms (runs after input)
                    apply_movement,
                    apply_player_rotation,
                    apply_camera_pitch,
                )
                    .after(gather_movement_input)
                    .after(mouse_look),
            );
    }
}
