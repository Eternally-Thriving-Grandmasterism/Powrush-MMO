/*!
 * client/src/faction.rs
 *
 * Shared Faction Components for client-side replication and UI.
 * v1.0 | Centralized definitions to eliminate duplication.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

/// Marks a player as belonging to a faction (replicated from server).
#[derive(Component, Clone, Debug)]
pub struct FactionMembership {
    pub faction_id: u64,
}

/// Tracks a player's standing/reputation with a specific faction (replicated from server).
#[derive(Component, Clone, Debug)]
pub struct FactionStanding {
    pub faction_id: u64,
    pub standing: f32,
}
