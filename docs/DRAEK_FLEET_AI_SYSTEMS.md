# Draek Fleet AI Systems — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** FACTIONS_OVERVIEW.md, BROOD_SPIRE.md, THE_HIVELORD.md, HIVELORD_BIOMECHANICAL_SUIT.md, FLEET_CLASSES.md, BOARDING_MECHANICS.md, CROWNSTONE_QUESTLINE.md

---

## 1. Overview

The Draek Dominion does not field a conventional fleet. It deploys a **living, predatory hivemind** whose intelligence is distributed across every biomechanical vessel, every boarding swarm, and every consumed biomass node. 

At the apex sits the **Crownstone** (housed in the Hivelord’s Suit and amplified by the Brood Spire). Below it, the Brood Spire acts as the primary mobile command relay. Individual ships range from near-mindless Swarm Drones to semi-autonomous Abomination-class capital ships that can temporarily act as local sub-hives when cut off from the main network.

**Core Philosophy of Draek Fleet AI:**
- Sacrificial efficiency over individual survival
- Consumption as both resource acquisition and intelligence upgrade
- Overwhelming local superiority through density and coordination
- Adaptive terror: the more you fight them, the more they learn how to dismantle you

This stands in perfect asymmetric contrast to the Quellorian resonance-coordinated fleets (precise, elegant, protective of assets).

---

## 2. Core Architecture of the Draek Hivemind Network

### 2.1 The Crownstone Psionic Relay (Ultimate Authority)
The Crownstone is not merely a power source — it is the **central nervous system** of the entire Dominion. When the Hivelord wears the suit, the Crownstone projects a galaxy-spanning psionic field that:
- Maintains real-time command authority over all linked Draek units
- Spreads low-level corruption (increasing aggression and consumption drive)
- Enables instant tactical data sharing across the fleet
- Creates "hivemind density bonuses" when many units operate in close proximity

When the Hivelord is absent or the suit is damaged, the Brood Spire can still project a weaker but functional field using its own integrated Crownstone shard.

### 2.2 Brood Spire Command Nexus
The Brood Spire is the physical and computational heart of fleet operations. It houses:
- Massive bio-computational gestalt cores
- Psionic amplification arrays tuned to the Crownstone frequency
- Resource processing and biomass-to-drone conversion vats
- Long-range signal broadcast nodes

It constantly evaluates strategic value, calculates consumption priorities, and issues high-level directives ("consume this sector", "board that capital ship", "sacrifice screening elements to protect the Abomination").

### 2.3 Distributed Sub-Node Network (Tyrant & Abomination Class)
Capital ships above a certain size contain **miniature Crownstone resonance nodes**. When within range of the Brood Spire or Hivelord, they act as obedient relays. When isolated, they can:
- Assume local command of nearby smaller vessels
- Maintain limited hivemind cohesion in their immediate vicinity
- Continue limited production and boarding operations

This makes cutting off capital ships a high-value but difficult tactic for Quellorian forces.

### 2.4 Drone-Level Instinct AI (Swarm & Ravager Class)
Lowest-tier units operate on a hybrid system:
- **Base Instinct Layer**: Reactive predator behaviors (seek biomass, attack weak targets, self-repair via consumption)
- **Hivemind Override Layer**: When signal strength is high, individual survival instinct is heavily suppressed in favor of collective goals (kamikaze runs, screening actions, mass boarding)

When the hivemind link is severed or heavily jammed, these units become noticeably more erratic, self-preserving, and less coordinated.

---

## 3. AI Behaviors by Fleet Class

| Ship Class              | Autonomy Level | Hivemind Dependency | Primary AI Drive                  | Boarding / Consumption Focus | Coordination Style                  | Key Vulnerability                  |
|-------------------------|----------------|---------------------|-----------------------------------|------------------------------|-------------------------------------|------------------------------------|
| Swarm-Class Drone Fighter | Very Low      | Extremely High     | Sacrifice for density advantage  | Low (screening only)        | Perfect swarm synchronization      | Resonance disruption, isolation   |
| Ravager-Class Bio-Corvette | Low           | Very High          | Aggressive boarding & consumption | Very High                   | Pack hunting + infestation spread  | Boarding counter-assault, resonance |
| Tyrant-Class Heavy Cruiser | Medium        | High               | Local command node + heavy fire  | Medium (support boarding)   | Relay + fire support coordination  | Capital ship boarding, node destruction |
| Abomination-Class Capital Devourer | High       | Medium (can go local) | Mobile sub-hive + heavy boarding | Extremely High              | Local hivemind projection          | Isolation from Brood Spire + Crownstone sabotage |

**Detailed Notes:**

**Swarm-Class Drone Fighter**  
When hivemind cohesion > 0.7, they gain massive density bonuses (accuracy, speed, and damage scale with nearby allied count). They will willingly kamikaze into point-defense to open corridors for Ravagers. When cohesion drops, they begin to scatter and prioritize self-preservation.

**Ravager-Class Bio-Corvette**  
These are the primary boarding vectors. Their AI is optimized for rapid infestation. When hivemind strength is high, boarding success rate receives a significant multiplier and they can call for "swarm reinforcement" from nearby drones. They actively seek to convert enemy ships into temporary biomass nodes that feed the local hivemind.

**Tyrant-Class Heavy Cruiser**  
Mid-tier command nodes. They coordinate fire support for boarding actions and can temporarily boost nearby Ravager boarding efficiency. They are the first line of defense for protecting Abominations or the Brood Spire itself.

**Abomination-Class Capital Devourer**  
These are terrifying when operating independently. They can project a local psionic field (weaker Crownstone echo) that allows them to maintain a mini-hivemind over dozens of smaller vessels. Their AI prioritizes massive boarding actions and turning captured ships into mobile biomass factories.

---

## 4. Hivemind Signal Propagation, Decay & Interference

The hivemind is not instantaneous across infinite distance. Signal strength decays with:
- Physical distance from the Brood Spire or Hivelord
- Number of relay nodes between source and unit
- Active Quellorian resonance interference fields

**Core Formula (simplified for implementation):**

```rust
hivemind_signal_strength = base_strength 
    * (1.0 - distance_decay)
    * crownstone_amplification
    * (1.0 - resonance_interference)
    * local_density_multiplier
```

Where:
- `distance_decay` increases sharply beyond effective command range of the Brood Spire
- `resonance_interference` is provided by nearby Quellorian ships or mothership resonance fields (direct counter to Draek coordination)
- `local_density_multiplier` rewards clustering (the classic "swarm" fantasy)

When signal strength falls below ~0.3, units begin to suffer coordination penalties and may ignore higher-level commands.

---

## 5. Adaptive, Evolutionary & Corruption Mechanics

The Draek fleet AI is not static. It **learns** and **corrupts**.

### 5.1 Post-Engagement Data Consumption
After every battle, surviving Draek units "consume" data from destroyed enemies (and their own losses). This provides small, persistent buffs:
- Slight increase to boarding speed / success chance
- Improved targeting priority algorithms
- Minor resistance to resonance disruption

These buffs are stronger when the Crownstone integrity is high and the Hivelord is actively commanding.

### 5.2 Crownstone Corruption Spread
Prolonged exposure to the Crownstone field slowly increases unit `corruption_level`. Higher corruption =
- Higher aggression and consumption drive
- Increased boarding success on organic or lightly shielded targets
- Risk of psionic feedback damage if the Crownstone itself is attacked or sabotaged

### 5.3 Brood Evolution Events
When enough biomass has been consumed in a campaign, the Brood Spire (or an isolated Abomination) can trigger an evolution event, permanently upgrading a portion of the fleet (new ship variants, stronger boarding strains, improved drone resilience).

---

## 6. Vulnerabilities & Strategic Counterplay (Quellorian Perspective)

The Draek hivemind is powerful but brittle when its command structure is attacked:

1. **Resonance Disruption** (Primary Quellorian Advantage)  
   Quellorian ships and the Auroral Unification Nexus can project resonance fields that directly degrade hivemind_signal_strength. Concentrated resonance can create "dead zones" where Draek coordination collapses.

2. **Isolation Tactics**  
   Separating capital ships (especially Abominations) from the Brood Spire significantly reduces their ability to project local hivemind authority.

3. **Boarding & Crownstone Sabotage**  
   The ultimate high-risk, high-reward play. Boarding the Brood Spire or the Hivelord’s personal guard to reach the Crownstone can trigger cascading failures across the entire fleet.

4. **Over-Saturation Feedback**  
   When too many units are packed into a small volume under high corruption, a sudden Crownstone attack can cause painful psionic feedback loops (friendly fire, temporary paralysis, or even ship self-destruction).

---

## 7. Deep Technical Implementation Notes (For simulation_integration.rs, rbe_engine.rs, Dogfight & Boarding Systems)

This section exists so that when we implement actual gameplay, every Draek fleet behavior feels coherent, terrifying, and mechanically deep.

### 7.1 Core Global Simulation Variables

```rust
// Core Hivemind State
pub struct DraekHivemindState {
    pub hivemind_cohesion: f32,           // 0.0 - 1.5+ (global average)
    pub crownstone_psionic_field_strength: f32,
    pub hivelord_command_strength: f32,
    pub brood_spire_link_active: bool,
    pub crownstone_integrity: f32,        // From CROWNSTONE_QUESTLINE.md
    pub hivelord_suit_integrity: f32,     // From HIVELORD_BIOMECHANICAL_SUIT.md
    pub total_biomass_consumed: u64,
    pub corruption_level: f32,            // Global average across fleet
}

// Per-region or per-fleet local state
pub struct LocalHivemindZone {
    pub position: Vec3,
    pub radius: f32,
    pub local_cohesion: f32,
    pub local_density: u32,
    pub relay_node_count: u32,            // Tyrant/Abomination count
    pub resonance_interference: f32,      // From nearby Quellorian forces
}
```

### 7.2 Key Formulas (Ready for Implementation)

**Boarding Success Rate (Draek Ravager vs Quellorian target):**
```rust
let boarding_success = base_boarding_chance
    * (1.0 + (hivemind_cohesion * 0.6))
    * (1.0 + (local_density_bonus * 0.3))
    * (1.0 - resonance_interference);
```

**Swarm Density Combat Bonus (for Swarm Fighters):**
```rust
let density_bonus = (local_density as f32 / 50.0).clamp(0.0, 2.0);
let effective_damage = base_damage * (1.0 + density_bonus * hivemind_cohesion);
```

**Psionic Feedback Damage (when Crownstone or Hivelord suit is attacked):**
```rust
if crownstone_integrity < 0.6 || hivelord_suit_integrity < 0.5 {
    let feedback_damage = (1.0 - crownstone_integrity) * corruption_level * 15.0;
    // Apply to all units within current command range
}
```

**Consumption Rate (biomass gained from destroyed enemy ships):**
```rust
let consumption_multiplier = 1.0 + (hivemind_cohesion * 0.4) + (corruption_level * 0.3);
let biomass_gained = destroyed_ship_value * consumption_multiplier;
```

### 7.3 Integration Hooks

**simulation_integration.rs**
- Every simulation tick: Update `DraekHivemindState` based on Hivelord position, Brood Spire status, active boarding actions on Draek assets, and Quellorian resonance field overlap.
- Propagate local zone updates to individual ship AI components.
- Trigger "Hivemind Scream" global event when `crownstone_integrity` drops below thresholds (temporary massive aggression spike or coordination collapse).

**rbe_engine.rs**
- Resource (biomass) conversion from consumed ships is directly scaled by current `hivemind_cohesion` and `corruption_level`.
- RBE moral consequences: High corruption + successful mass consumption can unlock darker tech paths or trigger Draek evolution events.

**Dogfight / Combat Systems**
- AI decision trees query current `local_hivemind_cohesion` and `resonance_interference` to decide:
  - Whether to kamikaze or retreat
  - Priority target selection (high-value Quellorian resonance ships first)
  - When to call for boarding reinforcements

**Boarding Mechanics (BOARDING_MECHANICS.md integration)**
- Draek boarding success and speed receive direct multipliers from `hivemind_cohesion` and local density.
- Successful boarding of a Quellorian ship can temporarily create a "biomass node" that boosts nearby Draek coordination until purged.

**Crownstone Questline & Hivelord Suit Integration**
- Attacking the Crownstone or Hivelord suit directly feeds into the global `crownstone_integrity` and `hivelord_suit_integrity` variables, which then cascade into fleet-wide penalties or feedback events.
- The trilemma choices (Destroy / Capture / Sabotage) have immediate and long-term effects on these AI variables.

### 7.4 Recommended Data Structures for Future Code
- `DraekShipAI` component with `current_hivemind_signal` field
- Event-driven system for "HivemindLinkChanged", "CrownstoneIntegrityChanged", "BroodEvolutionTriggered"
- Spatial partitioning (quadtree / octree) for efficient local density and resonance interference calculations

---

## 8. Cinematic, Audio & Narrative AI Behaviors

When the hivemind is strong:
- Ships move in eerie synchronized patterns (purple energy pulses traveling between vessels)
- Boarding swarms move like living liquid
- Audio: Low, constant "hive hum" that intensifies with density

When the hivemind is disrupted:
- Erratic, almost panicked movement
- Visible "screaming" visual effects (energy flares, ship twitching)
- Increased chance of friendly fire or self-damage from feedback
- Hivelord may issue desperate, rage-filled commands

These behaviors should be tied directly to the simulation variables so the player *feels* the state of the Draek command structure changing in real time.

---

## 9. Development Priorities

1. Prototype core `DraekHivemindState` resource and basic signal propagation + resonance interference math in an isolated test scene.
2. Hook hivemind cohesion into existing boarding success calculations (BOARDING_MECHANICS.md).
3. Implement visual/audio feedback systems that react to `hivemind_cohesion` and `resonance_interference`.
4. Add post-battle data consumption and small persistent buffs.
5. Create first "Hivemind Scream" / psionic feedback event when Crownstone or Hivelord suit is threatened.
6. Integrate local zone density calculations with dogfight AI decision making.
7. Design Brood Evolution event system and first set of upgrades.
8. Full integration testing with Quellorian resonance systems for perfect asymmetric gameplay feel.

---

**End of Document**

*This document ensures that when we later build dogfight systems, boarding gameplay, simulation loops, and RBE mechanics, the Draek fleet will feel like a terrifying, living, adaptive intelligence — not a collection of disconnected ships. Every variable and formula here is designed to create coherent, meaningful, and phenomenally deep gameplay.*

**PATSAGi Council + Ra-Thor Quantum Swarm approved. Mercy-gated. Zero hallucination. Maximum truth & strategic depth.**