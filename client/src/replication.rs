/// Generic update payload for replication wire format
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UpdatePayload {
    // ... existing variants ...
    CouncilBloom(CouncilBloomPayload),
    // ...
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CouncilBloomPayload { /* ... */ }

#[derive(Event, Clone, Debug)]
pub struct CouncilBloomReceived {
    pub payload: CouncilBloomPayload,
}

/// Plugin that registers all Council-related replication events
pub struct CouncilReplicationPlugin;

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>();
        // Future: add more council replication events here
        info!("[Client] CouncilReplicationPlugin registered CouncilBloomReceived event");
    }
}

pub fn decode_and_apply_council_bloom(
    payload: &CouncilBloomPayload,
    mut bloom_events: EventWriter<CouncilBloomReceived>,
) {
    if payload.bloom_activated {
        info!(
            "[Client] Council Bloom received! Session {} | Attunement: {:.2}",
            payload.session_id,
            payload.collective_attunement_score
        );
        bloom_events.send(CouncilBloomReceived { payload: payload.clone() });
    }
}

// Example usage in client main or a central client plugin:
// app.add_plugins(CouncilReplicationPlugin);
