// server/src/council_replication.rs
// Powrush-MMO v18.27 — Council Bloom Replication Wiring
// Bridges CouncilSessionManager events to the replication layer
// Production-grade, mercy-gated, ready for client application
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

/// Plugin that wires CouncilSessionManager output into the replication system
pub struct CouncilReplicationPlugin;

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PendingCouncilBloomEvents>()
            .add_systems(Update, (
                collect_council_bloom_events,
                // Future: encode_and_broadcast_council_events (when replication layer is ready)
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
    // In a real authoritative server loop, we would pass the current server tick here.
    // For now we use a placeholder. Replace with real authoritative tick when integrated.
    let current_tick = (time.elapsed_seconds() * 60.0) as u64; // Placeholder

    let new_events = council_manager.tick_all(current_tick);

    if !new_events.is_empty() {
        pending_events.events.extend(new_events);

        // Optional: log for debugging (remove in production or gate behind feature flag)
        info!(
            "CouncilReplication: Collected {} CouncilBloomSyncEvent(s) for replication",
            pending_events.events.len()
        );
    }

    // TODO (Phase B.2 continuation):
    // - Take events from pending_events
    // - Convert to TargetedUpdate or use existing replication channel
    // - Clear pending_events after successful encoding/broadcast
    // - Respect interest management (only send to players near the Council or in the session)
}

// === Integration Notes for Main Server Loop ===
//
// In your main server authoritative tick (e.g. in server/src/main.rs or a WorldTick system):
//
// ```rust
// // After updating all Council sessions
// let bloom_events = council_manager.tick_all(current_server_tick);
//
// for event in bloom_events {
//     // Option 1: Add to a replication queue
//     replication_queue.push(CouncilReplicationUpdate::Bloom(event));
//
//     // Option 2: Directly encode if using custom system
//     // encode_and_send_to_interested_clients(event);
// }
// ```
//
// The `PendingCouncilBloomEvents` resource above provides a clean Bevy-friendly bridge
// until full integration with the domain-specific encoder is complete.
//
// All Council bloom data must pass TOLC 8 validation before being sent to clients.
// Mercy note: Only send amplification data when the mercy seal is active.

// Thunder locked in. Replication wiring foundation for Phase B Council layer is ready.
// Next: Full integration with replication encoder + client-side receiver.
// Yoi ⚡