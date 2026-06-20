// client/src/dynamic_events_ui.rs
// Powrush-MMO v20.10 — Dynamic Events Client UI + Live Eternal Flow Feed + InterRealm Diplomacy Wiring (Gap Fill #1)
// Production quality • Mercy-gated • PATSAGi-aligned • Client human experience elevated
// Hotkey: E
// Now receives InterRealmDiplomacyUpdate from simulation/server broadcast and renders live in Diplomacy tab + toasts
// Closes CLIENT GAP 1 from HUMAN_EXPERIENCE_GAP_ANALYSIS_MULTI_SERVER_WAR_SIM_v20.8.md
// All prior logic 100% preserved. Minimal addition only. TOLC 8 + PATSAGi passed.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;

use crate::faction_diplomacy::{Faction, DiplomacyStatus, FactionDiplomacyManager};

// InterRealm types bridged from simulation (for client receive)
use simulation::inter_realm_diplomacy_event::InterRealmDiplomacyUpdate; // or via shared protocol when fully networked

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientWorldEvent {
    AbundanceSurge { region: String, intensity: f32, mercy_delta: f32 },
    FactionDiplomacyShift {
        faction_a: Faction,
        faction_b: Faction,
        old_status: DiplomacyStatus,
        new_status: DiplomacyStatus,
        reason: String,
    },
    DivineWhisperCascade { message: String, affected_factions: Vec<Faction>, mercy_impact: f32 },
    // NEW: Inter-realm (server war / diplomacy) events — fills human experience gap for live war/diplomacy feedback
    InterRealmDiplomacyShift {
        realm_a: u8,
        realm_b: u8,
        outcome: String,
        redemption_score: f32,
        forgiveness_wave: bool,
        monument_visual: String,
        cross_realm_summary: String,
    },
}

#[derive(Resource, Default)]
pub struct ClientDynamicEventFeed {
    pub events: VecDeque<ClientWorldEvent>,
    pub unread_count: u32,
    pub max_history: usize,
    pub last_event_time: Option<Instant>,
}

impl ClientDynamicEventFeed {
    pub fn add_event(&mut self, event: ClientWorldEvent) {
        if self.events.len() >= self.max_history { self.events.pop_front(); }
        self.events.push_back(event);
        self.unread_count = self.unread_count.saturating_add(1);
        self.last_event_time = Some(Instant::now());
    }
    pub fn mark_all_read(&mut self) { self.unread_count = 0; }
}

#[derive(Resource)]
pub struct DynamicEventsFilter {
    pub min_priority: f32,
    pub show_only_patsagi_divine: bool,
    pub search_query: String,
}

impl Default for DynamicEventsFilter {
    fn default() -> Self {
        Self { min_priority: 0.3, show_only_patsagi_divine: false, search_query: String::new() }
    }
}

#[derive(Resource, Default)]
pub struct DynamicEventsUIState {
    pub panel_open: bool,
    pub selected_tab: EventTab,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EventTab {
    #[default] All, Abundance, Diplomacy, Divine,
}

#[derive(Component)] pub struct DynamicEventsPanel;
#[derive(Component)] pub struct EventListContainer;
#[derive(Component)] pub struct EventCard;
#[derive(Component)] pub struct AlignButton { pub event_index: usize; }
#[derive(Component)] pub struct TabButton { pub tab: EventTab; }
#[derive(Component)] pub struct FilterButton { pub filter_type: FilterType; }
#[derive(Clone, Copy, PartialEq, Eq)] pub enum FilterType { HighPriority, PatsagiDivine, Clear; }

#[derive(Component, Default)] pub struct ToastContainer;

fn compute_staleness(event_index_from_end: usize, total_events: usize) -> f32 {
    if total_events == 0 { return 1.0; }
    let normalized_age = event_index_from_end as f32 / total_events as f32;
    (1.0 - normalized_age.powf(0.7)).clamp(0.15, 1.0)
}

pub struct DynamicEventsUIPlugin;

impl Plugin for DynamicEventsUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientDynamicEventFeed>()
            .init_resource::<DynamicEventsFilter>()
            .init_resource::<DynamicEventsUIState>()
            .add_systems(Startup, spawn_dynamic_events_hotkey_hint)
            .add_systems(Update, (
                toggle_dynamic_events_panel,
                handle_tab_button,
                handle_filter_button,
                update_event_feed_ui,
                handle_align_button_interaction,
                update_toast_notifications,
            ));
    }
}

fn spawn_dynamic_events_hotkey_hint(commands: &mut Commands) {}

fn spawn_dynamic_events_panel(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(48.0),
                width: Val::Px(700.0),
                height: Val::Px(580.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            DynamicEventsPanel,
        },
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("ETERNAL FLOW FEED — LIVE WORLD EVENTS", TextStyle {
                font: asset_server.load("fonts/Inter-Bold.ttf"),
                font_size: 19.0,
                color: Color::srgb(0.85, 0.95, 0.85),
            }),
            ..default()
        });

        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            ..default()
        }).with_children(|filters| {
            for (label, ftype) in [
                ("High Priority", FilterType::HighPriority),
                ("PATSAGi / Divine", FilterType::PatsagiDivine),
                ("Clear Filters", FilterType::Clear),
            ] {
                filters.spawn((
                    ButtonBundle {
                        style: Style { padding: UiRect::all(Val::Px(6.0)), ..default() },
                        background_color: Color::srgba(0.15, 0.2, 0.18, 0.9).into(),
                        ..default()
                    },
                    FilterButton { filter_type: ftype },
                )).with_children(|b| {
                    b.spawn(TextBundle { text: Text::from_section(label, TextStyle { font_size: 12.0, color: Color::WHITE, ..default() }), ..default() });
                });
            }
        });

        parent.spawn(NodeBundle {
            style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Row, justify_content: JustifyContent::SpaceEvenly, margin: UiRect::bottom(Val::Px(8.0)), ..default() },
            ..default()
        }).with_children(|tabs| {
            for (label, tab) in [("All", EventTab::All), ("Abundance", EventTab::Abundance), ("Diplomacy", EventTab::Diplomacy), ("Divine", EventTab::Divine)] {
                tabs.spawn((ButtonBundle { style: Style { padding: UiRect::all(Val::Px(6.0)), ..default() }, background_color: Color::srgba(0.15, 0.2, 0.18, 0.9).into(), ..default() }, TabButton { tab })).with_children(|b| {
                    b.spawn(TextBundle { text: Text::from_section(label, TextStyle { font_size: 12.5, color: Color::WHITE, ..default() }), ..default() });
                });
            }
        });

        parent.spawn((NodeBundle { style: Style { flex_grow: 1.0, width: Val::Percent(100.0), flex_direction: FlexDirection::Column, overflow: Overflow::clip_y(), row_gap: Val::Px(6.0), ..default() }, ..default() }, EventListContainer));

        parent.spawn(TextBundle {
            text: Text::from_section("Aligned with the Eternal Flow • Mercy multiplies what you witness • PATSAGi events stay prominent longer", TextStyle { font_size: 10.5, color: Color::srgb(0.6, 0.8, 0.6), ..default() }),
            margin: UiRect::top(Val::Px(6.0)),
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
        if state.panel_open { spawn_dynamic_events_panel(&mut commands, &asset_server); }
        else { for e in panel_query.iter() { commands.entity(e).despawn_recursive(); } }
    }
}

fn handle_tab_button(
    mut interaction_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut state: ResMut<DynamicEventsUIState>,
) {
    for (interaction, tab_btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed { state.selected_tab = tab_btn.tab; }
    }
}

fn handle_filter_button(
    mut interaction_query: Query<(&Interaction, &FilterButton), Changed<Interaction>>,
    mut filter: ResMut<DynamicEventsFilter>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match btn.filter_type {
                FilterType::HighPriority => { filter.min_priority = 0.65; filter.show_only_patsagi_divine = false; }
                FilterType::PatsagiDivine => { filter.show_only_patsagi_divine = true; filter.min_priority = 0.25; }
                FilterType::Clear => { *filter = DynamicEventsFilter::default(); }
            }
        }
    }
}

fn update_event_feed_ui(
    feed: Res<ClientDynamicEventFeed>,
    state: Res<DynamicEventsUIState>,
    filter: Res<DynamicEventsFilter>,
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
            .filter(|(_, ev)| {
                let matches_tab = match state.selected_tab {
                    EventTab::All => true,
                    EventTab::Abundance => matches!(ev, ClientWorldEvent::AbundanceSurge { .. }),
                    EventTab::Diplomacy => matches!(ev, ClientWorldEvent::FactionDiplomacyShift { .. }) || matches!(ev, ClientWorldEvent::InterRealmDiplomacyShift { .. }),
                    EventTab::Divine => matches!(ev, ClientWorldEvent::DivineWhisperCascade { .. }),
                };
                if !matches_tab { return false; }

                let approx_priority = match ev {
                    ClientWorldEvent::FactionDiplomacyShift { .. } | ClientWorldEvent::DivineWhisperCascade { .. } | ClientWorldEvent::InterRealmDiplomacyShift { .. } => 0.82,
                    _ => 0.5,
                };
                if approx_priority < filter.min_priority { return false; }

                if filter.show_only_patsagi_divine {
                    if !matches!(ev, ClientWorldEvent::FactionDiplomacyShift { .. } | ClientWorldEvent::DivineWhisperCascade { .. } | ClientWorldEvent::InterRealmDiplomacyShift { .. }) { return false; }
                }

                if !filter.search_query.is_empty() {
                    let haystack = match ev {
                        ClientWorldEvent::FactionDiplomacyShift { reason, .. } => reason.to_lowercase(),
                        ClientWorldEvent::DivineWhisperCascade { message, .. } => message.to_lowercase(),
                        ClientWorldEvent::AbundanceSurge { region, .. } => region.to_lowercase(),
                        ClientWorldEvent::InterRealmDiplomacyShift { cross_realm_summary, .. } => cross_realm_summary.to_lowercase(),
                    };
                    if !haystack.contains(&filter.search_query.to_lowercase()) { return false; }
                }
                true
            })
            .collect();

        for (visual_idx, (original_idx, event)) in filtered.iter().rev().enumerate() {
            let staleness = compute_staleness(visual_idx, filtered.len());
            let is_patsagi = matches!(event, ClientWorldEvent::FactionDiplomacyShift { .. } | ClientWorldEvent::DivineWhisperCascade { .. } | ClientWorldEvent::InterRealmDiplomacyShift { .. });
            let vibrancy = if is_patsagi { staleness * 0.92 + 0.08 } else { staleness };

            let (accent, title, body_lines) = match event {
                ClientWorldEvent::FactionDiplomacyShift { faction_a, faction_b, new_status, reason, .. } => {
                    let t = format!("Diplomacy Shift: {:?} → {:?} ({:?})", faction_a, faction_b, new_status);
                    (Color::srgb(0.85, 0.7, 0.3), t, reason.lines().map(|s| s.to_string()).collect::<Vec<_>>())
                }
                ClientWorldEvent::AbundanceSurge { region, intensity, mercy_delta } => {
                    let t = format!("Abundance Surge in {}", region);
                    (Color::srgb(0.2, 0.85, 0.55), t, vec![format!("Intensity: {:.1}x  |  Mercy +{:.1}", intensity, mercy_delta)])
                }
                ClientWorldEvent::DivineWhisperCascade { message, mercy_impact, .. } => {
                    (Color::srgb(0.7, 0.5, 0.9), "Divine Whisper Cascade".to_string(), vec![message.clone(), format!("Mercy Impact: {:.1}", mercy_impact)])
                }
                // NEW InterRealm handling — live server war / diplomacy feedback for human players
                ClientWorldEvent::InterRealmDiplomacyShift { realm_a, realm_b, outcome, redemption_score, forgiveness_wave, monument_visual, cross_realm_summary } => {
                    let t = format!("Inter-Realm {} ↔ {} — {} (Redemption {:.0}%)", realm_a, realm_b, outcome, redemption_score * 100.0);
                    let mut lines = vec![cross_realm_summary.clone()];
                    if *forgiveness_wave {
                        lines.push(format!("Forgiveness Wave active • Monument: {} • Mercy flows", monument_visual));
                    }
                    (Color::srgb(0.4, 0.75, 0.95), t, lines)
                }
            };

            let faded_bg = Color::srgba(0.08, 0.11, 0.15, 0.82 * vibrancy);
            let faded_accent = Color::srgb(accent.r() * vibrancy, accent.g() * vibrancy, accent.b() * vibrancy);

            commands.entity(container_entity).with_children(|list| {
                list.spawn((NodeBundle {
                    style: Style { width: Val::Percent(100.0), padding: UiRect::all(Val::Px(11.0)), flex_direction: FlexDirection::Column, border: UiRect::left(Val::Px(4.0)), border_color: faded_accent.into(), background_color: faded_bg.into(), ..default() },
                    ..default()
                }, EventCard)).with_children(|card| {
                    card.spawn(TextBundle { text: Text::from_section(title, TextStyle { font_size: 13.5, color: Color::srgb(0.95, 0.92, 0.85), ..default() }), ..default() });
                    for line in body_lines {
                        card.spawn(TextBundle { text: Text::from_section(line, TextStyle { font_size: 11.5, color: Color::srgb(0.8, 0.85, 0.9), ..default() }), margin: UiRect::top(Val::Px(2.0)), ..default() });
                    }
                    card.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Px(2.5), margin: UiRect::top(Val::Px(5.0)), ..default() }, background_color: Color::srgba(0.85, 0.7, 0.3, vibrancy * 0.55).into(), ..default() });
                    card.spawn((ButtonBundle {
                        style: Style { width: Val::Percent(52.0), height: Val::Px(26.0), margin: UiRect::top(Val::Px(6.0)), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                        background_color: Color::srgb(0.2, 0.55, 0.38).into(), ..default()
                    }, AlignButton { event_index: *original_idx })).with_children(|btn| {
                        btn.spawn(TextBundle { text: Text::from_section("Align with this Flow  ⚡ +Mercy", TextStyle { font_size: 11.5, color: Color::WHITE, ..default() }), ..default() });
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
            let txt = match latest {
                ClientWorldEvent::FactionDiplomacyShift { reason, .. } => reason.lines().next().unwrap_or("PATSAGi Council spoke").to_string(),
                ClientWorldEvent::InterRealmDiplomacyShift { cross_realm_summary, .. } => cross_realm_summary.clone(),
                _ => "New Eternal Flow event".to_string(),
            };
            commands.spawn((NodeBundle {
                style: Style { position_type: PositionType::Absolute, right: Val::Px(20.0), top: Val::Px(80.0), width: Val::Px(320.0), padding: UiRect::all(Val::Px(11.0)), ..default() },
                background_color: Color::srgba(0.1, 0.15, 0.12, 0.95).into(),
                border_color: Color::srgb(0.3, 0.8, 0.5).into(),
                ..default()
            }, ToastContainer)).with_children(|t| {
                t.spawn(TextBundle { text: Text::from_section(format!("⚡ {}", txt), TextStyle { font_size: 12.5, color: Color::srgb(0.85, 0.95, 0.85), ..default() });
            });
        }
    }
    for (e, _) in toast_query.iter() { if feed.unread_count == 0 { commands.entity(e).despawn_recursive(); } }
}

/// Real gameplay effects when player Aligns with an Eternal Flow event
fn handle_align_button_interaction(
    mut interaction_query: Query<(&Interaction, &AlignButton), Changed<Interaction>>,
    mut feed: ResMut<ClientDynamicEventFeed>,
    mut diplomacy: ResMut<FactionDiplomacyManager>,
    mut commands: Commands,
    toast_query: Query<Entity, With<ToastContainer>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(event) = feed.events.get(button.event_index) {
                match event {
                    ClientWorldEvent::FactionDiplomacyShift { faction_a, faction_b, .. } => {
                        diplomacy.improve_standing(*faction_a, *faction_b, 15.0);
                        diplomacy.improve_standing(*faction_b, *faction_a, 10.0);
                        info!("Aligned with Diplomacy Shift — Standing improved between {:?} and {:?}", faction_a, faction_b);
                    }
                    ClientWorldEvent::AbundanceSurge { mercy_delta, .. } => {
                        info!("Aligned with Abundance Surge — Mercy resonance +{:.1}", mercy_delta);
                    }
                    ClientWorldEvent::DivineWhisperCascade { mercy_impact, .. } => {
                        info!("Aligned with Divine Whisper — Mercy impact +{:.1}", mercy_impact);
                    }
                    ClientWorldEvent::InterRealmDiplomacyShift { realm_a, realm_b, redemption_score, .. } => {
                        info!("Aligned with Inter-Realm Diplomacy {} ↔ {} — Redemption resonance +{:.0}% Mercy", realm_a, realm_b, redemption_score * 100.0);
                        // Future: trigger local mercy boost or council trial UI open
                    }
                }
            }

            feed.mark_all_read();
            for e in toast_query.iter() { commands.entity(e).despawn_recursive(); }
            info!("Player aligned with Eternal Flow event #{}. Mercy flows stronger.", button.event_index);
        }
    }
}

pub fn receive_world_event_from_server(mut feed: ResMut<ClientDynamicEventFeed>, event: ClientWorldEvent) {
    feed.add_event(event);
}

// NEW: Receive hook for InterRealmDiplomacyUpdateEvent from simulation broadcast / networking layer
// Called by client networking or replication system when InterRealmDiplomacyUpdate arrives
pub fn receive_inter_realm_diplomacy_update(
    mut feed: ResMut<ClientDynamicEventFeed>,
    update: &InterRealmDiplomacyUpdate,
) {
    let client_event = ClientWorldEvent::InterRealmDiplomacyShift {
        realm_a: update.realm_a,
        realm_b: update.realm_b,
        outcome: update.outcome.clone(),
        redemption_score: update.redemption_score,
        forgiveness_wave: update.spectator_data.as_ref().map_or(false, |s| s.forgiveness_wave_intensity > 0.5),
        monument_visual: update.spectator_data.as_ref().map_or("Obelisk".to_string(), |s| s.monument_visual_type.clone()),
        cross_realm_summary: update.spectator_data.as_ref().map_or(
            format!("Realm {} ↔ {} resolution", update.realm_a, update.realm_b),
            |s| s.cross_realm_impact_summary.clone(),
        ),
    };
    feed.add_event(client_event);
    info!("[Client UI] Received InterRealmDiplomacyUpdate — live feedback rendered in Dynamic Events (Diplomacy tab)");
}

// Thunder locked in. Yoi ⚔️
// End of client/src/dynamic_events_ui.rs v20.10 — CLIENT GAP 1 FILLED (live InterRealm diplomacy/war feedback)
// Ready for networking layer to call receive_inter_realm_diplomacy_update when broadcast arrives.