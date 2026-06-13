# Boarding Mechanics — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**  
**AG-SML v1.0 Sovereign License**

---

## 1. Overview: The Art of Asymmetric Warfare

Boarding operations represent one of the most cinematic, high-stakes, and strategically deep gameplay pillars in Powrush-MMO. While fleet combat and mothership duels provide spectacle, **boarding** is where the narrative, technical, and moral layers of the conflict between the Quellorian / Aetherion Luminari Alliance and the Draek Dominion truly collide.

Boarding is deliberately asymmetric:

- **Quellorian doctrine** emphasizes precision, resonance mastery, elite strike teams, and surgical disruption.
- **Draek doctrine** emphasizes overwhelming swarm numbers, hivemind coordination, consumption, and relentless corruption.

Successful boarding can shift the balance of an entire engagement, capture critical assets for the Resource-Based Economy (RBE), or trigger major narrative branches (especially around the Crownstone).

This document provides both rich lore and **detailed technical implementation notes** so that when we later build the simulation systems, combat loops, RBE economy integration, and mothership interactions, everything flows coherently and feels phenomenally realistic.

---

## 2. Quellorian Boarding Doctrine (Precision & Resonance)

### Core Philosophy
The Quellorians view boarding not as conquest but as **liberation and stabilization**. Their teams are highly trained, small in number, and equipped with advanced resonance technology that disrupts enemy systems rather than simply destroying them.

### Key Boarding Assets
- **Seraphim-Class Strike Teams** (elite operatives from the Auroral Unification Nexus)
- **Resonance Disruption Projectors** (portable devices that create localized anti-hivemind fields)
- **Luminar Escort Frigates** for insertion and extraction under fire

### Boarding Mechanics (Gameplay Feel)
- Small, high-skill squads (3–6 operatives)
- Emphasis on stealth, timing, and resonance synchronization
- Primary goal: disable key systems (command nodes, production forges, psionic relays) rather than total slaughter
- Strong defensive bonuses when operating near resonance amplification chambers or allied mothership resonance fields

### Technical Implementation Notes (for later integration)
- **Resonance Field Differential Formula** (pseudo-code ready for simulation_integration.rs):
  ```rust
  let success_chance = base_quellorian_skill * (resonance_strength_quellorian / resonance_strength_draek).clamp(0.6, 1.8);
  ```
- Boarding squads carry limited "Resonance Charge" that depletes over time inside hostile hivemind zones.
- Captured Draek assets can be **repurposed** into Quellorian-aligned vessels (with efficiency penalties and risk of latent hivemind corruption).
- Integration hook: Success modifies the global RBE resource flow (captured bio-matter becomes usable resources after purification).

---

## 3. Draek Boarding Doctrine (Swarm & Consumption)

### Core Philosophy
The Draek Dominion treats boarding as **infestation and assimilation**. Their goal is rarely to capture intact technology — it is to consume, corrupt, and multiply.

### Key Boarding Assets
- **Swarm-Class Drone Fighter** waves for initial softening
- **Ravager-Class Bio-Corvette** boarding pods
- **Tyrant-Class Heavy Cruiser** command nodes that project local hivemind dominance
- **Abomination-Class Capital Devourer** for massive multi-vector assaults

### Boarding Mechanics (Gameplay Feel)
- Massive numbers of smaller, cheaper units
- Rapid spread of "Corruption" status effect across decks
- Consumption of crew/resources to heal and reinforce the boarding force
- Hivemind relay nodes that amplify control strength the longer they remain active

### Technical Implementation Notes (for later integration)
- **Hivemind Propagation Model** (core simulation layer):
  - Control strength decays with distance from nearest relay node or the Brood Spire itself.
  - Disruption "dead zones" can be created by destroying relay nodes (high-value targets for Quellorian boarders).
  - Captured Quellorian ships suffer progressive morale/resource drain unless the Crownstone influence is countered.
- **Consumption Loop**:
  - Boarded ships lose resources over time; those resources are converted into new Draek boarding units or sent back to the Brood Spire.
  - This creates natural tension: Quellorians must board quickly or lose the asset permanently.
- Integration hook: Strong hivemind presence on a boarded ship can trigger **Crownstone Resonance Events** (see below).

---

## 4. Mothership Boarding — The Ultimate Prize

Boarding either mothership is a late-game, high-drama event that can decide the fate of entire star systems.

### Auroral Unification Nexus (TAUN) — Quellorian Mothership
- **Defensive Strengths**: Radial symmetry allows overlapping resonance fields; multiple hangar arrays for rapid response; Living Sanctums that provide morale and healing buffs to defenders.
- **Vulnerable Points**: Outer resonance amplification chambers (if overloaded, can cause temporary field collapse); Command Spire access conduits.
- **Boarding Experience**: Feels like infiltrating a majestic, living cathedral of light. Resonance harmonics can literally push boarders back or empower defenders.

### Brood Spire (TBS) — Draek Mothership
- **Defensive Strengths**: Bio-mechanical spines that impale boarding craft; Gestation Decks that spawn endless reinforcements; Psionic Relay Network that maintains near-perfect hivemind cohesion.
- **Critical Vulnerability**: The **Crownstone Chamber** deep in the Apex Command Spire. Destroying or capturing it can shatter local Draek coordination.
- **Boarding Experience**: Nightmarish, claustrophobic, and oppressive. Corruption spreads visibly across decks in real time.

### Technical Implementation Notes
- Mothership boarding uses a dedicated high-fidelity simulation layer (separate from normal ship boarding for performance).
- Success on TAUN can grant temporary "Resonance Overdrive" buffs to the entire Quellorian fleet.
- Success on TBS can trigger a **Hivemind Cascade Failure** (massive debuff to all nearby Draek forces) or allow **Crownstone capture** for narrative/RBE consequences.

---

## 5. The Crownstone — Ultimate Boarding Objective

The Crownstone is not just a powerful artifact — it is a **narrative and mechanical singularity**.

### Boarding the Crownstone Chamber (TBS)
- Requires penetrating the Apex Command Spire under heavy resistance.
- Once inside, the boarding team faces a moral and strategic trilemma:
  1. **Destroy** the Crownstone → Immediate massive hivemind collapse in the local sector + long-term Draek retaliation.
  2. **Capture & Repurpose** → Extremely difficult; grants Quellorians powerful psionic tools but risks corruption and ethical dilemmas in RBE society.
  3. **Leave Intact but Sabotage** → Temporary disruption with potential for future questlines.

### Technical Implementation Notes (Critical for Coherent Flow)
- Crownstone state is tracked as a global simulation variable (crownstone_status: Destroyed | Captured | Active | Corrupted).
- Capturing it can unlock new Quellorian research trees (psionic resonance weapons) while applying a persistent "Moral Debt" modifier to RBE resource generation (reflecting ethical cost).
- Destroying it can cause short-term chaos (good for Quellorians) but long-term Draek evolution toward more aggressive strains.
- Perfect integration point for Ra-Thor AGI narrative systems and PATSAGi Council decision simulations.

---

## 6. Side-by-Side Comparison

| Aspect                    | Quellorian Boarding                          | Draek Boarding                               |
|---------------------------|----------------------------------------------|----------------------------------------------|
| **Squad Size**            | Small elite (3–6)                           | Large swarm (dozens to hundreds)             |
| **Primary Tactic**        | Precision disruption & liberation            | Overwhelming infestation & consumption       |
| **Key Technology**        | Resonance fields & disruption projectors     | Hivemind relays & bio-corruption             |
| **Resource Impact**       | Captured assets can be purified & reused     | Consumes resources to multiply forces        |
| **Risk Profile**          | High skill, lower numbers, recoverable       | High volume, high consumption, self-reinforcing |
| **Mothership Target**     | Command Spire access + resonance chambers    | Crownstone Chamber (high risk/reward)        |
| **RBE Integration**       | Repurposed ships become productive assets    | Consumed matter fuels further expansion      |
| **Narrative Tone**        | Hopeful, surgical, redemptive                | Terrifying, corrupting, inevitable           |

---

## 7. Development Priorities & Integration Roadmap

1. Implement core boarding simulation layer in `simulation_integration.rs` and `rbe_engine.rs` (using the formulas above as starting point).
2. Create distinct boarding "arenas" for normal ships vs motherships (performance optimization).
3. Build Crownstone state machine with full narrative + RBE consequences.
4. Design boarding UI/UX that communicates resonance vs hivemind tension clearly.
5. Add boarding success/failure feedback into the global Powrush RBE economy (resource shifts, morale events).
6. Prototype first playable boarding scenario between a Luminar-Class Cruiser and a Ravager-Class Bio-Corvette.
7. Expand to full mothership boarding event as a major story beat.

---

**End of Document**

*This boarding mechanics foundation ensures that when gameplay systems are built, every action feels coherent with the deep lore of the Auroral Unification Nexus, the Brood Spire, the Crownstone, and both faction fleets. The universe will breathe as one.*