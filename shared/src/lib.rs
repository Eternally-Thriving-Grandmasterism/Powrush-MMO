//! shared/src/lib.rs
//! Powrush-MMO Shared Protocol & Types — Common definitions between client and server
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod spatial; // HierarchicalGrid + Vec3 for spatial queries (reverb, interest, occlusion, etc.)

/// Core protocol messages sent between client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    WorldUpdate { entities: Vec<EntityUpdate>, timestamp: f64 },
    ValenceUpdate { player_id: u64, new_valence: f32, reason: String },
    MercyGateBlocked { reason: String, valence: f32 },
    Error { message: String },
    RbeTransaction { resource_type: RbeResourceType, amount: f32 },
}

// ... (rest of file unchanged for minimal diff)