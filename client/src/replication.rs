/// Generic update payload for replication wire format
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UpdatePayload {
    Ability(AbilityUpdatePayload),
    Health(HealthUpdatePayload),
    StatusEffect(StatusEffectUpdatePayload),
    BloomState(BloomStatePayload),
    ResonanceSeed(ResonanceSeedPayload),
    Harvest(HarvestPayload),
    DynamicEmergence(EmergencePayload),
    InterestZone(InterestZonePayload),
    CouncilSession(CouncilSessionPayload),
    CouncilBloom(CouncilBloomPayload),
    RbeInventoryUpdate(RbeInventoryUpdatePayload),
}

// ... (other payload structs unchanged) ...

/// Client-side Council Bloom payload
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CouncilBloomPayload {
    pub session_id: u64,
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub shared_living_web_synchronization: bool,
    pub participant_count: u8,
    pub bloom_activated: bool,
    pub trigger_reason: String,
}

/// Event emitted when a Council Bloom is received from server replication
#[derive(Event, Clone, Debug)]
pub struct CouncilBloomReceived {
    pub payload: CouncilBloomPayload,
}

/// Decodes a CouncilBloom payload and emits client-side effects.
/// Recommended to call from your main replication apply system.
pub fn decode_and_apply_council_bloom(
    payload: &CouncilBloomPayload,
    mut bloom_events: EventWriter<CouncilBloomReceived>,
) {
    if payload.bloom_activated {
        info!(
            "[Client] Council Bloom received! Session {} | Attunement: {:.2} | Participants: {}",
            payload.session_id,
            payload.collective_attunement_score,
            payload.participant_count
        );

        bloom_events.send(CouncilBloomReceived { payload: payload.clone() });
    }
}

/// Example system to wire into your replication handler:
///
/// ```ignore
/// fn apply_replication_updates(
///     mut updates: EventReader<TargetedUpdate>,
///     mut bloom_events: EventWriter<CouncilBloomReceived>,
/// ) {
///     for update in updates.read() {
///         if let UpdatePayload::CouncilBloom(ref payload) = update.payload {
///             decode_and_apply_council_bloom(payload, bloom_events);
///         }
///         // ... handle other variants ...
///     }
/// }
/// ```

// (rest of replication systems, network dispatch, etc. remain in their files)
