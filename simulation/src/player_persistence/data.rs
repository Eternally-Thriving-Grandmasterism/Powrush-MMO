/*!
 * Player Persistence Data Layer
 *
 * v19.3.20: Added ShareInfo for metadata on Shamir recovery shares.
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

/// Metadata for one Shamir recovery share
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShareInfo {
    pub index: u8,
    pub label: Option<String>,
    pub created_at: u64,
}

/// Configuration for Shamir’s Secret Sharing recovery
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecoveryConfig {
    pub enabled: bool,
    pub total_shares: u8,
    pub threshold: u8,
    /// List of shares the user has generated (for reference)
    pub shares: Vec<ShareInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    #[serde(default)]
    pub epiphanies: Vec<EpiphanyRecord>,

    #[serde(default)]
    pub agent_ability_states: HashMap<String, AgentAbilityState>,

    pub dirty: bool,
    pub pending_persistence_updates: usize,

    // Crash Recovery
    pub last_shutdown_was_clean: bool,
    pub last_save_timestamp: u64,
    pub save_version: u32,
    pub checksum: String,
    pub total_playtime_seconds: u64,

    // Sovereign Recovery
    pub recovery: RecoveryConfig,
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            epiphanies: Vec::new(),
            agent_ability_states: HashMap::new(),
            dirty: false,
            pending_persistence_updates: 0,
            last_shutdown_was_clean: false,
            last_save_timestamp: 0,
            save_version: 1,
            checksum: String::new(),
            total_playtime_seconds: 0,
            recovery: RecoveryConfig::default(),
        }
    }

    pub fn enable_shamir_recovery(&mut self, total_shares: u8, threshold: u8) {
        if threshold >= 2 && threshold <= total_shares {
            self.recovery = RecoveryConfig {
                enabled: true,
                total_shares,
                threshold,
                shares: Vec::new(),
            };
            self.dirty = true;
        }
    }

    pub fn disable_shamir_recovery(&mut self) {
        self.recovery = RecoveryConfig::default();
        self.dirty = true;
    }

    /// Record metadata for a newly generated share
    pub fn record_share(&mut self, index: u8, label: Option<String>) {
        let info = ShareInfo {
            index,
            label,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.recovery.shares.push(info);
        self.dirty = true;
    }

    pub fn mark_session_started(&mut self) {
        self.last_shutdown_was_clean = false;
        self.dirty = true;
    }

    pub fn mark_clean_shutdown(&mut self) {
        self.last_shutdown_was_clean = true;
    }

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

        self.pending_persistence_updates += 1;
        if self.pending_persistence_updates >= 8 {
            self.dirty = true;
            self.pending_persistence_updates = 0;
        }
    }

    pub fn force_dirty(&mut self) {
        if self.pending_persistence_updates > 0 {
            self.dirty = true;
            self.pending_persistence_updates = 0;
        }
    }

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

// End of simulation/src/player_persistence/data.rs v19.3.20
// ShareInfo + recovery metadata storage implemented.
// Thunder locked in. Yoi ⚡
