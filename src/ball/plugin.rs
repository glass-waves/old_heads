use bevy::prelude::*;

use crate::ball::resources::ShootingChargeState;
use crate::ball::systems::{
    ball_hold_position, ball_pickup, ball_throw, clear_has_dribbled_on_free,
    dribble_physics_update, dribble_start, dribble_update, pass_input, shot_charge_cancel,
    shot_charge_indicator, shot_charge_start, shot_charge_update,
};

/// Plugin for basketball pickup, hold, dribble, and throw mechanics
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShootingChargeState>().add_systems(
            Update,
            (
                // Input processing
                ball_pickup,
                dribble_start,
                pass_input,  // Before dribble_update so passes work while dribbling
                dribble_update,
                shot_charge_start,
                // Throw must run before charge_update so just_released() is checked
                // before the fallback reset in charge_update clears is_charging
                ball_throw,
                shot_charge_update,
                shot_charge_cancel,
                // State cleanup
                clear_has_dribbled_on_free,
                // Position updates
                ball_hold_position,
                dribble_physics_update,
                // Visual feedback
                shot_charge_indicator,
            )
                .chain(),
        );
    }
}
