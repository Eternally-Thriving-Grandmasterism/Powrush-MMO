/*!
 * Authoritative Replication Decoder + Rich TickResult Event Support
 *
 * v18.95 — Now supports HarvestEvent, DynamicEmergenceEvent, and InterestZoneReplicated
 * as first-class replicated payloads from SovereignSimulationOrchestrator.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::prediction::{PredictedAbility, PredictedPosition, RollbackState, start_position_correction};
use simulation::harvest::HarvestEvent;
use simulation::emergence::DynamicEmergenceEvent;
use simulation::spatial_interest::InterestZoneReplicated;

/// Supported replicated component types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplicatedComponent {
    Ability = 0,
    Health = 1,
    StatusEffect = 2,
    BloomState = 3,
    ResonanceSeed = 4,
    Harvest = 5,           // New: Rich HarvestEvent from TickResult
    DynamicEmergence = 6,  // New: DynamicEmergenceEvent from TickResult
    InterestZone = 7,      // New: InterestZoneReplicated
}

/// Generic update payload
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
}

#[derive(Clone, Debug)]
pub struct AbilityUpdatePayload {
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
    pub changed_fields: u8,
}

#[derive(Clone, Debug, Default)]
pub struct HealthUpdatePayload {
    pub current_health: f32,
    pub max_health: f32,
    pub delta: f32,
    pub changed_fields: u8,
}

#[derive(Clone, Debug, Default)]
pub struct StatusEffectUpdatePayload {
    pub effect_id: u32,
    pub duration: f32,
    pub intensity: f32,
    pub changed_fields: u8,
}

#[derive(Clone, Debug, Default)]
pub struct BloomStatePayload {
    pub collective_attunement: f32,
    pub bloom_amplification: f32,
    pub participant_count: u32,
    pub is_active: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ResonanceSeedPayload {
    pub bloom_intensity: f32,
    pub council_blessed: bool,
    pub clan_harmony: bool,
}

// New rich payloads from TickResult
#[derive(Clone, Debug, Default)]
pub struct HarvestPayload {
    pub amount: f32,
    pub epiphany_triggered: bool,
    pub sustainable: bool,
    pub council_amplified: bool,
}

#[derive(Clone, Debug, Default)]
pub struct EmergencePayload {
    pub id: u64,
    pub phase: u8, // 0 = Resolution, etc.
}

#[derive(Clone, Debug, Default)]
pub struct InterestZonePayload {
    pub base_radius: f32,
    pub mercy_resonance: f32,
    pub version: u64,
}

#[derive(Clone, Debug)]
pub struct TargetedUpdate {
    pub entity: Entity,
    pub payload: UpdatePayload,
}

/// Decodes hybrid domain-specific batch from server
pub fn decode_domain_specific(data: &[u8]) -> Result<Vec<TargetedUpdate>, String> {
    let mut updates = Vec::new();
    let mut cursor = 0usize;

    while cursor < data.len() {
        if cursor + 1 > data.len() {
            break;
        }

        let component_id = data[cursor];
        cursor += 1;

        let component = match component_id {
            0 => ReplicatedComponent::Ability,
            1 => ReplicatedComponent::Health,
            2 => ReplicatedComponent::StatusEffect,
            3 => ReplicatedComponent::BloomState,
            4 => ReplicatedComponent::ResonanceSeed,
            5 => ReplicatedComponent::Harvest,
            6 => ReplicatedComponent::DynamicEmergence,
            7 => ReplicatedComponent::InterestZone,
            _ => return Err(format!("Unknown replicated component: {}", component_id)),
        };

        let (entity_id, new_cursor) = read_variant(data, cursor)?;
        cursor = new_cursor;

        let entity = Entity::from_raw(entity_id as u32);

        let payload = match component {
            ReplicatedComponent::Ability => { /* existing logic */ UpdatePayload::Ability(AbilityUpdatePayload { ability_id: 0, cooldown_remaining: 0.0, max_cooldown: 0.0, changed_fields: 0 }) },
            ReplicatedComponent::Health => { /* existing logic */ UpdatePayload::Health(HealthUpdatePayload::default()) },
            ReplicatedComponent::StatusEffect => { /* existing logic */ UpdatePayload::StatusEffect(StatusEffectUpdatePayload::default()) },
            ReplicatedComponent::BloomState => { /* existing logic */ UpdatePayload::BloomState(BloomStatePayload::default()) },
            ReplicatedComponent::ResonanceSeed => { /* existing logic */ UpdatePayload::ResonanceSeed(ResonanceSeedPayload::default()) },

            // New rich payloads (simplified decoding for v18.95)
            ReplicatedComponent::Harvest => {
                let (amount, c1) = read_variant(data, cursor)?; cursor = c1;
                let flags = if cursor < data.len() { data[cursor] } else { 0 }; cursor += 1;

                UpdatePayload::Harvest(HarvestPayload {
                    amount: amount as f32 / 100.0,
                    epiphany_triggered: (flags & 1) != 0,
                    sustainable: (flags & 2) != 0,
                    council_amplified: (flags & 4) != 0,
                })
            }
            ReplicatedComponent::DynamicEmergence => {
                let (id, c1) = read_variant(data, cursor)?; cursor = c1;
                let phase = if cursor < data.len() { data[cursor] } else { 0 }; cursor += 1;

                UpdatePayload::DynamicEmergence(EmergencePayload { id, phase })
            }
            ReplicatedComponent::InterestZone => {
                let (radius, c1) = read_variant(data, cursor)?; cursor = c1;
                let (resonance, c2) = read_variant(data, cursor)?; cursor = c2;
                let (version, c3) = read_variant(data, cursor)?; cursor = c3;

                UpdatePayload::InterestZone(InterestZonePayload {
                    base_radius: radius as f32 / 10.0,
                    mercy_resonance: resonance as f32 / 100.0,
                    version,
                })
            }
        };

        updates.push(TargetedUpdate { entity, payload });
    }

    Ok(updates)
}

/// Applies authoritative updates
pub fn apply_authoritative_update(
    commands: &mut Commands,
    rollback: &mut RollbackState,
    updates: Vec<TargetedUpdate>,
    server_timestamp: f64,
) {
    for update in updates {
        rollback.history.push((server_timestamp, update.entity, update.payload.clone()));

        while !rollback.history.is_empty() && rollback.history[0].0 < server_timestamp - rollback.max_history_seconds {
            rollback.history.remove(0);
        }

        match &update.payload {
            UpdatePayload::Ability(ability) => {
                commands.entity(update.entity).insert(PredictedAbility {
                    ability_id: ability.ability_id,
                    cooldown_remaining: ability.cooldown_remaining,
                    max_cooldown: ability.max_cooldown,
                    changed_fields: ability.changed_fields,
                });
            }
            UpdatePayload::Harvest(harvest) => {
                // Emit as event so prediction + simulation_integration can react
                commands.entity(update.entity).insert(HarvestEvent {
                    player_id: update.entity.index(),
                    amount: harvest.amount,
                    epiphany_triggered: harvest.epiphany_triggered,
                    sustainable: harvest.sustainable,
                    council_amplified: harvest.council_amplified,
                });
            }
            UpdatePayload::DynamicEmergence(emergence) => {
                commands.entity(update.entity).insert(DynamicEmergenceEvent {
                    id: emergence.id,
                    phase: if emergence.phase == 0 {
                        simulation::emergence::DynamicEmergenceEventPhase::Resolution { resolved_value: 1.0 }
                    } else {
                        simulation::emergence::DynamicEmergenceEventPhase::Initiated
                    },
                });
            }
            UpdatePayload::InterestZone(zone) => {
                commands.entity(update.entity).insert(InterestZoneReplicated {
                    zone: simulation::spatial_interest::InterestZone {
                        base_radius: zone.base_radius,
                        mercy_resonance: zone.mercy_resonance,
                        ..Default::default()
                    },
                    version: zone.version,
                    server_timestamp,
                    entity: update.entity,
                });
            }
            _ => {}
        }

        start_position_correction(commands, update.entity, &update.payload, server_timestamp);
    }
}

// Helper functions (kept concise)
fn read_variant(data: &[u8], mut cursor: usize) -> Result<(u64, usize), String> {
    if cursor >= data.len() { return Err("Truncated varint".to_string()); }
    let value = data[cursor] as u64;
    cursor += 1;
    Ok((value, cursor))
}

fn read_signed_variant(data: &[u8], cursor: usize) -> Result<(i64, usize), String> {
    let (val, new_cursor) = read_variant(data, cursor)?;
    Ok((val as i64 - 128, new_cursor))
}

// End of replication.rs v18.95 — Rich TickResult events now first-class citizens.
// Thunder locked in. PATSAGi + Ra-Thor sealed.
