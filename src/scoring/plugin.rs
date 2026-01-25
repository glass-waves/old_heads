use bevy::prelude::*;

use super::resources::Score;
use super::systems::{debug_teleport_ball, detect_scoring, spawn_hoop_sensors};

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_hoop_sensors)
            .add_systems(Update, (detect_scoring, debug_teleport_ball));
    }
}
