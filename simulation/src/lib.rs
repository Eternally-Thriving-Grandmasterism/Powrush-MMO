pub mod epiphany_catalyst;
pub mod bot_detection;
pub mod divine_whispers;
pub mod player_persistence;
pub mod cloud_sync;
pub mod emergence;
pub mod ra_thor_bridge;
pub mod spatial_interest;

pub use emergence::{
    EmergenceEffect, EmergenceOrchestrator, EmergenceSeed, DynamicEmergenceEvent,
    DynamicEmergenceEventPhase, EmergenceSource, EmergencePlugin, CouncilGuidance,
};

pub use ra_thor_bridge::{RaThorBridge, CouncilQueryRequest, CouncilQueryResponse, RaThorCouncilQuery};

// Spatial Interest Layer (Hybrid Chunk + Continuous Interest)
pub use spatial_interest::{
    SpatialHash,
    InterestManager,
    InterestZone,
    CouncilBloomZone,
    update_spatial_hash_system,
    update_interest_zones_system,
    query_entities_in_interest,
};
