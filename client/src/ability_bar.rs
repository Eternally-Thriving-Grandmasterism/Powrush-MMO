// client/src/ability_bar.rs
// Powrush-MMO v17.60 — Client Side Ability Bar + Server Input Wiring
// Updated to prepare ability inputs for server transmission

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct AbilitySlot {
    pub slot_index: usize,
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
    pub is_on_gcd: bool,
}

#[derive(Resource, Default)]
pub struct PlayerAbilities {
    pub abilities: Vec<AbilitySlot>,
}

#[derive(Component)]
pub struct AbilityBar;

#[derive(Component)]
pub struct AbilitySlotUI;

pub struct AbilityBarPlugin;

impl Plugin for AbilityBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAbilities>()
            .add_systems(Startup, spawn_ability_bar)
            .add_systems(Update, (
                update_ability_cooldowns,
                update_ability_bar_ui,
                handle_ability_input,
            ));
    }
}

fn spawn_ability_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Percent(50.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.12, 0.1, 0.85).into(),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        AbilityBar,
    )).with_children(|parent| {
        for i in 0..4 {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.25, 0.2).into(),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                AbilitySlotUI,
                AbilitySlot {
                    slot_index: i,
                    ability_id: (i + 1) as u32,
                    cooldown_remaining: 0.0,
                    max_cooldown: 3.0,
                    is_on_gcd: false,
                },
            )).with_children(|slot| {
                slot.spawn(TextBundle {
                    text: Text::from_section(
                        format!("{}", i + 1),
                        TextStyle {
                            font: asset_server.load("fonts/Inter-Bold.ttf"),
                            font_size: 14.0,
                            color: Color::srgb(0.9, 0.95, 0.9),
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(4.0),
                        left: Val::Px(6.0),
                        ..default()
                    },
                    ..default()
                });

                slot.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                        ..default()
                    },
                    CooldownOverlay { slot_index: i },
                ));
            });
        }
    });
}

#[derive(Component)]
struct CooldownOverlay {
    slot_index: usize,
}

fn update_ability_cooldowns(
    time: Res<Time>,
    mut abilities: ResMut<PlayerAbilities>,
) {
    let delta = time.delta_seconds();
    for ability in &mut abilities.abilities {
        if ability.cooldown_remaining > 0.0 {
            ability.cooldown_remaining -= delta;
            if ability.cooldown_remaining < 0.0 {
                ability.cooldown_remaining = 0.0;
            }
        }
    }
}

fn update_ability_bar_ui(
    abilities: Res<PlayerAbilities>,
    mut overlay_query: Query<(&mut BackgroundColor, &CooldownOverlay)>,
) {
    for (mut bg_color, overlay) in overlay_query.iter_mut() {
        if let Some(ability) = abilities.abilities.get(overlay.slot_index) {
            let progress = if ability.max_cooldown > 0.0 {
                (ability.cooldown_remaining / ability.max_cooldown).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let alpha = progress * 0.75;
            *bg_color = Color::srgba(0.05, 0.05, 0.05, alpha).into();
        }
    }
}

/// Handles keyboard input and prepares ability use for server transmission
fn handle_ability_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut abilities: ResMut<PlayerAbilities>,
    // In a full networked game, you would have an EventWriter or networking channel here
) {
    for i in 0..4 {
        let key = match i {
            0 => KeyCode::Digit1,
            1 => KeyCode::Digit2,
            2 => KeyCode::Digit3,
            3 => KeyCode::Digit4,
            _ => continue,
        };

        if keyboard.just_pressed(key) {
            if let Some(ability) = abilities.abilities.get_mut(i) {
                if ability.cooldown_remaining <= 0.0 {
                    // === CLIENT -> SERVER WIRING POINT ===
                    // Here we would send an AbilityUse message to the server.
                    // Example: commands.trigger(AbilityUseEvent { slot_index: i, ability_id: ability.ability_id });
                    // Or send via your networking transport (TokioTransport, etc.)

                    println!("[CLIENT] Requesting ability use: slot {} (id: {}) -> Sending to server", i, ability.ability_id);

                    // Local simulation (remove/replace when full server sync is active)
                    ability.cooldown_remaining = ability.max_cooldown;
                } else {
                    println!("Ability on cooldown");
                }
            }
        }
    }
}

// Notes:
// - Server now has AbilityUseEvent + handle_ability_use_requests (v17.60)
// - Client should eventually send inputs through the networking layer to trigger that event on server.
// - This file is structured to make that integration straightforward.
