use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;

mod ball;
mod player;
mod scoring;

use ball::{BallPlugin, BallState, Basketball};
use player::PlayerPlugin;
use scoring::ScoringPlugin;

fn spawn_light(mut commands: Commands) {
    let light = DirectionalLight {
        illuminance: 5000.0,
        ..default()
    };
    let light_transform = Transform::from_xyz(3.0, 8.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((light, light_transform));
}

fn spawn_court(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Visual court model with automatic colliders for fence
    commands.spawn((
        SceneRoot(asset_server.load("street_basketball_court.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, -5.0),
        RigidBody::Static,
        // Auto-generate colliders from mesh hierarchy (fences, hoops, etc.)
        // Exclude net rope and rim meshes so ball can pass through hoops
        // Note: without_constructor_for_name uses exact matching on the actual mesh entities
        // (children of Hoop_Net/Hoop_Rim, not the parent containers)
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh)
            // Net rope meshes (under Hoop_Net and Hoop.001_Net)
            .without_constructor_for_name("Cube.001.rope.003")
            .without_constructor_for_name("Cube.001.rope.002")
            .without_constructor_for_name("Cube.002.rope.003")
            .without_constructor_for_name("Cube.002.rope.002")
            // Rim meshes (under Hoop_Rim and Hoop.001_Rim)
            .without_constructor_for_name("Cube.003.M_Hoop")
            .without_constructor_for_name("Cube.004.M_Hoop"),
    ));

    // Floor collider - invisible physics surface (more reliable than mesh-based)
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 30.0),
        Transform::from_xyz(0.0, -0.05, -5.0),
        Friction::new(0.8),
    ));
}

fn spawn_basketball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create shared mesh and material handles once
    let mesh_handle = meshes.add(Sphere::new(0.12));
    let material_handle = materials.add(Color::srgb(0.9, 0.4, 0.1));

    // Spawn 50 balls in a grid pattern
    for i in 0..50 {
        let row = i / 10;
        let col = i % 10;
        let x = (col as f32 - 4.5) * 0.4; // Spread across X
        let y = 2.0 + (row as f32 * 0.5);  // Stack vertically with spacing
        let z = 0.0;

        commands.spawn((
            Basketball,
            BallState::Free,
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            Transform::from_xyz(x, y, z),
            RigidBody::Dynamic,
            Collider::sphere(0.12),
            Restitution::new(0.85),
            Friction::new(0.5),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BrpExtrasPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(ScoringPlugin)
        .add_systems(Startup, (spawn_light, spawn_court, spawn_basketball))
        .run();
}
