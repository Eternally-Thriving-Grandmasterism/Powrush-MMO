// game/hit_detection.rs
// Powrush-MMO — Hit Detection with Advanced Lag Compensation

use crate::game::lag_compensation::LagCompensation;
use crate::game::types::PlayerSnapshot;
use serde::{Deserialize, Serialize};

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
    pub rewind_ticks: u64,
}

pub struct HitDetection {
    lag_comp: LagCompensation,
}

impl HitDetection {
    pub fn new(lag_comp: LagCompensation) -> Self {
        Self { lag_comp }
    }

    pub fn record_snapshot(&mut self, player_id: u64, snapshot: PlayerSnapshot) {
        self.lag_comp.record_snapshot(player_id, snapshot);
    }

    pub fn check_hit(&self, request: &HitRequest, current_tick: u64) -> HitResult {
        let rewind_allowed = self.lag_comp.is_rewind_allowed(current_tick, request.tick);
        let rewind_ticks = self.lag_comp.calculate_rewind_ticks(current_tick, request.tick);

        if !rewind_allowed {
            return HitResult {
                attacker_id: request.attacker_id,
                target_id: request.target_id,
                hit: false,
                damage_dealt: 0.0,
                hit_position: None,
                server_tick: current_tick,
                rewind_ticks,
            };
        }

        let target_snapshot = self.lag_comp.get_snapshot_at_tick(request.target_id, request.tick);

        let (hit, hit_position) = if let Some(snapshot) = target_snapshot {
            let distance = crate::game::hit_detection::distance_3d(snapshot.position, (0.0, 0.0, 0.0));
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
            server_tick: current_tick,
            rewind_ticks,
        }
    }
}

pub fn distance_3d(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}