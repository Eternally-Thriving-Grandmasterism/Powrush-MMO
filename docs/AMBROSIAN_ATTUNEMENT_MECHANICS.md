# Ambrosian Attunement Mechanics — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** QUELLORIAN_RESONANCE_AI_SYSTEMS.md, RESONANCE_BURST_MECHANICS.md, CROWNSTONE_TRILEMMA_PATHS.md, AURORAL_NEXUS.md, FLEET_CLASSES.md

---

## 1. Overview

The **Ambrosians** are an ancient, symbiotic species of crystalline-organic resonance beings who have been allied with the Quellorian civilization for millennia. They do not "crew" ships in the traditional sense — they **attune** to them.

Ambrosian Attunement is the process by which Ambrosian choirs merge their collective harmonic field with Quellorian vessels and the Auroral Unification Nexus (TAUN), dramatically amplifying resonance capabilities, providing psionic shielding, and enabling unique cooperative abilities that have no direct Draek counterpart.

This system creates the elegant, harmonious, and technologically transcendent feel of the Quellorian side — in direct philosophical and mechanical contrast to the predatory, consumption-driven Draek hivemind.

---

## 2. Nature of the Ambrosians

### Biology & Existence
- Semi-crystalline, semi-organic lifeforms that exist in a constant state of harmonic resonance.
- They communicate and exist through **resonance choirs** — groups of 3 to 12 individuals who synchronize their crystalline lattices into a single amplified field.
- They do not reproduce conventionally; new Ambrosians are "attuned" into existence through existing choirs when harmonic conditions are perfect.
- They experience time differently — a single attunement session can feel like centuries to them while only minutes pass in normal space.

### Philosophy
Ambrosians believe in **Universal Harmonic Convergence** — the idea that all sentient life should eventually resonate in harmony. They see the Quellorians as the most promising current vector for this vision and have therefore bound their fate to them.

They view the Draek Dominion and the Crownstone as a **disharmonic cancer** that must be either purified or isolated.

---

## 3. Attunement Process

### How Attunement Works
1. A Quellorian ship (or TAUN) generates a base resonance field.
2. An Ambrosian choir is invited aboard or links remotely via the Resonance Network.
3. The choir begins synchronized humming/singing that physically vibrates the ship’s resonance conduits.
4. Over 30–120 seconds, the Ambrosian field merges with the ship’s systems.
5. Once fully attuned, the ship gains massive bonuses and new abilities.

**Attunement is not permanent.** It requires continuous harmonic maintenance. If the choir is disrupted or the ship takes heavy damage, attunement degrades.

---

## 4. Core Mechanics & Formulas

### Global Simulation Resource

```rust
#[derive(Resource)]
pub struct AmbrosianAttunementState {
    pub global_harmony_level: f32,           // 0.0 – 1.0 (overall network health)
    pub active_choirs: u32,                  // Total Ambrosian choirs currently attuned
    pub taun_attunement: f32,                // Specific TAUN attunement strength
    pub fleet_attunement_bonus: f32,         // Average bonus across all Quellorian ships
    pub last_burst_amplification: f32,       // How much the last Resonance Burst was boosted
    pub crownstone_purification_progress: f32, // Only relevant during Capture & Repurpose path
}
```

### Key Formulas (Ready for simulation_integration.rs)

```rust
// Attunement Strength for a single ship
fn calculate_attunement_strength(
    base_resonance: f32,
    choir_count: u32,
    harmony_factor: f32,
    distance_from_taun: f32,
) -> f32 {
    let choir_multiplier = 1.0 + (choir_count as f32 * 0.12);
    let distance_falloff = (1.0 - (distance_from_taun / 15000.0).clamp(0.0, 0.6));
    (base_resonance * choir_multiplier * harmony_factor * distance_falloff).clamp(0.3, 2.5)
}

// Harmony Bonus applied to Quellorian forces near attuned assets
fn harmony_bonus(attunement_level: f32) -> f32 {
    (attunement_level * 0.35).clamp(0.0, 0.6)  // Up to +60% effectiveness
}

// Psionic Shielding vs Hivemind (used in boarding and fleet combat)
fn psionic_shielding(attunement_level: f32, hivemind_pressure: f32) -> f32 {
    let base_shield = attunement_level * 0.8;
    (base_shield - hivemind_pressure * 0.4).max(0.1)
}

// Resonance Burst Amplification (when Ambrosians are present)
fn resonance_burst_amplification(
    base_burst: f32,
    ambrosian_choirs_nearby: u32,
    global_harmony: f32,
) -> f32 {
    let choir_boost = 1.0 + (ambrosian_choirs_nearby as f32 * 0.18);
    base_burst * choir_boost * (0.7 + global_harmony * 0.6)
}
```

---

## 5. Integration with Existing Systems

### With Quellorian Resonance AI
- Ambrosian choirs act as **mobile resonance nodes** that can be moved between ships.
- They dramatically increase the effective range and strength of the Resonance Network.
- In QUELLORIAN_RESONANCE_AI_SYSTEMS.md, attunement now provides the "Harmony Field" layer on top of standard resonance.

### With Resonance Burst
Ambrosian attunement is the **primary multiplier** for Resonance Burst effectiveness (see RESONANCE_BURST_MECHANICS.md). A fully attuned TAUN with 8+ choirs can increase burst strength by 80–120%.

### With Crownstone Trilemma Paths
- **Destroy Path**: Ambrosians provide the final harmonic key needed to shatter the Crownstone. Without them, success chance drops significantly.
- **Capture & Repurpose Path**: Ambrosians are essential for the multi-stage **Redemption Protocol**. They perform the actual attunement/purification of the Crownstone. This path can eventually turn the Crownstone into a **benevolent resonance artifact** that benefits all factions.
- **Sabotage Path**: Ambrosians can detect the sabotage early and may attempt to warn the player or even rebel if the player chooses this path without their consent.

### With Boarding Mechanics
- Ambrosian choirs on a boarded ship can create powerful **resonance sanctuaries** that slow Draek infestation dramatically.
- If a choir is captured by Draek forces, they suffer horrific corruption and can become "Discordant Ambrosians" — a terrifying new enemy type.

### With Hivelord Biomechanical Suit
High Ambrosian attunement creates a **resonance interference field** that weakens the Hivelord’s Crownstone connection and suit integrity over time (especially during boarding actions on the Brood Spire).

---

## 6. Vulnerabilities & Draek Counterplay

- Ambrosians are physically fragile. A single well-placed boarding party or psionic strike can wipe out an entire choir.
- They require **harmonic stability**. Heavy combat or Crownstone corruption waves can force them to disattune to survive.
- Draek can deliberately target Ambrosian choirs to collapse Quellorian coordination.
- Over-attunement risk: If too many choirs attune to one ship, it can cause "Resonance Feedback" — temporary stunning of the vessel.

---

## 7. Technical Implementation Notes

### Recommended ECS Components
```rust
#[derive(Component)]
pub struct AmbrosianChoir {
    pub size: u32,
    pub attunement_level: f32,
    pub current_ship: Option<Entity>,
    pub harmony_contribution: f32,
}

#[derive(Component)]
pub struct AttunedAsset {
    pub attunement_strength: f32,
    pub last_attunement_update: f32,
}
```

### Integration Points
- `simulation_integration.rs`: Add `AmbrosianAttunementState` as a top-level resource. Update it every simulation tick.
- `rbe_engine.rs`: Track attunement as a form of "shared abundance" resource. High attunement can generate passive RBE harmony income for the Quellorian player.
- Boarding systems: When a ship with an Ambrosian choir is boarded, trigger special events.
- Crownstone trilemma resolution: Ambrosian presence should be a hard requirement or massive multiplier for the Capture & Repurpose path.

### Data-Driven Tuning
All formulas above should be exposed in a `.ron` or `.toml` tuning file so designers can balance without recompiling.

---

## 8. Development Priorities

1. Implement `AmbrosianAttunementState` resource and basic attunement calculation system.
2. Create visual/audio feedback for attunement (glowing runes, harmonic audio layers).
3. Add Ambrosian choir entities that can be deployed from TAUN.
4. Wire attunement bonuses into existing Quellorian AI and combat systems.
5. Build the three different Crownstone path interactions with Ambrosians.
6. Create Discordant Ambrosian corrupted enemy type for the Sabotage path.

---

**End of Document**

*This document ensures that when we later implement actual gameplay systems, Ambrosian attunement will feel like a living, meaningful, and strategically deep layer that perfectly complements the Quellorian philosophy of harmony while providing clear mechanical advantages and vulnerabilities.*