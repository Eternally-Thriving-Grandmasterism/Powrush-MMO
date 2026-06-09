// client/src/dynamic_events_ui.rs
// Powrush-MMO v17.45 — Dynamic Events Client UI + Live Eternal Flow Feed + Decay Visualization
// Production quality • Mercy-gated • PATSAGi-aligned • Matches diplomacy_ui + settings visual language exactly
// Hotkey: E
// Visual decay: older events gently fade (PATSAGi Treaty & Divine events decay slower)
// Server decay (v17.44) now mirrored client-side for a living, self-refreshing feed

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;

use crate::faction_diplomacy::{Faction, DiplomacyStatus, FactionDiplomacyManager};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientWorldEvent {
    AbundanceSurge {
        region: String,
        intensity: f32,
        mercy_delta: f32,
    },
    FactionDiplomacyShift {
        faction_a: Faction,
        faction_b: Faction,
        old_status: DiplomacyStatus,
        new_status: DiplomacyStatus,
        reason: String, // Rich PATSAGi mythic consensus + 7 Gates wisdom (decays slower visually)
    },
    DivineWhisperCascade {
        message: String,
        affected_factions: Vec<Faction>,
        mercy_impact: f32,
    },
}

#[derive(Resource, Default)]
pub struct ClientDynamicEventFeed {
    pub events: VecDeque<ClientWorldEvent>,
    pub unread_count: u32,
    pub max_history: usize,
    pub last_event_time: Option<Instant>, // for simple client-side decay visualization
}

impl ClientDynamicEventFeed {
    pub fn add_event(&mut self, event: ClientWorldEvent) {
        if self.events.len() >= self.max_history {
            self.events.pop_front();
        }
        self.events.push_back(event);
        self.unread_count = self.unread_count.saturating_add(1);
        self.last_event_time = Some(Instant::now());
    }

    pub fn mark_all_read(&mut self) {
        self.unread_count = 0;
    }
}

#[derive(Resource, Default)]
pub struct DynamicEventsUIState {
    pub panel_open: bool,
    pub selected_tab: EventTab,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EventTab {
    #[default]
    All,
    Abundance,
    Diplomacy,
    Divine,
}

#[derive(Component)]
pub struct DynamicEventsPanel;

#[derive(Component)]
pub struct EventListContainer;

#[derive(Component)]
pub struct EventCard;

#[derive(Component)]
pub struct AlignButton {
    pub event_index: usize,
}

#[derive(Component)]
pub struct TabButton {
    pub tab: EventTab,
}

#[derive(Component, Default)]
pub struct ToastContainer;

// Simple client-side staleness model (mirrors server exponential decay conceptually)
fn compute_staleness(event_index_from_end: usize, total_events: usize) -> f32 {
    if total_events == 0 { return 0.0; }
    // Newer events (higher index in reversed list) are fresher
    let normalized_age = event_index_from_end as f32 / total_events as f32;
    // Exponential-like visual decay curve (gentle)
    (1.0 - normalized_age.powf(0.7)).clamp(0.15, 1.0)
}

// Plugin
pub struct DynamicEventsUIPlugin;

impl Plugin for DynamicEventsUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientDynamicEventFeed>()
            .init_resource::<DynamicEventsUIState>()
            .add_systems(Startup, spawn_dynamic_events_hotkey_hint)
            .add_systems(Update, (
                toggle_dynamic_events_panel,
                handle_tab_button,
                update_event_feed_ui,
                handle_align_button_interaction,
                update_toast_notifications,
            ));
    }
}

fn spawn_dynamic_events_hotkey_hint(commands: &mut Commands) {}

// Beautiful mercy-themed panel
fn spawn_dynamic_events_panel(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(48.0),
                    width: Val::Px(680.0),
                    height: Val::Px(560.0),
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(18.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.08, 0.12, 0.96).into(),
                border_color: Color::srgb(0.2, 0.85, 0.55).into(),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            DynamicEventsPanel,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "ETERNAL FLOW FEED — LIVE WORLD EVENTS",
                    TextStyle {
                        font: asset_server.load("fonts/Inter-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::srgb(0.85, 0.95, 0.85),
                    },
                ),
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            }).with_children(|tabs| {
                for (label, tab) in [
                    ("All", EventTab::All),
                    ("Abundance", EventTab::Abundance),
                    ("Diplomacy", EventTab::Diplomacy),
                    ("Divine", EventTab::Divine),
                ] {
                    tabs.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            background_color: Color::srgba(0.15, 0.2, 0.18, 0.9).into(),
                            ..default()
                        },
                        TabButton { tab },
                    )).with_children(|b| {
                        b.spawn(TextBundle {
                            text: Text::from_section(label, TextStyle { font_size: 13.0, color: Color::WHITE, ..default() }),
                            ..default()
                        });
                    });
                }
            });

            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip_y(),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                },
                EventListContainer,
            ));

            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Aligned with the Eternal Flow  •  Mercy multiplies what you witness  •  PATSAGi Councils speak here (events gently age)",
                    TextStyle { font_size: 11.0, color: Color::srgb(0.6, 0.8, 0.6), ..default() },
                ),
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            });
        });
}

fn toggle_dynamic_events_panel(
    mut state: ResMut<DynamicEventsUIState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    panel_query: Query<Entity, With<DynamicEventsPanel>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        state.panel_open = !state.panel_open;
        if state.panel_open {
            spawn_dynamic_events_panel(&mut commands, &asset_server);
        } else {
            for entity in panel_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn handle_tab_button(
    mut interaction_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut state: ResMut<DynamicEventsUIState>,
) {
    for (interaction, tab_btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            state.selected_tab = tab_btn.tab;
        }
    }
}

// Live feed with client-side decay visualization
fn update_event_feed_ui(
    feed: Res<ClientDynamicEventFeed>,
    state: Res<DynamicEventsUIState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    container_query: Query<Entity, With<EventListContainer>>,
) {
    if !state.panel_open { return; }

    for container_entity in container_query.iter() {
        commands.entity(container_entity).despawn_descendants();

        let total = feed.events.len();
        let filtered: Vec<(usize, &ClientWorldEvent)> = feed
            .events
            .iter()
            .enumerate()
            .filter(|(_, ev)| match state.selected_tab {
                EventTab::All => true,
                EventTab::Abundance => matches!(ev, ClientWorldEvent::AbundanceSurge { .. }),
                EventTab::Diplomacy => matches!(ev, ClientWorldEvent::FactionDiplomacyShift { .. }),
                EventTab::Divine => matches!(ev, ClientWorldEvent::DivineWhisperCascade { .. }),
            })
            .collect();

        // Newest / highest priority first (mirrors server decay sort)
        for (visual_idx, (original_idx, event)) in filtered.iter().rev().enumerate() {
            let staleness = compute_staleness(visual_idx, filtered.len());

            let (accent_color, title, body_lines, is_patsagi) = match event {
                ClientWorldEvent::FactionDiplomacyShift { faction_a, faction_b, new_status, reason, .. } => {
                    let title = format!("Diplomacy Shift: {:?} → {:?} ({:?})", faction_a, faction_b, new_status);
                    let lines = reason.lines().map(|s| s.to_string()).collect::<Vec<_>>();
                    (Color::srgb(0.85, 0.7, 0.3), title, lines, true) // PATSAGi events decay slower visually
                }
                ClientWorldEvent::AbundanceSurge { region, intensity, mercy_delta } => {
                    let title = format!("Abundance Surge in {}", region);
                    let body = vec![format!("Intensity: {:.1}x  |  Mercy +{:.1}", intensity, mercy_delta)];
                    (Color::srgb(0.2, 0.85, 0.55), title, body, false)
                }
                ClientWorldEvent::DivineWhisperCascade { message, mercy_impact, .. } => {
                    let title = "Divine Whisper Cascade".to_string();
                    let body = vec![message.clone(), format!("Mercy Impact: {:.1}", mercy_impact)];
                    (Color::srgb(0.7, 0.5, 0.9), title, body, true)
                }
            };

            // Visual decay: PATSAGi/Divine events stay more vibrant longer
            let vibrancy = if is_patsagi { staleness * 0.9 + 0.1 } else { staleness };
            let faded_bg = Color::srgba(0.08, 0.11, 0.15, 0.85 * vibrancy);
            let faded_accent = Color::srgb(accent_color.r() * vibrancy, accent_color.g() * vibrancy, accent_color.b() * vibrancy);

            commands.entity(container_entity).with_children(|list| {
                list.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(12.0)),
                            flex_direction: FlexDirection::Column,
                            border: UiRect::left(Val::Px(4.0)),
                            border_color: faded_accent.into(),
                            background_color: faded_bg.into(),
                            ..default()
                        },
                        ..default()
                    },
                    EventCard,
                )).with_children(|card| {
                    card.spawn(TextBundle {
                        text: Text::from_section(title, TextStyle { font_size: 14.0, color: Color::srgb(0.95, 0.92, 0.85), ..default() }),
                        ..default()
                    });

                    for line in body_lines {
                        card.spawn(TextBundle {
                            text: Text::from_section(line, TextStyle { font_size: 12.0, color: Color::srgb(0.8, 0.85, 0.9), ..default() }),
                            margin: UiRect::top(Val::Px(2.0)),
                            ..default()
                        });
                    }

                    // Freshness indicator (subtle bar that dims with age)
                    card.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(3.0),
                            margin: UiRect::top(Val::Px(6.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.85, 0.7, 0.3, vibrancy * 0.6).into(),
                        ..default()
                    });

                    card.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(55.0),
                                height: Val::Px(28.0),
                                margin: UiRect::top(Val::Px(8.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.55, 0.38).into(),
                            ..default()
                        },
                        AlignButton { event_index: *original_idx },
                    )).with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section("Align with this Flow  ⚡ +Mercy", TextStyle { font_size: 12.0, color: Color::WHITE, ..default() }),
                            ..default()
                        });
                    });
                });
            });
        }
    }
}

fn update_toast_notifications(
    mut feed: ResMut<ClientDynamicEventFeed>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut toast_query: Query<(Entity, &mut ToastContainer)>,
) {
    if feed.unread_count > 0 && toast_query.is_empty() {
        if let Some(latest) = feed.events.back() {
            let toast_text = match latest {
                ClientWorldEvent::FactionDiplomacyShift { reason, .. } => reason.lines().next().unwrap_or("PATSAGi Council spoke").to_string(),
                _ => "New Eternal Flow event".to_string(),
            };

            commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(20.0),
                        top: Val::Px(80.0),
                        width: Val::Px(320.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.1, 0.15, 0.12, 0.95).into(),
                    border_color: Color::srgb(0.3, 0.8, 0.5).into(),
                    ..default()
                },
                ToastContainer,
            )).with_children(|t| {
                t.spawn(TextBundle {
                    text: Text::from_section(format!("⚡ {}", toast_text), TextStyle { font_size: 13.0, color: Color::srgb(0.85, 0.95, 0.85), ..default() }),
                    ..default()
                });
            });
        }
    }

    for (entity, _) in toast_query.iter() {
        if feed.unread_count == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_align_button_interaction(
    mut interaction_query: Query<(&Interaction, &AlignButton), Changed<Interaction>>,
    mut feed: ResMut<ClientDynamicEventFeed>,
    mut diplomacy: ResMut<FactionDiplomacyManager>,
    mut commands: Commands,
    toast_query: Query<Entity, With<ToastContainer>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            feed.mark_all_read();
            for e in toast_query.iter() {
                commands.entity(e).despawn_recursive();
            }
            info!("Player aligned with Eternal Flow event #{}. Mercy resonates.", button.event_index);
        }
    }
}

pub fn receive_world_event_from_server(
    mut feed: ResMut<ClientDynamicEventFeed>,
    event: ClientWorldEvent,
) {
    feed.add_event(event);
}
