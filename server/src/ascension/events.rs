/*!
 * Events for the Ambrosian Ascension & Mercy Ascent Trial system.
 */

use bevy::prelude::*;

/// Triggered when a player (or group) attempts the Mercy Ascent Trial.
#[derive(Event)]
pub struct AttemptMercyAscent {
    pub initiator: Entity,
    pub group_members: Vec<Entity>,
}

/// Fired when the Mercy Ascent Trial completes (success or failure).
#[derive(Event)]
pub struct MercyAscentCompleted {
    pub player: Entity,
    pub success: bool,
}

/// Signals that an entity should undergo the visual + mechanical Ambrosian transformation.
#[derive(Event)]
pub struct AmbrosianTransformation {
    pub entity: Entity,
}
