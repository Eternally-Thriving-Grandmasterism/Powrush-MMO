# Ambrosian Crystalline Ship Designs — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development

---

## 1. Lore Integration & Core Philosophy

The Ambrosians are crystalline-organic resonance beings who embody **Universal Harmonic Convergence**. Pre-fall, they maintained peaceful, symbiotic relations with many races, including the ancient Draeks and early Cydruids. They observed the Great Betrayal from afar — watching the cloning catastrophe destroy Draek ethics and the subsequent theft of Quellorian resonance technology. For centuries they chose detachment, advancing their own lattices rather than sacrificing for others.

This changed when the Draek threat reached Earth. Forced to support the Quellorian alliance, Ambrosian ships now enter the war as living extensions of their Attunement Lattice.

**Core Philosophy:** Ambrosian ships are not built — they are **grown and sung** into existence. Every facet, every floating shard, every lattice strut is a living crystal that resonates with the collective harmony (or discord) of its crew and the greater network.

## 2. Visual Language & Color Palette

- **Harmonious State**: Iridescent crystal lattices shifting between deep sapphire blue, radiant gold, and soft auroral white. Internal light pulses in perfect rhythmic waves. Floating crystal shards orbit gracefully.
- **Discordant / Corrupted State**: Jagged purple-red-black crystal growths, visible cracks leaking unstable energy, sharp discordant spikes, and chaotic floating shard movement. The lattice looks "sick" and painful.
- **Propulsion**: No traditional engines. Ships "sing" themselves through space via resonance fields that leave beautiful auroral wakes when harmonious, or painful jagged distortion trails when corrupted.
- **Key Motif**: The **Lattice Crown** — a glowing crystalline structure at the core of every Ambrosian ship that represents its attunement level and connection to the greater lattice.

## 3. The Five Core Ship Classes

### 3.1 Lattice Weaver Frigate
**Role:** Support / Attunement Specialist  
**Size:** ~180m

**Design:** A elegant, elongated crystal lattice frame with multiple floating "weaver" shards that orbit and connect via glowing resonance threads. The ship looks delicate but is incredibly resilient.

**Harmonious Version:** Graceful floating shards that weave protective resonance fields around allies. Auroral shimmer.
**Discordant Version:** Shards spin erratically, firing painful resonance spikes. Cracks leak purple energy.

**Gameplay:** Deploys small crystal drones that link into temporary resonance networks, granting harmony bonuses or spreading Discordant corruption.

### 3.2 Harmonic Blade Cruiser
**Role:** Mainline Combatant  
**Size:** ~420m

**Design:** Sharp, blade-like crystal structures with extendable resonance "blades" that can project harmonic lances or perform devastating close-range strikes. The hull has a more aggressive, faceted geometry than other Ambrosian ships.

**Harmonious Version:** Blades glow with pure blue-white light and leave beautiful resonance trails.
**Discordant Version:** Blades become jagged and black-veined, capable of rending enemy lattices with painful feedback.

**Gameplay:** High-mobility resonance striker. Can temporarily "sing" itself into a higher attunement state for burst damage or shielding.

### 3.3 Choir Shard Carrier
**Role:** Drone / Swarm Support  
**Size:** ~380m

**Design:** A large central crystal "choir" core surrounded by hundreds of smaller floating crystal shards that act as living fighters. The ship looks like a floating crystalline constellation.

**Harmonious Version:** Shards move in perfect synchronized patterns, creating beautiful geometric formations.
**Discordant Version:** Shards move chaotically, colliding with each other and enemies in painful, self-destructive swarms.

**Gameplay:** Deploys living crystal fighter swarms that can be commanded to focus fire, create resonance shields, or sacrifice themselves in explosive attunement bursts.

### 3.4 Attunement Nexus Cruiser
**Role:** Command / Buff Ship  
**Size:** ~510m

**Design:** A majestic, multi-layered lattice structure with a glowing central "Nexus Core" that pulses with the ship’s attunement level. Smaller crystal nodes orbit at varying distances.

**Harmonious Version:** The Nexus Core shines with warm golden light. Resonance waves visibly ripple outward to allied ships.
**Discordant Version:** The Core throbs with unstable purple-red energy. Resonance waves become painful and disruptive.

**Gameplay:** Projects powerful attunement fields that amplify nearby Quellorian resonance or Ambrosian self-redemption attempts. High-value target for Hivelord forces.

### 3.5 Lattice Sovereign Capital
**Role:** Rare Capital / Flagship  
**Size:** ~1.8km

**Design:** A breathtaking, cathedral-like crystalline structure with multiple layered lattice "wings" and a massive central Lattice Crown. It looks like a floating, living crystal palace.

**Harmonious Version:** Breathtaking auroral phenomena surround the ship during high-attunement operations.
**Discordant Version:** Massive jagged spikes erupt from the lattice. The ship can trigger localized Discordant outbreaks.

**Gameplay:** Can perform a limited **Self-Redemption Burst** or, if corrupted, trigger a devastating **Discordant Choir Outbreak**. Extremely rare and high-stakes vessel.

## 4. Enslaved / Corrupted vs Redeemed / Attuned States

| Aspect                    | Enslaved / Discordant State                  | Redeemed / Harmonious State                     |
|---------------------------|----------------------------------------------|-------------------------------------------------|
| **Hull Appearance**       | Jagged, cracked, purple-red-black crystal   | Smooth, elegant facets, iridescent blue-gold   |
| **Energy Effects**        | Painful spikes, chaotic floating shards     | Graceful auroral waves, synchronized movement  |
| **Movement**              | Erratic, painful                            | Elegant, harmonic resonance flight             |
| **Sound Design**          | Dissonant shrieking crystal layers          | Beautiful multi-layered choral resonance       |
| **Redemption Potential**  | High (via Ambrosian Self-Redemption Path)   | Already pure — can assist others            |

## 5. Gameplay Integration

- **Dogfights**: Extremely high mobility and resonance-based shielding. Weak against sustained focused fire but can rapidly re-attune if given breathing room.
- **Boarding**: Extremely difficult due to living crystal lattice resistance. Successful boarding can trigger powerful attunement feedback (positive or negative depending on player alignment).
- **Redemption**: Ambrosian ships can participate in **Ambrosian Self-Redemption Path** rituals and support **Grove Communion Rituals** for Sylvaris.
- **Mirror Reckoning**: Servers with high Ambrosian detachment manifest terrifying **Fractured Choir Mirrors** — beautiful crystalline horrors that sing painful dissonance.
- **Human Hybrid Protocol**: Humans can attempt to mount Ambrosian lattice modules, but with very high instability risk (and high reward if mitigated).

## 6. Technical Implementation Notes

- **Bevy Pipeline**: Fully compatible with existing `velocity_prepass.rs` + TAA. Use velocity buffers for beautiful motion trails on floating shards.
- **Shaders**: Custom iridescent crystal shader with dynamic `corruption_level` and `attunement_strength` parameters. Recommended: emissive + refraction + subtle vertex displacement for living lattice feel.
- **Particles**: Floating crystal shards (GPU instanced), resonance wave rings, auroral particle fields, Discordant energy leaks.
- **ECS Resources**: Extend `AmbrosianRaceState` and `AmbrosianSelfRedemptionState` with per-ship attunement and corruption tracking.
- **Integration Hooks**: Direct ties to `WorldSimulationState`, `CrownstoneState`, `DiscordantAmbrosianState`, Hivelord Counter-Strategies, Mirror Reckoning, and VoiceDirector (ships should audibly shift from choral beauty to painful dissonance).

## 7. Development Priorities

1. Finalize concept art and 3D models for Lattice Weaver Frigate and Harmonic Blade Cruiser (highest immediate need).
2. Implement core iridescent crystal shader with dynamic corruption parameter.
3. Create particle systems for floating shards and resonance waves.
4. Integrate ship attunement/corruption state into `AmbrosianRaceState` and simulation systems.
5. Add visual transformation effects for Ambrosian Self-Redemption Path and Discordant outbreaks.

---

**End of Document**