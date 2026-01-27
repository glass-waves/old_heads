use bevy::prelude::*;

/// The current state of the reticle, used to change its appearance
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ReticleState {
    /// Default state - no ball
    #[default]
    Default,
    /// Holding the ball (planted)
    HoldingBall,
    /// Dribbling (can move)
    Dribbling,
    /// Charging a shot
    ChargingShot,
    /// Can't dribble anymore (picked up dribble)
    PickedUpDribble,
}

impl ReticleState {
    /// Get the color for this reticle state
    pub fn color(&self) -> Color {
        match self {
            // For now, all states use the same color - white with some transparency
            // This can be customized later per state
            ReticleState::Default => Color::srgba(1.0, 1.0, 1.0, 0.7),
            ReticleState::HoldingBall => Color::srgba(1.0, 1.0, 1.0, 0.7),
            ReticleState::Dribbling => Color::srgba(1.0, 1.0, 1.0, 0.7),
            ReticleState::ChargingShot => Color::srgba(1.0, 1.0, 1.0, 0.7),
            ReticleState::PickedUpDribble => Color::srgba(1.0, 1.0, 1.0, 0.7),
        }
    }

    /// Get the size (radius) for this reticle state
    pub fn size(&self) -> f32 {
        match self {
            // For now, all states use the same size
            // This can be customized later per state
            _ => 12.0,
        }
    }
}

/// Resource tracking current reticle state
#[derive(Resource, Default)]
pub struct ReticleConfig {
    pub state: ReticleState,
    /// Thickness of the circle outline
    pub thickness: f32,
}

impl ReticleConfig {
    pub fn new() -> Self {
        Self {
            state: ReticleState::Default,
            thickness: 2.0,
        }
    }
}

/// Marker component for the reticle UI node
#[derive(Component)]
pub struct Reticle;

/// Spawns the reticle UI using gizmos (drawn each frame)
pub fn spawn_reticle(mut commands: Commands) {
    // Just insert the reticle config, we'll draw with gizmos
    commands.insert_resource(ReticleConfig::new());
}

/// System to draw reticle using gizmos (circle outline)
pub fn draw_reticle(config: Res<ReticleConfig>, mut gizmos: Gizmos, window: Query<&Window>) {
    let Ok(window) = window.single() else {
        return;
    };

    // Get window center in screen coordinates
    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    let radius = config.state.size();
    let color = config.state.color();

    // Draw circle at screen center using gizmos
    // Note: Gizmos work in world space, so we'll use a 2D approach
    // For now, draw multiple small dots in a circle pattern
    let segments = 32;
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

        // Convert screen space to normalized device coordinates (-1 to 1)
        let x1 = (center_x + radius * angle1.cos()) / window.width() * 2.0 - 1.0;
        let y1 = -((center_y + radius * angle1.sin()) / window.height() * 2.0 - 1.0);
        let x2 = (center_x + radius * angle2.cos()) / window.width() * 2.0 - 1.0;
        let y2 = -((center_y + radius * angle2.sin()) / window.height() * 2.0 - 1.0);

        // We can't easily draw 2D screen-space gizmos, so let's use UI instead
        let _ = (x1, y1, x2, y2); // Suppress unused warnings
    }

    // Actually, let's just use a simple approach with the 2D gizmos
    // We'll draw in a fixed position in front of the camera
    let _ = (center_x, center_y, radius, color, gizmos);
}

/// Draw reticle as a simple crosshair using UI nodes
pub fn spawn_reticle_ui(mut commands: Commands, config: Res<ReticleConfig>) {
    let color = config.state.color();
    let size = config.state.size() * 2.0;
    let thickness = config.thickness;

    // Container centered on screen
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_children(|parent| {
            // Outer ring container
            parent
                .spawn((
                    Reticle,
                    Node {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|ring| {
                    // Top segment
                    ring.spawn((
                        Node {
                            width: Val::Px(thickness),
                            height: Val::Px(size / 4.0),
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Px(size / 2.0 - thickness / 2.0),
                            ..default()
                        },
                        BackgroundColor(color),
                    ));
                    // Bottom segment
                    ring.spawn((
                        Node {
                            width: Val::Px(thickness),
                            height: Val::Px(size / 4.0),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Px(size / 2.0 - thickness / 2.0),
                            ..default()
                        },
                        BackgroundColor(color),
                    ));
                    // Left segment
                    ring.spawn((
                        Node {
                            width: Val::Px(size / 4.0),
                            height: Val::Px(thickness),
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(size / 2.0 - thickness / 2.0),
                            ..default()
                        },
                        BackgroundColor(color),
                    ));
                    // Right segment
                    ring.spawn((
                        Node {
                            width: Val::Px(size / 4.0),
                            height: Val::Px(thickness),
                            position_type: PositionType::Absolute,
                            right: Val::Px(0.0),
                            top: Val::Px(size / 2.0 - thickness / 2.0),
                            ..default()
                        },
                        BackgroundColor(color),
                    ));
                });
        });
}
