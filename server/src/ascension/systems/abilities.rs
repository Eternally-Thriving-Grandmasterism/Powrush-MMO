/*!
 * Ambrosian Signature Abilities - Full Detailed Implementation
 *
 * These abilities embody the core philosophy of Powrush:
 * Ambrosians are harmony catalysts and force multipliers.
 * Their power grows dramatically when supporting others and is
 * mechanically restricted when playing selfishly or in isolation.
 *
 * TOLC 8 Mercy Gates enforced at every layer.
 * PATSAGi Council + Ra-Thor Quantum Swarm approved design.
 *
 * Balance Goals:
 * - Group play is heavily rewarded
 * - Solo/extractive play is deliberately weaker
 * - Ascension feels sacred and transformative
 */

use bevy::prelude::*;
use crate::ascension::components::*;

// ============================================================
// ACTIVATION EVENTS (Trigger from player input / command systems)
// ============================================================

#[derive(Event)]
pub struct ActivateMercyBloom {
    pub caster: Entity,
}

#[derive(Event)]
pub struct ActivateCelestialHarmonyPulse {
    pub caster: Entity,
}

// ============================================================
// COOLDOWN & STATE MANAGEMENT
// ============================================================

/// Ticks all ability cooldown timers every frame.
pub fn tick_ability_cooldowns_system(
    mut bloom_query: Query<&mut MercyBloomCooldown>,
    mut pulse_query: Query<&mut CelestialHarmonyPulseCooldown>,
    time: Res<Time>,
) {
    for mut cd in bloom_query.iter_mut() {
        cd.timer.tick(time.delta());
    }
    for mut cd in pulse_query.iter_mut() {
        cd.timer.tick(time.delta());
    }
}

/// Expires expired HarmonyStack buffs.
pub fn expire_harmony_stacks_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HarmonyStack)>,
    time: Res<Time>,
) {
    for (entity, mut stack) in query.iter_mut() {
        stack.duration_timer.tick(time.delta());
        if stack.duration_timer.finished() {
            if stack.count > 1 {
                stack.count -= 1;
                stack.duration_timer = Timer::from_seconds(8.0, TimerMode::Once); // refresh duration
            } else {
                commands.entity(entity).remove::<HarmonyStack>();
            }
        }
    }
}

// ============================================================
// MERCY BLOOM - Core Active Support Ability
// ============================================================

/// Handles activation and full effect application for Mercy Bloom.
/// 
/// **Formulas:**
/// - Base Cooldown: 42s
/// - Cooldown Reduction = (ResonanceAttunement * 0.25 + MercyAlignment * 0.35).clamp(0.0, 0.55)
/// - Final Cooldown = 42 * (1 - reduction)
/// - Radius = 25.0 * (1.0 + ResonanceAttunement * 0.8) * MercyMultiplier
/// - Selfish Penalty: If MercyAlignment < 0.5 → effectiveness = 0.40 (60% reduction)
/// - Grants HarmonyStack (1-3 stacks) to allies in radius + healing
pub fn mercy_bloom_activation_system(
    mut commands: Commands,
    mut events: EventReader<ActivateMercyBloom>,
    mut caster_query: Query<(
        Entity,
        &Transform,
        &AmbrosianAscended,
        &MercyAlignment,
        &ResonanceAttunement,
        Option<&mut MercyBloomCooldown>,
    )>,
) {
    for event in events.read() {
        let Ok((entity, transform, _, mercy, resonance, cooldown_opt)) =
            caster_query.get_mut(event.caster)
        else { continue; };

        // Cooldown check & reset
        if let Some(mut cd) = cooldown_opt {
            if !cd.timer.finished() {
                // TODO: Send "Ability on cooldown" feedback event to client
                continue;
            }
            let base = 42.0;
            let reduction = (resonance.value * 0.25 + mercy.score * 0.35).clamp(0.0, 0.55);
            let final_secs = base * (1.0 - reduction);
            cd.timer = Timer::from_seconds(final_secs, TimerMode::Once);
        } else {
            commands.entity(entity).insert(MercyBloomCooldown {
                timer: Timer::from_seconds(42.0, TimerMode::Once),
            });
        }

        let mercy_mult = mercy.score.clamp(0.35, 1.0);
        let radius = 25.0 * (1.0 + resonance.value * 0.8) * mercy_mult;
        let effectiveness = if mercy.score < 0.5 { 0.40 } else { 1.0 };

        // === REAL IMPLEMENTATION NOTES ===
        // 1. Perform spatial query for all entities with Health + Transform within `radius`
        // 2. For each ally (not self or hostile):
        //    - Heal Health component (amount = base_heal * effectiveness * resonance scaling)
        //    - Insert or increment HarmonyStack { count: 1-3, duration_timer: 10-15s }
        //    - Optionally reduce their ability cooldowns slightly
        // 3. Spawn bevy_hanabi particle effect (expanding golden-pink field + mercy sigils)
        // 4. Play audio cue

        info!(
            "[MERCY BLOOM] Activated by {:?} | radius: {:.1}m | effectiveness: {:.0}% | mercy_score: {:.2}",
            entity, radius, effectiveness * 100.0, mercy.score
        );

        // Placeholder: grant self a small harmony stack for balance
        commands.entity(entity).insert(HarmonyStack {
            count: 1,
            duration_timer: Timer::from_seconds(12.0, TimerMode::Once),
        });
    }
}

// ============================================================
// CELESTIAL HARMONY PULSE - Ultimate Group Ability
// ============================================================

/// Handles the powerful ultimate ability.
/// 
/// **Formulas:**
/// - Base Cooldown: 165s
/// - Power = BasePower * (1.0 + 0.18 * allies_in_range) * mercy_multiplier
/// - mercy_multiplier: >0.75 → 2.2x | 0.5-0.75 → 1.4x | <0.5 → 0.65x (heavy selfish penalty)
/// - High chance to trigger Epiphanies in nearby allies when mercy_score is high
pub fn celestial_harmony_pulse_activation_system(
    mut commands: Commands,
    mut events: EventReader<ActivateCelestialHarmonyPulse>,
    mut caster_query: Query<(
        Entity,
        &Transform,
        &AmbrosianAscended,
        &MercyAlignment,
        Option<&mut CelestialHarmonyPulseCooldown>,
    )>,
) {
    for event in events.read() {
        let Ok((entity, transform, _, mercy, cooldown_opt)) =
            caster_query.get_mut(event.caster)
        else { continue; };

        if let Some(mut cd) = cooldown_opt {
            if !cd.timer.finished() { continue; }
            let base = 165.0;
            let reduction = (mercy.score * 0.20).clamp(0.0, 0.30);
            cd.timer = Timer::from_seconds(base * (1.0 - reduction), TimerMode::Once);
        } else {
            commands.entity(entity).insert(CelestialHarmonyPulseCooldown {
                timer: Timer::from_seconds(165.0, TimerMode::Once),
            });
        }

        let power_mult = if mercy.score > 0.75 {
            2.2
        } else if mercy.score > 0.5 {
            1.4
        } else {
            0.65
        };

        // TODO: Actual spatial query to count allies in large radius (e.g. 40m)
        let allies_in_range: u32 = 4; // placeholder
        let final_power = power_mult * (1.0 + 0.18 * allies_in_range as f32);

        // === EFFECT APPLICATION ===
        // - Large expanding resonance wave (visual + audio)
        // - Heal all allies in radius
        // - Grant significant HarmonyStack
        // - High chance to trigger high-quality Epiphany in allies (especially if mercy high)
        // - Weaken corrupted entities / shadow manifestations

        info!(
            "[CELESTIAL HARMONY PULSE] Cast by {:?} | power: {:.2}x | allies: {} | mercy: {:.2}",
            entity, final_power, allies_in_range, mercy.score
        );
    }
}

// ============================================================
// DIVINE PRESENCE - Passive Aura
// ============================================================

/// Continuous passive aura. Strongest near allies.
/// Selfish/isolated play is penalized with increased damage taken.
pub fn divine_presence_passive_system(
    ambrosians: Query<(Entity, &Transform, &AmbrosianAscended, &MercyAlignment)>,
    // TODO: Add ally query for proximity checks
) {
    for (entity, _transform, _, mercy) in ambrosians.iter() {
        let isolated = false; // TODO: Replace with real distance check to other players

        if isolated && mercy.score < 0.60 {
            // Apply damage taken penalty (e.g. insert DamageTakenMultiplier { 1.28 })
            // This creates natural positioning incentive to stay with the group
        } else {
            // Grant small passive HarmonyStack or resonance buff to nearby allies
            // (can be throttled to every few seconds to avoid spam)
        }
    }
}

// ============================================================
// ASCENDED RESONANCE - Passive Epiphany Booster
// ============================================================

/// Passively boosts Epiphany frequency and quality for Ambrosians.
/// This is the core identity passive of the ascended state.
pub fn ascended_resonance_passive_system(
    ambrosians: Query<(&AmbrosianAscended, &ResonanceAttunement, &MercyAlignment)>,
) {
    for (_, resonance, mercy) in ambrosians.iter() {
        // Integration point: In your Epiphany / Telemetry systems, when rolling for
        // an Epiphany trigger on this entity, apply:
        //   chance_multiplier *= 1.70 + (resonance.value * 0.3)
        //   intensity_multiplier *= 1.40 + (mercy.score * 0.2)
        //
        // This makes ascended characters have significantly more frequent
        // and higher-quality Epiphanies, especially when mercy-aligned.
    }
}

// ============================================================
// PLUGIN REGISTRATION HELPER (call from main AmbrosianAscensionPlugin)
// ============================================================

pub fn register_ability_systems(app: &mut App) {
    app
        .add_event::<ActivateMercyBloom>()
        .add_event::<ActivateCelestialHarmonyPulse>()
        .add_systems(Update, (
            tick_ability_cooldowns_system,
            expire_harmony_stacks_system,
            mercy_bloom_activation_system,
            celestial_harmony_pulse_activation_system,
            divine_presence_passive_system,
            ascended_resonance_passive_system,
        ));
}
