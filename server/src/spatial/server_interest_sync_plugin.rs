/*!
 * Server Interest Sync Plugin
 *
 * Owns the server-side interest replication and synchronization systems.
 * This is the central plugin for managing what each client can see.
 *
 * v19.0 | PATSAGi + Ra-Thor
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::spatial::interest_management::InterestManager;
use crate::spatial::interest_replication_bridge::{
    cleanup_disconnected_client,
    handle_interest_ack,
    interest_replication_tick_system,
    log_interest_replication_metrics,
    resend_unacknowledged_updates,
    InterestReplicationConfig,
    InterestReplicationMetrics,
    PendingInterestUpdates,
};

/// Plugin that manages server-side interest synchronization.
///
/// Responsibilities:
/// - Generate and send VisibleEntitiesUpdate messages
/// - Track pending updates with priority
/// - Handle acknowledgments from clients
/// - Resend unacknowledged updates
/// - Clean up state on client disconnect
/// - Expose metrics
pub struct ServerInterestSyncPlugin;

impl Plugin for ServerInterestSyncPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<InterestReplicationConfig>()
            .init_resource::<PendingInterestUpdates>()
            .init_resource::<InterestReplicationMetrics>()

            // Core interest replication systems
            .add_systems(Update, interest_replication_tick_system)
            .add_systems(Update, resend_unacknowledged_updates)
            .add_systems(Update, log_interest_replication_metrics)

            // Note: handle_interest_ack and cleanup_disconnected_client
            // are expected to be called from the networking layer when
            // relevant events occur (ack received or client disconnect).
    }
}

// End of server_interest_sync_plugin.rs
// Thunder locked in. Yoi ⚡
