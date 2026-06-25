/// Generic update payload for replication wire format
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct AbilityUpdatePayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct HealthUpdatePayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct StatusEffectUpdatePayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct BloomStatePayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct ResonanceSeedPayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct HarvestPayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct EmergencePayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct InterestZonePayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct CouncilSessionPayload { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct CouncilBloomPayload { /* ... */ }

/// Wire format payload for RBE inventory updates (distribution, transfers, etc.)
#[derive(Clone, Debug, Default)]
pub struct RbeInventoryUpdatePayload {
    pub resource_type: String,
    pub amount: f32,
    pub delta: f32,
}

// (rest of file unchanged)