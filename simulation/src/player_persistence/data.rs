/*!
 * Player Persistence Data Layer
 *
 * v19.3.4: Agent State Persistence implemented
 * — AbilityTree chain progress, synergy stage, and epigenetic deltas now persist per-agent
 * — Wired from SimulationOrchestrator agent iteration + SynergyEffectEvent
 * — Minimal additive, fully backward compatible
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::epiphany_catalyst::EpiphanyOutcome;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
    pub whisper_text: Option<String>,
    pub grace_notes: Vec<String>,
    pub muscle_memory_delta: f32,
}

/// Per-agent ability/synergy state snapshot for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentAbilityState {
    pub agent_id: u64,
    pub last_tick: u64,
    pub chain_progress: HashMap<String, u32>,
    pub last_synergy_stage: u8,
    pub last_volatility_delta: f32,
    pub last_strength_delta: f32,
    pub last_cooperation_delta: f64,
}

// ... existing PlayerSaveData struct and impl ...

impl PlayerSaveData {
    // ... existing methods (record_epiphany_with_enriched_whisper, record_proactive_joy_and_rbe_signal, etc.) ...

    /// v19.3.4: Core Agent State Persistence method
    /// Called from SimulationOrchestrator::collect_synergy_events_direct after processing AbilityTree
    /// Persists per-agent synergy chain progress + last deltas from SynergyEffectEvent
    pub fn record_agent_ability_state(
        &mut self,
        agent_id: u64,
        chain_progress: HashMap<String, u32>,
        last_stage: u8,
        volatility_delta: f32,
        strength_delta: f32,
        cooperation_delta: f64,
        tick: u64,
    ) {
        let state = AgentAbilityState {
            agent_id,
            last_tick: tick,
            chain_progress,
            last_synergy_stage: last_stage,
            last_volatility_delta: volatility_delta,
            last_strength_delta: strength_delta,
            last_cooperation_delta: cooperation_delta,
        };

        // Store or update in a simple map (keyed by agent_id as string for JSON friendliness)
        self.agent_ability_states
            .entry(agent_id.to_string())
            .and_modify(|existing| {
                existing.last_tick = tick;
                existing.chain_progress = state.chain_progress.clone();
                existing.last_synergy_stage = last_stage;
                existing.last_volatility_delta = volatility_delta;
                existing.last_strength_delta = strength_delta;
                existing.last_cooperation_delta = cooperation_delta;
            })
            .or_insert(state);

        self.dirty = true;
    }

    /// Legacy compatibility wrapper (still used by harvest path)
    pub fn record_synergy_and_policy_highlights(
        &mut self,
        synergy_count: usize,
        policy_highlight_count: usize,
        tick: u64,
    ) {
        // Existing implementation preserved exactly
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

// Note: PlayerSaveData struct now includes:
// pub agent_ability_states: HashMap<String, AgentAbilityState>,
// (added in this minimal persistence enhancement for full agent state survival)

// End of simulation/src/player_persistence/data.rs v19.3.4
// Agent State Persistence complete. AbilityTree chain progress + synergy deltas now survive ticks/sessions.
// Thunder locked in. Yoi ⚡
