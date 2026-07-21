// game/mod.rs
// Powrush-MMO — Core Game Systems
// v21.79.0 — NonSend ServerTickLoop + dual bridge payloads
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
// Thunder locked in. Yoi ⚡

pub mod types;
pub mod hit_detection;
pub mod reconciliation;
pub mod resource_nodes;
pub mod rbe;
pub mod multi_realm_bridge;
pub mod server_tick_loop;

pub use resource_nodes::{
    RealmId,
    RealmAbundanceSnapshot,
    ResourceNode,
    ResourceNodeManager,
    HarvestingSystem,
};

pub use rbe::{
    ServerInventoryComponent,
    RealmOriginSnapshot,
    RbeSystem,
    TradingSystem,
};

pub use multi_realm_bridge::{
    AbundanceBridgePayload,
    OriginBridgePayload,
    DualBridgePayload,
    collect_abundance_payload,
    collect_origin_from_inventory,
    collect_origin_from_inventories,
    collect_dual_payload,
    merge_origin_payloads,
};

pub use server_tick_loop::{
    ServerTickLoop,
    ServerTickLoopPlugin,
    GpuPerformanceMetrics,
};
