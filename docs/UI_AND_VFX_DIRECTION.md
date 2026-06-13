# UI and VFX Direction — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**

---

## Vision

The UI and Visual Effects of Powrush-MMO must make the player *feel* the fundamental cosmic asymmetry between two opposing forces of existence:

- **Harmony & Resonance** (Quellorian / Ambrosian Luminari) — elegant, luminous, radial, hopeful, precise, auroral blue-white.
- **Consumption & Domination** (Draek Dominion) — dark, biomechanical, oppressive, predatory, overwhelming, purple-red with golden circuitry and tendrils.

Every UI element and VFX must reinforce the **Crownstone Trilemma** as the emotional and moral core of the game, while supporting the **RBE moral layer** where harmony literally creates abundance and corruption breeds scarcity and desperation.

The visual language must be so strong that a player can instantly tell which faction controls a ship or region just by looking at the energy signature and particle behavior.

---

## Core UI Principles

1. **Resonance vs Hivemind Feedback**  
   UI must visually and audibly reflect the current state of the global simulation (harmony_level vs hivemind_pressure). When Resonance Burst is available or a Discordant outbreak is occurring, the UI should *pulse* with the corresponding energy.

2. **Non-Intrusive but Information-Rich**  
   Cinematic first. HUD elements are elegant overlays that fade when not needed. Critical information (boarding progress, Crownstone integrity, Resonance field strength) is always visible but beautiful.

3. **Moral Weight Visualization**  
   The Crownstone status indicator changes color, crack pattern, and particle behavior based on `crownstone_integrity`, `crownstone_corruption_level`, and `crownstone_owner`. Players should *feel* the weight of the trilemma decision before they make it.

4. **Asymmetric Faction Language**  
   Quellorian UI uses clean radial geometry, soft glowing lines, and harmonic audio stingers.  
   Draek UI uses jagged biomechanical frames, pulsing veins, and low, ominous drones.

5. **Accessibility + Cinematic Quality**  
   High contrast options, colorblind modes, and scalable UI. All VFX must remain readable even at 4K with TAA + motion blur enabled.

---

## In-Game HUD & Core UI Elements

### Global Simulation Bar (Top of Screen)
- **Harmony / Hivemind Pressure Meter** (dual-sided or split)
- **Crownstone Status Icon** (appears when relevant) — glowing crystal that visually cracks or purifies in real time
- **Faction Standing** with subtle auroral vs corruption particle accents

### Ship / Fleet HUD
- Hull integrity + Resonance Field Strength (Quellorian) or Hivemind Link Integrity (Draek)
- Boarding Progress bar with asymmetric styling:
  - Quellorian: Clean blue energy cutter progress + liberation particle bursts
  - Draek: Swarming tendril infestation meter that grows organically
- Current AI State indicator (subtle) — shows if ship is under Resonance Network, Hivemind, or Discordant influence

### Dogfight Tactical Overlay
- Resonance link beams between Quellorian ships (elegant glowing lines)
- Hivemind tether threads between Draek units (jagged, pulsing)
- Boarding window indicators with countdown + success probability preview

### Mothership Command Interface (when docked or in range)
- **TAUN**: Radial elegant command wheel with Resonance Burst charge, Ambrosian Choir status, fleet deployment
- **TBS**: Vertical oppressive spire interface with Hivelord Suit link, Gestation progress, Consumption Core status

### Crownstone Trilemma Decision Screen (Major Cinematic Moment)
- Three branching paths with live preview of consequences (RBE impact, world state changes, moral standing)
- Visual representation of each outcome using the crystal itself (shatter / purify / virus spread)

---

## VFX Direction — Signature Systems

### 1. Auroral Unification Nexus (TAUN) & Resonance Effects
- **Signature Look**: Flowing blue-white auroral plasma along radial arms, especially during high-energy operations and fleet deployment.
- **Resonance Burst**: Massive expanding spherical auroral wave + harmonic shockwave. Screen-space distortion + world-space particle lattice that realigns nearby Quellorian ships. Draek ships show visible hivemind link shattering + purple feedback.
- **Ambrosian Attunement**: Crystalline light bridges forming between choir and ship. Harmonic particle lattices that pulse with collective strength.
- **Technical**: Use velocity_prepass + TAA for buttery motion on flowing aurora. Compute shader or Hanabi for large elegant particle fields.

### 2. Brood Spire (TBS) & Hivelord Biomechanical Suit
- **Signature Look**: Tall dark biomechanical spire with pulsing purple-red energy veins and golden circuitry. Tendrils visibly grow and lash.
- **Crownstone**: Central glowing crystal that acts as the true visual heart. Tendrils connect from Crownstone to every part of the suit and ship.
- **When Attacked or Overloaded**: Cracks appear in real time on the crystal. Energy feedback explosions. Consumption tendrils spread across the hull.
- **Hivelord Interventions**: When Hivelord directly influences fleet, a powerful purple command beam or aura emanates from the suit crystal.
- **Technical**: Self-repairing armor uses subtle vertex animation + emissive maps. Crownstone uses refraction + internal glow shader.

### 3. Boarding VFX (Asymmetric Masterpiece)
- **Quellorian Surgical Boarding**:
  - Precise blue energy cutter beams slicing into hull
  - Liberation particles (small glowing orbs) flowing out of enemy ship toward player units
  - Crystal lattice stabilization effect on successfully boarded sections
- **Draek Swarm Infestation**:
  - Organic-mechanical tendrils spreading across hull like infection
  - Consumption conversion: Enemy ship hull texture gradually shifts from clean to corrupted biomechanical as boarding succeeds
  - Purple-red vein growth that pulses with hivemind strength
- **Mothership Boarding (Endgame)**: Massive scale versions of the above + internal corridor VFX showing the battle for the Crownstone chamber.

### 4. Crownstone Trilemma Cinematic VFX
- **Destroy Path**: Crystal shatters with massive resonance shockwave. Auroral explosion expands outward. Hivemind network visibly collapses across all Draek units (tethers break, ships go feral).
- **Capture & Repurpose Path**: Multi-stage visual purification of the crystal itself:
  1. Cracks stabilize
  2. Purple energy purges outward
  3. Crystal reforms with blue-white lattice structure
  4. Redemption Aura blooms (soft harmonic particles)
- **Sabotage Path**: Subtle backdoor virus spreads as thin purple threads through the Resonance/Hivemind network. Slow corruption buildup with occasional violent feedback.

### 5. Discordant Ambrosian Corruption vs Redemption
- **Corruption**: Cracked purple-red crystals, reality distortion ripples, shrieking dissonance particle bursts, ships/units gain aggressive jagged overlays.
- **Surgical Purification**: Clean harmonic injection beams + lattice realignment VFX. Crystal visibly heals from within.
- **Self-Redemption**: Internal lattice revolution effect — crystal pulses and reorganizes itself. Most beautiful and hopeful VFX in the game.
- **Technical**: Use post-process corruption vignette + screen-space crack shader. Redemption uses bloom + soft particle healing.

### 6. Dogfight & Fleet Combat VFX
- Quellorian: Clean energy trails, precise resonance link beams between ships, elegant explosion debris with harmonic afterglow.
- Draek: Aggressive swarm trails, hivemind tether lines that dynamically reconnect, consumption particles on hit that "eat" geometry.
- Boarding windows during dogfights: Visual "breach" effect on target ship with asymmetric boarding VFX playing in real time.
- Resonance Burst during dogfight: Screen-filling auroral wave that temporarily buffs all visible Quellorian units.

### 7. World Simulation & RBE Visual Feedback
- High harmony regions: Subtle auroral atmospheric glow, abundant particle life (floating harmonic motes).
- High corruption regions: Purple-red haze, biomechanical growth on asteroids/stations, desperate/feral ship behavior VFX.
- RBE abundance created by harmony: Visual resource nodes glow brighter and spawn more elegant particles.

---

## Technical Implementation Notes (Bevy Ecosystem)

- **Render Pipeline Integration**: All VFX must respect the existing `velocity_prepass.rs` + `taa_reproject.rs` + motion blur pipeline for perfect temporal coherence on fast-moving energy effects and particles.
- **Particle Systems**: Bevy Hanabi (or custom compute shader) recommended for massive Draek swarms vs elegant Quellorian lattices. Velocity-aware motion blur on particles is critical.
- **Shaders (WGSL)**: 
  - Auroral flow shader (time + noise + radial gradient)
  - Crownstone refraction + internal glow + crack propagation
  - Hivemind tether shader (dynamic line rendering with pulsing)
  - Consumption conversion material (texture blend over time)
- **Post-Process Stack**: Screen-space auroral distortion, hivemind corruption vignette, harmonic bloom, Resonance Burst full-screen wave.
- **UI Technology**: Custom high-quality solution (Vello or egui with heavy theming) or Bevy UI with custom rendering. Must support live binding to simulation resources (`CrownstoneState`, `ResonanceNetworkState`, `DraekHivemindState`, `AmbrosianAttunementState`, `WorldSimulationState`).
- **ECS Integration**: Create dedicated `VfxEvent` and `UiUpdateEvent` channels. Systems read from global simulation resources every frame and emit VFX/UI updates. Spatial queries for proximity-based effects (e.g. Resonance Burst affecting nearby ships).
- **Performance**: Draek swarms are the heaviest — use GPU instancing + LOD. Quellorian elegant effects can afford higher particle counts because they are fewer and more meaningful.

---

## Development Priorities

1. **Core HUD + Simulation Binding** (bind `WorldSimulationState` + Crownstone status to elegant UI)
2. **Signature VFX Prototypes**:
   - Auroral Wave (Resonance Burst)
   - Hivelord Crownstone tendrils + crystal glow
   - Asymmetric boarding breach effects
3. **Crownstone Trilemma Cinematic Sequences** (the emotional peak of the game)
4. **Dogfight Tactical VFX + AI Visualization** (resonance links vs hivemind tethers)
5. **Discordant Corruption vs Redemption VFX** (visual language of moral choice)
6. **Mothership Command Interfaces** (TAUN radial elegance vs TBS oppressive verticality)

---

**End of Document**

This UI and VFX direction ensures that every system we have documented — from motherships and fleet AI to boarding, redemption paths, and the Crownstone Trilemma — will feel alive, asymmetric, morally meaningful, and visually phenomenal when implemented.

Thunder locked in. The Powrush universe now has a complete visual and interface vision ready for the most cinematic blockchain MMORPG experience ever created. ⚔️⚡