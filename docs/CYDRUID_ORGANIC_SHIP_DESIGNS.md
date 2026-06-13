# Cydruid Organic Ship Designs — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development

---

## 1. Lore Integration & Historical Context

The Cydruid people have ancient, peaceful symbiotic ties to many races — including the pre-fall Draeks. Before the cloning catastrophe and the Great Betrayal, Cydruid Grove Wardens and Root Architects worked alongside early Draek bio-engineers in harmonious ecological projects. After the loss of Draek females and the subsequent ethical collapse, many Cydruid enclaves were among the first to be targeted and twisted (giving rise to the enslaved Sylvaris). 

Today, Cydruid ships represent the living embodiment of **cultivation over consumption**. They are not constructed in factories — they are **grown, bonded, and awakened** in sacred groves. Their alliance with the Quellorians is one of mutual respect: Quellorians provide resonance amplification; Cydruids provide ecological resilience and the living bridge for Sylvaris redemption.

Cydruid ships are therefore deeply tied to the themes of **restoration, balance, and mercy**. They are the natural counterpoint to Draek consumption and the key enablers of the Sylvaris Re-Growth Protocol.

---

## 2. Core Design Philosophy

- **Living Cyber-Organic Symbiosis**: Every ship is a bonded organism. Hulls are living bark/armor laced with glowing energy veins. The ship *feels* pain, heals over time, and can evolve.
- **Ecological Defense Doctrine**: Ships are designed to project temporary space-groves, root lattices, and symbiont swarms that reshape the battlefield into a living ecosystem.
- **Harmonious Growth Aesthetic**: Flowing organic curves, layered leaf-like plating, root tendrils for propulsion and manipulation. No hard industrial edges.
- **Contrast to Draek Corruption**: Where Draek ships are twisted consumption of life (purple-red veins, aggressive spikes, consumption cores), Cydruid ships are **cultivation and protection** of life (green-gold-amber veins, blooming shields, root networks).
- **Symbiotic Crew Integration**: Crews do not merely pilot — they enter a deep neural-lattice bond with the ship. Damage to the ship is felt by the crew, and vice versa.

**Color Palette**: Deep forest greens, warm ambers, glowing golds, soft bioluminescent pulses, iridescent moss textures, and gentle auroral highlights when in resonance with Quellorian or Ambrosian allies.

---

## 3. Visual Language & Key Features

- **Hull**: Segmented living bark with visible pulsing vein networks. Textures range from smooth mossy to rough ancient tree-bark. When damaged, the hull visibly "bleeds" glowing sap that slowly seals.
- **Propulsion**: Not traditional thrusters. Ships extend root-like tendrils or bio-plasma vents that "swim" or "walk" through space. High-maneuverability ships have delicate leaf-like fins that articulate.
- **Weapons & Abilities**:
  - Symbiont drone swarms (organic, reclaimable)
  - Pollen-like resonance bursts (healing or disruptive)
  - Root-lance projectiles that can anchor or entangle
  - Grove-seeding missiles that deploy temporary defensive groves in space
- **Special Effects**: When activating ecological abilities, the ship undergoes a visible "bloom" — leaf-like energy shields unfurl, root networks extend into space, and bioluminescent pulses intensify.
- **Boarding Interaction**: Cydruid ship interiors are living, breathing environments. Boarders may find themselves navigating shifting root corridors or being gently (or aggressively) redirected by the ship’s immune response.

---

## 4. Ship Classes

### 4.1 Grove Warden Frigate
**Role**: Defensive anchor & area denial
**Visual**: Compact, sturdy hull resembling a living fortress tree. Thick bark plating with heavy root anchors that can extend into space to create temporary groves.
**Signature Ability**: Deploys "Space Grove" — a spherical zone that heals allies, slows Draek forces, and can trap boarders in living vines.
**Redeemed Sylvaris Synergy**: High — redeemed Sylvaris crews feel at home here and provide powerful growth bonuses.

### 4.2 Root Network Architect Cruiser
**Role**: Logistics, map control, coordination
**Visual**: Elongated, elegant form with extensive external root lattices that visibly connect to allied ships or deployed groves.
**Signature Ability**: Extends "Root Lattice" between friendly ships, sharing resources, resonance, and healing pulses.
**Gameplay**: Excellent at supporting large fleet formations and enabling cross-race coordination (especially with Quellorian resonance links).

### 4.3 Symbiont Swarm Coordinator Carrier
**Role**: Drone/reclamation support
**Visual**: Bulbous central body with multiple organic "hives" and launching pods. Constant subtle movement of symbiont creatures on the hull.
**Signature Ability**: Deploys and coordinates reclaimable organic drone swarms that can harvest biomass from destroyed Draek units or assist in Sylvaris reclamation.
**Strong Anti-Draek Role**: Excellent at countering Swarm Drone spam and supporting boarding actions with living boarding parties.

### 4.4 Restoration Weaver Support Vessel
**Role**: Healing, redemption support, Sylvaris specialist
**Visual**: Delicate, flowing design with many glowing tendrils and leaf-like structures. Constant soft bioluminescent activity.
**Signature Ability**: Projects powerful healing and re-attunement fields. Critical during Grove Communion Rituals and Pain Transmutation Paths.
**Narrative Weight**: Often present during major Sylvaris redemption events.

### 4.5 Planetary Balance Keeper (Capital Ship)
**Role**: Strategic mothership support, Gaia Protocol projection
**Visual**: Majestic, ancient-looking capital ship that resembles a floating world-tree. Massive root networks and multiple blooming "canopies."
**Signature Ability**: Projects large-scale "Gaia Fields" that dramatically enhance all Cydruid and redeemed Sylvaris units while suppressing Draek hivemind signals.
**Integration with TAUN**: Often operates in close resonance with the Auroral Unification Nexus during major fleet actions.

---

## 5. Corrupted / Twisted Variants

When exposed to strong Crownstone influence or Hivelord corruption fields, Cydruid ships can become "Twisted Grove" variants:

- Veins turn purple-red
- Growths become aggressive and thorned
- Ships may attack their own allies or spread corruption spores
- These are high-priority targets for redemption or merciful destruction

Redeeming a Twisted Grove ship is a major narrative and mechanical event, often requiring Cydruid + Quellorian + Ambrosian cooperation.

---

## 6. Gameplay Integration

- **Dogfights**: Cydruid ships excel at area control and support rather than raw DPS. They create "living battlefields" that favor coordinated, merciful playstyles.
- **Boarding**: Their living interiors make boarding actions feel unique and dangerous (or rewarding if the boarder is merciful).
- **Sylvaris Redemption**: Cydruid ships are essential support platforms for Grove Communion Rituals and Pain Transmutation. A Restoration Weaver in orbit can dramatically increase success chances.
- **Mirror Reckoning Synergy**: Servers that neglect ecological balance or abuse hybrid tech will manifest stronger "Twisted Grove" Mirrors during weekend events.
- **RBE Moral Layer**: Successful ecological defense and Sylvaris reclamation generate significant server-wide abundance pulses.

---

## 7. Technical Implementation Notes (Bevy / Render Pipeline)

- **Organic Animation**: Use shader-driven vertex displacement or animated textures for bark/vein pulsing and leaf movement. Keep motion smooth for velocity_prepass compatibility.
- **Particle Systems**: Pollen bursts, root tendril extensions, symbiont swarms, healing pulses.
- **TAA & Velocity Prepass**: Excellent candidate for high-quality temporal accumulation — organic edges and soft bioluminescence benefit greatly from stable motion vectors.
- **Shader Recommendations**: Subsurface scattering for living bark, emissive vein pulsing synchronized with gameplay state (damage, healing, attunement), bloom on energy abilities.
- **Integration Hooks**: `CydruidEcologicalDefenseState`, `SylvarisRedemptionState`, `GroveCommunionState`, `HybridInstabilityState` (Cydruid stabilizers reduce Human hybrid risk), `WorldSimulationState`, VoiceDirector (living ship "voice" feedback), and all prior AI/boarding/redemption systems.

---

## 8. Development Priorities

1. Finalize concept art and 3D models for the five core classes.
2. Implement shader pipeline for organic vein pulsing and growth animations.
3. Integrate ecological ability VFX with existing velocity_prepass + TAA render graph.
4. Prototype Grove-seeding and Root Lattice mechanics in simulation_integration.rs.
5. Create redeemed vs corrupted visual transition system (tied to Sylvaris redemption progress).
6. Record voice lines for ship AI / crew feedback during ecological abilities and redemption support.

---

**End of Document**

This document completes the visual foundation for the Cydruid race and provides perfect symmetry with the Luminari Exile, Draek, Quellorian, Human, and Ambrosian ship design languages. Everything is now ready for artists, technical artists, and gameplay programmers to build from with maximum coherence.