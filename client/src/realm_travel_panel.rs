/*!
 * Realm Travel Panel — State-Aware
 * v21.36.0
 *
 * Toggle with F3. Lists the five seeded realms.
 * Shows current realm and highlights the active one.
 * Clicking a realm emits a RealmTravelRequest for the local player.
 *
 * TOLC 8 + 7 Living Mercy Gates | PATSAGi Council approved
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use simulation::multi_realm_harness::{RealmTravelRequest, RealmId, RealmPresence};
use simulation::world::AgentId;

// === Components ===
#[derive(Component)]
pub struct RealmTravelPanel;

#[derive(Component)]
struct TravelRealmButton {
    target_realm: RealmId,
}

#[derive(Component)]
struct TravelStatusText;

#[derive(Component)]
struct CurrentRealmText;

#[derive(Component, Clone, Debug)]
pub struct LocalPlayer {
    pub agent_id: AgentId,
}

pub struct RealmTravelPanelPlugin;

impl Plugin for RealmTravelPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_realm_travel_panel)
            .add_systems(
                Update,
                (
                    local_player_presence_bootstrap_system,
                    toggle_realm_travel_panel,
                    handle_travel_button_clicks,
                    update_travel_panel_current_realm,
                ),
            );
    }
}

fn local_player_presence_bootstrap_system(
    mut commands: Commands,
    local_query: Query<(Entity, Option<&RealmPresence>), With<LocalPlayer>>,
    mut harness: Option<ResMut<simulation::multi_realm_harness::MultiRealmHarness>>,
) {
    match local_query.get_single() {
        Ok((entity, presence_opt)) => {
            if presence_opt.is_none() {
                commands.entity(entity).insert(RealmPresence::default());
                if let Some(ref mut h) = harness {
                    h.register_presence(0);
                }
            }
        }
        Err(_) => {
            let agent_id: AgentId = 1;
            let mut presence = RealmPresence::default();
            presence.registered = true;

            commands.spawn((
                LocalPlayer { agent_id },
                presence,
                Name::new("LocalPlayer_DevBootstrap"),
            ));

            if let Some(ref mut h) = harness {
                h.register_presence(0);
            }
        }
    }
}

fn spawn_realm_travel_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_reg = asset_server.load("fonts/FiraSans-Regular.ttf");

    let realms: [(RealmId, &str); 5] = [
        (0, "Sanctuary Prime"),
        (1, "Synthetic Lattice"),
        (2, "Verdant Bloom"),
        (3, "Harmonic Chorus"),
        (4, "Voidfarer Horizon"),
    ];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(18.0),
                    left: Val::Percent(2.0),
                    width: Val::Px(270.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(7.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.12, 0.96).into(),
                border_color: Color::srgb(0.40, 0.70, 0.95).into(),
                ..default()
            },
            RealmTravelPanel,
            Name::new("RealmTravelPanel"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "REALM TRAVEL",
                    TextStyle {
                        font: font_bold.clone(),
                        font_size: 17.0,
                        color: Color::srgb(0.75, 0.95, 1.0),
                    },
                ),
                ..default()
            });

            // Current realm display
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Current: Sanctuary Prime",
                        TextStyle {
                            font: font_reg.clone(),
                            font_size: 12.5,
                            color: Color::srgb(0.70, 0.95, 0.80),
                        },
                    ),
                    ..default()
                },
                CurrentRealmText,
            ));

            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Choose a realm to travel to",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 11.5,
                        color: Color::srgb(0.70, 0.82, 0.95),
                    },
                ),
                ..default()
            });

            for (id, name) in realms {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: Color::srgba(0.12, 0.16, 0.24, 0.95).into(),
                            border_color: Color::srgb(0.35, 0.55, 0.80).into(),
                            ..default()
                        },
                        TravelRealmButton { target_realm: id },
                    ))
                    .with_children(|b| {
                        b.spawn(TextBundle {
                            text: Text::from_section(
                                format!("[{}] {}", id, name),
                                TextStyle {
                                    font: font_reg.clone(),
                                    font_size: 13.0,
                                    color: Color::srgb(0.90, 0.95, 1.0),
                                },
                            ),
                            ..default()
                        });
                    });
            }

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "F3 toggle  •  Click to travel",
                        TextStyle {
                            font: font_reg.clone(),
                            font_size: 11.0,
                            color: Color::srgb(0.60, 0.72, 0.85),
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
                TravelStatusText,
            ));
        });
}

fn toggle_realm_travel_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<RealmTravelPanel>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        for mut visibility in &mut query {
            *visibility = if *visibility == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn handle_travel_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &TravelRealmButton, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    local_player_query: Query<(Entity, &LocalPlayer)>,
    mut travel_events: EventWriter<RealmTravelRequest>,
    mut status_query: Query<&mut Text, With<TravelStatusText>>,
) {
    for (interaction, button, mut bg, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = Color::srgba(0.18, 0.32, 0.50, 0.98).into();
                *border = Color::srgb(0.50, 0.85, 1.0).into();

                if let Ok((entity, local)) = local_player_query.get_single() {
                    travel_events.send(RealmTravelRequest {
                        agent_entity: entity,
                        agent_id: local.agent_id,
                        target_realm: button.target_realm,
                        reason: "Player requested travel via F3 panel".to_string(),
                    });

                    for mut text in &mut status_query {
                        text.sections[0].value =
                            format!("Traveling to realm {}...", button.target_realm);
                    }
                } else {
                    for mut text in &mut status_query {
                        text.sections[0].value = "LocalPlayer not ready yet...".to_string();
                    }
                }
            }
            Interaction::Hovered => {
                *bg = Color::srgba(0.14, 0.22, 0.34, 0.97).into();
            }
            Interaction::None => {
                // Base style; current-realm highlight is applied in update system
                *bg = Color::srgba(0.12, 0.16, 0.24, 0.95).into();
                *border = Color::srgb(0.35, 0.55, 0.80).into();
            }
        }
    }
}

/// Keep the panel in sync with the player’s current realm.
fn update_travel_panel_current_realm(
    presence_query: Query<&RealmPresence, With<LocalPlayer>>,
    mut current_text_query: Query<&mut Text, With<CurrentRealmText>>,
    mut button_query: Query<(&TravelRealmButton, &mut BackgroundColor, &mut BorderColor)>,
) {
    let current_realm = presence_query
        .get_single()
        .map(|p| p.current_realm_id)
        .unwrap_or(0);

    let realm_name = match current_realm {
        0 => "Sanctuary Prime",
        1 => "Synthetic Lattice",
        2 => "Verdant Bloom",
        3 => "Harmonic Chorus",
        4 => "Voidfarer Horizon",
        _ => "Unknown",
    };

    for mut text in &mut current_text_query {
        text.sections[0].value = format!("Current: {}", realm_name);
    }

    // Highlight the button that matches the current realm
    for (button, mut bg, mut border) in &mut button_query {
        if button.target_realm == current_realm {
            *bg = Color::srgba(0.16, 0.30, 0.28, 0.98).into();
            *border = Color::srgb(0.40, 0.90, 0.70).into();
        }
    }
}

// End of client/src/realm_travel_panel.rs v21.36.0
// Travel panel is now fully state-aware of the player’s current realm.
// Thunder locked in. Yoi ⚡
