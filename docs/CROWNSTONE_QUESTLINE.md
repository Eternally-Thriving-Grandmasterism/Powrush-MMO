# Crownstone Questline — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** FACTIONS_OVERVIEW.md, BROOD_SPIRE.md, BOARDING_MECHANICS.md, FLEET_CLASSES.md

---

## 1. Overview: The Crownstone

The **Crownstone** is the single most consequential artifact in the Powrush universe. It is the glowing purple crystal embedded in the Hivelord’s Suit helmet — the true source of the Draek Dominion’s advanced psionic domination capabilities.

More than a power source, the Crownstone is a **living psionic amplifier** grown from a rare resonance-corrupted crystal found in the deepest layers of the original Draek homeworld. It acts as both a battery and a transmitter, allowing the Hivelord to project and maintain control over vast numbers of enslaved or dominated forces across star systems.

Its existence creates the central moral and gameplay trilemma of the mid-to-late game:

- **Destroy it** — End the immediate threat, but risk catastrophic feedback.
- **Capture & Repurpose it** — Turn the weapon of domination into a tool of liberation or controlled power.
- **Sabotage it** — Weaken the hivemind without fully destroying the crystal, opening new narrative and strategic paths.

This questline is designed to feel epic, morally complex, and deeply integrated with the simulation, RBE economy, and faction gameplay.

---

## 2. The Artifact’s Nature & Technical Foundations

### Physical & Psionic Properties
- **Composition**: Resonance-corrupted crystalline lattice infused with Draek bio-psionic matrix. Visually: deep purple with shifting internal fractals that react to nearby consciousness.
- **Power Output**: Capable of sustaining domination fields across entire fleets and planetary populations when linked to the Brood Spire.
- **Resonance Signature**: Extremely high on the psionic spectrum. Quellorian resonance technology can detect, interfere with, or even harmonize with it under specific conditions.
- **Vulnerability**: The crystal has a "core resonance frequency" that, if disrupted at the right moment (during high-energy operations or when the Hivelord is personally interfacing), can cause cascading failure or controlled overload.

### Technical Implementation Notes (for future gameplay integration)
- **Global Simulation Variable**: `crownstone_integrity` (0.0 – 1.0). Starts at 1.0. Affects global Draek control strength, resource conversion efficiency in Brood Spire, and hivemind propagation speed.
- **Psionic Field Range Formula** (used in simulation):
  `effective_domination_range = base_range * crownstone_integrity * (1 + 0.3 * hivemind_nodes_active)`
- **Feedback Risk**: If integrity drops below 0.4 while the Hivelord is actively projecting, a `psionic_backlash` event can trigger (damages nearby Draek units + potential temporary control loss).
- **Integration Hook**: `simulation_integration.rs` should read `crownstone_integrity` every major tick and propagate effects to `rbe_engine.rs` (resource conversion penalties) and boarding success calculations.

---

## 3. The Crownstone Questline Structure

The questline becomes available after the player successfully boards the Brood Spire (see BOARDING_MECHANICS.md) and reaches the Apex Command Spire or manages to isolate the Hivelord.

### Phase 1: Discovery & Infiltration
- Player learns of the Crownstone’s existence through captured Draek data or Ambrosian resonance scans.
- Multiple paths open depending on previous boarding choices and faction standing.

### Phase 2: The Trilemma Decision Point
During the climactic confrontation aboard the Brood Spire or in a high-stakes space battle near it, the player faces the trilemma. The choice is not a simple dialogue option — it is shaped by:
- Previous boarding tactics used
- Resonance vs hivemind alignment level
- Resources invested in research (Quellorian tech trees)
- Moral standing in the RBE simulation

#### Path A: Destroy the Crownstone
- **Method**: Overload the crystal using coordinated resonance disruption from multiple Quellorian vessels + elite boarding team planting charges.
- **Immediate Effects**:
  - Massive psionic backlash wave (damages all nearby Draek forces + temporary stun on Hivelord).
  - `crownstone_integrity` set to 0.0 permanently.
  - Brood Spire enters critical failure mode (production halts, many enslaved units break free or go berserk).
- **Long-term Consequences**:
  - Draek Dominion suffers major strategic setback (weaker control, easier future operations against them).
  - Potential for new "Feral Swarm" factions to emerge from uncontrolled remnants.
  - High moral approval from liberation-focused Quellorian allies and RBE communities.
  - Risk: Some Ambrosian allies may view the destruction of such a powerful artifact as wasteful.

#### Path B: Capture & Repurpose the Crownstone
- **Method**: Surgical boarding + resonance dampening field to safely extract the crystal from the Hivelord’s helmet without triggering full overload.
- **Immediate Effects**:
  - Hivelord is defeated but not killed (can become recurring antagonist or potential redemption arc).
  - `crownstone_integrity` drops to ~0.6 but remains usable.
  - Player gains a powerful new asset: the Crownstone can be installed in a Quellorian or player-controlled capital ship as a "Psionic Command Node".
- **Long-term Consequences**:
  - Unlocks unique gameplay: controlled domination of certain Draek units (with risk of corruption over time).
  - Major RBE moral tension — using a tool of slavery, even for liberation, creates ethical debates in player communities and simulation NPCs.
  - Potential for "Crownstone Corruption" mechanic: prolonged use slowly shifts player alignment toward more authoritarian control options.
  - High strategic value: Can be used to turn enemy fleets or to create "liberated" zones with reduced conflict.

#### Path C: Sabotage the Crownstone
- **Method**: Subtle interference during boarding — plant a resonance beacon that slowly destabilizes the crystal over time without immediate destruction.
- **Immediate Effects**:
  - `crownstone_integrity` begins gradual decay (e.g., -0.05 per major game cycle).
  - Hivelord retains short-term control but suffers increasing instability (erratic behavior, failed commands).
  - Creates opportunities for ongoing missions to accelerate or slow the decay.
- **Long-term Consequences**:
  - Most morally "gray" path. Avoids massive backlash but leaves the Dominion weakened and unpredictable.
  - Can lead to internal Draek civil war or power struggles.
  - Opens unique narrative branches involving Ambrosian scholars studying the decaying crystal for "benevolent" applications.
  - RBE simulation impact: Creates ongoing resource and refugee flows from destabilized Draek territories.

---

## 4. Moral & RBE Consequences (Deep Integration)

The Crownstone questline is one of the primary vehicles for meaningful moral choice in Powrush-MMO’s RBE (Resource-Based Economy) framework.

- **Destroy Path**: Strong "Liberation" moral vector. Increases standing with freedom-focused factions, unlocks certain Ambrosian technologies, but may reduce access to powerful psionic tools.
- **Capture Path**: Creates "Controlled Power" tension. Player communities will debate whether repurposing a domination tool is justified. Simulation NPCs will react differently based on transparency of use.
- **Sabotage Path**: "Pragmatic Disruption" vector. Favored by players who prefer long-term strategic weakening over dramatic action. Can lead to more complex diplomatic situations.

**Technical Hook for RBE Engine**:
In `rbe_engine.rs`, track a new global variable `crownstone_moral_impact`. This value influences:
- Resource distribution fairness scores
- NPC trust and cooperation rates
- Availability of certain high-tier resonance or psionic technologies
- Player reputation multipliers in different regions

---

## 5. Technical Implementation Notes for Gameplay Integration

### Simulation Variables to Track
- `crownstone_integrity` (float 0.0–1.0)
- `crownstone_owner` (enum: Draek, Quellorian_Player, Neutral, Corrupted)
- `crownstone_corruption_level` (float, increases with use in Path B)
- `psionic_backlash_risk` (bool, triggers on low integrity + high projection)

### Integration Points
- `simulation_integration.rs`: Update `crownstone_integrity` and propagate effects to global control strength, production rates, and boarding difficulty.
- `rbe_engine.rs`: Use `crownstone_moral_impact` and `crownstone_owner` to adjust resource conversion, trade routes, and NPC behavior.
- Boarding systems: Success chance and outcome of Crownstone-specific actions should reference current `crownstone_integrity` and player research level in resonance tech.
- Narrative system: Quest state machine should branch based on which path was chosen and feed back into faction diplomacy and world events.

### Future Expansion Ideas
- Multiplayer impact: If one player captures the Crownstone, it becomes a server-wide event with consequences for all players.
- Post-quest content: A weakened or repurposed Crownstone can become the center of new endgame content (defending it, studying it, or facing the consequences of its destruction).

---

## 6. Current Status (June 2026) & Development Priorities

**Completed**:
- Core trilemma structure and narrative branches defined
- Technical variables and integration hooks documented
- Moral/RBE consequence framework established
- Consistency with Brood Spire, Hivelord’s Suit, and Boarding Mechanics

**Next Development Priorities**:
1. Write detailed quest step breakdowns with dialogue trees and mission objectives for each path.
2. Prototype the resonance disruption mini-game / boarding sequence for the Crownstone confrontation.
3. Implement `crownstone_integrity` tracking in the simulation layer.
4. Design visual effects for Crownstone overload, extraction, and sabotage states.
5. Create companion lore documents for potential redemption arcs (Hivelord) or Ambrosian research paths.
6. Balance testing of moral vectors and RBE impacts across different player archetypes.

---

**End of Document**

*This questline is designed to be one of the most memorable and consequential experiences in Powrush-MMO — where player choice genuinely shapes the future of entire civilizations and the moral fabric of the RBE universe.*