use bevy::prelude::*;

pub struct UITooltipPlugin;

impl Plugin for UITooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_tooltip);
    }
}

fn spawn_tooltip(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(200.0),
                top: Val::Px(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.2, 0.9)),
            ..default()
        },
        TextBundle::from_section(
            "Mercy active — share to thrive",
            TextStyle { font_size: 32.0, color: Color::CYAN, ..default() },
        ),
    ));
}
