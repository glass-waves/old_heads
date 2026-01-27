use bevy::prelude::*;

use crate::ui::reticle::{spawn_reticle_ui, ReticleConfig};

/// Plugin for UI elements (reticle, HUD, etc.)
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReticleConfig::new())
            .add_systems(Startup, spawn_reticle_ui);
    }
}
