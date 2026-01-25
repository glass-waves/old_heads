use bevy::prelude::*;

/// Marker component for basketball entities
#[derive(Component, Default)]
pub struct Basketball;

/// Current state of the basketball
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BallState {
    /// Ball is affected by physics (bouncing, rolling)
    #[default]
    Free,
    /// Ball is held by a player
    Held,
}

/// Tracks which entity is holding the ball
#[derive(Component)]
pub struct HeldBy(pub Entity);
