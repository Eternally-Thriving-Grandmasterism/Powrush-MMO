# Diplomacy and World Simulation — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**

---

## 1. Overview

The **Diplomacy and World Simulation** layer is the living heart of Powrush-MMO. It transforms all prior faction lore, mothership mechanics, fleet AI, boarding systems, Crownstone Trilemma, Ambrosian attunement/corruption, Resonance Burst, and redemption paths into a coherent, dynamic, mercy-aligned universe that evolves based on player and NPC actions.

This system ensures that every major decision (especially around the Crownstone) has cascading, realistic consequences across diplomacy, economy (RBE), fleet behavior, and long-term world state.

**Core Philosophy**  
- Harmony (Quellorian/Ambrosian) vs Consumption (Draek) as opposing forces  
- Mercy, choice, and redemption as meaningful mechanical paths  
- RBE abundance as the reward for harmonious play  
- True asymmetry: both factions feel alive, adaptive, and terrifyingly coherent

---

## 2. Core Simulation Systems

### 2.1 Global World State (`WorldSimulationState`)

```rust
#[derive(Resource)]
pub struct WorldSimulationState {
    pub crownstone_state: CrownstoneState,
    pub resonance_network: ResonanceNetworkState,
    pub draek_hivemind: DraekHivemindState,
    pub ambrosian_attunement: AmbrosianAttunementState,
    pub discordant_ambrosian: DiscordantAmbrosianState,
    pub hivelord_state: HivelordState,
    pub auroral_sovereign_state: AuroralSovereignState,
    pub faction_standing: FactionStanding,
    pub rbe_economy: RBEconomyState,
    pub last_resonance_burst: f32, // game time
    pub total_harmony: f32,
    pub total_corruption: f32,
}
```

### 2.2 Faction Standing & Reputation

**Quellorian Standing** (0.0 – 1.0+)
- Increased by: Successful purifications, Resonance Bursts that protect civilians, Ambrosian attunement, merciful boarding outcomes, Crownstone Capture & Repurpose path.
- Decreased by: Failed purifications leading to Discordant outbreaks, collateral damage, Resonance Burst backlash, Crownstone destruction without mercy.

**Draek Standing** (inverted scale for player perception)
- Increased by: Successful consumption, Hivelord interventions, Crownstone sabotage that weakens Quellorians.
- The hivemind does not "care" about standing in a human sense — it simply grows stronger or weaker.

**Technical Note**: Standing affects boarding success rates, AI aggression, trade access, and RBE resource multipliers.

---

## 3. Crownstone Trilemma Impact on Diplomacy & Simulation

The choice made in the Crownstone Trilemma is the single largest branching point in the entire simulation:

| Path                    | Immediate Effect                          | Long-term Diplomatic Consequence                          | RBE Impact                          | Simulation Variables Changed                  |
|-------------------------|-------------------------------------------|-----------------------------------------------------------|-------------------------------------|-----------------------------------------------|
| **Destroy**            | Hivemind collapse, feral remnants        | Quellorian moral victory but Draek desperation spikes    | Short-term harmony bonus, long-term scarcity risk | `crownstone_integrity = 0`, `total_corruption += 40` |
| **Capture & Repurpose**| Hybrid units + Ambrosian attunement      | New "Redemption Alliance" faction emerges                | Major abundance multiplier         | `crownstone_owner = Quellorian`, `harmony += 60` |
| **Sabotage**           | Slow civil war in Draek Dominion         | Hivelord instability events, possible internal Draek rebellion | Resource node corruption           | `crownstone_corruption_level` rises slowly   |

**Technical Hook**: The `CrownstoneState` enum directly feeds into `WorldSimulationState` and triggers global events.

---

## 4. RBE (Resource-Based Economy) Integration

Powrush-MMO uses a true Resource-Based Economy (RBE) where resources are abundant when harmony is high and scarce when corruption dominates.

**Key Mechanics**:
- **Harmony Level** directly increases resource regeneration rates across Quellorian-controlled space.
- **Corruption Level** creates "starvation zones" where Draek forces suffer production penalties but also become more desperate/aggressive.
- **Redemption actions** (Surgical Purification, Self-Redemption, successful Capture & Repurpose) create permanent abundance nodes.
- **Hivelord interventions** can temporarily spike consumption in an area, creating localized scarcity events.

**Formula Example** (simplified):
```rust
pub fn calculate_resource_regen(zone_harmony: f32, base_rate: f32) -> f32 {
    base_rate * (1.0 + zone_harmony * 2.0) // Harmony dramatically increases abundance
}
```

---

## 5. Event-Driven World Simulation

Major events that reshape the universe:

- **Resonance Burst** (Quellorian) — Massive temporary harmony spike + hivemind disruption. Can trigger Discordant outbreaks if overused.
- **Hivelord Direct Intervention** — When Crownstone is threatened, the Hivelord can appear in fleet combat or trigger emergency corruption waves.
- **Ambrosian Outbreak** — Discordant corruption spreading from failed purifications or Crownstone exposure.
- **Redemption Wave** — Successful Self-Redemption or Surgical Purification can cause chain reactions of attunement across nearby Ambrosian choirs.
- **Brood Evolution Event** — Draek AI learns from repeated losses and evolves new ship variants or tactics.

These events are queued in `WorldSimulationState` and processed every simulation tick.

---

## 6. Leadership Influence on Simulation

### The Hivelord
- High `hivelord_command_strength` increases Draek fleet coordination and aggression.
- Crownstone damage directly reduces his command effectiveness and can trigger desperate retaliation.
- If Crownstone is captured/repurposed, the Hivelord enters a permanent weakened or enraged state.

### The Auroral Sovereign (Elyndor)
- Projects harmony across Quellorian space.
- Can personally participate in major Resonance Bursts (increasing their power).
- Vulnerable during boarding actions on the Auroral Unification Nexus.

### High Resonance Keeper (Veyra)
- Directly controls Ambrosian attunement success rates.
- Key figure in all three redemption paths.

---

## 7. Technical Implementation Notes

### Recommended Global Resources (Bevy ECS)

```rust
// In simulation_integration.rs or a dedicated world_sim module
#[derive(Resource)]
pub struct DiplomacySimulationPlugin;

impl Plugin for DiplomacySimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSimulationState>()
           .add_systems(Update, (
               update_faction_standing,
               process_world_events,
               apply_crownstone_effects,
               update_rbe_economy,
           ).chain());
    }
}
```

### Key Integration Points
- `simulation_integration.rs` — Core tick loop that updates all states.
- `rbe_engine.rs` — Reads harmony/corruption to adjust resource nodes and player abundance.
- `dogfight_mechanics.rs` / combat systems — Read standing and Crownstone state to modify AI behavior and boarding windows.
- Boarding systems — Report outcomes back to `WorldSimulationState` (e.g. successful purification increases harmony).

### Event System Recommendation
Use Bevy Events for major simulation changes:
```rust
#[derive(Event)]
pub struct CrownstoneTrilemmaResolved {
    pub path: TrilemmaPath,
    pub success: bool,
}
```

This allows decoupled systems (UI, VFX, AI, economy) to react cleanly.

---

## 8. Development Priorities

1. Implement `WorldSimulationState` and core update systems in `simulation_integration.rs`.
2. Wire Crownstone Trilemma resolution into the global state and trigger appropriate events.
3. Connect RBE economy to harmony/corruption levels.
4. Add standing modifiers from boarding outcomes and Resonance Burst usage.
5. Create event listeners for Hivelord interventions and Ambrosian outbreaks.
6. Build UI/debug tools to visualize current world state (harmony map, standing, active events).
7. Balance formulas so that both harmonious and aggressive playstyles remain viable and meaningful.

---

**End of Document**

*This document ensures that every system previously designed (motherships, fleets, AI, boarding, Crownstone, redemption, leadership) now operates inside one living, mercy-aligned universe.*