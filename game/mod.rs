// game/mod.rs
// Powrush-MMO — Core Game Systems
// v21.47.0 — resource_nodes exported for multi-realm abundance bridge
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
// Thunder locked in. Yoi ⚡

pub mod types;
pub mod hit_detection;
pub mod reconciliation;
pub mod resource_nodes;

// Re-export the living abundance surface for clean external use
pub use resource_nodes::{
    RealmId,
    RealmAbundanceSnapshot,
    ResourceNode,
    ResourceNodeManager,
    HarvestingSystem,
};
