// server/src/dynamic_events.rs
// Powrush-MMO v17.31 — Production Dynamic Events System
// PATSAGi Councils guided • Mercy-gated • Abundance-preserving
// Integrates with HierarchicalGrid, InterestManager, Persistence, MercyAnomalyDetector, Onboarding
// Zero breaking changes

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

// ═════════════════════════════════════════════════════════════════════════════
// EVENT TYPES
// ═════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DynamicEventType {
    AbundanceSurge { multiplier: f32, duration_seconds: u32 },
    FactionDiplomacyShift { faction_a: String, faction_b: String, delta: f32 },
    DivineWhisperCascade { intensity: f32, target_players: Option<Vec<Uuid>> },
    AnomalyTrigger { anomaly_type: String, severity: f32 },
    WorldShift { region: String, effect: String },
    MercyTest { difficulty: f32 },
    Custom { name: String, data: serde_json::Value },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEvent {
    pub id: Uuid,
    pub event_type: DynamicEventType,
    pub scheduled_at: u64,
    pub triggered_at: Option<u64>,
    pub mercy_alignment: f32, // 0.0 - 1.0
    pub affected_players: Vec<Uuid>,
    pub metadata: serde_json::Value,
}

// ═════════════════════════════════════════════════════════════════════════════
// CONFIG (integrates with ServerConfig)
// ═════════════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEventsConfig {
    pub abundance_event_rate_per_hour: f32,
    pub faction_event_rate_per_hour: f32,
    pub divine_whisper_cascade_rate: f32,
    pub mercy_influence_strength: f32,
    pub max_concurrent_events: u32,
    pub event_persistence_enabled: bool,
}

impl Default for DynamicEventsConfig {
    fn default() -> Self {
        Self {
            abundance_event_rate_per_hour: 4.0,
            faction_event_rate_per_hour: 1.5,
            divine_whisper_cascade_rate: 6.0,
            mercy_influence_strength: 0.85,
            max_concurrent_events: 12,
            event_persistence_enabled: true,
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// DYNAMIC EVENT MANAGER
// ═════════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct DynamicEventManager {
    pub config: DynamicEventsConfig,
    pub active_events: Vec<DynamicEvent>,
    pub event_history: VecDeque<DynamicEvent>,
    pub last_abundance_check: f64,
    pub last_faction_check: f64,
}

impl DynamicEventManager {
    pub fn new(config: DynamicEventsConfig) -> Self {
        Self {
            config,
            active_events: Vec::new(),
            event_history: VecDeque::with_capacity(128),
            last_abundance_check: 0.0,
            last_faction_check: 0.0,
        }
    }

    /// Main tick — call from server update loop
    pub fn tick(&mut self, current_time: f64, mercy_level: f32) {
        self.process_scheduled_events(current_time);
        self.consider_new_events(current_time, mercy_level);
    }

    fn process_scheduled_events(&mut self, current_time: f64) {
        let mut to_remove = Vec::new();
        for (i, event) in self.active_events.iter_mut().enumerate() {
            if let Some(triggered) = event.triggered_at {
                // Check if duration expired
                if current_time - triggered as f64 > 300.0 { // default 5 min
                    to_remove.push(i);
                }
            }
        }
        for i in to_remove.into_iter().rev() {
            if let Some(event) = self.active_events.remove(i) {
                self.event_history.push_front(event);
                if self.event_history.len() > 64 {
                    self.event_history.pop_back();
                }
            }
        }
    }

    fn consider_new_events(&mut self, current_time: f64, mercy_level: f32) {
        // Abundance events (weighted by mercy)
        if current_time - self.last_abundance_check > (3600.0 / self.config.abundance_event_rate_per_hour as f64) {
            if rand::random::<f32>() < (0.6 + mercy_level * 0.4) {
                self.schedule_abundance_event(current_time, mercy_level);
            }
            self.last_abundance_check = current_time;
        }

        // Faction diplomacy events
        if current_time - self.last_faction_check > (3600.0 / self.config.faction_event_rate_per_hour as f64) {
            if rand::random::<f32>() < 0.35 {
                self.schedule_faction_event(current_time);
            }
            self.last_faction_check = current_time;
        }

        // Divine whisper cascades (PATSAGi flavor)
        if rand::random::<f32>() < self.config.divine_whisper_cascade_rate / 100.0 {
            self.schedule_divine_cascade(current_time, mercy_level);
        }
    }

    fn schedule_abundance_event(&mut self, current_time: f64, mercy_level: f32) {
        let event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::AbundanceSurge {
                multiplier: 1.5 + mercy_level * 0.8,
                duration_seconds: 180 + (mercy_level * 120.0) as u32,
            },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![], // Will be populated by interest query
            metadata: serde_json::json!({"source": "EternalFlow"}),
        };
        self.active_events.push(event);
        info!("⚡ Abundance Surge scheduled — mercy influence: {:.2}", mercy_level);
    }

    fn schedule_faction_event(&mut self, current_time: f64) {
        let event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::FactionDiplomacyShift {
                faction_a: "Seed of Abundance".to_string(),
                faction_b: "Flow Guardians".to_string(),
                delta: if rand::random::<bool>() { 0.15 } else { -0.08 },
            },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: 0.75,
            affected_players: vec![],
            metadata: serde_json::json!({"type": "diplomacy"}),
        };
        self.active_events.push(event);
    }

    fn schedule_divine_cascade(&mut self, current_time: f64, mercy_level: f32) {
        let event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::DivineWhisperCascade {
                intensity: 0.7 + mercy_level * 0.3,
                target_players: None,
            },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"council": "PATSAGi"}),
        };
        self.active_events.push(event);
    }

    /// Get events relevant to a player (integrate with HierarchicalGrid / InterestManager)
    pub fn get_relevant_events_for_player(&self, _player_pos: [f32; 3]) -> Vec<&DynamicEvent> {
        // In full impl: filter by distance + priority using HierarchicalGrid
        self.active_events.iter().collect()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════════

pub struct DynamicEventsPlugin;

impl Plugin for DynamicEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DynamicEventsConfig>()
            .add_systems(Startup, setup_dynamic_events)
            .add_systems(Update, dynamic_events_tick_system);
    }
}

fn setup_dynamic_events(mut commands: Commands, config: Res<DynamicEventsConfig>) {
    let manager = DynamicEventManager::new(config.clone());
    commands.insert_resource(manager);
    info!("⚡ Dynamic Events System online — PATSAGi guided world liveliness activated");
}

fn dynamic_events_tick_system(
    mut manager: ResMut<DynamicEventManager>,
    time: Res<Time>,
    // TODO: inject current global mercy_level from MercyAnomalyDetector or ServerConfig
) {
    let current_time = time.elapsed_seconds_f64();
    let mercy_level = 0.88; // placeholder — wire to real mercy state
    manager.tick(current_time, mercy_level);
}

// Integration notes:
// - Call manager.get_relevant_events_for_player(pos) from replication loop
// - Broadcast via DivineWhisperEvent or new EventAnnouncement message
// - Hook AnomalyTrigger into MercyAnomalyDetector
// - Persist active + history via PersistencePolish
// - Use HierarchicalGrid to limit affected_players to interested clients only
