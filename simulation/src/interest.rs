/*!
 * Shared Interest Types
 *
 * This module contains types used by both server and client for interest/visibility.
 * VisibleEntitiesUpdate is the network message sent from server to client.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use serde::{Deserialize, Serialize};

/// Network message sent from server to client containing the current set of visible entities.
/// This is the single source of truth type used on both sides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleEntitiesUpdate {
    pub client_entity_id: u64,
    pub visible_entity_ids: Vec<u64>,
    pub server_tick: u64,
}

// End of simulation/src/interest.rs
// Shared type for replication bridge.
// Thunder locked in. Yoi ⚡
