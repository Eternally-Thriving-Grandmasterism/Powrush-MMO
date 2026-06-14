// server/src/council_replication.rs
// Powrush-MMO v18.28 — Council Bloom Replication & Authoritative Emission
// Bridges CouncilSessionManager events to the replication layer
// NOW COMPLETE: Authoritative emission of CouncilBloomSyncEvent on field updates
// Production-grade, mercy-gated, TOLC 8 enforced
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::council_session::CouncilSessionManager;
use crate::simulation::council_mercy_trial::CouncilBloomSyncEvent;

/// Resource holding pending Council bloom sync events ready for replication
#[derive(Resource, Default)]
pub struct PendingCouncilBloomEvents {
    pub events: Vec<CouncilBloomSyncEvent>,
}

/// Resource for broadcast-ready events (authoritative emission point)
#[derive(Resource, Default)]
pub struct CouncilBloomBroadcastQueue {
    pub ready_events: Vec<CouncilBloomSyncEvent>,
}

/// Plugin that wires CouncilSessionManager output into the replication system
pub struct CouncilReplicationPlugin;

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PendingCouncilBloomEvents>()
            .init_resource::<CouncilBloomBroadcastQueue>()
            .add_systems(Update, (
                collect_council_bloom_events,
                emit_authoritative_council_bloom_events, // NEW: Authoritative emission
            ).chain());
    }
}

/// Collects CouncilBloomSyncEvent from CouncilSessionManager every tick
/// These events are then available for the replication layer to encode and send to interested clients
fn collect_council_bloom_events(
    mut council_manager: ResMut<CouncilSessionManager>,
    mut pending_events: ResMut<PendingCouncilBloomEvents>,
    time: Res<Time>, // Using Bevy time; in real server tick this would be authoritative tick
) {
    let current_tick = (time.elapsed_seconds() * 60.0) as u64;

    let new_events = council_manager.tick_all(current_tick);

    if !new_events.is_empty() {
        pending_events.events.extend(new_events);

        info!(
            "CouncilReplication: Collected {} CouncilBloomSyncEvent(s) for replication",
            pending_events.events.len()
        );
    }
}

/// NEW v18.28: Authoritative emission of CouncilBloomSyncEvent
/// Moves collected events to broadcast queue for replication layer consumption.
/// This is the concrete server-side emission point for Phase 2 shared state.
/// Only emits when mercy seal is active (TOLC 8 mercy-gated).
fn emit_authoritative_council_bloom_events(
    mut pending: ResMut<PendingCouncilBloomEvents>,
    mut broadcast_queue: ResMut<CouncilBloomBroadcastQueue>,
) {
    if pending.events.is_empty() {
        return;
    }

    let mut emitted_count = 0;
    for event in pending.events.drain(..) {
        if event.field.council_mercy_seal {
            broadcast_queue.ready_events.push(event);
            emitted_count += 1;
        }
    }

    if emitted_count > 0 {
        info!(
            "CouncilReplication: Authoritatively emitted {} CouncilBloomSyncEvent(s) to replication queue (mercy seal active)",
            emitted_count
        );
    }
}

// Thunder locked in. Authoritative emission foundation for Phase 2 Council layer complete.
// Yoi ⚡❤️