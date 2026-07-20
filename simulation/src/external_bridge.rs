//! simulation/src/external_bridge.rs
//! Bevy adapter for game-crate multi_realm_bridge pure payloads.
//! v21.64.0 — HostBridgeAutoPublish concrete host path
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
    MultiRealmHarness, derive_abundance_from_harness, derive_origin_from_harness,
};

// ============================================================================
// INBOX
// ============================================================================

#[derive(Resource, Clone, Debug, Default)]
pub struct ExternalBridgeInbox {
    pub abundance: Option<(Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>, u64)>,
    pub origin: Option<(Vec<(u8, f32, u32)>, u64)>,
    pub has_received_external: bool,
}

impl ExternalBridgeInbox {
    pub fn push_abundance(
        &mut self,
        views: Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>,
        tick_ms: u64,
    ) {
        if !views.is_empty() {
            self.abundance = Some((views, tick_ms));
        }
    }

    pub fn push_origin(&mut self, views: Vec<(u8, f32, u32)>, tick_ms: u64) {
        if !views.is_empty() {
            self.origin = Some((views, tick_ms));
        }
    }

    pub fn push_dual(
        &mut self,
        abundance_views: Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>,
        abundance_tick: u64,
        origin_views: Vec<(u8, f32, u32)>,
        origin_tick: u64,
    ) {
        self.push_abundance(abundance_views, abundance_tick);
        self.push_origin(origin_views, origin_tick);
    }

    pub fn is_empty(&self) -> bool {
        self.abundance.is_none() && self.origin.is_none()
    }
}

// ============================================================================
// SHARED-APP SOURCE — host fill point
// ============================================================================

#[derive(Resource, Clone, Debug, Default)]
pub struct SharedAppBridgeSource {
    pub abundance: Option<(Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>, u64)>,
    pub origin: Option<(Vec<(u8, f32, u32)>, u64)>,
    pub dirty: bool,
    pub publish_count: u64,
}

impl SharedAppBridgeSource {
    pub fn set_dual(
        &mut self,
        abundance_views: Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>,
        abundance_tick: u64,
        origin_views: Vec<(u8, f32, u32)>,
        origin_tick: u64,
    ) {
        if !abundance_views.is_empty() {
            self.abundance = Some((abundance_views, abundance_tick));
        }
        if !origin_views.is_empty() {
            self.origin = Some((origin_views, origin_tick));
        }
        self.dirty = true;
    }

    pub fn set_abundance(
        &mut self,
        views: Vec<(u8, u32, f32, f32, f32, f32, u32, u32)>,
        tick_ms: u64,
    ) {
        if !views.is_empty() {
            self.abundance = Some((views, tick_ms));
            self.dirty = true;
        }
    }

    pub fn set_origin(&mut self, views: Vec<(u8, f32, u32)>, tick_ms: u64) {
        if !views.is_empty() {
            self.origin = Some((views, tick_ms));
            self.dirty = true;
        }
    }
}

// ============================================================================
// HOST AUTO-PUBLISH — concrete host path when ServerTickLoop not yet NonSend
// ============================================================================

/// Soft host stand-in: publishes harness-derived dual through SharedAppBridgeSource.
///
/// When a binary later owns `ServerTickLoop`, replace this by calling
/// `source.set_dual(...)` from dual_payload() after each authoritative tick
/// and set `enabled = false` on this config.
#[derive(Resource, Clone, Debug)]
pub struct HostBridgeAutoPublish {
    pub enabled: bool,
    pub interval_secs: f32,
}

impl Default for HostBridgeAutoPublish {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 2.5,
        }
    }
}

pub fn host_bridge_auto_publish_system(
    config: Res<HostBridgeAutoPublish>,
    harness: Res<MultiRealmHarness>,
    mut source: ResMut<SharedAppBridgeSource>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    if !config.enabled {
        return;
    }
    if harness.realms.is_empty() {
        return;
    }

    let now = time.elapsed_seconds();
    if (now - *last) < config.interval_secs {
        return;
    }
    *last = now;

    // Prefer living activity; still publish soft dual so EXTERNAL path is exercised
    let tick = (now * 1000.0) as u64;
    let abundance = derive_abundance_from_harness(&harness);
    let origin = derive_origin_from_harness(&harness);

    if abundance.is_empty() && origin.is_empty() {
        return;
    }

    let a_views: Vec<_> = abundance
        .into_iter()
        .map(|v| {
            (
                v.realm_id,
                v.node_count,
                v.total_current_yield,
                v.average_sustainability,
                v.average_abundance_flow,
                v.average_stress,
                v.restricted_node_count,
                v.thriving_node_count,
            )
        })
        .collect();

    let o_views: Vec<_> = origin
        .into_iter()
        .map(|v| (v.realm_id, v.total_amount, v.resource_types))
        .collect();

    source.set_dual(a_views, tick, o_views, tick);
}

pub fn shared_app_bridge_publish_system(
    mut source: ResMut<SharedAppBridgeSource>,
    mut inbox: ResMut<ExternalBridgeInbox>,
) {
    if !source.dirty {
        return;
    }

    let mut pushed_a = false;
    let mut pushed_o = false;

    if let Some((views, tick)) = source.abundance.take() {
        inbox.push_abundance(views, tick);
        pushed_a = true;
    }
    if let Some((views, tick)) = source.origin.take() {
        inbox.push_origin(views, tick);
        pushed_o = true;
    }

    source.dirty = false;

    if pushed_a || pushed_o {
        source.publish_count = source.publish_count.saturating_add(1);
        info!(
            target: "ra_thor::multi_realm::shared_app_bridge",
            publish_count = source.publish_count,
            abundance = pushed_a,
            origin = pushed_o,
            "SharedAppBridgeSource published → ExternalBridgeInbox"
        );
    }
}

// ============================================================================
// EMIT + DRAIN
// ============================================================================

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
        app.init_resource::<ExternalBridgeInbox>()
            .init_resource::<SharedAppBridgeSource>()
            .init_resource::<HostBridgeAutoPublish>()
            .add_systems(
                Update,
                (
                    host_bridge_auto_publish_system
                        .before(shared_app_bridge_publish_system),
                    shared_app_bridge_publish_system
                        .before(external_bridge_drain_system),
                    external_bridge_drain_system
                        .before(crate::multi_realm_harness::abundance_ingest_system),
                ),
            );

        info!("ExternalBridgePlugin — HostBridgeAutoPublish + inbox drain active");
    }
}

// Thunder locked in.
// Host path: auto-publish (stand-in) or set_dual from ServerTickLoop.
// Yoi ⚡
