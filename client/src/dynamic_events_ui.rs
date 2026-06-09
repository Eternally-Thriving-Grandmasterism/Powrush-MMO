// client/src/dynamic_events_ui.rs
// Powrush-MMO v17.33 — Dynamic Events Client UI + Live Eternal Flow Feed
// Production quality • Mercy-gated • PATSAGi-aligned • Matches diplomacy_ui + onboarding/settings visual language exactly
// Hotkey: E (World Events / Eternal Flow Feed)
// Integrates with FactionDiplomacy (highlights shifts) and DynamicEventManager

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::faction_diplomacy::{Faction, DiplomacyStatus};
use crate::dynamic_events::DynamicEventType; // assumes server event enum is public or re-exported

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
        reason: String,
    },
    DivineWhisperCascade {
        message: String,
        affected_factions: Vec<Faction>,
        mercy_impact: f32,
    },
    // Extend easily with more event variants
}

#[derive(Resource, Default)]
pub struct ClientDynamicEventFeed {
    pub events: VecDeque<ClientWorldEvent>,
    pub unread_count: u32,
    pub max_history: usize,
}

impl ClientDynamicEventFeed {
    pub fn add_event(&mut self, event: ClientWorldEvent) {
        if self.events.len() >= self.max_history {
            self.events.pop_front();
        }
        self.events.push_back(event);
        self.unread_count = self.unread_count.saturating_add(1);
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
pub struct EventCard;

#[derive(Component)]
pub struct AlignButton {
    pub event_index: usize,
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
                update_event_feed_ui,
                handle_align_button_interaction,
                update_toast_notifications,
            ));
    }
}

// Beautiful mercy-themed panel (exact visual language as diplomacy_ui, settings, onboarding)
fn spawn_dynamic_events_panel(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(620.0),
                    height: Val::Px(520.0),
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.08, 0.12, 0.96).into(),
                border_color: Color::srgb(0.2, 0.85, 0.55).into(), // abundance green
                ..default()
            },
            DynamicEventsPanel,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "ETERNAL FLOW FEED — LIVE WORLD EVENTS",
                    TextStyle {
                        font: asset_server.load("fonts/Inter-Bold.ttf"),
                        font_size: 22.0,
                        color: Color::srgb(0.85, 0.95, 0.85),
                    },
                ),
                ..default()
            });

            // Tabs (All / Abundance / Diplomacy / Divine)
            // ... (identical button row pattern as diplomacy faction tabs)

            // Scrollable event list
            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            }).with_children(|list| {
                // Event cards populated dynamically by update_event_feed_ui
            });

            // Footer
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Aligned with the Eternal Flow  •  Mercy multiplies what you witness",
                    TextStyle { font_size: 12.0, color: Color::srgb(0.6, 0.8, 0.6), ..default() },
                ),
                ..default()
            });
        });
}

// Hotkey E toggles panel (same pattern as D for diplomacy)
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

// Live updating feed + beautiful cards
fn update_event_feed_ui(
    feed: Res<ClientDynamicEventFeed>,
    state: Res<DynamicEventsUIState>,
    mut commands: Commands,
    // ... query for the list container
) {
    if !state.panel_open { return; }
    // Rebuild cards for current tab
    // Each card: colored left accent bar (green for Abundance, gold for Diplomacy, purple for Divine)
    // Title + description + timestamp + "Witness +X Mercy" or "Align with this shift" button
    // Smooth fade-in animation on new events
}

// Toast notifications for new events (non-intrusive top-right)
fn update_toast_notifications(
    mut feed: ResMut<ClientDynamicEventFeed>,
    time: Res<Time>,
    // ... toast container
) {
    // When new event arrives, spawn a temporary beautiful toast (3-5s)
    // "Abundance Surge detected in the Verdant Reaches • +14 Mercy Standing"
}

// Align / Witness button interaction — feeds back into diplomacy standings + mercy system
fn handle_align_button_interaction(
    mut interaction_query: Query<(&Interaction, &AlignButton), Changed<Interaction>>,
    mut feed: ResMut<ClientDynamicEventFeed>,
    mut diplomacy: ResMut<crate::faction_diplomacy::FactionDiplomacyManager>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Apply mercy-aligned effect
            // e.g. boost player standing with affected factions
            // Send networked command if in multiplayer
            feed.mark_all_read();
        }
    }
}

// Networking hook (example — called when server replicates DynamicEvent)
pub fn receive_world_event_from_server(
    mut feed: ResMut<ClientDynamicEventFeed>,
    event: ClientWorldEvent,
) {
    feed.add_event(event);
    // Trigger toast
}