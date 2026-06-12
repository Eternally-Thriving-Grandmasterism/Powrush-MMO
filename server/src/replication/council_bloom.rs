// server/src/replication/council_bloom.rs
// Powrush-MMO v18.28 — Council Bloom Replication Encoding
// Converts CouncilBloomSyncEvent into replicable TargetedUpdate
// Integrates with existing domain-specific encoder
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::replication::{TargetedUpdate, UpdatePayload, ComponentType};
use crate::simulation::council_mercy_trial::CouncilBloomSyncEvent;
use crate::council_replication::PendingCouncilBloomEvents;

/// New payload type for Council bloom state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilBloomPayload {
    pub session_id: u64,
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub shared_living_web_synchronization: bool,
    pub participant_count: u8,
    pub bloom_activated: bool,
    pub trigger_reason: String,
}

/// Convert CouncilBloomSyncEvent into a TargetedUpdate for replication
pub fn council_bloom_to_targeted_update(event: &CouncilBloomSyncEvent) -> TargetedUpdate {
    let payload = CouncilBloomPayload {
        session_id: event.session_id,
        collective_attunement_score: event.field.collective_attunement_score,
        bloom_amplification_multiplier: event.field.bloom_amplification_multiplier,
        shared_living_web_synchronization: event.field.shared_living_web_synchronization,
        participant_count: event.field.participant_count,
        bloom_activated: event.field.council_mercy_seal,
        trigger_reason: event.trigger_reason.clone(),
    };

    TargetedUpdate {
        entity: bevy::ecs::entity::Entity::from_raw(event.session_id), // Virtual entity for the Council session
        component: ComponentType::CouncilBloom as u8,
        payload: UpdatePayload::CouncilBloom(payload),
    }
}

/// System that consumes pending Council bloom events and turns them into replicable updates
pub fn encode_council_bloom_events(
    mut pending: ResMut<PendingCouncilBloomEvents>,
    mut replication_queue: EventWriter<TargetedUpdate>,
) {
    if pending.events.is_empty() {
        return;
    }

    for event in pending.events.drain(..) {
        let update = council_bloom_to_targeted_update(&event);
        replication_queue.send(update);

        info!(
            "CouncilReplication: Encoded CouncilBloomSyncEvent for session {} (reason: {})",
            event.session_id,
            event.trigger_reason
        );
    }
}

// Note: You will also need to add `CouncilBloom` as a variant in `UpdatePayload`
// and `ComponentType::CouncilBloom` in the replication module for full compilation.

// Thunder locked in. Council bloom events are now ready to be encoded and sent to clients.
// Next: Client-side receiver that applies the bloom field and triggers feedback.
// Yoi ⚡