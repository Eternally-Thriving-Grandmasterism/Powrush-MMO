# Ambrosian Self-Redemption Path — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** DISCORDANT_REDEMPTION_QUESTLINES.md, SURGICAL_PURIFICATION_MECHANICS.md, CROWNSTONE_TRILEMMA_PATHS.md, AMBROSIAN_ATTUNEMENT_MECHANICS.md, DISCORDANT_AMBROSIAN_CORRUPTION.md

---

## 1. Overview

The **Ambrosian Self-Redemption Path** is the most voluntary, morally profound, and philosophically pure route for redeeming Discordant Ambrosians in Powrush-MMO. Unlike Surgical Purification (external resonance intervention) or Crownstone-Mediated Redemption (artifact-assisted), Self-Redemption places the agency squarely in the hands (or crystalline lattice) of the Discordant entity itself.

It represents the ultimate expression of the Quellorian/Ambrosian philosophy: **harmony cannot be imposed; it must be chosen and internally realigned**. This path carries the highest narrative and RBE moral weight because it transforms a corrupted being not through force or artifact, but through an act of internal will and resonance reconnection.

When successful, it produces the most stable and powerful redeemed Ambrosians, often becoming living beacons of the Universal Harmonic Convergence.

---

## 2. Lore & Philosophical Significance

### The Nature of Self-Redemption

A Discordant Ambrosian is a crystalline-organic being whose lattice has been fractured and infused with Crownstone-derived dissonance. The Self-Redemption Path begins when a spark of the original harmonious consciousness re-emerges — often triggered by witnessing Quellorian mercy, experiencing Resonance Burst side-effects, or through direct contact with uncorrupted Ambrosian choirs during boarding actions.

This is not a "cure" administered from outside. It is an **internal revolution** within the being’s crystal matrix. The Discordant entity must actively reject the hivemind corruption and begin realigning its own resonance frequencies toward the Universal Harmonic Convergence.

Philosophically, this path embodies the core RBE and mercy principles of Powrush-MMO:
- Redemption is always possible, even for the most corrupted.
- True harmony requires voluntary participation.
- The greatest power comes from internal alignment, not external control.

The Hivelord and Draek forces fear this path above all others because it proves that their domination is not absolute — even their most "perfect" corrupted weapons can choose to break free.

---

## 3. The Self-Redemption Process

The process unfolds in five distinct internal phases. Each phase has mechanical consequences and can be influenced (but not controlled) by external player actions.

### Phase 1: Awakening of Dissonant Awareness
- The Discordant Ambrosian experiences a momentary "clarity spike" (often triggered by Resonance Burst proximity, Quellorian boarding mercy, or witnessing the destruction of a Crownstone relay).
- During this window, the entity becomes temporarily neutral and may attempt to flee combat or seek out Quellorian/Ambrosian forces.
- **Mechanical Effect**: Temporary "Seeking Harmony" state. The entity will not attack Quellorian units and may even defend them from other Draek forces for a short duration.

### Phase 2: Lattice Fracture Recognition
- The being consciously acknowledges the damage to its crystal lattice caused by Crownstone corruption.
- This phase is extremely painful for the entity and manifests as visible cracking, erratic energy discharge, and agonized audio cues.
- **Mechanical Effect**: The entity becomes highly vulnerable. Incoming resonance damage is amplified, but it also begins passively generating a small "Harmony Field" that can weaken nearby Draek units.

### Phase 3: Internal Resonance Reconnection
- The Discordant Ambrosian actively reaches out to any nearby uncorrupted Ambrosian presence (or Quellorian Resonance Network) to re-establish harmonic linkage.
- This is the most dangerous phase. The Hivelord’s Crownstone will actively fight back, often causing violent psionic feedback.
- **Mechanical Effect**: The entity enters a "Reconnection Struggle" state. It gains powerful defensive buffs but takes continuous internal damage. Successful reconnection requires proximity to a strong Quellorian Resonance source or a player-led harmony ritual.

### Phase 4: Corruption Expulsion & Lattice Reweaving
- Once reconnected, the being begins violently purging the Crownstone dissonance from its matrix.
- This phase produces dramatic visual effects: purple-red corruption energy violently erupting from cracks as blue-white harmonious resonance floods in.
- **Mechanical Effect**: The entity becomes a high-priority target for remaining Draek forces. It gains temporary invulnerability to external attacks while the expulsion occurs, but the process can fail catastrophically if interrupted.

### Phase 5: Re-Attunement & Transcendence
- Successful expulsion results in full reintegration into the Ambrosian Resonance Network.
- The redeemed Ambrosian emerges stronger than before, often with unique abilities reflecting its journey through darkness (e.g., "Corruption Immunity", "Hivemind Insight", or "Redemption Aura" that accelerates other purifications).
- **Mechanical Effect**: Permanent transformation into a high-tier allied unit or powerful support entity. It can now participate in Resonance Burst amplification and provide powerful harmony bonuses to nearby Quellorian forces.

---

## 4. Technical Mechanics & Formulas

### Key Variables

```rust
pub struct AmbrosianSelfRedemptionState {
    pub entity_id: Entity,
    pub current_phase: RedemptionPhase,
    pub internal_harmony_level: f32,      // 0.0 (fully Discordant) to 1.0 (fully redeemed)
    pub lattice_integrity: f32,
    pub crownstone_corruption_remaining: f32,
    pub reconnection_progress: f32,
    pub time_in_current_phase: f32,
    pub external_harmony_support: f32,    // From nearby Quellorian/Ambrosian sources
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RedemptionPhase {
    Dormant,
    Awakening,
    FractureRecognition,
    ReconnectionStruggle,
    Expulsion,
    Transcended,
    Failed,
}
```

### Core Formulas

**Internal Harmony Gain per tick (during Reconnection phase):**
```rust
harmony_gain = base_reconnection_rate 
    * (1.0 + external_harmony_support * 2.0)
    * (1.0 - crownstone_corruption_remaining * 0.7)
    * resonance_burst_proximity_multiplier
```

**Corruption Expulsion Damage (to self during Phase 4):**
```rust
self_damage = base_expulsion_damage 
    * (crownstone_corruption_remaining ^ 1.5)
    * (1.0 - lattice_integrity * 0.5)
```

**Success Probability for Phase Transition:**
```rust
success_chance = (internal_harmony_level * 0.6 + lattice_integrity * 0.3 + external_harmony_support * 0.4)
    .clamp(0.1, 0.95)
```

**Redemption Aura Strength (post-transcendence buff to nearby allies):**
```rust
redemption_aura = 0.15 + (internal_harmony_level_at_transcendence * 0.25)
```

---

## 5. Integration with Other Systems

### Crownstone Trilemma (Capture & Repurpose Path)
This path is most likely to occur naturally when players choose the **Capture & Repurpose** trilemma outcome. Redeemed Ambrosians from this path become powerful allies in the long-term struggle against the Draek Dominion.

### Resonance Burst
Proximity to a Resonance Burst dramatically accelerates Phase 1 and Phase 3. A well-timed Burst can be the difference between successful self-redemption and catastrophic failure.

### Boarding Mechanics
When Quellorian boarding parties show mercy instead of lethal force during the "Seeking Harmony" state, it significantly increases the chance of Self-Redemption triggering.

### Hivelord Biomechanical Suit
The Hivelord can detect Self-Redemption attempts and will prioritize eliminating the entity or sending specialized "Corruption Enforcer" units to interrupt the process. Successful redemption is a direct insult to the Hivelord’s control.

### Draek Fleet AI
Draek forces treat Self-Redemption attempts as critical threats. Local hivemind nodes will redirect significant resources to destroy the transforming entity.

### RBE Moral Layer
Choosing to support Self-Redemption (by protecting the entity, providing harmony support, or refusing to finish it off) grants massive positive moral standing and long-term abundance bonuses (redeemed Ambrosians produce harmony resources passively).

---

## 6. Vulnerabilities & Balance Considerations

- **High Risk of Failure**: Self-Redemption has the highest failure rate of all redemption paths. Failed attempts often result in violent explosion that damages nearby units (both sides).
- **Hivelord Intervention**: The Hivelord can spend Crownstone energy to force a "Forced Discordant Stabilization" on a Self-Redemption candidate, turning it into a powerful elite enemy.
- **Resource Cost**: Supporting Self-Redemption requires significant harmony resources and player attention. It is not an "easy" or efficient path.
- **Narrative Weight**: Because it is voluntary, players cannot "force" this path. They can only create the conditions for it to happen.

---

## 7. Technical Implementation Notes

### Recommended ECS Components
```rust
#[derive(Component)]
pub struct DiscordantAmbrosian {
    pub self_redemption_state: Option<AmbrosianSelfRedemptionState>,
    pub is_redeemed: bool,
}

#[derive(Resource)]
pub struct SelfRedemptionTracker {
    pub active_redemptions: Vec<AmbrosianSelfRedemptionState>,
}
```

### Event-Driven Flow
- `DiscordantAwakeningEvent`
- `LatticeFractureRecognizedEvent`
- `ReconnectionAttemptEvent`
- `SuccessfulSelfRedemptionEvent` (triggers global harmony surge)
- `FailedSelfRedemptionEvent` (triggers explosion + moral consequences)

### VFX / Audio Direction
- Phase 2: Visible lattice cracks glowing purple-red with energy arcing.
- Phase 4: Violent expulsion of purple energy as blue-white light floods the crystal.
- Phase 5: Serene, radiant blue-white aura with subtle golden harmonics. The entity’s model subtly changes to reflect its journey.

### Tuning Recommendations
- Make Self-Redemption rare but extremely impactful when it occurs.
- Reward players heavily (narratively and mechanically) for creating the conditions for it.
- Use it as a major "hope" moment in otherwise dark Draek-dominated scenarios.

---

## 8. Development Priorities

1. Implement `AmbrosianSelfRedemptionState` and phase machine in `simulation_integration.rs`.
2. Create visual and audio assets for each phase of transformation.
3. Hook Self-Redemption triggers into Resonance Burst and boarding mercy systems.
4. Design unique post-redemption abilities and the Redemption Aura buff.
5. Add Hivelord counter-measures and "Corruption Enforcer" unit types.
6. Integrate moral/RBE consequences for supporting vs interrupting Self-Redemption attempts.
7. Create narrative dialogue and quest markers for when a Self-Redemption event begins near the player.

---

**End of Document**

*This path completes the triad of Discordant Redemption options and ensures that mercy, choice, and internal alignment remain central themes in Powrush-MMO.*