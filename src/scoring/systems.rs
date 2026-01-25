use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ball::Basketball;

use super::components::HoopSensor;
use super::resources::Score;

/// Hoop positions in Bevy world coordinates
/// Based on actual rim centers from Blender (Hoop_Rim and Hoop.001_Rim objects)
/// Blender X → Bevy X (same), Blender Z → Bevy Y, Blender Y → Bevy Z = -Y
/// Court offset: -5 on Z axis
/// Hoop_Rim: Blender (-0.23, -10.89, 3.04) → Bevy (-0.23, 3.04, 10.89 - 5) = (-0.23, 3.04, 5.89)
/// Hoop.001_Rim: Blender (-0.22, 10.95, 3.04) → Bevy (-0.22, 3.04, -10.95 - 5) = (-0.22, 3.04, -15.95)
const HOOP_1_POS: Vec3 = Vec3::new(-0.23, 3.04, 5.89);
const HOOP_2_POS: Vec3 = Vec3::new(-0.22, 3.04, -15.95);

/// Sensor dimensions - short cylinder matching rim radius
const SENSOR_HEIGHT: f32 = 0.05;
const SENSOR_RADIUS: f32 = 0.22;

/// Spawns invisible sensor colliders just below each hoop rim
pub fn spawn_hoop_sensors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create debug visualization mesh (shared between both sensors)
    let debug_mesh = meshes.add(Cylinder::new(SENSOR_RADIUS, SENSOR_HEIGHT));
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.0, 0.5), // Semi-transparent green
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Sensor for Hoop 1 (far end)
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(SENSOR_RADIUS, SENSOR_HEIGHT),
        Transform::from_translation(HOOP_1_POS),
        Sensor,
        CollisionEventsEnabled,
        HoopSensor { hoop_id: 1 },
        // Debug visualization
        Mesh3d(debug_mesh.clone()),
        MeshMaterial3d(debug_material.clone()),
    ));

    // Sensor for Hoop 2 (near end)
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(SENSOR_RADIUS, SENSOR_HEIGHT),
        Transform::from_translation(HOOP_2_POS),
        Sensor,
        CollisionEventsEnabled,
        HoopSensor { hoop_id: 2 },
        // Debug visualization
        Mesh3d(debug_mesh.clone()),
        MeshMaterial3d(debug_material.clone()),
    ));

    info!("Hoop sensors spawned at {:?} and {:?}", HOOP_1_POS, HOOP_2_POS);
}

/// Debug: Press T to teleport a ball above hoop 1, Y for hoop 2
pub fn debug_teleport_ball(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ball_query: Query<(&mut Transform, &mut LinearVelocity), With<Basketball>>,
) {
    let target_pos = if keyboard.just_pressed(KeyCode::KeyT) {
        Some(HOOP_1_POS + Vec3::Y * 1.0) // 1m above hoop 1
    } else if keyboard.just_pressed(KeyCode::KeyY) {
        Some(HOOP_2_POS + Vec3::Y * 1.0) // 1m above hoop 2
    } else {
        None
    };

    if let Some(pos) = target_pos {
        // Teleport the first ball we find
        if let Some((mut transform, mut velocity)) = ball_query.iter_mut().next() {
            transform.translation = pos;
            velocity.0 = Vec3::ZERO; // Reset velocity so it drops straight down
            info!("Teleported ball to {:?}", pos);
        }
    }
}

/// Detects when the basketball passes through a hoop sensor and increments score
pub fn detect_scoring(
    mut collision_reader: MessageReader<CollisionStart>,
    sensor_query: Query<&HoopSensor>,
    ball_query: Query<Entity, With<Basketball>>,
    mut score: ResMut<Score>,
) {
    for event in collision_reader.read() {
        // Check if one entity is a sensor and one is the ball
        let (sensor_entity, _ball_entity) =
            if sensor_query.contains(event.collider1) && ball_query.contains(event.collider2) {
                (event.collider1, event.collider2)
            } else if sensor_query.contains(event.collider2) && ball_query.contains(event.collider1)
            {
                (event.collider2, event.collider1)
            } else {
                continue;
            };

        if let Ok(sensor) = sensor_query.get(sensor_entity) {
            match sensor.hoop_id {
                1 => {
                    score.team1 += 2;
                    info!(
                        "SCORE! Team 1 basket - Team 1: {} | Team 2: {}",
                        score.team1, score.team2
                    );
                }
                2 => {
                    score.team2 += 2;
                    info!(
                        "SCORE! Team 2 basket - Team 1: {} | Team 2: {}",
                        score.team1, score.team2
                    );
                }
                _ => {}
            }
        }
    }
}
