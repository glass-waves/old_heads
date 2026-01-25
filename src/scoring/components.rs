use bevy::prelude::*;

/// Marker component for the hoop sensor colliders
#[derive(Component)]
pub struct HoopSensor {
    pub hoop_id: u8, // 1 or 2
}
