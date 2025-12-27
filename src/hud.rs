use bevy::prelude::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, minimap_hud_system);
    }
}

fn minimap_hud_system(
    mut commands: Commands,
    lattice: Res<LatticeStats>,
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
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Lattice Map",
            TextStyle { font_size: 24.0, color: Color::GOLD, ..default() },
        ));
        parent.spawn(TextBundle::from_section(
            format!("Nodes: {}", lattice.nodes),
            TextStyle { font_size: 20.0, color: Color::CYAN, ..default() },
        ));
        // Procedural dots for players/lattice
    });
}
