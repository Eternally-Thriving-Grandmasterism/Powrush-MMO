// game/multi_realm_bridge.rs
// Powrush-MMO v21.56.0 — External Game→Simulation Multi-Realm Bridge
// Zero-cycle pure payloads. No simulation crate dependency.
// Field order matches simulation::multi_realm_harness::RealmAbundanceView::from_raw
// and OriginProvenanceView { realm_id, total_amount, resource_types }.
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

/// Collect living abundance snapshots from ResourceNodeManager.
/// Call from server tick (or any authoritative loop) on a soft cadence.
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

/// Merge origin snapshots from many inventories (e.g. all online players).
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

/// Documented Bevy wiring (call site when game + simulation share an App):
///
/// ```ignore
/// use simulation::multi_realm_harness::{
///     AbundanceIngestEvent, OriginIngestEvent,
///     RealmAbundanceView, OriginProvenanceView,
/// };
///
/// // Abundance
/// let payload = collect_abundance_payload(&manager, now_ms);
/// let views: Vec<_> = payload.views.into_iter().map(|(id, n, y, sus, flow, stress, rest, thr)| {
///     RealmAbundanceView::from_raw(id, n, y, sus, flow, stress, rest, thr)
/// }).collect();
/// abundance_writer.send(AbundanceIngestEvent { views, tick: payload.tick_ms });
///
/// // Origin
/// let origin = collect_origin_from_inventory(&inventory, now_ms);
/// let views: Vec<_> = origin.views.into_iter().map(|(id, amt, types)| {
///     OriginProvenanceView { realm_id: id, total_amount: amt, resource_types: types }
/// }).collect();
/// origin_writer.send(OriginIngestEvent { views, tick: origin.tick_ms });
/// ```
pub mod bevy_wiring_docs {}

// Thunder locked in.
// Pure payloads. Zero cycles. Live when both sides share an app.
// Yoi ⚡
