// game/hit_detection.rs
// Powrush-MMO — Server-Authoritative Hit Detection with Lag Compensation

use crate::game::types::PlayerSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitRequest {
    pub attacker_id: u64,
    pub target_id: u64,
    pub tick: u64,
    pub weapon_range: f32,
    pub damage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitResult {
    pub attacker_id: u64,
    pub target_id: u64,
    pub hit: bool,
    pub damage_dealt: f32,
    pub hit_position: Option<(f32, f32, f32)>,
    pub server_tick: u64,
}

pub struct HitDetection {
    player_history: HashMap<u64, Vec<PlayerSnapshot>>,
    max_history_ticks: usize,
}

impl HitDetection {
    pub fn new(max_history_ticks: usize) -> Self {
        Self {
            player_history: HashMap::new(),
            max_history_ticks,
        }
    }

    pub fn record_snapshot(&mut self, player_id: u64, snapshot: PlayerSnapshot) {
        let history = self.player_history.entry(player_id).or_default();
        history.push(snapshot);
        if history.len() > self.max_history_ticks {
            history.remove(0);
        }
    }

    fn find_snapshot(&self, player_id: u64, requested_tick: u64) -> Option<&PlayerSnapshot> {
        self.player_history.get(&player_id)?.iter()
            .min_by_key(|s| (s.tick as i64 - requested_tick as i64).abs())
    }

    pub fn check_hit(&self, request: &HitRequest) -> HitResult {
        let target_snapshot = self.find_snapshot(request.target_id, request.tick);

        let (hit, hit_position) = if let Some(snapshot) = target_snapshot {
            let distance = distance_3d(snapshot.position, (0.0, 0.0, 0.0));
            let hit = distance <= request.weapon_range;
            (hit, Some(snapshot.position))
        } else {
            (false, None)
        };

        HitResult {
            attacker_id: request.attacker_id,
            target_id: request.target_id,
            hit,
            damage_dealt: if hit { request.damage } else { 0.0 },
            hit_position,
            server_tick: request.tick,
        }
    }
}

fn distance_3d(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}