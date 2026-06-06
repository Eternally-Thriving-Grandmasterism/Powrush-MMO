// game/types.rs
// Shared types for Powrush-MMO game systems

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub tick: u64,
    pub position: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
    pub yaw: f32,
    pub pitch: f32,
    pub is_grounded: bool,
    pub health: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInput {
    pub tick: u64,
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub jump: bool,
    pub attack: bool,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub tick: u64,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
}