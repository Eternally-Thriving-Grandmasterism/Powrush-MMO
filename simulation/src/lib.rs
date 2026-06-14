pub mod epiphany_catalyst;
pub mod bot_detection;
pub mod divine_whispers;
pub mod player_persistence;
pub mod cloud_sync;
pub mod emergence;

// Re-export key emergence types for convenient use across the simulation crate
pub use emergence::{
    EmergenceEffect, EmergenceOrchestrator, EmergenceSeed, DynamicEmergenceEvent,
    DynamicEmergenceEventPhase, EmergenceSource, EmergencePlugin,
};