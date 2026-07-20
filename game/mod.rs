// game/mod.rs
// Powrush-MMO — Core Game Systems
// v21.56.0 — multi_realm_bridge + rbe exported for external observability bridge
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
// Thunder locked in. Yoi ⚡

pub mod types;
pub mod hit_detection;
pub mod reconciliation;
pub mod resource_nodes;
pub mod rbe;
pub mod multi_realm_bridge;

// Re-export the living abundance surface for clean external use
pub use resource_nodes::{
    RealmId,
    RealmAbundanceSnapshot,
    ResourceNode,
    ResourceNodeManager,
    HarvestingSystem,
};

// Soft origin inventory + bridge payloads
pub use rbe::{
    ServerInventoryComponent,
    RealmOriginSnapshot,
    RbeSystem,
    TradingSystem,
};

pub use multi_realm_bridge::{
    AbundanceBridgePayload,
    OriginBridgePayload,
    collect_abundance_payload,
    collect_origin_from_inventory,
    merge_origin_payloads,
};
