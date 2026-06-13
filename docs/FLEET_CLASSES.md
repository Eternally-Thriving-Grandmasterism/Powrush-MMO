# Fleet Classes — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related:** FACTIONS_OVERVIEW.md, AURORAL_NEXUS.md, BROOD_SPIRE.md

---

## 1. Overview & Strategic Context

The naval forces of both civilizations are built around their respective motherships. The **Auroral Unification Nexus (TAUN)** serves as a majestic mobile carrier and resonance hub, while the **Brood Spire (TBS)** functions as a terrifying mobile bio-factory and domination platform.

Fleet composition reflects core philosophies:

- **Quellorian / Aetherion Luminari**: Elegant, coordinated, resonance-enhanced vessels optimized for precision, protection, and long-range projection of unity.
- **Draek Dominion**: Overwhelming numbers, biomechanical adaptability, hivemind coordination, and predatory efficiency through consumption and assimilation.

All ship classes below include **technical implementation notes** for future gameplay systems (combat, boarding, production, diplomacy, and simulation integration) so that mechanics feel coherent, realistic, and deeply tied to the lore.

---

## 2. Quellorian / Aetherion Luminari Fleet

### Design Philosophy
Clean radial symmetry, glowing blue-white energy conduits, elegant curves, and visible resonance field emitters. Ships produce subtle auroral light trails during high-performance maneuvers or when resonating with the TAUN.

### Ship Classes

#### Aether-Class Light Interceptor (Escort / Recon)
- **Role**: Rapid response, scouting, harassment, point defense.
- **Size**: ~45m length, highly maneuverable.
- **Crew**: 2 (pilot + resonance operator) or fully autonomous resonance-linked mode.
- **Armament**: Dual resonance pulse cannons (disruptive to organic/hivemind targets), micro-missile pods, light boarding harpoons for capture ops.
- **Special Systems**: Resonance Link (can temporarily boost nearby friendly shields or disrupt Draek psionic signals within 2km).
- **Technical Gameplay Notes**:
  - Fastest Quellorian ship; ideal for hit-and-run and escorting larger vessels.
  - Resonance pulse deals bonus damage to Draek bio-units and can "jam" nearby hivemind links (reduces Draek coordination temporarily).
  - Boarding harpoons enable early-game capture of small Draek drones for study or reverse-engineering (ties into Crownstone research paths).

#### Luminar-Class Heavy Cruiser (Line Ship / Command)
- **Role**: Mainline combatant, fleet anchor, mobile command node.
- **Size**: ~380m length.
- **Crew**: 180 + automated resonance drones.
- **Armament**: Triple heavy resonance lances, broadside plasma batteries, resonance shield projectors, 4x boarding shuttle bays.
- **Special Systems**: Auroral Field Projector (extends TAUN-style aurora effect to protect nearby allies with temporary damage reduction).
- **Technical Gameplay Notes**:
  - Backbone of any Quellorian fleet. Can link with TAUN for massive shield boost when within resonance range.
  - Boarding mechanics: Can launch coordinated marine teams into enemy capital ships; success chance increased if target is already disrupted by resonance pulses.
  - Production tie-in: Requires refined resonance crystals (produced in TAUN Research Wings).

#### Harmony-Class Support Carrier (Logistics / Drone Deployment)
- **Role**: Drone carrier, repair & resupply, resonance amplification relay.
- **Size**: ~620m length (large but not capital).
- **Crew**: 420 + extensive automated systems.
- **Armament**: Defensive resonance turrets, anti-fighter screens, limited offensive lances.
- **Special Systems**: Resonance Relay Array (extends TAUN resonance field to nearby ships, improving accuracy and shield regeneration).
- **Technical Gameplay Notes**:
  - Critical for sustained operations far from the TAUN. Can deploy repair drones that heal both hull and shield of friendly ships.
  - When linked to TAUN, can temporarily open "Resonance Gates" allowing rapid reinforcement deployment (gameplay: instant small-fleet teleport within system).
  - Vulnerable to boarding; if captured, Draek can corrupt the relay and turn it into a temporary hivemind beacon.

#### Seraphim-Class Capital Escort (Heavy Escort / Anti-Capital)
- **Role**: Protect the TAUN and larger fleet elements from capital threats.
- **Size**: ~920m length.
- **Crew**: 650.
- **Armament**: Spinal resonance cannon (devastating vs capital ships), heavy point-defense networks, multiple boarding teams.
- **Special Systems**: Divine Aegis (temporary invulnerability bubble when synchronized with TAUN).
- **Technical Gameplay Notes**:
  - Designed specifically to counter Brood Spire escort swarms and protect the mothership during major engagements.
  - Spinal cannon can target and destroy exposed Crownstone relay nodes on Draek capital ships (high-value target for skilled players).
  - High resource cost; losing one impacts faction morale and resonance network stability.

---

## 3. Draek Dominion Fleet

### Design Philosophy
Dark biomechanical, jagged aggressive silhouettes, exposed organic tubing and glowing red-purple energy veins. Ships visibly "breathe" and can self-repair by consuming resources or biomass from the battlefield.

### Ship Classes

#### Swarm-Class Drone Fighter (Swarm / Cannon Fodder)
- **Role**: Overwhelming numbers, screening, suicide runs, resource harvesting.
- **Size**: ~18m length (smallest combat unit).
- **Crew**: 0 (pure hivemind drone) or 1 enslaved pilot.
- **Armament**: Bio-plasma spitters, ramming spikes, self-destruct consumption charge.
- **Special Systems**: Hivemind Link (instant coordination; if link is strong, they swarm intelligently).
- **Technical Gameplay Notes**:
  - Extremely cheap and fast to produce in Brood Spire gestation decks.
  - When hivemind connection is disrupted (by Quellorian resonance), they become erratic and easier to destroy.
  - Can perform "Consumption Ram" on disabled enemy ships to harvest resources directly into the Brood Spire economy.
  - Boarding target: Very difficult to board successfully due to small size and self-destruct tendency.

#### Ravager-Class Bio-Corvette (Fast Attack / Raider)
- **Role**: Hit-and-run, boarding operations, terror tactics.
- **Size**: ~95m length.
- **Crew**: 12 + 40 boarding drones.
- **Armament**: Heavy bio-plasma cannons, boarding tendrils, toxin injectors.
- **Special Systems**: Adaptive Carapace (regenerates hull when consuming nearby biomass or resources).
- **Technical Gameplay Notes**:
  - Primary boarding platform for early-to-mid game. Can latch onto Quellorian ships and begin draining resources while deploying boarders.
  - If successful boarding, can corrupt Quellorian resonance systems (temporarily disables special abilities).
  - Weak vs concentrated resonance fire; Quellorian interceptors can exploit their exposed organic sections.

#### Tyrant-Class Heavy Cruiser (Line Breaker)
- **Role**: Heavy assault, breaking enemy lines, supporting Brood Spire advances.
- **Size**: ~410m length.
- **Crew**: 85 + extensive slave labor + automated bio-systems.
- **Armament**: Multiple bio-plasma batteries, spinal consumption beam, heavy boarding claws.
- **Special Systems**: Hivemind Overdrive (temporarily boosts all nearby Draek ships at the cost of crew/biomass).
- **Technical Gameplay Notes**:
  - Can directly interface with Brood Spire for massive power boosts when within psionic range.
  - Consumption beam can steal resources from enemy ships or stations (transfers to player economy if controlled).
  - High-value boarding target: Capturing the bridge allows temporary control or sabotage of nearby hivemind nodes.

#### Abomination-Class Capital Devourer (Super Heavy / Anti-Mothership)
- **Role**: Capital ship hunter, Brood Spire escort, planetary assault.
- **Size**: ~1.1km length.
- **Crew**: 300 + thousands of drones.
- **Armament**: Massive bio-plasma cannons, multiple boarding tendril arrays, planetary bombardment spores.
- **Special Systems**: Crownstone Relay (direct link to Hivelord’s Crownstone for command override and power amplification).
- **Technical Gameplay Notes**:
  - Designed to threaten the TAUN directly. Its Crownstone Relay makes it extremely dangerous when the Hivelord is active.
  - Can launch massive boarding waves; if it successfully boards the TAUN, it can begin corrupting Resonance Chambers (major endgame threat).
  - Destroying its exposed Crownstone Relay nodes (visible weak points) significantly weakens the ship and nearby Draek forces (ties into Crownstone narrative quests).
  - Extremely expensive to produce; losing one is a major strategic blow.

---

## 4. Direct Comparison

| Aspect                    | Quellorian Fleet                              | Draek Fleet                                   |
|---------------------------|-----------------------------------------------|-----------------------------------------------|
| **Aesthetic**             | Elegant, radial, luminous blue-white          | Jagged, biomechanical, aggressive red-purple  |
| **Coordination**          | Resonance-linked, precise, elegant            | Hivemind swarm, overwhelming numbers          |
| **Production Style**      | High-quality, resonance-crystal intensive     | Rapid bio-growth, resource consumption        |
| **Boarding Capability**   | Precision marine teams + harpoons             | Tendril swarms + corruption                   |
| **Special Mechanics**     | Resonance boosts, auroral shields, disruption | Adaptive regeneration, consumption, hivemind  |
| **Mothership Synergy**    | Direct resonance link with TAUN               | Direct psionic link with TBS + Hivelord       |
| **Strategic Weakness**    | Expensive; vulnerable to swarm tactics        | Disrupted by strong resonance fields          |
| **Gameplay Fantasy**      | Precision, protection, technological mastery  | Overwhelming force, terror, assimilation      |

---

## 5. Technical Integration Notes for Gameplay Systems

These notes ensure future systems feel coherent:

- **Boarding & Capture**: Quellorian boarding is surgical and high-skill. Draek boarding is swarm-based and corrupting. Captured ships can be repurposed (Quellorian: study & improve; Draek: consume or assimilate).
- **Hivemind vs Resonance**: Strong Quellorian resonance fields temporarily weaken Draek coordination (global debuff near TAUN or Harmony carriers). Draek Crownstone Relays can corrupt resonance nodes if they get too close.
- **Production & Economy**: Quellorian ships require rare resonance crystals (mined or synthesized in TAUN). Draek ships grow quickly but require constant biomass/resources (can harvest from battlefield or captured ships).
- **Mothership Interaction**: TAUN can recall/repair Quellorian ships rapidly. TBS can rapidly spawn new Swarm/Ravager units. Both motherships become vulnerable if their supporting fleet is destroyed.
- **Narrative Hooks**: Destroying or capturing an Abomination-Class with an active Crownstone Relay can trigger major story branches (Crownstone research, Hivelord confrontation, or even redemption paths).

---

## 6. Current Status & Development Priorities (June 2026)

**Completed**:
- Core ship classes defined for both factions
- Technical specs and gameplay integration hooks established
- Clear contrast between resonance vs hivemind mechanics

**Next Priorities**:
1. Detailed 3D model references and concept art briefs for all classes.
2. Balance pass on boarding success rates, resource costs, and resonance/hivemind interaction ranges.
3. Integration with existing simulation systems (resource flows, crew morale, faction diplomacy).
4. Crownstone questline hooks (especially around Abomination and Ravager classes).
5. Dogfight / fleet engagement prototype scenarios between the two fleets.

---

**End of Document**