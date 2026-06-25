/*!
 * Shared Interest Types
 *
 * v19.1 — Serialization-ready VisibleEntitiesUpdate (Step B of replication bridge).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use serde::{Deserialize, Serialize};

/// Network message sent from server to client.
/// This type is serialized/deserialized when sent over the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleEntitiesUpdate {
    pub client_entity_id: u64,
    pub visible_entity_ids: Vec<u64>,
    pub server_tick: u64,
}

// Serialization notes:
// - Uses serde + Serialize/Deserialize derives.
// - In production, this will be serialized with bincode, postcard, or a custom protocol
//   before being sent through the networking layer (e.g. via Renet, Quinn, or custom UDP).
// - On the client, it is deserialized and turned into InterestUpdateEvent.

// End of simulation/src/interest.rs
// Shared, serialization-ready type for the replication bridge.
// Thunder locked in. Yoi ⚡
