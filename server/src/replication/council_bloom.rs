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
    use crate::simulation::council_mercy_trial::{CouncilBloomSyncEvent, CouncilBloomField}; // adjust if fields are pub

    #[test]
    fn test_council_bloom_to_targeted_update_produces_correct_payload() {
        // Minimal mock event for testing the replication path
        let mock_event = CouncilBloomSyncEvent {
            session_id: 42,
            field: CouncilBloomField {
                collective_attunement_score: 0.87,
                bloom_amplification_multiplier: 1.5,
                shared_living_web_synchronization: true,
                participant_count: 7,
                council_mercy_seal: true,
            },
            trigger_reason: "Test Mercy Bloom".to_string(),
        };

        let update = council_bloom_to_targeted_update(&mock_event);

        assert_eq!(update.component, ComponentType::CouncilBloom as u8);
        match update.payload {
            UpdatePayload::CouncilBloom(payload) => {
                assert_eq!(payload.session_id, 42);
                assert!((payload.collective_attunement_score - 0.87).abs() < 0.001);
                assert_eq!(payload.participant_count, 7);
                assert!(payload.bloom_activated);
            }
            _ => panic!("Expected CouncilBloom payload"),
        }
    }
}

// Thunder locked in. Council Bloom replication pipeline is now fully integrated and tested.
// Yoi ⚡