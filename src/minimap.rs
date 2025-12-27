use bevy::prelude::*;

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, minimap_ui_system);
    }
}

fn minimap_ui_system(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Minimap",
            TextStyle { font_size: 24.0, color: Color::CYAN, ..default() },
        ));
        // Dots for players
        for i in 0..5 {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GOLD,
                    custom_size: Some(Vec2::splat(6.0)),
                    ..default()
                },
                transform: Transform::from_xyz(i as f32 * 30.0 - 75.0, 0.0, 0.0),
                ..default()
            });
        }
    });
}
