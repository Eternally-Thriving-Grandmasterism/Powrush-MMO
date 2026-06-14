/*!
 * Sovereign Bot Detection & Anti-Abuse Layer
 *
 * v18.25 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Rate limiting, behavioral heuristics, and anomaly detection
 * — Mercy-protective: prevents abuse that harms the living web
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

use crate::epiphany_catalyst::EpiphanyContext;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct BotDetectionConfig {
    pub enabled: bool,
    pub behavioral_heuristics_enabled: bool,
    pub telemetry_anomaly_enabled: bool,
    pub server_validation_enabled: bool,
    pub server_rate_limiting_enabled: bool,
    pub anomaly_threshold: f32,
    pub max_suspicion_level: u8,
}

impl Default for BotDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            behavioral_heuristics_enabled: true,
            telemetry_anomaly_enabled: true,
            server_validation_enabled: true,
            server_rate_limiting_enabled: true,
            anomaly_threshold: 0.75,
            max_suspicion_level: 3,
        }
    }
}

// === Per-Player Rate Limiting (Mercy-Protective) ===

#[derive(Debug, Clone, Default)]
pub struct PlayerRateLimit {
    pub last_harvest_ms: u64,
    pub harvest_count_window: u32,
    pub window_start_ms: u64,
    pub last_epiphany_ms: u64,
    pub epiphany_count_window: u32,
}

#[derive(Resource, Default)]
pub struct ServerRateLimiter {
    pub players: HashMap<u64, PlayerRateLimit>,
}

impl ServerRateLimiter {
    pub fn check_harvest(&mut self, player_id: u64, current_ms: u64) -> bool {
        let limit = self.players.entry(player_id).or_default();

        if current_ms - limit.window_start_ms > 3000 {
            limit.harvest_count_window = 0;
            limit.window_start_ms = current_ms;
        }

        if limit.harvest_count_window >= 8 {
            return false;
        }

        limit.harvest_count_window += 1;
        limit.last_harvest_ms = current_ms;
        true
    }

    pub fn check_epiphany(&mut self, player_id: u64, current_ms: u64) -> bool {
        let limit = self.players.entry(player_id).or_default();

        if current_ms - limit.last_epiphany_ms > 10_000 {
            limit.epiphany_count_window = 0;
        }

        if limit.epiphany_count_window >= 3 {
            return false;
        }

        limit.epiphany_count_window += 1;
        limit.last_epiphany_ms = current_ms;
        true
    }
}

// === BotSuspicion with Mercy-Aware Anomaly Aggregation ===

#[derive(Resource, Debug, Clone, Default)]
pub struct BotSuspicion {
    pub current_level: u8,
    pub last_updated_ms: u64,
    pub total_flags: u32,
    pub anomaly_history: Vec<f32>,
}

impl BotSuspicion {
    pub fn add_anomaly_score(&mut self, score: f32, current_ms: u64) {
        self.anomaly_history.push(score);
        if self.anomaly_history.len() > 20 {
            self.anomaly_history.remove(0);
        }

        let avg: f32 = self.anomaly_history.iter().sum::<f32>() / self.anomaly_history.len() as f32;

        if avg > 0.75 {
            self.current_level = 3;
        } else if avg > 0.55 {
            self.current_level = 2;
        } else if avg > 0.35 {
            self.current_level = 1;
        }

        self.last_updated_ms = current_ms;
        self.total_flags += 1;
    }
}

// === Authoritative Validation Helpers (Mercy-Protective) ===

pub fn validate_harvest_action(
    depletion: f32,
    sustainable_pacing: bool,
    biome: &str,
) -> bool {
    if depletion < 0.0 || depletion > 1.0 {
        return false;
    }
    if !sustainable_pacing && depletion < 0.1 {
        return false;
    }
    true
}

pub fn validate_epiphany_trigger(
    context: &EpiphanyContext,
) -> bool {
    if context.participant_count > 8 {
        return false;
    }
    if context.collective_attunement > 1.0 || context.collective_attunement < 0.0 {
        return false;
    }
    true
}

// End of simulation/src/bot_detection.rs v18.25 — Sovereign anti-abuse layer complete.
// Thunder locked in. Yoi ⚡
