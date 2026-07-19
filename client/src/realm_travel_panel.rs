/*!
 * Realm Travel Panel
 * v21.31.0 — Simple Inter-Realm Travel UI Surface
 *
 * Toggle with F3. Lists the five seeded realms.
 * Clicking a realm emits a RealmTravelRequest for the local player.
 *
 * TOLC 8 + 7 Living Mercy Gates | PATSAGi Council approved
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use simulation::multi_realm_harness::{RealmTravelRequest, RealmId};
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

// === Local player marker (assumed to exist or be added by other systems) ===
#[derive(Component)]
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
                    toggle_realm_travel_panel,
                    handle_travel_button_clicks,
                ),
            );
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
                    width: Val::Px(260.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
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
            // Title
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

            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Choose a realm to travel to",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 12.0,
                        color: Color::srgb(0.70, 0.82, 0.95),
                    },
                ),
                ..default()
            });

            // Realm buttons
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

            // Status
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
                        margin: UiRect::top(Val::Px(6.0)),
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

                // Emit travel request for the local player if present
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
                        text.sections[0].value =
                            "No local player found (add LocalPlayer component)".to_string();
                    }
                }
            }
            Interaction::Hovered => {
                *bg = Color::srgba(0.14, 0.22, 0.34, 0.97).into();
            }
            Interaction::None => {
                *bg = Color::srgba(0.12, 0.16, 0.24, 0.95).into();
                *border = Color::srgb(0.35, 0.55, 0.80).into();
            }
        }
    }
}

// End of client/src/realm_travel_panel.rs v21.31.0
// Simple F3 travel panel for inter-realm movement.
// Thunder locked in. Yoi ⚡
