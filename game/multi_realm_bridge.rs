// game/multi_realm_bridge.rs
// Powrush-MMO v21.58.0 — DualBridgePayload + inventory origin collection
// Zero-cycle pure payloads. No simulation crate dependency.
// Field order matches simulation::multi_realm_harness views + ExternalBridgeInbox.
//
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
// Thunder locked in. Yoi ⚡

use crate::game::resource_nodes::ResourceNodeManager;
use crate::game::rbe::ServerInventoryComponent;

/// Pure abundance payload — safe to ship across crate boundaries.
/// Each tuple is (realm_id, node_count, total_current_yield,
/// average_sustainability, average_abundance_flow, average_stress,
/// restricted_node_count, thriving_node_count).
#[derive(Clone, Debug, Default)]
pub struct AbundanceBridgePayload {
    pub views: Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>,
    pub tick_ms: u64,
}

impl AbundanceBridgePayload {
    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }

    pub fn realm_count(&self) -> usize {
        self.views.len()
    }
}

/// Pure origin provenance payload — safe to ship across crate boundaries.
/// Each tuple is (realm_id, total_amount, resource_types).
#[derive(Clone, Debug, Default)]
pub struct OriginBridgePayload {
    pub views: Vec<(u8, f32, u32)>,
    pub tick_ms: u64,
}

impl OriginBridgePayload {
    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }

    pub fn realm_count(&self) -> usize {
        self.views.len()
    }

    pub fn total_harvested(&self) -> f32 {
        self.views.iter().map(|(_, amt, _)| *amt).sum()
    }
}

/// Paired abundance + origin for a single shared-app publish step.
#[derive(Clone, Debug, Default)]
pub struct DualBridgePayload {
    pub abundance: AbundanceBridgePayload,
    pub origin: OriginBridgePayload,
}

impl DualBridgePayload {
    pub fn is_empty(&self) -> bool {
        self.abundance.is_empty() && self.origin.is_empty()
    }

    pub fn tick_ms(&self) -> u64 {
        self.abundance.tick_ms.max(self.origin.tick_ms)
    }
}

/// Collect living abundance snapshots from ResourceNodeManager.
pub fn collect_abundance_payload(
    manager: &ResourceNodeManager,
    now_ms: u64,
) -> AbundanceBridgePayload {
    let snaps = manager.snapshot_all_realms(now_ms);
    AbundanceBridgePayload {
        views: snaps.into_iter().map(|s| s.into_view_tuple()).collect(),
        tick_ms: now_ms,
    }
}

/// Collect soft origin provenance from a single inventory.
pub fn collect_origin_from_inventory(
    inv: &ServerInventoryComponent,
    tick_ms: u64,
) -> OriginBridgePayload {
    let snaps = inv.origin_snapshot();
    OriginBridgePayload {
        views: snaps
            .into_iter()
            .map(|s| (s.realm_id, s.total_amount, s.resource_types))
            .collect(),
        tick_ms,
    }
}

/// Collect and merge origin from many inventories (all online players).
pub fn collect_origin_from_inventories<'a>(
    inventories: impl IntoIterator<Item = &'a ServerInventoryComponent>,
    tick_ms: u64,
) -> OriginBridgePayload {
    let payloads = inventories
        .into_iter()
        .map(|inv| collect_origin_from_inventory(inv, tick_ms));
    merge_origin_payloads(payloads, tick_ms)
}

/// Merge origin snapshots from many inventories.
/// Amounts sum; resource_types takes the max seen per realm.
pub fn merge_origin_payloads(
    payloads: impl IntoIterator<Item = OriginBridgePayload>,
    tick_ms: u64,
) -> OriginBridgePayload {
    use std::collections::HashMap;
    let mut by_realm: HashMap<u8, (f32, u32)> = HashMap::new();

    for payload in payloads {
        for (realm_id, amount, types) in payload.views {
            let entry = by_realm.entry(realm_id).or_insert((0.0, 0));
            entry.0 += amount;
            entry.1 = entry.1.max(types);
        }
    }

    let mut views: Vec<_> = by_realm
        .into_iter()
        .map(|(realm_id, (total_amount, resource_types))| (realm_id, total_amount, resource_types))
        .collect();
    views.sort_by_key(|(id, _, _)| *id);

    OriginBridgePayload { views, tick_ms }
}

/// Collect both abundance (from nodes) and origin (from inventories) in one step.
pub fn collect_dual_payload<'a>(
    manager: &ResourceNodeManager,
    inventories: impl IntoIterator<Item = &'a ServerInventoryComponent>,
    now_ms: u64,
) -> DualBridgePayload {
    DualBridgePayload {
        abundance: collect_abundance_payload(manager, now_ms),
        origin: collect_origin_from_inventories(inventories, now_ms),
    }
}

/// Shared-app publish pattern (when game + simulation share a Bevy App):
///
/// ```ignore
/// // After ServerTickLoop::tick (or any authoritative collect):
/// let dual = tick_loop.dual_payload(); // or collect_dual_payload(...)
///
/// let mut inbox = world.resource_mut::<simulation::ExternalBridgeInbox>();
/// if !dual.abundance.is_empty() {
///     inbox.push_abundance(dual.abundance.views.clone(), dual.abundance.tick_ms);
/// }
/// if !dual.origin.is_empty() {
///     inbox.push_origin(dual.origin.views.clone(), dual.origin.tick_ms);
/// }
/// // external_bridge_drain_system promotes Demo → Live on next Update
/// ```
pub mod shared_app_publish_docs {}

// Thunder locked in.
// Dual payloads. Inventory origin. Zero cycles.
// Yoi ⚡
