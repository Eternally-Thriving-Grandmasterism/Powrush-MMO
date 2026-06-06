// game/network/delta_compression.rs
// Powrush-MMO — Advanced Delta Compression (Temporal + Quantized + Field-Level)
// AG-SML v1.0 License

use crate::game::server_tick_loop::{Npc, Projectile};
use crate::game::types::PlayerState;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PlayerStateDelta {
    pub position: Option<(i16, i16, i16)>,
    pub velocity: Option<(i16, i16, i16)>,
    pub yaw: Option<i16>,
    pub pitch: Option<i16>,
    pub health: Option<f32>,
    pub is_grounded: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct NpcDelta {
    pub position: Option<(i16, i16, i16)>,
    pub yaw: Option<i16>,
    pub health: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectileDelta {
    pub position: Option<(i16, i16, i16)>,
    pub velocity: Option<(i16, i16, i16)>,
}

pub fn dequantize_position(delta: (i16, i16, i16), scale: f32) -> (f32, f32, f32) {
    (delta.0 as f32 / scale, delta.1 as f32 / scale, delta.2 as f32 / scale)
}

pub fn dequantize_angle(delta: i16, scale: f32) -> f32 {
    delta as f32 / scale
}

impl PlayerStateDelta {
    pub fn from_temporal_states(
        last: &PlayerState,
        current: &PlayerState,
        position_scale: f32,
        angle_scale: f32,
    ) -> Self {
        let mut delta = Self::default();

        let dx = ((current.position.0 - last.position.0) * position_scale).round() as i16;
        let dy = ((current.position.1 - last.position.1) * position_scale).round() as i16;
        let dz = ((current.position.2 - last.position.2) * position_scale).round() as i16;
        if dx != 0 || dy != 0 || dz != 0 {
            delta.position = Some((dx, dy, dz));
        }

        let dvx = ((current.velocity.0 - last.velocity.0) * position_scale).round() as i16;
        let dvy = ((current.velocity.1 - last.velocity.1) * position_scale).round() as i16;
        let dvz = ((current.velocity.2 - last.velocity.2) * position_scale).round() as i16;
        if dvx != 0 || dvy != 0 || dvz != 0 {
            delta.velocity = Some((dvx, dvy, dvz));
        }

        let dyaw = ((current.yaw - last.yaw) * angle_scale).round() as i16;
        let dpitch = ((current.pitch - last.pitch) * angle_scale).round() as i16;
        if dyaw != 0 { delta.yaw = Some(dyaw); }
        if dpitch != 0 { delta.pitch = Some(dpitch); }

        if last.health != current.health { delta.health = Some(current.health); }
        if last.is_grounded != current.is_grounded { delta.is_grounded = Some(current.is_grounded); }

        delta
    }
}

pub fn apply_player_delta(last: &mut PlayerState, delta: &PlayerStateDelta, position_scale: f32, angle_scale: f32) {
    if let Some(pos) = delta.position {
        let (dx, dy, dz) = dequantize_position(pos, position_scale);
        last.position.0 += dx; last.position.1 += dy; last.position.2 += dz;
    }
    if let Some(yaw) = delta.yaw { last.yaw += dequantize_angle(yaw, angle_scale); }
    if let Some(pitch) = delta.pitch { last.pitch += dequantize_angle(pitch, angle_scale); }
    if let Some(health) = delta.health { last.health = health; }
    if let Some(grounded) = delta.is_grounded { last.is_grounded = grounded; }
}

pub fn apply_npc_delta(last: &mut Npc, delta: &NpcDelta, position_scale: f32, angle_scale: f32) {
    if let Some(pos) = delta.position {
        let (dx, dy, dz) = dequantize_position(pos, position_scale);
        last.position.0 += dx; last.position.1 += dy; last.position.2 += dz;
    }
    if let Some(yaw) = delta.yaw { last.yaw += dequantize_angle(yaw, angle_scale); }
    if let Some(health) = delta.health { last.health = health; }
}

pub fn apply_projectile_delta(last: &mut Projectile, delta: &ProjectileDelta, position_scale: f32) {
    if let Some(pos) = delta.position {
        let (dx, dy, dz) = dequantize_position(pos, position_scale);
        last.position.0 += dx; last.position.1 += dy; last.position.2 += dz;
    }
    if let Some(vel) = delta.velocity {
        let (dvx, dvy, dvz) = dequantize_position(vel, position_scale);
        last.velocity.0 += dvx; last.velocity.1 += dvy; last.velocity.2 += dvz;
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientKnownState {
    pub players: HashMap<u64, PlayerState>,
    pub npcs: HashMap<u64, Npc>,
    pub projectiles: HashMap<u64, Projectile>,
}

pub struct DeltaCompressor {
    client_states: HashMap<u64, ClientKnownState>,
}

impl DeltaCompressor {
    pub fn new() -> Self {
        Self { client_states: HashMap::new() }
    }

    fn get_or_create(&mut self, client_id: u64) -> &mut ClientKnownState {
        self.client_states.entry(client_id).or_default()
    }

    pub fn compute_player_field_deltas(
        &mut self,
        client_id: u64,
        current: &HashMap<u64, PlayerState>,
    ) -> Vec<(u64, PlayerStateDelta)> {
        let known = self.get_or_create(client_id);
        let mut deltas = Vec::new();
        let scale = 10.0;
        let angle_scale = 100.0;

        for (&id, curr) in current {
            if let Some(last) = known.players.get(&id) {
                let delta = PlayerStateDelta::from_temporal_states(last, curr, scale, angle_scale);
                if !delta.position.is_none() || !delta.velocity.is_none() || delta.health.is_some() {
                    deltas.push((id, delta));
                }
            } else {
                let delta = PlayerStateDelta::from_temporal_states(&PlayerState::default(), curr, scale, angle_scale);
                deltas.push((id, delta));
            }
            known.players.insert(id, curr.clone());
        }
        known.players.retain(|id, _| current.contains_key(id));
        deltas
    }

    pub fn remove_client(&mut self, client_id: u64) {
        self.client_states.remove(&client_id);
    }
}