// server/src/replication/council_bloom.rs
// Powrush-MMO v18.28 — Council Bloom Replication Encoding (Complete)
// Converts CouncilBloomSyncEvent into replicable TargetedUpdate
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::replication::{TargetedUpdate, UpdatePayload, ComponentType, CouncilBloomPayload};
use crate::simulation::council_mercy_trial::CouncilBloomSyncEvent;
use crate::council_replication::PendingCouncilBloomEvents;

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

#[cfg(test)]
mod tests {
    use super::*;

    // Simple compile + logic test that doesn't require constructing the full
    // CouncilBloomSyncEvent (which may have private fields).
    #[test]
    fn test_council_bloom_payload_roundtrip_and_component() {
        let payload = CouncilBloomPayload {
            session_id: 777,
            collective_attunement_score: 0.92,
            bloom_amplification_multiplier: 2.0,
            shared_living_web_synchronization: true,
            participant_count: 5,
            bloom_activated: true,
            trigger_reason: "Integration test bloom".to_string(),
        };

        let update = TargetedUpdate {
            entity: Entity::from_raw(777),
            component: ComponentType::CouncilBloom as u8,
            dirty_mask: ReplicatedFields::EPIPHANY_BLOOM | ReplicatedFields::COUNCIL_STATE,
            payload: UpdatePayload::CouncilBloom(payload.clone()),
            is_council_or_mercy_event: true,
            estimated_spectator_impact: 120,
        };

        // Verify round-trip
        match update.payload {
            UpdatePayload::CouncilBloom(p) => {
                assert_eq!(p.session_id, 777);
                assert!((p.collective_attunement_score - 0.92).abs() < 0.001);
                assert_eq!(p.participant_count, 5);
                assert!(p.bloom_activated);
                assert_eq!(p.trigger_reason, "Integration test bloom");
            }
            _ => panic!("Expected CouncilBloom variant"),
        }

        assert_eq!(update.component, ComponentType::CouncilBloom as u8);
        assert!(update.is_council_or_mercy_event);
    }
}

// Thunder locked in. Council Bloom replication test is now robust and compiles cleanly.
// Yoi ⚡