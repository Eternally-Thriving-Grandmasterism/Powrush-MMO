//! simulation/src/external_bridge.rs
//! Bevy adapter for game-crate multi_realm_bridge pure payloads.
//! v21.57.0 — External Bridge Inbox + EventWriter emission
//!
//! Field order matches game::multi_realm_bridge:
//!   Abundance: (realm_id, node_count, yield, sust, flow, stress, restricted, thriving)
//!   Origin:    (realm_id, total_amount, resource_types)
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use bevy::prelude::*;
use tracing::info;

use crate::multi_realm_harness::{
    AbundanceIngestEvent, OriginIngestEvent, RealmAbundanceView, OriginProvenanceView,
};

// ============================================================================
// INBOX — push pure tuples from outside the ECS schedule
// ============================================================================

/// Shared inbox for authoritative game-side payloads.
/// Fill from server tick / shared-app glue; drained each Update into ingest events.
#[derive(Resource, Clone, Debug, Default)]
pub struct ExternalBridgeInbox {
    /// (views, tick_ms) — abundance tuples from ResourceNodeManager
    pub abundance: Option<(Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>, u64)>,
    /// (views, tick_ms) — origin tuples from ServerInventoryComponent(s)
    pub origin: Option<(Vec<(u8, f32, u32)>, u64)>,
    /// True after at least one external payload was drained (promotes Demo → Live)
    pub has_received_external: bool,
}

impl ExternalBridgeInbox {
    /// Push abundance payload (overwrites pending if not yet drained).
    pub fn push_abundance(
        &mut self,
        views: Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>,
        tick_ms: u64,
    ) {
        if !views.is_empty() {
            self.abundance = Some((views, tick_ms));
        }
    }

    /// Push origin payload (overwrites pending if not yet drained).
    pub fn push_origin(&mut self, views: Vec<(u8, f32, u32)>, tick_ms: u64) {
        if !views.is_empty() {
            self.origin = Some((views, tick_ms));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.abundance.is_none() && self.origin.is_none()
    }
}

// ============================================================================
// PURE EMIT HELPERS — call directly when you hold EventWriter
// ============================================================================

/// Convert game-bridge abundance tuples → AbundanceIngestEvent.
pub fn emit_abundance_from_tuples(
    writer: &mut EventWriter<AbundanceIngestEvent>,
    views: impl IntoIterator<Item = (u8, u32, f32, f32, f32, f32, u32, u32)>,
    tick: u64,
) -> usize {
    let converted: Vec<RealmAbundanceView> = views
        .into_iter()
        .map(|(id, n, y, sus, flow, stress, rest, thr)| {
            RealmAbundanceView::from_raw(id, n, y, sus, flow, stress, rest, thr)
        })
        .collect();
    let count = converted.len();
    if count > 0 {
        writer.send(AbundanceIngestEvent {
            views: converted,
            tick,
        });
    }
    count
}

/// Convert game-bridge origin tuples → OriginIngestEvent.
pub fn emit_origin_from_tuples(
    writer: &mut EventWriter<OriginIngestEvent>,
    views: impl IntoIterator<Item = (u8, f32, u32)>,
    tick: u64,
) -> usize {
    let converted: Vec<OriginProvenanceView> = views
        .into_iter()
        .map(|(id, amount, types)| OriginProvenanceView {
            realm_id: id,
            total_amount: amount,
            resource_types: types,
        })
        .collect();
    let count = converted.len();
    if count > 0 {
        writer.send(OriginIngestEvent {
            views: converted,
            tick,
        });
    }
    count
}

// ============================================================================
// DRAIN SYSTEM — runs in Update, promotes Demo → Live on external data
// ============================================================================

pub fn external_bridge_drain_system(
    mut inbox: ResMut<ExternalBridgeInbox>,
    mut abundance_writer: EventWriter<AbundanceIngestEvent>,
    mut origin_writer: EventWriter<OriginIngestEvent>,
) {
    if inbox.is_empty() {
        return;
    }

    let mut emitted_a = 0usize;
    let mut emitted_o = 0usize;

    if let Some((views, tick)) = inbox.abundance.take() {
        emitted_a = emit_abundance_from_tuples(&mut abundance_writer, views, tick);
    }
    if let Some((views, tick)) = inbox.origin.take() {
        emitted_o = emit_origin_from_tuples(&mut origin_writer, views, tick);
    }

    if emitted_a > 0 || emitted_o > 0 {
        inbox.has_received_external = true;
        info!(
            target: "ra_thor::multi_realm::external_bridge",
            abundance_realms = emitted_a,
            origin_realms = emitted_o,
            "External game bridge drained → live ingest events"
        );
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct ExternalBridgePlugin;

impl Plugin for ExternalBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExternalBridgeInbox>().add_systems(
            Update,
            external_bridge_drain_system.before(crate::multi_realm_harness::abundance_ingest_system),
        );

        info!("ExternalBridgePlugin — game→simulation inbox drain active");
    }
}

// Thunder locked in.
// Push pure tuples into ExternalBridgeInbox; drain emits ingest events.
// Yoi ⚡
