# Draek Counter-Purification Strategies — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Classification:** Draek Strategic Doctrine (Hivelord Eyes Only)

---

## 1. Overview & Philosophical Core

While the Quellorian Alliance views **Surgical Purification** as a merciful act of harmonic restoration, the Draek Dominion sees it as an existential threat to their very nature. Purification represents the ultimate rejection of consumption, domination, and the hivemind’s sacred right to assimilate all life into perfect unity under the Crownstone.

Draek strategy is therefore not defensive — it is **predatory and retaliatory**. Every Quellorian attempt to redeem Discordant Ambrosians is met with calculated escalation designed to:

- Accelerate corruption beyond the point of no return
- Weaponize the purification process itself
- Punish the Quellorians for their “harmonic arrogance”
- Generate new strategic assets (corrupted choirs, berserker Discordants, resonance feedback bombs)

The Hivelord has issued a standing directive: **"Where they bring light, we bring the feast."**

---

## 2. The Four Pillars of Draek Counter-Purification

### Pillar 1: Corruption Acceleration Protocols
**Goal:** Force Discordant entities past the stabilization threshold before Quellorian purification teams can complete their work.

**Primary Methods:**
- Direct Crownstone psionic amplification through the Hivelord’s biomechanical suit
- Overloading local hivemind sub-nodes on nearby capital ships
- Sacrificial injection of raw consumption biomass into Discordant lattices

**Technical Formula (Rust-ready):**
```rust
pub fn calculate_corruption_acceleration(
    base_rate: f32,
    hivelord_amplification: f32,      // from Hivelord Biomechanical Suit
    crownstone_integrity: f32,
    purification_progress: f32,       // 0.0 = just started, 1.0 = near completion
    nearby_brood_density: f32,
) -> f32 {
    let desperation_multiplier = 1.0 + (purification_progress * 2.5);
    let crownstone_boost = crownstone_integrity * 0.8;
    
    (base_rate * desperation_multiplier * hivelord_amplification * crownstone_boost) 
        + (nearby_brood_density * 0.4)
}
```

### Pillar 2: Resonance Isolation & Containment
**Goal:** Sever the Discordant entity from the Quellorian Resonance Network before or during purification.

**Tactics:**
- Deploy “Silence Swarms” (specialized bio-corvettes) to create local resonance dead zones
- Hivelord suit generates targeted psionic interference fields
- Force Discordants to self-isolate by triggering consumption instincts

**Global Effect:** Reduces Quellorian `harmony_level` and `attunement_strength` in the affected sector.

### Pillar 3: Weaponization of the Purification Process
**Goal:** Turn failed or interrupted purifications into Draek assets.

**Outcomes:**
- **Discordant Berserkers**: Entities that survive partial purification become hyper-aggressive and resistant to future harmonic effects.
- **Resonance Feedback Bombs**: When a purification fails spectacularly, the backlash can be harvested as a one-time high-damage psionic weapon.
- **Corrupted Choir Nodes**: Redeemed Ambrosian choirs that were partially converted become powerful new sub-nodes for the Draek hivemind.

### Pillar 4: Retaliatory Corruption Campaigns
**Goal:** Punish Quellorian forces for attempting purification and spread corruption into their own ranks.

**Primary Vector:** Boarding actions specifically targeting Quellorian purification vessels and resonance amplification chambers on the Auroral Unification Nexus.

---

## 3. Technical Implementation — `DraekCounterPurificationState`

```rust
#[derive(Resource)]
pub struct DraekCounterPurificationState {
    pub active_counter_operations: Vec<CounterPurificationOperation>,
    pub total_corruption_spread_this_frame: f32,
    pub hivelord_intervention_active: bool,
    pub crownstone_amplification_level: f32, // 0.0 - 2.0
    pub resonance_dead_zones: Vec<DeadZone>,
}

#[derive(Clone)]
pub struct CounterPurificationOperation {
    pub target_discordant_id: Entity,
    pub purification_progress: f32,        // tracked from Quellorian side
    pub current_corruption_rate: f32,
    pub acceleration_multiplier: f32,
    pub isolation_strength: f32,
    pub weaponization_potential: f32,
}

#[derive(Clone)]
pub struct DeadZone {
    pub position: Vec3,
    pub radius: f32,
    pub strength: f32, // how strongly it blocks resonance
    pub duration_remaining: f32,
}
```

---

## 4. Deep Integration with Existing Powrush Systems

This system is designed to create meaningful tension with **every** previously documented mechanic:

- **Surgical Purification Mechanics** — Directly competes with and can override purification success rolls.
- **Discordant Ambrosian Corruption** — Provides the "supply" of entities that Draek can accelerate or weaponize.
- **Crownstone Trilemma Paths** — Especially powerful on the **Sabotage** path (increases backfire risk) and **Capture & Repurpose** path (Draek will fight hardest to prevent successful redemption).
- **Hivelord Biomechanical Suit** — Primary amplifier and command node for all counter-purification operations.
- **Draek Fleet AI Systems** — Local hivemind nodes automatically prioritize protecting or accelerating nearby Discordants under purification.
- **Quellorian Resonance AI & Ambrosian Attunement** — Creates dynamic push-pull between harmony and consumption in shared battlespaces.
- **Resonance Burst Mechanics** — Draek can deliberately trigger early, weak Bursts in corrupted areas to cause catastrophic feedback.
- **Boarding Mechanics** — High-value targets now include Quellorian purification teams and resonance choirs.
- **RBE Moral & Abundance Layer** — Successful Draek counter-purification generates "Corrupted Biomass" resources but increases long-term galactic instability and moral horror for players who witness or allow it.

---

## 5. Narrative & Thematic Weight

Draek doctrine frames purification as the ultimate blasphemy against the natural order of consumption. The Hivelord teaches that "light that cannot be consumed must be broken and remade in our image."

Every successful counter-purification operation is celebrated within the Brood Spire as a holy act of defiance and expansion of the Dominion’s living biomass.

---

## 6. Balance Considerations & Quellorian Counter-Counterplay

**Draek Advantages:**
- Faster reaction time via hivemind
- Can sacrifice units without moral cost
- Direct Crownstone amplification is extremely powerful near the Brood Spire

**Quellorian Advantages:**
- Can protect purification operations with Resonance Bursts and elite boarding teams
- Ambrosian Self-Redemption path is harder for Draek to interfere with
- Long-term harmony network can eventually overwhelm isolated Draek efforts

**Recommended Tuning Levers:**
- Distance from Brood Spire / Hivelord dramatically reduces Draek acceleration effectiveness
- Successful Quellorian boarding of key hivemind nodes can temporarily disable counter-purification in a sector

---

## 7. Development Priorities

1. Implement `DraekCounterPurificationState` and core acceleration formula in `simulation_integration.rs`
2. Create visual/audio feedback for Corruption Acceleration (dark energy tendrils, screaming crystal effects)
3. Add Draek AI priority targeting for active purification operations
4. Design boarding missions specifically to disrupt Draek counter-purification nodes
5. Balance Resonance Burst interaction with accelerated corruption (risk/reward)
6. Integrate moral and RBE consequences for players who allow or participate in failed purifications

---

**End of Document**

*This doctrine is sealed under the Crownstone. Only the Hivelord and his chosen Brood Lords may read these words.*