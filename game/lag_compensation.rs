// game/lag_compensation.rs
// Powrush-MMO — Advanced Lag Compensation System
// Works together with hit_detection and reconciliation systems.
// AG-SML v1.0 License

use crate::game::types::PlayerSnapshot;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LagCompensationConfig {
    pub max_history_ticks: usize,
    pub max_rewind_ms: u64,
    pub tick_rate: u64,
}

impl Default for LagCompensationConfig {
    fn default() -> Self {
        Self {
            max_history_ticks: 120,
            max_rewind_ms: 250,
            tick_rate: 60,
        }
    }
}

pub struct LagCompensation {
    history: HashMap<u64, Vec<PlayerSnapshot>>,
    config: LagCompensationConfig,
}

impl LagCompensation {
    pub fn new(config: LagCompensationConfig) -> Self {
        Self {
            history: HashMap::new(),
            config,
        }
    }

    pub fn record_snapshot(&mut self, player_id: u64, snapshot: PlayerSnapshot) {
        let history = self.history.entry(player_id).or_default();
        history.push(snapshot);

        if history.len() > self.config.max_history_ticks {
            history.remove(0);
        }
    }

    pub fn get_snapshot_at_tick(&self, player_id: u64, target_tick: u64) -> Option<&PlayerSnapshot> {
        let history = self.history.get(&player_id)?;
        if history.is_empty() {
            return None;
        }

        history.iter()
            .min_by_key(|s| (s.tick as i64 - target_tick as i64).abs())
    }

    pub fn calculate_rewind_ticks(&self, current_tick: u64, requested_tick: u64) -> u64 {
        if current_tick > requested_tick {
            current_tick - requested_tick
        } else {
            0
        }
    }

    pub fn is_rewind_allowed(&self, current_tick: u64, requested_tick: u64) -> bool {
        let rewind_ticks = self.calculate_rewind_ticks(current_tick, requested_tick);
        let max_rewind_ticks = (self.config.max_rewind_ms * self.config.tick_rate) / 1000;
        rewind_ticks <= max_rewind_ticks
    }

    pub fn get_history(&self, player_id: u64) -> Option<&Vec<PlayerSnapshot>> {
        self.history.get(&player_id)
    }
}