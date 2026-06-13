# Dogfight Mechanics — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**  
**AG-SML v1.0 Sovereign License**

---

## 1. Overview: The Heart of Naval Warfare

Dogfights in Powrush-MMO are not simple "shoot the other guy" encounters. They are **asymmetric, high-stakes, living simulations** of two fundamentally opposed philosophies of war:

- **Quellorian Doctrine**: Precision, harmony, resonance coordination, surgical strikes, and elegant boarding actions. Every ship fights as part of a greater resonant whole.
- **Draek Doctrine**: Overwhelming swarm pressure, hivemind coordination, consumption, corruption spread, and brutal boarding infestation. Numbers and adaptive terror win through sheer volume and psychological domination.

Dogfights serve as the primary tactical layer between fleet movements and full mothership engagements. They are the place where **boarding mechanics**, **AI systems**, **Resonance Burst**, **Crownstone Trilemma effects**, **Hivelord suit influence**, and **Ambrosian attunement/corruption** all collide in real time.

This document provides the complete mechanical foundation so that when we implement `simulation_integration.rs` and combat systems, everything flows coherently and feels phenomenal.

---

## 2. Core Dogfight Loop

Every dogfight engagement follows this high-level loop:

1. **Initiation** — Triggered by fleet proximity, player orders, or AI aggression thresholds.
2. **Positioning & Targeting** — Ships maneuver into optimal ranges while AI (Resonance Network or Hivemind) influences priority targets.
3. **Engagement** — Weapons fire, special abilities, boarding attempts, and status effects (resonance fields, corruption, harmony buffs).
4. **Boarding Windows** — Successful damage or positioning creates short-lived boarding opportunities (see BOARDING_MECHANICS.md).
5. **Resolution** — One side breaks, retreats, or is destroyed. Victorious side may pursue, consolidate, or trigger larger fleet response.
6. **Aftermath** — Resource gains/losses, corruption spread, harmony level changes, Crownstone integrity effects, and global simulation state updates.

---

## 3. Quellorian Dogfight Doctrine (Resonance-Coordinated Precision)

Quellorian forces fight with **elegant, coordinated lethality**. Their ships benefit from the Resonance Network, allowing real-time harmony bonuses, predictive targeting, and shared sensor data.

### Key Mechanics
- **Resonance Field Aura**: Ships within range of a capital ship or TAUN projection gain +accuracy, +evasion, and reduced incoming corruption.
- **Harmony Targeting Priority**: AI prioritizes high-value threats (Hivelord-linked ships, high-corruption carriers) using resonance strength calculations.
- **Surgical Boarding**: High success rate on damaged targets when resonance field is strong. Focus on liberating crew or sabotaging key systems rather than consumption.
- **Resonance Burst Integration**: When triggered, all Quellorian ships in range gain massive temporary buffs to damage, speed, and boarding success while heavily disrupting Draek hivemind links.

### Per-Class Behaviors (see FLEET_CLASSES.md for full stats)

| Class                    | Role in Dogfight                          | Key Ability                              | AI Priority                          | Boarding Style                  |
|--------------------------|-------------------------------------------|------------------------------------------|--------------------------------------|---------------------------------|
| Aether Interceptor       | Hit-and-run harassment, anti-drone       | Resonance Dash (short blink + damage)   | High-mobility Draek drones          | Quick surgical strikes         |
| Luminar Heavy Cruiser    | Anchor & fire support                    | Resonance Cannon (piercing + harmony)   | Capital ships & Hivelord suits      | Precision boarding teams       |
| Harmony Support Carrier  | Force multiplier & repair                | Harmony Link (shared buffs)             | Protect damaged allies              | Defensive boarding support     |
| Seraphim Capital Escort  | Mothership guardian & heavy hitter       | Seraphim Protocol (area denial)         | Protect TAUN / high-value targets   | Elite boarding when opportunity|

---

## 4. Draek Dogfight Doctrine (Hivemind Swarm Overwhelm)

Draek forces fight as a **living, adaptive swarm**. Individual ships are expendable; the hivemind learns and evolves from every engagement.

### Key Mechanics
- **Hivemind Density Bonus**: More Draek ships in local area = higher coordination, faster reaction, increased boarding success, and corruption spread rate.
- **Consumption & Corruption**: Successful damage has a chance to apply corruption stacks. High stacks enable boarding infestation that can turn the target into a temporary Draek asset.
- **Adaptive Learning**: After each dogfight, the local hivemind node gains small permanent bonuses against the enemy composition it faced (see DRAEK_FLEET_AI_SYSTEMS.md).
- **Hivelord Suit Influence**: When the Hivelord is linked (via Crownstone), nearby Draek ships gain massive aggression and boarding bonuses. The suit can also project direct psionic commands.

### Per-Class Behaviors

| Class                    | Role in Dogfight                          | Key Ability                              | AI Priority                          | Boarding Style                     |
|--------------------------|-------------------------------------------|------------------------------------------|--------------------------------------|------------------------------------|
| Swarm Drone Fighter      | Cannon fodder & screening                | Self-destruct (area corruption)         | High-value Quellorian targets       | Swarm infestation (low individual success, high volume) |
| Ravager Bio-Corvette     | Fast harassment & boarding               | Bio-Corruption Pulse                    | Damaged or isolated ships           | Aggressive quick boarding        |
| Tyrant Heavy Cruiser     | Heavy fire & command node                | Tyrant Command (local hivemind boost)   | Quellorian capitals & carriers      | Heavy boarding teams + corruption|
| Abomination Capital      | Mobile fortress & production             | Consumption Field (drains + converts)   | Protect Brood Spire / high-value    | Mass boarding + ship conversion  |

---

## 5. Integration with Major Systems

### 5.1 Boarding During Dogfights
Dogfights create the most common and dynamic boarding opportunities. Damage dealt, positioning, and current AI state (resonance strength vs hivemind density) directly modify boarding success chance (see BOARDING_MECHANICS.md for full formulas).

**Quellorian Advantage**: High resonance field strength dramatically increases precision boarding success and reduces risk of counter-infestation.
**Draek Advantage**: High hivemind density + corruption stacks allow "infestation boarding" that can flip ships mid-fight.

### 5.2 Resonance Burst Effects
When Quellorian forces trigger Resonance Burst:
- All Quellorian ships gain +40% damage, +30% speed, +50% boarding success for 30s.
- Draek hivemind links within range suffer massive disruption (formula in RESONANCE_BURST_MECHANICS.md).
- Hivelord suit receives direct psionic feedback damage.
- Crownstone integrity takes minor damage if linked.

### 5.3 Crownstone Trilemma Influence
- **Destroy path** aftermath: Draek ships become feral and uncoordinated (huge Quellorian advantage in dogfights).
- **Capture & Repurpose path**: Hybrid Quellorian-Draek units appear with unique resonance + consumption abilities.
- **Sabotage path**: Random Draek ships may turn on their own side or suffer internal explosions.

### 5.4 Hivelord Biomechanical Suit
When the Hivelord is active in or near the engagement:
- Nearby Draek ships gain +25% aggression and boarding success.
- Quellorian ships suffer psionic harassment (accuracy debuff).
- Direct targeting of the Hivelord suit becomes a high-value objective (see HIVELORD_BIOMECHANICAL_SUIT.md).

### 5.5 Ambrosian Attunement & Discordant Corruption
- **Attuned Ambrosians**: Provide strong harmony field that boosts Quellorian evasion and reduces incoming corruption.
- **Discordant Ambrosians**: Act as terrifying wildcards — they can randomly boost Draek corruption spread or cause friendly-fire resonance backlash on Quellorian forces.

---

## 6. Global Simulation Struct (Ready for simulation_integration.rs)

```rust
#[derive(Resource)]
pub struct SpaceCombatState {
    pub active_dogfights: Vec<DogfightInstance>,
    pub global_resonance_field_strength: f32,      // 0.0 - 1.0+
    pub global_hivemind_density: f32,              // 0.0 - 1.0+
    pub crownstone_influence_multiplier: f32,      // affected by trilemma path
    pub hivelord_suit_link_active: bool,
    pub last_resonance_burst_frame: u64,
}

#[derive(Clone)]
pub struct DogfightInstance {
    pub id: u64,
    pub location: Vec3,
    pub qu_elorian_strength: f32,
    pub draek_strength: f32,
    pub resonance_field_local: f32,
    pub hivemind_density_local: f32,
    pub corruption_level: f32,                     // average on Draek side
    pub harmony_level: f32,                        // average on Quellorian side
    pub boarding_windows_open: u32,
    pub active_boarding_actions: Vec<BoardingAction>,
}
```

---

## 7. Key Formulas (Production-Ready)

**Boarding Success Chance during Dogfight** (simplified):
```rust
fn calculate_boarding_success(
    damage_dealt_percent: f32,
    resonance_or_hivemind_strength: f32,
    is_quellorian: bool,
    crownstone_influence: f32,
) -> f32 {
    let base = damage_dealt_percent * 0.6;
    let ai_mod = if is_quellorian {
        resonance_or_hivemind_strength * 0.8
    } else {
        resonance_or_hivemind_strength * 1.2   // Draek benefits more from density
    };
    let crownstone_mod = if crownstone_influence > 0.7 { 0.15 } else { 0.0 };
    (base + ai_mod + crownstone_mod).clamp(0.05, 0.85)
}
```

**Corruption Spread Rate** (Draek advantage):
```rust
fn corruption_spread_per_second(density: f32, hivelord_link: bool) -> f32 {
    let base = density * 0.08;
    if hivelord_link { base * 1.6 } else { base }
}
```

---

## 8. Technical Implementation Notes

- Use spatial partitioning (KD-tree or grid) for efficient "ships within resonance/hivemind range" queries.
- Dogfight instances should be spawned as ECS entities with `DogfightInstance` component for easy querying and event emission.
- Boarding attempts should emit events that `boarding_mechanics` system can consume.
- AI decision systems (Resonance Network & Hivemind) should run at lower frequency than rendering but high enough for responsive feel.
- VFX: Quellorian ships leave elegant blue-white resonance trails; Draek ships leave corrupting purple-red energy veins and tendrils.
- Audio: Layered resonance harmonics vs guttural hivemind chittering and wet bio-mechanical sounds.

Direct integration points:
- `simulation_integration.rs` → `SpaceCombatState` resource
- `rbe_engine.rs` → moral/abundance consequences from dogfight outcomes (especially boarding results)
- Combat rendering pipeline → velocity prepass + TAA already prepared for fast-moving ships

---

## 9. Development Priorities

1. Implement core `SpaceCombatState` and `DogfightInstance` management.
2. Wire per-class AI behaviors from FLEET_CLASSES + AI system docs into dogfight loop.
3. Add boarding window creation and resolution during engagements.
4. Integrate Resonance Burst, Crownstone trilemma effects, and Hivelord suit influence.
5. Add Ambrosian attunement / Discordant corruption modifiers.
6. Create visual and audio feedback layers.
7. Balance testing with PATSAGi Council review.

---

**End of Document**

*This document ensures that when we build the actual dogfight systems, every previous lore and technical decision (motherships, boarding, AI, Crownstone, redemption paths, leadership) flows together into one coherent, asymmetric, and phenomenally deep combat experience.*