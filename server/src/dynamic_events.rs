// server/src/dynamic_events.rs
// Powrush-MMO v17.49 — Dynamic Events + Decay Rate Tuning
// PATSAGi Councils guided • 7 Living Mercy Gates aware
// Fully tunable decay rates via DynamicEventsConfig (type multipliers, mercy influence, floors, boost decay)
// Enables fine control over how quickly events fade in the Eternal Flow Feed

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

// ═════════════════════════════════════════════════════════════════════════
// EVENT TYPES
// ═════════════════════════════════════════════════════════════════════════

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
    pub mercy_alignment: f32,
    pub affected_players: Vec<Uuid>,
    pub metadata: serde_json::Value,
    pub priority: f32,
    pub base_priority: f32,
    pub priority_boost: f32,
}

impl DynamicEvent {
    pub fn compute_base_priority(&self) -> f32 {
        let type_priority = match &self.event_type {
            DynamicEventType::DivineWhisperCascade { intensity, .. } => 0.93 + intensity.clamp(0.0, 0.07),
            DynamicEventType::FactionDiplomacyShift { .. } => 0.83 + (self.mercy_alignment * 0.14),
            DynamicEventType::AbundanceSurge { multiplier, .. } => 0.54 + (multiplier - 1.0).clamp(0.0, 0.26) + (self.mercy_alignment * 0.11),
            DynamicEventType::AnomalyTrigger { severity, .. } => 0.68 + severity.clamp(0.0, 0.22),
            DynamicEventType::WorldShift { .. } => 0.64,
            DynamicEventType::MercyTest { difficulty, .. } => 0.59 + difficulty.clamp(0.0, 0.26),
            DynamicEventType::Custom { .. } => 0.38,
        };
        type_priority.clamp(0.1, 1.4)
    }

    pub fn apply_boost(&mut self, amount: f32) {
        self.priority_boost = (self.priority_boost + amount.max(0.0)).min(1.5);
        self.priority = (self.priority + amount.max(0.0)).min(2.0);
    }

    /// Tunable decay using config-driven multipliers and mercy influence.
    pub fn current_priority(&self, current_time: f64, config: &DynamicEventsConfig) -> f32 {
        if let Some(triggered) = self.triggered_at {
            let age = current_time - triggered as f64;
            if age <= 0.0 { return self.base_priority + self.priority_boost; }

            // Tunable type-specific multipliers
            let type_mult = match &self.event_type {
                DynamicEventType::DivineWhisperCascade { .. } => config.divine_half_life_multiplier,
                DynamicEventType::FactionDiplomacyShift { .. } => config.patsagi_treaty_half_life_multiplier,
                DynamicEventType::AbundanceSurge { .. } => 1.0,
                _ => config.generic_half_life_multiplier,
            };

            // Tunable mercy influence on decay speed
            let mercy_mult = 1.0 + (self.mercy_alignment * config.mercy_half_life_influence);

            let effective_half_life = config.priority_half_life_seconds * type_mult * mercy_mult;

            let decay_factor = 2f64.powf(-age / effective_half_life as f64);
            let decayed = (self.base_priority as f64 * decay_factor).max(config.min_priority_floor as f64) as f32;

            // Optional gentle decay on boosts themselves
            let remaining_boost = self.priority_boost * (1.0 - (age as f32 / (config.boost_decay_half_life * 2.0)).min(1.0));

            (decayed + remaining_boost.max(0.0)).min(2.0)
        } else {
            self.base_priority + self.priority_boost
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// CONFIG - Fully tunable decay rates
// ═════════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEventsConfig {
    pub abundance_event_rate_per_hour: f32,
    pub faction_event_rate_per_hour: f32,
    pub divine_whisper_cascade_rate: f32,
    pub mercy_influence_strength: f32,
    pub max_concurrent_events: u32,
    pub event_persistence_enabled: bool,

    // === DECAY RATE TUNING ===
    pub priority_half_life_seconds: f32,           // Base half-life (higher = slower global decay)
    pub divine_half_life_multiplier: f32,          // >1.0 = Divine events persist much longer
    pub patsagi_treaty_half_life_multiplier: f32,  // >1.0 = Treaty council events persist longer
    pub generic_half_life_multiplier: f32,
    pub mercy_half_life_influence: f32,            // How strongly mercy_alignment slows decay (0.0 = no effect)
    pub min_priority_floor: f32,                   // Never decay below this floor
    pub boost_decay_half_life: f32,                // How fast priority_boost itself fades (0 = permanent boosts)
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

            // Tuned defaults for balanced mythic feel
            priority_half_life_seconds: 165.0,
            divine_half_life_multiplier: 1.65,
            patsagi_treaty_half_life_multiplier: 1.45,
            generic_half_life_multiplier: 1.0,
            mercy_half_life_influence: 0.8,
            min_priority_floor: 0.04,
            boost_decay_half_life: 240.0, // boosts last ~4 minutes by default
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// DYNAMIC EVENT MANAGER
// ═════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct DynamicEventManager {
    pub config: DynamicEventsConfig,
    pub active_events: Vec<DynamicEvent>,
    pub event_history: VecDeque<DynamicEvent>,
    pub last_abundance_check: f64,
    pub last_faction_check: f64,
    pub pending_replication: Vec<DynamicEvent>,
}

impl DynamicEventManager {
    pub fn new(config: DynamicEventsConfig) -> Self {
        Self { config, active_events: Vec::new(), event_history: VecDeque::with_capacity(128), last_abundance_check: 0.0, last_faction_check: 0.0, pending_replication: Vec::new() }
    }

    pub fn tick(&mut self, current_time: f64, mercy_level: f32) {
        self.process_scheduled_events(current_time);
        self.consider_new_events(current_time, mercy_level);
        self.apply_priority_decay(current_time);
    }

    fn apply_priority_decay(&mut self, current_time: f64) {
        for event in self.active_events.iter_mut() {
            event.priority = event.current_priority(current_time, &self.config);
        }
        for event in self.pending_replication.iter_mut() {
            event.priority = event.current_priority(current_time, &self.config);
        }
    }

    fn process_scheduled_events(&mut self, current_time: f64) { /* ... */ }
    fn consider_new_events(&mut self, current_time: f64, mercy_level: f32) { /* ... */ }

    fn schedule_abundance_event(&mut self, current_time: f64, mercy_level: f32) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::AbundanceSurge { multiplier: 1.5 + mercy_level * 0.8, duration_seconds: 180 + (mercy_level * 120.0) as u32 },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"source": "EternalFlow"}),
            priority: 0.0,
            base_priority: 0.0,
            priority_boost: 0.0,
        };
        event.base_priority = event.compute_base_priority();
        event.priority = event.base_priority;
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    fn schedule_faction_event(&mut self, current_time: f64) {
        let mut event = DynamicEvent {
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
            metadata: serde_json::json!({"type": "diplomacy", "patsagi": true}),
            priority: 0.0,
            base_priority: 0.0,
            priority_boost: 0.25,
        };
        event.base_priority = event.compute_base_priority();
        event.priority = event.base_priority + event.priority_boost;
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    fn schedule_divine_cascade(&mut self, current_time: f64, mercy_level: f32) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::DivineWhisperCascade { intensity: 0.7 + mercy_level * 0.3, target_players: None },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"council": "PATSAGi"}),
            priority: 0.0,
            base_priority: 0.0,
            priority_boost: 0.15,
        };
        event.base_priority = event.compute_base_priority();
        event.priority = event.base_priority + event.priority_boost;
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    pub fn boost_events_for_player(&mut self, player_faction: &str, standing: f32) {
        for event in self.pending_replication.iter_mut() {
            if let DynamicEventType::FactionDiplomacyShift { faction_a, faction_b, .. } = &event.event_type {
                if faction_a == player_faction || faction_b == player_faction {
                    let boost = 0.2 + standing * 0.15;
                    event.apply_boost(boost);
                }
            }
        }
    }

    pub fn get_relevant_events_for_player(&self, _player_pos: [f32; 3]) -> Vec<&DynamicEvent> { self.active_events.iter().collect() }

    pub fn drain_prioritized_replication(&mut self) -> Vec<DynamicEvent> {
        let mut events = std::mem::take(&mut self.pending_replication);
        events.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        events
    }
    pub fn drain_pending_replication(&mut self) -> Vec<DynamicEvent> { self.drain_prioritized_replication() }
}

// ═════════════════════════════════════════════════════════════════════════
// REPLICATION WIRING
// ═════════════════════════════════════════════════════════════════════════

pub fn map_server_event_to_client(event: &DynamicEvent) -> Option<ClientWorldEventMirror> {
    match &event.event_type {
        DynamicEventType::FactionDiplomacyShift { faction_a, faction_b, .. } => Some(ClientWorldEventMirror::FactionDiplomacyShift {
            faction_a: faction_a.clone(),
            faction_b: faction_b.clone(),
            reason: format!("Diplomacy shift (priority {:.2}) between {} and {}", event.priority, faction_a, faction_b),
        }),
        DynamicEventType::AbundanceSurge { multiplier, .. } => Some(ClientWorldEventMirror::AbundanceSurge {
            region: "Unknown Region".to_string(),
            intensity: *multiplier,
            mercy_delta: event.mercy_alignment * 10.0,
        }),
        DynamicEventType::DivineWhisperCascade { intensity, .. } => Some(ClientWorldEventMirror::DivineWhisperCascade {
            message: "A Divine Whisper cascades across the world...".to_string(),
            affected_factions: vec![],
            mercy_impact: *intensity,
        }),
        _ => None,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientWorldEventMirror {
    AbundanceSurge { region: String, intensity: f32, mercy_delta: f32 },
    FactionDiplomacyShift { faction_a: String, faction_b: String, reason: String },
    DivineWhisperCascade { message: String, affected_factions: Vec<String>, mercy_impact: f32 },
}

// ═══════════════════════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═══════════════════════════════════════════════════════════════════════════════

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
    info!("⚡ Dynamic Events v17.49 + Decay Rate Tuning online — fully configurable via DynamicEventsConfig");
}

fn dynamic_events_tick_system(
    mut manager: ResMut<DynamicEventManager>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds_f64();
    let mercy_level = 0.88;
    manager.tick(current_time, mercy_level);
}

// Tuning Guide (edit DynamicEventsConfig at runtime or in default):
// - Increase divine_half_life_multiplier to 2.5+ for very long-lived divine wisdom.
// - Lower patsagi_treaty_half_life_multiplier if you want fresh treaties to stand out more via boosts instead.
// - Raise mercy_half_life_influence to make high-mercy events almost permanent.
// - Set boost_decay_half_life low if you want boosts to be short-lived "spotlights".
