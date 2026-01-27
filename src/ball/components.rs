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
    /// Ball is held by a player (stationary in front of camera)
    Held,
    /// Ball is being dribbled by a player
    Dribbling,
}

/// Tracks which entity is holding the ball
#[derive(Component)]
pub struct HeldBy(pub Entity);

/// Marker component indicating the player has already used their dribble this possession.
/// Once set, the player cannot dribble again until the ball becomes Free.
#[derive(Component)]
pub struct HasDribbled;

/// Which hand is being used for dribbling
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Hand {
    #[default]
    Left,
    Right,
}

impl Hand {
    /// Returns the opposite hand
    pub fn opposite(&self) -> Hand {
        match self {
            Hand::Left => Hand::Right,
            Hand::Right => Hand::Left,
        }
    }

    /// Returns the side offset multiplier (negative for left, positive for right)
    pub fn side_multiplier(&self) -> f32 {
        match self {
            Hand::Left => -1.0,
            Hand::Right => 1.0,
        }
    }
}

/// Phase of the dribble bounce cycle
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DribblePhase {
    /// Ball is moving down towards the ground
    #[default]
    GoingDown,
    /// Ball is bouncing back up
    ComingUp,
    /// Ball is at the peak, waiting for next dribble input
    AtPeak,
}

/// Component tracking the current dribble state
#[derive(Component, Clone, Debug)]
pub struct DribbleState {
    /// Which hand is currently dribbling
    pub current_hand: Hand,
    /// Time of last dribble action (in elapsed seconds)
    pub last_dribble_time: f32,
    /// Current phase of the bounce cycle
    pub phase: DribblePhase,
    /// Progress through current phase (0.0 to 1.0)
    pub phase_progress: f32,
    /// Whether a crossover is in progress
    pub crossover_in_progress: bool,
    /// Target hand for crossover
    pub crossover_target: Hand,
    /// Progress through crossover (0.0 to 1.0)
    pub crossover_progress: f32,
}

impl Default for DribbleState {
    fn default() -> Self {
        Self {
            current_hand: Hand::Right,
            last_dribble_time: 0.0,
            phase: DribblePhase::GoingDown,
            phase_progress: 0.0,
            crossover_in_progress: false,
            crossover_target: Hand::Right,
            crossover_progress: 0.0,
        }
    }
}
