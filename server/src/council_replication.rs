//! server/src/council_replication.rs
//! Powrush-MMO v18.39 Eternal Polish — Council State Replication to Clients
//! Handles authoritative emission of CouncilBloomSyncEvent and periodic council state
//! to the replication layer so clients can update ActionContext (council_engagement, council_trust).
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use crate::council_mercy_trial::CouncilBloomSyncEvent;
use crate::council_session::CouncilSessionManager;

/// Plugin that wires council session events into the replication layer.
pub struct CouncilReplicationPlugin;

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomSyncEvent>()
            .add_systems(Update, replicate_council_bloom_events);
    }
}

/// System that consumes CouncilBloomSyncEvent from CouncilSessionManager
/// and forwards them to the replication layer for client delivery.
/// In production this would serialize and send via WorldServer / QUIC / WebSocket.
fn replicate_council_bloom_events(
    mut events: EventReader<CouncilBloomSyncEvent>,
    // In full implementation: ResMut<WorldServer> or replication channel
) {
    for event in events.read() {
        // Production path:
        // if let Some(world_server) = world_server_res {
        //     world_server.broadcast_council_update(event);
        // }

        tracing::info!(
            "[CouncilReplication] Replicating bloom event | session={} | reason={} | attunement={:.2} | seal={}",
            event.session_id,
            event.trigger_reason,
            event.field.collective_attunement_score,
            event.field.council_mercy_seal
        );

        // The replicated data updates client-side ActionContext:
        // - council_engagement_score
        // - last_council_bloom_tick
        // - divine_whisper_resonance (from field.divine_whisper_flavor)
    }
}

/// Helper to manually trigger a council state sync (useful for testing or forced refresh).
pub fn force_council_state_sync(
    session_manager: &mut CouncilSessionManager,
    current_tick: u64,
) -> Vec<CouncilBloomSyncEvent> {
    session_manager.tick_all(current_tick)
}
