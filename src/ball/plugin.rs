use bevy::prelude::*;

use crate::ball::resources::{BallSettings, ShootingChargeState};
use crate::ball::systems::{
    ball_hold_position, ball_pickup, ball_throw, shot_charge_cancel, shot_charge_camera_feedback,
    shot_charge_start, shot_charge_update,
};

/// Plugin for basketball pickup, hold, and throw mechanics
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallSettings>()
            .init_resource::<ShootingChargeState>()
            .add_systems(
                Update,
                (
                    ball_pickup,
                    shot_charge_start,
                    shot_charge_update,
                    shot_charge_cancel,
                    ball_throw,
                    ball_hold_position,
                    shot_charge_camera_feedback,
                )
                    .chain(),
            );
    }
}
