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
    /// New: Inventory updates from RBE distribution (triggered by RbeInventoryUpdatedEvent on server)
    RbeInventoryUpdate(RbeInventoryUpdatePayload),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AbilityUpdatePayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HealthUpdatePayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StatusEffectUpdatePayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BloomStatePayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResonanceSeedPayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HarvestPayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EmergencePayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InterestZonePayload { /* ... */ }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CouncilSessionPayload { /* ... */ }

/// Client-side Council Bloom payload (matches server CouncilBloomPayload)
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

/// Wire format payload for RBE inventory updates (distribution, transfers, etc.)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RbeInventoryUpdatePayload {
    pub resource_type: String,
    pub amount: f32,
    pub delta: f32,
}

/// Decodes a CouncilBloom payload and emits client-side effects / UI events.
/// Call this from your main replication apply system when receiving UpdatePayload::CouncilBloom.
pub fn decode_and_apply_council_bloom(
    payload: &CouncilBloomPayload,
    // TODO: pass commands, event writers, or UI resources as needed
) {
    if payload.bloom_activated {
        // Example: trigger client bloom VFX, UI update, sound, etc.
        info!(
            "[Client] Council Bloom activated! Session {} | Attunement: {:.2} | Participants: {} | Reason: {}",
            payload.session_id,
            payload.collective_attunement_score,
            payload.participant_count,
            payload.trigger_reason
        );

        // TODO: send client event e.g.
        // commands.spawn(... bloom effect ...);
        // event_writer.send(CouncilBloomReceived { ... });
    }
}

// (rest of file unchanged - existing replication systems, network handlers, etc.)
