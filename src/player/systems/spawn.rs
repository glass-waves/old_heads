use avian3d::prelude::*;
use bevy::prelude::*;

use crate::player::{CameraMount, MovementInput, Player, PlayerConfig, PlayerYaw};

/// Spawns the player entity hierarchy:
/// Player (physics, yaw rotation)
/// └── CameraMount (pitch rotation only)
///     └── Camera3d
pub fn spawn_player(mut commands: Commands) {
    // Spawn player entity with physics and nested hierarchy using with_children
    commands
        .spawn((
            Player,
            PlayerYaw::default(),
            PlayerConfig::default(),
            MovementInput::default(),
            Transform::from_xyz(0.0, 1.0, 5.0), // Start position
            Visibility::default(),
            // Physics components
            RigidBody::Kinematic,
            Collider::capsule(0.3, 0.8), // Radius 0.3m, height 0.8m (total ~1.4m)
            LockedAxes::ROTATION_LOCKED, // Prevent physics from rotating player
            LinearVelocity::default(),
        ))
        .with_children(|player_children| {
            // Spawn camera mount as child of player
            player_children
                .spawn((
                    CameraMount::default(),
                    Transform::from_xyz(0.0, 0.6, 0.0), // Eye level offset from player center
                    Visibility::default(),
                ))
                .with_children(|camera_mount_children| {
                    // Spawn camera as child of camera mount
                    camera_mount_children.spawn((Camera3d::default(), Transform::default()));
                });
        });
}
