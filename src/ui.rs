use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
           .add_systems(Update, update_ui);
    }
}

fn spawn_ui(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Trust: ??? | Lattice: ???",
            TextStyle {
                font_size: 24.0,
                color: Color::GOLD,
                ..default()
            },
        ));
    });
}

fn update_ui() {
    // Update from resources
}
