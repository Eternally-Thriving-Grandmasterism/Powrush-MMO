use bevy::prelude::*;
use bevy::input::mouse::{MouseButton, MouseMotion};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            interactive_tooltip_system,
            inventory_hover_system,
            trading_button_system,
        ));
    }
}

fn interactive_tooltip_system(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(pos) = window.cursor_position() {
                commands.spawn((
                    TextBundle::from_section(
                        "Mercy click — lattice responds",
                        TextStyle { font_size: 32.0, color: Color::GOLD, ..default() },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(pos.x),
                        top: Val::Px(pos.y),
                        ..default()
                    }),
                ));
            }
        }
    }
}

fn inventory_hover_system(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    query: Query<&Inventory>,
) {
    if mouse.pressed(MouseButton::Right) {
        if let Ok(inv) = query.get_single() {
            commands.spawn(TextBundle::from_section(
                format!("Inventory: {} items", inv.items.len()),
                TextStyle { font_size: 28.0, color: Color::CYAN, ..default() },
            ));
        }
    }
}

fn trading_button_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::T) {
        commands.spawn(ButtonBundle {
            style: Style { padding: UiRect::all(Val::Px(20.0)), ..default() },
            background_color: BackgroundColor(Color::GOLD),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Open Trade",
                TextStyle { font_size: 36.0, color: Color::BLACK, ..default() },
            ));
        });
    }
}
