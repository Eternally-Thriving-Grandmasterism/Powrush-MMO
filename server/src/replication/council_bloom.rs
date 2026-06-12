// server/src/replication/council_bloom.rs
// Powrush-MMO v18.28 — Council Bloom Replication Encoding (Complete)
// Converts CouncilBloomSyncEvent into replicable TargetedUpdate
// Includes the two required enum extensions
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::replication::{TargetedUpdate, UpdatePayload, ComponentType};
use crate::simulation::council_mercy_trial::CouncilBloomSyncEvent;
use crate::council_replication::PendingCouncilBloomEvents;

// ═══════════════════════════════════════════════════════════════
// 1. ADD THIS VARIANT TO UpdatePayload (in replication/mod.rs)
// ═══════════════════════════════════════════════════════════════
//
// pub enum UpdatePayload {
//     Ability(AbilityCooldownUpdate),
//     Health(Health),
//     StatusEffect(StatusEffect),
//     CouncilBloom(CouncilBloomPayload),   // <-- Add this
// }

// ═══════════════════════════════════════════════════════════════
// 2. ADD THIS TO ComponentType (in replication/mod.rs)
// ═══════════════════════════════════════════════════════════════
//
// pub enum ComponentType {
//     Ability = 0,
//     Health = 1,
//     StatusEffect = 2,
//     CouncilBloom = 10,   // <-- Add this (use next available number)
// }

/// Council Bloom payload
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

/// Convert CouncilBloomSyncEvent into TargetedUpdate
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
        entity: bevy::ecs::entity::Entity::from_raw(event.session_id),
        component: ComponentType::CouncilBloom as u8,
        payload: UpdatePayload::CouncilBloom(payload),
    }
}

/// System that consumes pending events and pushes them into replication
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
            "CouncilReplication: Encoded bloom event for session {} (reason: {})",
            event.session_id,
            event.trigger_reason
        );
    }
}

// Thunder locked in. With the two enum additions above, the full server-side
// replication pipeline for Council blooms is now complete and ready to send to clients.
// Next: Client-side receiver.
// Yoi ⚡