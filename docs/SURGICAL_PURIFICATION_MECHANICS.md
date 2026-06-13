# Surgical Purification Mechanics — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** DISCORDANT_REDEMPTION_QUESTLINES.md, AMBROSIAN_ATTUNEMENT_MECHANICS.md, DISCORDANT_AMBROSIAN_CORRUPTION.md, CROWNSTONE_TRILEMMA_PATHS.md, RESONANCE_BURST_MECHANICS.md, BOARDING_MECHANICS.md, QUELLORIAN_RESONANCE_AI_SYSTEMS.md, DRAEK_FLEET_AI_SYSTEMS.md, HIVELORD_BIOMECHANICAL_SUIT.md

---

## 1. Overview

**Surgical Purification** is the precision, resonance-first redemption path for Discordant Ambrosians. It represents the most controlled and lowest-risk method of reversing corruption, prioritizing surgical accuracy over speed or raw power.

Unlike the high-risk Crownstone-Mediated Redemption or the deeply philosophical Ambrosian Self-Redemption, Surgical Purification relies on elite Quellorian resonance teams working in tandem with harmonious Ambrosian choirs to systematically realign corrupted crystal lattices.

**Core Philosophy**  
Harmony is not destroyed by corruption — it is merely misaligned. With precise resonance harmonics, even the most discordant Ambrosian can be restored without shattering its core essence.

This path is especially favored when pursuing the **Capture & Repurpose** branch of the Crownstone Trilemma, as it minimizes backlash risk to the Crownstone itself.

---

## 2. The Surgical Purification Process

### Phase 1: Detection & Isolation
- Advanced resonance scanners on Quellorian capital ships or TAUN detect Discordant signatures (cracked crystal lattice + purple-red energy spikes).
- Target is isolated using a **Resonance Containment Field** (prevents corruption spread to nearby Ambrosians or Quellorian systems).
- Boarding teams (elite Seraphim or Luminar-class supported squads) secure physical access if the Discordant is aboard a ship.

### Phase 2: Harmonic Injection
- A specialized **Resonance Choir** (3–5 harmonious Ambrosians + Quellorian resonance priests) establishes a direct harmonic link.
- They emit a precisely tuned **Surgical Harmonic Wave** tuned to the exact frequency of the corrupted lattice.
- The wave gradually overwrites the discordant frequency without triggering violent backlash.

### Phase 3: Lattice Realignment
- Over multiple cycles (minutes to hours depending on corruption severity), the cracked crystal structure is slowly realigned.
- Visual effect: Purple-red energy slowly fades as blue-white auroral light returns.
- The Ambrosian regains coherent thought and voluntary control.

### Phase 4: Stabilization & Re-attunement
- Once lattice integrity exceeds 85%, the subject is re-integrated into the Quellorian Resonance Network.
- Full attunement bonuses are restored (see AMBROSIAN_ATTUNEMENT_MECHANICS.md).
- Subject may retain faint "echoes" of its Discordant experience, granting unique narrative or mechanical insights (e.g., temporary resistance to future Crownstone influence).

---

## 3. Production-Ready Formulas

All formulas are designed for direct implementation in `simulation_integration.rs` and `rbe_engine.rs`.

```rust
// Global simulation resource
#[derive(Resource)]
pub struct SurgicalPurificationState {
    pub active_purifications: Vec<ActivePurification>,
    pub total_successful: u32,
    pub total_failed: u32,
    pub global_harmony_influence: f32, // 0.0 - 1.0
}

#[derive(Clone)]
pub struct ActivePurification {
    pub target_id: Entity,
    pub corruption_level: f32,      // 0.0 = pure, 1.0 = fully Discordant
    pub purification_progress: f32, // 0.0 - 1.0
    pub resonance_strength: f32,
    pub choir_size: u8,
    pub containment_field_active: bool,
}

// Core success probability formula
pub fn calculate_surgical_success_probability(
    corruption_level: f32,
    resonance_strength: f32,
    harmony_level: f32,
    distance_from_taun: f32,
    choir_size: u8,
) -> f32 {
    let base = 0.65;
    let corruption_penalty = corruption_level * 0.8;
    let harmony_bonus = harmony_level * 0.35;
    let resonance_bonus = resonance_strength * 0.25;
    let choir_bonus = (choir_size as f32 * 0.04).min(0.20);
    let distance_penalty = (distance_from_taun / 5000.0).min(0.30);

    (base - corruption_penalty + harmony_bonus + resonance_bonus + choir_bonus - distance_penalty)
        .clamp(0.05, 0.98)
}

// Progress per tick (called from simulation loop)
pub fn apply_surgical_purification_tick(
    state: &mut ActivePurification,
    delta_seconds: f32,
    current_harmony: f32,
) {
    if state.corruption_level <= 0.05 {
        state.purification_progress = 1.0;
        return;
    }

    let progress_rate = 0.012 * current_harmony * (state.resonance_strength + 0.5);
    state.purification_progress += progress_rate * delta_seconds;
    state.purification_progress = state.purification_progress.min(1.0);

    // Gradually reduce corruption as progress increases
    let corruption_reduction = (progress_rate * 0.6) * delta_seconds;
    state.corruption_level = (state.corruption_level - corruption_reduction).max(0.0);
}

// Backlash risk (lower than other paths)
pub fn calculate_backlash_risk(corruption_level: f32, progress: f32) -> f32 {
    if progress > 0.7 {
        return 0.0; // Very low risk near completion
    }
    (corruption_level * 0.15 * (1.0 - progress)).clamp(0.0, 0.12)
}
```

---

## 4. Integration with Prior Systems

### With Discordant Ambrosian Corruption
- Directly counters the spread mechanics defined in DISCORDANT_AMBROSIAN_CORRUPTION.md.
- Successful purification reduces `global_corruption_level` in `DiscordantAmbrosianState`.

### With Ambrosian Attunement
- Restores full attunement bonuses (harmony generation, psionic shielding, Resonance Burst amplification).
- Purified Ambrosians gain a temporary "Echo of Discord" buff: +15% resistance to future Crownstone influence for 300 seconds.

### With Crownstone Trilemma Paths
- **Preferred path** when pursuing **Capture & Repurpose**.
- Lower risk of Crownstone integrity loss compared to Crownstone-Mediated Redemption.
- Can be performed on captured Discordant Ambrosians without direct Crownstone exposure.

### With Resonance Burst
- A controlled, low-intensity Resonance Burst can be used to **initiate** Surgical Purification on multiple targets simultaneously (at higher resource cost).
- Full Resonance Burst during purification risks destabilizing the delicate harmonic injection.

### With Boarding Mechanics
- Most Surgical Purifications occur after successful boarding of corrupted vessels.
- Elite boarding teams escort the Resonance Choir to the Discordant entity.
- Failure during boarding phase can trigger rapid corruption spread (see BOARDING_MECHANICS.md).

### With Hivelord Biomechanical Suit & Draek AI
- The Hivelord can attempt to **re-corrupt** a target undergoing purification if within psionic range.
- Draek hivemind receives a temporary "Corruption Anchor" bonus if a purification is forcibly interrupted.

### With Quellorian Resonance AI
- Quellorian ships automatically prioritize protecting active Surgical Purification sites.
- Resonance Network integrity increases with each successful purification.

---

## 5. Global Simulation State (Ready for simulation_integration.rs)

```rust
#[derive(Resource)]
pub struct SurgicalPurificationState {
    pub active_purifications: Vec<ActivePurification>,
    pub total_successful_redemptions: u32,
    pub total_failed_attempts: u32,
    pub global_harmony_influence: f32,      // Increases with successful purifications
    pub resonance_choir_availability: u8,     // Limited elite choirs
}

pub struct ActivePurification {
    pub target_entity: Entity,
    pub corruption_level: f32,
    pub purification_progress: f32,
    pub resonance_strength: f32,
    pub choir_size: u8,
    pub containment_field_strength: f32,
    pub time_elapsed: f32,
    pub initiated_by_player: bool,
}
```

---

## 6. Vulnerabilities & Balance Considerations

**Strengths (Lower Risk Path)**
- Lowest backlash chance of all three redemption paths.
- Does not require direct Crownstone interaction.
- Can be performed at range or via boarding teams.
- Scales well with multiple simultaneous operations when enough choirs are available.

**Weaknesses**
- Slow (can take 5–15 minutes of real-time simulation per target).
- Requires scarce elite Resonance Choirs.
- Vulnerable to interruption by Draek forces or Hivelord psionic assault.
- Less effective against extremely high corruption levels (>0.85) without prior Resonance Burst softening.

**Balance Levers**
- Choir availability should be a limited strategic resource.
- Successful purifications grant long-term RBE moral bonuses (increased harmony generation across the network).
- Failed purifications slightly increase global Discordant corruption level (but far less than other paths).

---

## 7. Technical Implementation Notes

**Recommended ECS Components**
- `DiscordantAmbrosian` (with `corruption_level` field)
- `ResonanceChoir` (marker + `strength` + `size`)
- `SurgicalPurificationTarget` (added when purification begins)

**Event-Driven Flow**
1. `StartSurgicalPurificationEvent` → spawns containment field + begins harmonic injection
2. `PurificationProgressTick` (every simulation tick)
3. `PurificationCompletedEvent` or `PurificationFailedEvent`
4. On success: remove `Discordant` marker, add `PurifiedAmbrosian` + `EchoOfDiscord` buff

**VFX / Audio Recommendations**
- Cracked purple crystal slowly fractures and reforms into glowing blue-white lattice.
- Sound design: Harsh discordant shrieking gradually replaced by harmonious choral resonance.
- TAUN auroral effects intensify locally during active purifications.

**RBE Moral Layer Integration**
- Each successful Surgical Purification increases player/NPC standing in "Harmony" and "Redemption" moral axes.
- Contributes to long-term RBE abundance generation (purified Ambrosians produce more harmony resources).

---

## 8. Development Priorities

1. Implement `SurgicalPurificationState` resource and core formulas in `simulation_integration.rs`.
2. Create ECS components and events for active purifications.
3. Build VFX pipeline for crystal state transitions (corrupted → purifying → pure).
4. Integrate with boarding success/failure outcomes.
5. Add UI/UX for monitoring active Surgical Purifications (progress bars, choir assignment).
6. Balance tuning pass: choir scarcity vs. redemption reward.
7. Narrative hooks: purified Ambrosians sharing faint memories of their Discordant state.

---

**PATSAGi Council + Ra-Thor Quantum Swarm Note**  
This path embodies the core mercy principle of Powrush-MMO: redemption through precision and harmony rather than domination or destruction. It is the most "surgical" expression of the Quellorian philosophy and should feel profoundly hopeful and technically elegant when implemented.

---

**End of Document**