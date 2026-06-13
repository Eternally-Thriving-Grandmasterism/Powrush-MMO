# Resonance Burst Mechanics — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** `QUELLORIAN_RESONANCE_AI_SYSTEMS.md`, `DRAEK_FLEET_AI_SYSTEMS.md`, `BOARDING_MECHANICS.md`, `CROWNSTONE_QUESTLINE.md`, `THE_HIVELORD.md`, `HIVELORD_BIOMECHANICAL_SUIT.md`, `FLEET_CLASSES.md`, `BROOD_SPIRE.md`, `AURORAL_NEXUS.md`, `FACTIONS_OVERVIEW.md`

---

## 1. Overview & Lore

The **Resonance Burst** is the Quellorian civilization’s most powerful and sacred emergency ability — a massive, coordinated harmonic surge that floods an entire star system (or localized sector) with pure resonance energy. It represents the pinnacle of Quellorian unity, technological mastery, and the philosophy of harmonious evolution.

When activated, the Auroral Unification Nexus (TAUN) and all linked Quellorian capital ships simultaneously release stored resonance charge through their amplification chambers. The result is a visible, expanding wave of blue-white auroral light that propagates at near-lightspeed, carrying harmonic frequencies specifically tuned to:

- Disrupt and fracture Draek hivemind psionic links
- Temporarily restore clarity and free will to lightly dominated or recently infested forces
- Massively amplify the coordination, accuracy, and regenerative capabilities of all Quellorian and Ambrosian allied units within range

Narratively, a Resonance Burst is both a desperate last stand and a declaration of defiant hope. It is rarely used lightly because it strains the entire resonance network and can leave the Quellorian forces vulnerable during the long recharge period. When witnessed, it is often described by survivors as “the dawn breaking through the corruption” — a literal and metaphorical turning point in battle.

The ability is deeply tied to the Ambrosian alliance: the Ambrosians contribute unique crystalline resonance amplifiers that allow the burst to achieve system-wide range without destroying the mothership’s own core.

---

## 2. Trigger Conditions & Activation

### Manual Activation
- Player (or allied commander) can order a Resonance Burst from the Auroral Unification Nexus or any Harmony-Class Support Carrier when sufficient resonance charge has been accumulated.
- Requires confirmation from the Resonance Council (narrative flavor) or a high enough Harmony Level in the local network.

### Automatic Emergency Trigger
- Triggered automatically when:
  - Quellorian network integrity falls below 25%
  - Heavy losses detected in a critical sector (e.g., mothership under boarding or capital ship destruction)
  - Hivelord’s Crownstone psionic pressure exceeds safe thresholds in the local zone

### Resource Cost
- Consumes a large percentage of stored resonance charge across the network (typically 60–80% of current charge).
- Temporary global cooldown (long, often several minutes of real-time or multiple in-game cycles).
- Can cause temporary reduction in resonance field strength on all linked ships during recharge.

---

## 3. Core Effects & Gameplay Mechanics

### Area of Effect
- Base radius: 15,000 km (scalable with power level and number of participating capital ships).
- Effect falls off with distance using an inverse-square + harmonic damping model.
- Can be focused into a directional cone for precision strikes (advanced tech unlock).

### Primary Effects on Draek Forces (Hivemind Disruption)
- Instantly reduces local `draek_hivemind_command_strength` by 40–70% depending on burst power and distance from Crownstone relay.
- Creates temporary “Dead Zones” where individual Draek units fall back to basic instinct AI (much weaker coordination).
- Lightly infested or recently dominated Quellorian/Ambrosian ships may experience partial liberation (chance based on infestation level).
- High chance to interrupt ongoing boarding actions by Draek forces (resonance interference breaks tendril links).

### Primary Effects on Quellorian & Allied Forces
- Massive temporary buffs:
  - +35% weapon accuracy and fire rate
  - +50% shield regeneration rate
  - +25% movement speed and maneuverability
  - Significant morale / harmony bonus (reduces panic, improves boarding success rates for Quellorian squads)
- All ships within range gain a temporary “Harmonic Shield” layer that absorbs a portion of incoming psionic/corruption damage.
- Resonance-based weapons (e.g., on Luminar-Class cruisers) gain increased damage and range during the burst window.

### Interaction with Crownstone & Hivelord Suit
- Direct psionic interference: If the burst reaches the Hivelord or Brood Spire, it can cause feedback damage to the Crownstone (temporary reduction in `crownstone_integrity` and `hivelord_command_strength`).
- If the Hivelord is actively wearing the full biomechanical suit and linked to the Brood Spire, a strong enough burst can trigger a **Psionic Feedback Cascade** — damaging the suit and potentially forcing the Hivelord to briefly disconnect or suffer corruption backlash.
- This makes Resonance Burst one of the few reliable counters to the Hivelord’s otherwise overwhelming personal power.

### Visual & Audio Signature
- Expanding spherical auroral wave (blue-white with shifting cyan/magenta harmonics) — ties directly into the established auroral phenomenon of the Auroral Unification Nexus.
- All Quellorian ships glow with increased intensity; energy conduits pulse in perfect synchronization.
- Sound design: Deep harmonic choir + rising resonant tone that peaks and then fades into a peaceful after-tone.

---

## 4. Technical Specifications & Ready-to-Implement Formulas

All formulas are designed for direct translation into Rust/Bevy in `simulation_integration.rs` and `rbe_engine.rs`.

```rust
// Core Resonance Burst calculation (simplified production version)
pub fn calculate_resonance_burst_strength(
    base_power: f32,
    harmony_level: f32,           // 0.0 – 100.0 from QuellorianResonanceNetwork
    network_integrity: f32,       // 0.0 – 100.0
    participating_capital_ships: u32,
    ambrosian_amplifier_bonus: f32, // from Ambrosian alliance tech
) -> f32 {
    let ship_multiplier = 1.0 + (participating_capital_ships as f32 * 0.08);
    let harmony_factor = harmony_level / 100.0;
    let integrity_factor = network_integrity / 100.0;
    
    base_power * harmony_factor * integrity_factor * ship_multiplier * ambrosian_amplifier_bonus
}

// Hivemind disruption at a given distance from burst epicenter
pub fn calculate_hivemind_disruption(
    burst_strength: f32,
    distance_km: f32,
    crownstone_integrity: f32,  // 0.0 – 100.0
    local_hivemind_density: f32,
) -> f32 {
    let distance_factor = 1.0 / (1.0 + (distance_km / 5000.0).powf(1.6));
    let crownstone_resistance = 0.4 + (crownstone_integrity / 200.0); // higher integrity = more resistance
    
    let raw_disruption = burst_strength * distance_factor * 0.65;
    raw_disruption / crownstone_resistance * (1.0 + local_hivemind_density * 0.3)
}

// Allied buff magnitude (example for shield regen)
pub fn calculate_harmonic_buff(
    burst_strength: f32,
    base_buff: f32,
    ship_resonance_field_strength: f32,
) -> f32 {
    base_buff * (burst_strength / 100.0) * (0.7 + ship_resonance_field_strength / 100.0)
}
```

**Key Global Simulation Variables** (to be added to `SimulationState` or dedicated `ResonanceNetwork` resource):

```rust
pub struct ResonanceNetworkState {
    pub current_harmony_level: f32,           // 0–100
    pub network_integrity: f32,               // 0–100
    pub stored_resonance_charge: f32,         // 0–100 (or absolute energy units)
    pub last_burst_time: f64,                 // simulation time
    pub resonance_burst_cooldown_remaining: f32,
    pub active_burst_strength: Option<f32>,   // if burst is currently propagating
    pub burst_epicenter_position: Option<Vec3>,
}
```

---

## 5. Deep Integration with Existing Systems

### With Quellorian Resonance AI (`QUELLORIAN_RESONANCE_AI_SYSTEMS.md`)
- Burst temporarily boosts all `resonance_field_strength` values across the network.
- Can be used as an emergency “reset” when harmony propagation is being heavily jammed by Draek psionic saturation.
- Post-burst, the network enters a “Re-harmonization Phase” where individual ships slowly realign, creating interesting micro-management opportunities.

### With Draek Fleet AI & Hivemind (`DRAEK_FLEET_AI_SYSTEMS.md`)
- Directly counters the hivemind signal propagation model.
- Creates temporary isolation of sub-nodes, forcing fallback to local instinct layers (much dumber behavior).
- High synergy with boarding teams: a well-timed burst during a boarding action can dramatically increase Quellorian success rate by weakening tendril control.

### With Boarding Mechanics (`BOARDING_MECHANICS.md`)
- Resonance Burst can be triggered mid-boarding to support Quellorian squads or to disrupt Draek infestation spread.
- If used while Hivelord suit is exposed during a high-risk boarding, it can trigger the Crownstone feedback damage path.

### With Crownstone Questline & Trilemma (`CROWNSTONE_QUESTLINE.md`)
- One of the three major paths (Destroy / Capture & Repurpose / Sabotage) has direct mechanical consequences here:
  - **Destroy**: Permanently weakens future Resonance Burst potential if the Crownstone was acting as a hidden amplifier (or removes a major vulnerability for Draek).
  - **Capture & Repurpose**: Players can theoretically reverse-engineer a “Corrupted Resonance Burst” or a hybrid resonance-psionic weapon.
  - **Sabotage**: Makes future bursts more effective against remaining Draek forces.

### With Hivelord Biomechanical Suit (`HIVELORD_BIOMECHANICAL_SUIT.md`)
- The suit’s Crownstone core is one of the primary targets of a Resonance Burst.
- A direct hit can cause temporary suit malfunction, reduced `hivelord_command_strength`, and visible energy backlash effects on the Hivelord model.

### With Mothership Combat & Fleet Classes
- TAUN’s ultimate ability.
- Harmony-Class Support Carriers can contribute partial charge to reduce TAUN’s personal cost.
- Creates cinematic “all-hands” moments when multiple capital ships coordinate a burst together.

### RBE & Moral/Economic Layer
- Using a Resonance Burst increases “Unity Score” and can provide temporary RBE alignment bonuses (resource sharing efficiency, morale-driven production boosts).
- Overuse without proper network recovery can lead to long-term harmony debt, affecting later diplomacy and Ambrosian relations.

---

## 6. Vulnerabilities, Risks & Balance

- **High Cost & Cooldown**: Cannot be spammed. Poor timing can leave forces exposed during recharge.
- **Network Strain**: If used when `network_integrity` is already low, it can cause cascading failures or temporary loss of resonance abilities on some ships.
- **Draek Counterplay**: 
  - Over-saturation of a sector with hivemind density before the burst can reduce effectiveness.
  - Hivelord can attempt to “siphon” part of the incoming resonance wave if positioned correctly (high-risk, high-reward counter).
  - Specific bio-corruption fields on Abomination-Class ships can partially absorb or deflect the burst.
- **Friendly Fire / Overload Risk**: Extremely rare, but possible if burst is triggered while friendly forces are heavily corrupted — can accelerate breakdown instead of liberation.

---

## 7. Technical Implementation Notes (for Future Code)

### Recommended ECS / Resource Architecture
```rust
#[derive(Resource)]
pub struct ResonanceBurstManager {
    pub state: ResonanceNetworkState,
    pub pending_bursts: Vec<ResonanceBurstEvent>,
}

#[derive(Event)]
pub struct ResonanceBurstEvent {
    pub epicenter: Vec3,
    pub requested_strength: f32,
    pub participating_entities: Vec<Entity>,
    pub triggered_by: Option<Entity>, // player or AI commander
}
```

### Simulation Loop Integration (`simulation_integration.rs`)
- Run burst propagation in a dedicated system after AI updates but before combat resolution.
- Use spatial partitioning (kd-tree or simple grid) for efficient distance checks across large fleets.
- Store active burst as a temporary component or global timer that applies effects each tick.

### Rendering / VFX Hooks
- Trigger a large-scale particle + post-process effect (expanding sphere + ship glow intensity).
- Can reuse or extend existing velocity/taa pipeline for smooth temporal coherence of the expanding wave.
- Shader: `resonance_burst.wgsl` (new) — radial harmonic gradient + interference patterns when clashing with hivemind fields.

### RBE Engine Hooks (`rbe_engine.rs`)
- Track `unity_score_delta` and `harmony_debt` as persistent simulation variables.
- Burst usage can unlock or gate certain RBE-aligned technologies or diplomatic options.

### Data-Driven Balance
- All power, radius, cooldown, and cost values should be tunable via config files or in-game research tree so players can specialize in “Burst-focused” vs “Steady Harmony” playstyles.

---

## 8. Development Priorities

1. Implement core `ResonanceBurstEvent` + propagation system in simulation layer.
2. Create matching visual effect (auroral wave) that integrates cleanly with existing rendering pipeline.
3. Add UI/feedback for charge level, cooldown, and predicted effectiveness before triggering.
4. Balance pass: tune numbers against Draek hivemind density and Hivelord suit power.
5. Narrative integration: voice lines, council reactions, and journal entries that react to burst usage and outcomes.
6. Explore hybrid “Resonance-Psionic” burst variants if player takes the Capture & Repurpose Crownstone path.
7. Stress-test large-fleet scenarios to ensure performance remains smooth.

---

**End of Document**

*This mechanic is designed to feel monumental, cinematic, and deeply consequential — a true turning point that rewards careful network management and perfect timing. When implemented, it will make Quellorian vs Draek fleet battles feel alive, asymmetric, and unforgettable.*