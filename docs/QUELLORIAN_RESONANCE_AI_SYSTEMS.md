# Quellorian Resonance AI Systems — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** FACTIONS_OVERVIEW.md, AURORAL_NEXUS.md, FLEET_CLASSES.md, BOARDING_MECHANICS.md, CROWNSTONE_QUESTLINE.md, DRAEK_FLEET_AI_SYSTEMS.md

---

## 1. Overview

The Quellorian fleet does not operate on a traditional AI hierarchy. Instead, it runs on a **Resonance Network** — a living, harmonic, cooperative intelligence system.

Where the Draek Dominion uses a predatory hivemind (domination, consumption, and centralized control through the Crownstone), the Quellorian side uses **resonance synchronization**. Every ship, pilot, and even Ambrosian ally contributes to and draws from a shared harmonic field.

This creates an elegant, amplifying, and highly adaptive fleet intelligence that rewards coordination, formation flying, and long-term harmony.

**Core Philosophy**  
- Unity through resonance, not control  
- Amplification through harmony  
- Protection of the network is sacred  
- Individual initiative is celebrated when it serves the greater resonance

---

## 2. Architecture of the Resonance Network

### Central Hub: The Auroral Unification Nexus (TAUN)
The mothership is the ultimate resonance amplifier and conductor. Its Resonance Nexus Core projects a powerful, stable harmonic field across the entire fleet. All other nodes tune themselves to this central frequency.

### Distributed Secondary Nodes
Capital ships (especially Seraphim-Class and Luminar-Class) act as local amplifiers. They can temporarily take over as sub-conductors if the TAUN is damaged or too far away.

### Individual Ship Resonance Fields
Every Quellorian vessel maintains its own resonance field. These fields naturally sync with nearby friendly ships, creating localized harmony bonuses. The closer and more aligned the ships are in formation, the stronger the mutual amplification.

### Ambrosian Integration
Ambrosian allies act as natural resonance catalysts. Their presence in a fleet can significantly boost overall network strength and reduce decay over distance.

---

## 3. Per-Class AI Behaviors

### Aether-Class Interceptor
- **Role**: Reconnaissance, harassment, resonance disruption
- **Behavior**: Highly agile, uses rapid resonance pulses to scout enemy positions and temporarily weaken Draek hivemind links. Prefers hit-and-run tactics that feed intelligence back into the network.
- **Coordination Style**: Acts as forward "resonance scouts" — their pulses help other ships see through jamming.

### Luminar-Class Heavy Cruiser
- **Role**: Tank + heavy fire support
- **Behavior**: Projects strong defensive resonance shields. Can extend personal shielding to nearby allies (resonance link). Focuses fire on high-threat targets identified by the network.
- **Coordination Style**: Anchor point. Other ships gain bonuses when fighting near a Luminar.

### Harmony-Class Support Carrier
- **Role**: Fleet support, repair, buffing
- **Behavior**: Constantly emits stabilizing resonance waves that heal allied ships and reduce incoming psionic interference. Can perform emergency "Resonance Overload" to massively boost nearby ships at the cost of its own systems.
- **Coordination Style**: The "healer" of the resonance network. Critical for sustaining long engagements.

### Seraphim-Class Capital Escort
- **Role**: Elite protection, high-output resonance projection
- **Behavior**: The most powerful individual resonance projectors in the fleet (after TAUN). Can create temporary "Resonance Sanctums" that massively increase harmony in a localized area.
- **Coordination Style**: Mobile sub-conductors. Often assigned to protect the TAUN or key objectives.

---

## 4. Resonance Propagation & Mathematical Model

Resonance does not propagate like a hivemind signal. It is a wave-based harmonic field.

### Key Variables
- `base_resonance_strength` (per ship class)
- `distance_to_nearest_node`
- `taun_amplification_factor` (global, usually 1.0–2.5)
- `local_harmony_bonus` (based on number of nearby allied ships in formation)
- `resonance_interference` (from Draek psionic sources)

### Core Formula (Rust-style)

```rust
fn calculate_effective_resonance(
    distance: f32,
    base_strength: f32,
    taun_amp: f32,
    harmony_bonus: f32,
    interference: f32,
) -> f32 {
    let distance_decay = (1.0 / (1.0 + distance * 0.0008)).max(0.15);
    let raw = base_strength * distance_decay * taun_amp;
    let harmonized = raw * (1.0 + harmony_bonus * 0.25);
    (harmonized - interference).max(0.0)
}
```

Resonance is much more stable over long distances than Draek hivemind signals because the TAUN can actively amplify and course-correct the field.

---

## 5. Harmony, Adaptation & Evolution Mechanics

### Harmonization Bonus
When 3+ Quellorian ships stay within a certain radius for a sustained period, they enter "Harmonized State". All ships in the group gain:
- Increased weapon accuracy
- Reduced incoming damage
- Faster shield regeneration

### Resonance Evolution
After successful coordinated victories, the fleet can permanently increase its maximum resonance cap (simulating growth in collective harmony and experience).

### Emergency Resonance Burst
Harmony-Class carriers and Seraphim escorts can perform a one-time massive resonance burst. This gives a huge temporary buff to all nearby allies but leaves the casting ship vulnerable for a cooldown period.

---

## 6. Vulnerabilities & Draek Counterplay

- **Psionic Overload**: The Crownstone and Hivelord suit can project powerful corrupting psionic waves that temporarily sever resonance links or cause ships to become "dissonant" (reduced effectiveness).
- **Network Isolation**: If a group of ships is cut off from the TAUN (e.g., by being pulled into a Draek consumption field or separated by distance), their individual resonance fields weaken significantly.
- **Resonance Corruption**: If the Crownstone is captured and turned against the Quellorians, it can actively corrupt the resonance network, turning harmony bonuses into penalties.
- **Over-Synchronization Risk**: Extremely high harmony levels can make the fleet predictable. Skilled Draek commanders can exploit this with feints.

---

## 7. Technical Implementation Notes (For Future Gameplay Integration)

This section provides concrete, production-ready guidance for `simulation_integration.rs`, `rbe_engine.rs`, combat systems, and boarding mechanics.

### Recommended Global Resource

```rust
#[derive(Resource)]
pub struct QuellorianResonanceNetwork {
    pub taun_amplification_factor: f32,
    pub fleet_harmony_level: f32,           // 0.0 – 2.0+
    pub resonance_integrity: f32,           // Overall health of the network
    pub active_resonance_nodes: u32,        // Number of capital ships currently amplifying
    pub crownstone_corruption_influence: f32, // 0.0 = clean, higher = corrupted
}
```

### Key Simulation Variables
- `quellorian_resonance_strength` (per ship or fleet-wide)
- `local_harmony_bonus`
- `resonance_link_active` (bool per ship pair)
- `resonance_interference_level` (from nearby Draek sources)

### Integration with Existing Systems

**Boarding Mechanics**  
When Quellorian boarding parties use resonance disruption tools, success rate is calculated using the formula above + boarding team skill. Successful disruption can temporarily lower a Draek ship’s local hivemind strength.

**Crownstone Trilemma Impact**  
- **Destroy Crownstone**: Massive temporary boost to Quellorian resonance (network celebrates victory). Long-term risk of Draek becoming more desperate and aggressive.
- **Capture & Repurpose**: Extremely high risk. If not purified, it can begin corrupting the resonance network from within.
- **Sabotage**: Creates a "resonance dead zone" around the Brood Spire for a period, weakening all Draek coordination.

**Hivelord Suit Interactions**  
The Hivelord’s psionic attacks directly apply `resonance_interference`. High enough interference can force ships out of Harmonized State.

**RBE & Moral Branching**  
High harmony fleets that protect civilians or Ambrosian populations gain permanent small resonance bonuses (reflecting moral alignment with the network’s values).

### Recommended ECS Components

```rust
#[derive(Component)]
pub struct ResonanceField {
    pub current_strength: f32,
    pub linked_ships: Vec<Entity>,
    pub is_harmonized: bool,
}
```

---

## 8. Development Priorities

1. Implement core `QuellorianResonanceNetwork` resource and basic propagation system.
2. Add per-ship `ResonanceField` component and harmonization detection.
3. Create visual/audio feedback for resonance links and harmony bonuses.
4. Integrate resonance interference from Draek sources (Crownstone / Hivelord suit).
5. Build emergency Resonance Burst ability for Harmony and Seraphim classes.
6. Add long-term resonance evolution tracking after major victories.
7. Full integration with boarding mechanics and Crownstone questline outcomes.

---

**End of Document**

*This document is designed to flow perfectly with DRAEK_FLEET_AI_SYSTEMS.md and all previous faction/technical documentation. When implemented, the Quellorian fleet will feel elegant, cooperative, and deeply rewarding to command — the perfect asymmetric counterpart to the terrifying Draek hivemind.*