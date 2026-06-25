/*!
 * Shared Interest Types
 *
 * v19.2 — Added InterestAck for acknowledgment / resend logic.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use serde::{Deserialize, Serialize};

/// Network message sent from server to client with current visible entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleEntitiesUpdate {
    pub client_entity_id: u64,
    pub visible_entity_ids: Vec<u64>,
    pub server_tick: u64,
}

/// Small acknowledgment sent from client back to server.
/// Used for explicit resend logic on top of reliable channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestAck {
    pub client_entity_id: u64,
    pub acknowledged_tick: u64,
}

// End of simulation/src/interest.rs
// Shared types including acknowledgment for resend logic.
// Thunder locked in. Yoi ⚡
