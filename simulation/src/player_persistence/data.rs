/*!
 * Player Persistence Data Layer
 *
 * v19.3.13: Implemented persistence write batching
 * Multiple agent state updates are now coalesced before marking dirty.
 * Reduces unnecessary auto-save triggers.
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::epiphany_catalyst::EpiphanyOutcome;

const PERSISTENCE_BATCH_SIZE: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
    pub whisper_text: Option<String>,
    #[serde(default)]
    pub grace_notes: Vec<String>,
    pub muscle_memory_delta: f32,
}

/// Per-agent ability/synergy state snapshot for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentAbilityState {
    pub agent_id: u64,
    pub last_tick: u64,
    #[serde(default)]
    pub chain_progress: HashMap<String, u32>,
    pub last_synergy_stage: u8,
    pub last_volatility_delta: f32,
    pub last_strength_delta: f32,
    pub last_cooperation_delta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    /// List of all recorded epiphanies and proactive joy events
    #[serde(default)]
    pub epiphanies: Vec<EpiphanyRecord>,

    /// Per-agent ability/synergy state (AbilityTree chain progress + deltas)
    #[serde(default)]
    pub agent_ability_states: HashMap<String, AgentAbilityState>,

    pub dirty: bool,

    /// Internal counter for batched persistence updates
    pending_persistence_updates: usize,
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            epiphanies: Vec::new(),
            agent_ability_states: HashMap::new(),
            dirty: false,
            pending_persistence_updates: 0,
        }
    }

    /// Core Agent State Persistence with write batching
    pub fn record_agent_ability_state(
        &mut self,
        agent_id: u64,
        chain_progress: &HashMap<String, u32>,
        last_stage: u8,
        volatility_delta: f32,
        strength_delta: f32,
        cooperation_delta: f64,
        tick: u64,
    ) {
        let state = AgentAbilityState {
            agent_id,
            last_tick: tick,
            chain_progress: chain_progress.clone(),
            last_synergy_stage: last_stage,
            last_volatility_delta: volatility_delta,
            last_strength_delta: strength_delta,
            last_cooperation_delta: cooperation_delta,
        };

        self.agent_ability_states
            .entry(agent_id.to_string())
            .and_modify(|existing| {
                existing.last_tick = tick;
                existing.chain_progress = chain_progress.clone();
                existing.last_synergy_stage = last_stage;
                existing.last_volatility_delta = volatility_delta;
                existing.last_strength_delta = strength_delta;
                existing.last_cooperation_delta = cooperation_delta;
            })
            .or_insert(state);

        // Batching: only mark dirty after enough updates
        self.pending_persistence_updates += 1;
        if self.pending_persistence_updates >= PERSISTENCE_BATCH_SIZE {
            self.dirty = true;
            self.pending_persistence_updates = 0;
        }
    }

    /// Force immediate dirty state (e.g. on exit or critical save)
    pub fn force_dirty(&mut self) {
        if self.pending_persistence_updates > 0 {
            self.dirty = true;
            self.pending_persistence_updates = 0;
        }
    }

    /// Preserved exactly
    pub fn record_synergy_and_policy_highlights(
        &mut self,
        synergy_count: usize,
        policy_highlight_count: usize,
        tick: u64,
    ) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(last) = self.epiphanies.last_mut() {
            last.grace_notes.push(format!(
                "Synergy events: {} | Policy highlights: {} at tick {}",
                synergy_count, policy_highlight_count, tick
            ));
        } else {
            self.epiphanies.push(EpiphanyRecord {
                scenario_id: "synergy_policy".to_string(),
                timestamp,
                intensity: 0.0,
                biome: "global".to_string(),
                whisper_text: Some(format!("Synergy + Policy at tick {}", tick)),
                grace_notes: vec![format!("Synergy: {} | Policy: {}", synergy_count, policy_highlight_count)],
                muscle_memory_delta: 0.0,
            });
        }

        self.dirty = true;
    }
}

// End of simulation/src/player_persistence/data.rs v19.3.13
// Persistence write batching implemented (batch size = 8).
// Thunder locked in. Yoi ⚡
