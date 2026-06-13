# Draek Fleet AI Behavior Patterns

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development

---

## 1. Overview

This document defines the **specific, implementable behavior patterns** for the Draek fleet AI in Powrush-MMO. It builds directly upon `DRAEK_FLEET_AI_SYSTEMS.md`, `FLEET_CLASSES.md`, `DOGFIGHT_MECHANICS.md`, `HIVELORD_COUNTER_STRATEGIES.md`, and the full redemption/enslaved species documentation.

The Draek AI is designed to feel like a **living, predatory, adaptive hivemind** — terrifying, efficient, and strategically deep. Every decision reinforces the themes of consumption, domination, and the erasure of individual will.

**Core Design Goals:**
- Hierarchical command (Crownstone → Brood Spire → Capital nodes → Drone instinct)
- High adaptability and learning from player actions
- Meaningful sacrifice and biomass/resource loops
- Strong integration with boarding, dogfights, Crownstone Trilemma, and RBE moral layer
- Asymmetric contrast to Quellorian resonance-coordinated precision

---

## 2. Core AI Architecture

### 2.1 Hierarchical Layers

1. **Strategic Layer (Hivelord / Brood Spire)**: Long-term goals, production waves, escalation protocols, Crownstone link management.
2. **Operational Layer (Tyrant / Abomination capitals)**: Anchor points, sub-node relays, heavy support, boarding coordination.
3. **Tactical Layer (Ravager / Swarm units)**: Individual + local swarm decisions, boarding actions, hit-and-run, envelopment.
4. **Instinct Layer (Drones & Enslaved Minions)**: Simple but deadly behaviors heavily influenced by local hivemind signal strength.

### 2.2 Decision Making Model

Each unit uses a **priority queue decision system** influenced by:
- Local hivemind signal strength (`local_hivemind_strength`)
- Crownstone link integrity (`crownstone_link_strength`)
- Threat assessment (player damage output, Resonance presence, boarding risk)
- Resource value (biomass potential, strategic position)
- Hivelord command override

**State Machine Base:** Every unit has states: `Patrol`, `Engage`, `Swarm`, `Board`, `Sacrifice`, `Regroup`, `Flee` (rare).

---

## 3. Per-Class Behavior Patterns

### 3.1 Swarm-Class Drone Fighter

**Role:** Cannon fodder / screening / biomass delivery

**Primary States & Transitions:**

| State       | Trigger                                      | Behavior                                                                 | Exit Condition                     |
|-------------|----------------------------------------------|--------------------------------------------------------------------------|------------------------------------|
| Patrol      | No high-value target nearby                  | Loose formation patrol, low energy consumption                           | Player or high-value target detected |
| Engage      | Target acquired                              | Direct approach, basic attack, call nearby drones                        | Target destroyed or heavy damage   |
| Swarm       | 4+ drones nearby + Crownstone command        | Envelopment maneuver, focus fire on weak points, kamikaze on high-value  | Target destroyed or swarm broken   |
| Sacrifice   | Low health + high-value target in range      | Kamikaze charge with biomass explosion on death                          | Death                              |
| Regroup     | Swarm broken + signal still strong           | Retreat to nearest capital or Brood Spire relay                          | Signal lost or new target          |

**Key Formulas:**
- Aggression scaling: `aggression = base_aggression * (1.0 + crownstone_corruption_level * 0.5)`
- Kamikaze damage on death: `explosion_damage = current_health * 2.5`

**Integration Notes:** Strong interaction with enslaved Veythari (when present) for coordinated swarm songs.

### 3.2 Ravager-Class Bio-Corvette

**Role:** Hit-and-run boarding, corruption spreader, elite harassment

**Primary States:**
- `Harass`: Hit-and-run attacks on isolated targets
- `Board`: When boarding window opens (low shields or isolated capital)
- `Corrupt`: Spread hivemind corruption to player assets or weak points
- `Support`: Assist larger capitals in envelopment

**Special Behaviors:**
- Prioritizes boarding over pure damage when `boarding_success_chance > 0.4`
- Uses enslaved Korrath or Sylvaris as boarding parties when available
- Retreats to Brood Spire when heavily damaged for rapid biomass repair

### 3.3 Tyrant-Class Heavy Cruiser

**Role:** Anchor, heavy fire support, sub-node relay

**Primary States:**
- `Anchor`: Hold position, provide fire support, maintain local hivemind node
- `Advance`: Push with drone screen when ordered by Brood Spire
- `Defend`: Protect Brood Spire or Abomination when threatened
- `Relay`: Boost local hivemind signal for nearby units

**Key Behavior:** When `local_hivemind_strength` drops below threshold, it calls for drone reinforcement waves.

### 3.4 Abomination-Class Capital Devourer

**Role:** Siege platform, consumption engine, boarding denial

**Primary States:**
- `Siege`: Long-range consumption beam on high-value targets (motherships, stations)
- `Boarding Denial`: Actively counters boarding attempts with internal defenses and enslaved troops
- `Devour`: When a target is crippled, moves in to consume biomass/resources
- `Hivelord Projection`: When Hivelord suit is active nearby, gains powerful buffs

**Special:** High resistance to Resonance Burst; becomes more aggressive the more corruption is present in the sector.

### 3.5 Brood Spire (Mothership)

**Role:** Central command, production hub, Hivelord throne

**Primary States:**
- `Command`: Issue global orders, manage production queues
- `Production Wave`: Spawn large numbers of drones / Ravagers when threatened or ordered
- `Hivelord Link`: When Hivelord is active, becomes extremely aggressive and coordinates elite boarding
- `Desperation`: When Crownstone integrity is low, triggers mass sacrifice and corruption spread

---

## 4. Swarm Coordination Patterns

### 4.1 Envelopment
- Drones form a tightening sphere around high-value targets.
- Ravagers and Tyrants provide supporting fire from outside the sphere.
- Goal: Isolate target, create boarding windows, maximize psychological pressure.

### 4.2 Sacrifice Waves
- Triggered when `hivelord_command_strength > 0.7` or major target is isolated.
- Large numbers of drones perform coordinated kamikaze runs to overwhelm point defenses.
- Creates biomass explosion fields that damage nearby units (including friendly).

### 4.3 Adaptive Density
- The more Crownstone corruption is present in the sector, the higher the spawn rate and aggression of drones.
- Formula: `spawn_rate_multiplier = 1.0 + (crownstone_corruption_level * 2.0)`

### 4.4 Feint & Ambush
- Small groups of Ravagers feint attacks to draw player forces away from main objective.
- Main force (Tyrants + Abominations) then strikes the weakened position.

---

## 5. Adaptive & Evolutionary Behaviors

- **Post-Battle Learning:** After surviving a Resonance Burst, units gain temporary resistance (`resonance_resistance += 0.15` for 60 seconds).
- **Crownstone Corruption Spread:** Successful boarding or proximity to high-corruption areas slowly increases `crownstone_corruption_level` on player assets.
- **Hivelord Escalation:** When Hivelord suit takes damage or Crownstone integrity drops, fleet aggression and spawn rates increase dramatically.
- **Biomass Economy:** Destroyed friendly units return a percentage of biomass to the Brood Spire for faster reinforcement.

---

## 6. Integration with Other Systems

- **Boarding Mechanics:** AI decides when to attempt boarding based on `boarding_success_chance` and current state priorities.
- **Dogfight Mechanics:** Directly feeds into `SpaceCombatState` and per-unit behavior in dogfights.
- **Hivelord Counter-Strategies:** Many patterns are direct responses to Quellorian purification or redemption attempts.
- **Enslaved Minion Species:** Behavior patterns change significantly when Veythari, Korrath, Sylvaris, Luminari Exiles, or Voidweavers are present (coordinated songs, honor duels, grove fortifications, etc.).
- **Crownstone Trilemma Paths:** Different outcomes (Destroy / Capture / Sabotage) dramatically alter fleet behavior and available units.
- **RBE Moral Layer:** High consumption increases scarcity and desperation in the simulation; successful redemption reduces Draek aggression over time.

---

## 7. Technical Implementation Notes

### Recommended Data Structures

```rust
#[derive(Resource)]
pub struct DraekFleetAIState {
    pub global_hivemind_strength: f32,
    pub crownstone_link_strength: f32,
    pub active_escalation_level: u8, // 0-3
    pub biomass_reserve: f32,
    // ... per-sector data
}

#[derive(Component)]
pub struct DraekUnitBehavior {
    pub current_state: DraekAIState,
    pub local_hivemind_strength: f32,
    pub target_priority: f32,
    pub last_resonance_exposure: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DraekAIState {
    Patrol,
    Engage,
    Swarm,
    Board,
    Sacrifice,
    Regroup,
}
```

### Key Formulas (ready for simulation_integration.rs)

- Threat assessment, aggression scaling, kamikaze damage, spawn rate multipliers (as shown in sections above).

### ECS Recommendations
- Use a shared `Blackboard` resource for local hivemind knowledge.
- Event-driven updates from `CrownstoneState`, `HivelordCounterStrategyState`, and boarding events.
- Spatial partitioning (quadtree / grid) for efficient swarm queries.

### Direct Integration Hooks
- `simulation_integration.rs`: Core update loop for all Draek units.
- `rbe_engine.rs`: Biomass economy and moral consequences.
- `DOGFIGHT_MECHANICS.md` and boarding systems: Real-time behavior execution.

---

## 8. Development Priorities

1. Implement core per-class state machines in `simulation_integration.rs`.
2. Add swarm coordination queries and envelopment/sacrifice wave logic.
3. Integrate with Hivelord Counter-Strategies and Crownstone Trilemma outcomes.
4. Add post-battle learning and adaptive resistance.
5. Full enslaved minion species behavior overrides.
6. Performance optimization for large drone swarms (1000+ units).

---

**End of Document**

This completes the detailed, implementable behavior patterns for the Draek fleet. When combined with all previous documentation, the Draek side now feels like a terrifying, living, strategically deep antagonist ready for phenomenal gameplay implementation.