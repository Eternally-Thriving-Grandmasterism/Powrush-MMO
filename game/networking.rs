// game/networking.rs
// Powrush-MMO — Networking Message Protocol
// AG-SML v1.0 License

use serde::{Deserialize, Serialize};

use crate::game::types::{PlayerInput, PlayerState};
use crate::game::hit_detection::HitResult;
use crate::game::hit_markers::HitMarkerData;
use crate::game::server_tick_loop::{Npc, Projectile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerInput(PlayerInput),
    AttackRequest {
        target_id: Option<u64>,
        ability_id: Option<u32>,
    },
    RequestProjectile {
        position: (f32, f32, f32),
        velocity: (f32, f32, f32),
        damage: f32,
    },
    Ping { timestamp: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    StateUpdate {
        tick: u64,
        players: Vec<(u64, PlayerState)>,
        npcs: Vec<Npc>,
        projectiles: Vec<Projectile>,
    },
    HitResults(Vec<HitResult>),
    HitMarkers(Vec<HitMarkerData>),
    Correction {
        tick: u64,
        corrected_state: PlayerState,
    },
    Pong { timestamp: u64 },
}