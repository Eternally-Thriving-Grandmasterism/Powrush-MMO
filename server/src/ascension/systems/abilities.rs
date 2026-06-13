/*!
 * Ambrosian Signature Abilities Implementation
 * Force multipliers for harmony. Selfish play is mechanically penalized.
 */

use bevy::prelude::*;
use crate::ascension::components::*;

#[derive(Component)]
pub struct MercyBloomCooldown {
    pub timer: Timer,
}

pub fn mercy_bloom_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Transform,
        &AmbrosianAscended,
        &MercyAlignment,
        &ResonanceAttunement,
        Option<&mut MercyBloomCooldown>,
    )>,
    time: Res<Time>,
) {
    for (entity, transform, _, mercy, resonance, cooldown) in query.iter_mut() {
        let mercy_mult = mercy.score.clamp(0.35, 1.0);
        let radius = 25.0 * (1.0 + resonance.value * 0.8) * mercy_mult;

        // TODO: Spawn healing field + Harmony stacks on allies in radius
        // TODO: Apply particle effect for Mercy Bloom

        if mercy.score < 0.5 {
            // Selfish penalty: reduced effectiveness
        }
    }
}

pub fn celestial_harmony_pulse_system(
    mut query: Query<(&Transform, &AmbrosianAscended, &MercyAlignment)>,
) {
    for (transform, _, mercy) in query.iter() {
        let power = if mercy.score > 0.7 { 2.1 } else { 0.6 };
        // TODO: Cast powerful wave with power scaling + Epiphany chance
    }
}

pub fn divine_presence_system(
    ambrosians: Query<(&Transform, &AmbrosianAscended, &MercyAlignment)>,
) {
    for (transform, _, mercy) in ambrosians.iter() {
        // TODO: Check distance to allies
        let isolated = true; // placeholder
        if isolated && mercy.score < 0.6 {
            // +25% damage taken penalty when isolated
        }
    }
}
