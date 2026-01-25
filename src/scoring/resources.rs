use bevy::prelude::*;

/// Tracks the score for each team
#[derive(Resource, Default)]
pub struct Score {
    pub team1: u32,
    pub team2: u32,
}
