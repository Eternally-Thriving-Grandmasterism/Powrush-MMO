# Playable Races of Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026 · door/landing law stamped 2026-09-19 · C0 Cydruid-not-treant 2026-09-19  
**Status:** Active Development  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**

---

## 1. Introduction: The Five Playable Races

Powrush-MMO features **five core playable races**, each with distinct visual identities, voice profiles, musical themes, gameplay mechanics, and deep narrative integration into the eternal conflict between harmony and consumption.

These five races form the foundation of player choice and faction alignment:

1. **Quellorians (Aetherion Luminari)** — Elegant resonance masters of light and unity.
2. **Draeks** — Hivemind biomechanical empire of consumption and domination.
3. **Humans** — Adaptable, resilient survivors caught between two cosmic powers.
4. **Cydruids** — Humans housed in a cyborg frame (Chakra: body kept, machine worn · Vessel: head/brain in robot body); nature is practice / attunement, not species.
5. **Ambrosians** — Crystalline-organic resonance beings, living embodiments of harmonic attunement.

Beyond these five, the Draek Dominion has enslaved **countless other species** as mind-controlled minions, broken-willed mercenaries, and cannon fodder. These enslaved races are not playable in their current state but serve as tragic narrative and mechanical elements (with potential future redemption or rebellion paths).

This document provides deep, immersive detail for each playable race — design, voice, music, symbolism, unique attributes, and technical integration notes — ensuring maximal believability and emotional resonance for all players.

---

## 1.1 Door → landing law (Eden plane)

**Cite:** steward 2026-09-19 God-plane · this file's existing five Peoples list (sections 2–6). **Budget:** 0 meshes. Garden / Eden plane law lives in `PLACE_DRESS_SPEC` §2.1 — Garden is the walkable title / light-body plane, **not** a fifth Place and **not** a sixth People.

**5 doors = 5 Peoples. Landings = 4 Places only.**

| People (existing) | Door lands in | Place count |
|---|---|---|
| **Human** | Sanctuary yard | Sanctuary (same Place as Ambrosian) |
| **Cydruid** | Heartwood | Heartwood |
| **Quellorian** | Threshold | Threshold |
| **Draek** | Depths (teal way-home) | Depths |
| **Ambrosian** | Sanctuary well-from-above | Sanctuary — **same Sanctuary Place as Human, not a 5th room** |

Four Places remain: Sanctuary · Heartwood · Threshold · Depths. Market is not a Place. Do not invent a sixth People or a fifth Place. Title stays Play / Continue / Settings · **Online grey**. No race portraits on Title from this stamp.

Cydruid lands Heartwood. Heartwood Place may stay living-wood; **the Cydruid is not a tree.** Cite steward C0 · this Cydruid block.

Doors ignite after one Tend. Crossing a door is one-way this session. Peace recall = vision home, not a hub. This section does not authorize `client/**` work or L2 cook.

---

## 2. Quellorians (Aetherion Luminari)

**Symbolism:** Light, unity, resonance, dawn of a new era, elegant evolution.
**Core Theme:** "Through harmony, we transcend."

### Visual Design & Aesthetic
- **Overall Form:** Tall, slender, graceful humanoids with elongated limbs and multiple subtle radial symmetry elements (echoing their motherships).
- **Color Palette:** Luminous blue-white, soft gold, and flowing auroral gradients. Skin often has a faint bioluminescent sheen.
- **Armor & Clothing:** Elegant layered robes mixed with advanced crystalline armor plates that shift and resonate with movement. Helmets feature flowing energy crests.
- **Ships & Technology:** Radial, elegant, Protoss-inspired but uniquely organic-tech hybrid. Clean lines, glowing energy conduits, auroral effects during high-energy operations.
- **Distinctive Features:** Subtle glowing runes on skin that pulse with resonance strength. Multiple eyes or eye-like sensory nodes in higher-ranking individuals.

### Voice Acting Direction
- **Vocal Quality:** Clear, resonant, melodic, with natural harmonic layering (subtle choir-like overtones when speaking in groups or using powers).
- **Delivery Style:** Calm, wise, inspiring, slightly formal but warm. Speech has a gentle rhythmic cadence, as if always slightly in song.
- **Emotional Range:** Serene confidence, righteous determination, deep compassion, and rare but powerful righteous anger (voice rises in harmonic intensity).
- **Example Lines:**
  - "The Resonance flows through all. We are its guardians."
  - "Your hivemind cannot silence the song of a united people."
  - (During Resonance Burst) "Let harmony drown the darkness!"
- **Technical Notes:** Layered harmonic processing increases with Resonance Network strength. Voice distorts slightly when near strong Draek corruption fields.

### Music & Audio Identity
- **Theme:** Majestic, hopeful, orchestral with strong choral and crystalline elements. Rising arpeggios and flowing legato melodies.
- **Key Instruments:** Ethereal choirs, crystal singing bowls, resonant strings, soft brass, and subtle electronic harmonics.
- **In-Game Usage:** Peaceful exploration themes swell into triumphant battle hymns when Resonance Burst is active. Ambient mothership hums are beautiful and calming.

### Unique Features & Attributes
- **Resonance Attunement:** Can project and receive harmonic fields. Stronger in groups and near Ambrosians.
- **Elegant Precision:** Superior long-range accuracy and coordinated fleet maneuvers.
- **Unity Bonus:** Bonuses when fighting alongside other Quellorians or attuned Ambrosians.
- **Vulnerability:** Over-reliance on network can leave them vulnerable if the Resonance Network is disrupted.

### Playable Role
Players can align fully with the Quellorian Alliance or act as independent agents who favor harmony but may cooperate with other races for pragmatic reasons.

### Technical Implementation Notes
- Global resource: `ResonanceNetworkState` (already defined in prior docs).
- ECS components: `ResonanceField`, `HarmonyLevel`, `AuroralAura`.
- Direct integration with `simulation_integration.rs`, boarding mechanics, and Crownstone Trilemma (Quellorians strongly favor Destroy or Capture & Repurpose paths).

---

## 3. Draeks

**Symbolism:** Consumption, domination, hivemind unity through erasure of self, predatory evolution.
**Core Theme:** "All will be one in the Brood."

### Visual Design & Aesthetic
- **Overall Form:** Tall, imposing, biomechanical humanoids with exposed organic-mechanical tendrils, chitinous plating, and glowing energy veins.
- **Color Palette:** Dark metallic blacks, deep purples, blood reds, and sickly bioluminescent greens.
- **Armor & Clothing:** Heavy biomechanical armor fused with living tissue. The Hivelord’s suit is the pinnacle of this aesthetic.
- **Ships & Technology:** Vertical, oppressive spires and jagged bio-mechanical vessels. Everything looks grown rather than built.
- **Distinctive Features:** Crownstone users have prominent glowing purple crystals embedded in their bodies. Lower drones have simplified, almost insectoid faces.

### Voice Acting Direction
- **Vocal Quality:** Multi-layered, guttural, dissonant chorus. Primary voice + multiple distorted undertones (representing the hivemind speaking through the individual).
- **Delivery Style:** Cold, commanding, relentless. Speech often has a wet, organic quality mixed with metallic distortion.
- **Emotional Range:** Limited individual emotion. Becomes more unhinged and screeching as corruption or Crownstone influence increases. The Hivelord’s voice fractures dramatically when the Crownstone is threatened.
- **Example Lines:**
  - "You will be consumed. Resistance is the last illusion of the self."
  - "The Brood grows. Your song will be silenced."
  - (Hivelord, high corruption) "The Crownstone... it burns... yet it is mine!"
- **Technical Notes:** Voice layering and distortion intensity scales with `hivelord_corruption_level` and proximity to Crownstone or Brood Spire.

### Music & Audio Identity
- **Theme:** Dark, oppressive, industrial-organic. Deep drones, distorted choirs, pulsing rhythms that feel like a heartbeat or marching swarm.
- **Key Instruments:** Low brass, distorted choirs, heavy percussion, biomechanical sound design (wet squelches, metallic grinding, organic pulses).
- **In-Game Usage:** Menacing ambient tracks near Draek forces. Swells into overwhelming, dissonant war anthems during large hivemind-coordinated assaults.

### Unique Features & Attributes
- **Hivemind Coordination:** Extremely strong group bonuses when connected to the network. Weakens significantly when isolated.
- **Consumption Mechanics:** Can convert defeated enemies into resources or temporary units.
- **Crownstone Dependency:** High-ranking individuals gain massive power but become vulnerable to Crownstone-targeted attacks.
- **Vulnerability:** Resonance fields and isolation tactics severely weaken them.

### Playable Role
Players can embrace the full horror of the Draek Dominion or play as a more independent "rogue brood" agent who still uses hivemind tools but seeks personal power.

### Technical Implementation Notes
- Global resource: `DraekHivemindState` (already defined).
- Strong integration with boarding, Crownstone Trilemma (Draeks favor Sabotage or direct domination paths), and fleet AI systems.

---

## 4. Humans

**Symbolism:** Adaptability, resilience, hope, moral complexity, the "middle path" between light and darkness.
**Core Theme:** "We endure. We choose. We become."

### Visual Design & Aesthetic
- **Overall Form:** Standard human proportions but with advanced, practical armor and cybernetic or resonance-enhanced augmentations depending on alignment.
- **Color Palette:** Earthy tones mixed with faction colors (blue-white if aligned with Quellorians, purple-red if leaning Draek, or neutral grays/golds if independent).
- **Armor & Clothing:** Highly customizable. Rugged, functional designs with modular attachments. Can incorporate resonance crystals or biomechanical grafts.
- **Distinctive Features:** Visible cybernetic implants or glowing resonance tattoos depending on player choices. Scars and weathering that tell personal stories.

### Voice Acting Direction
- **Vocal Quality:** Natural human range — warm, gritty, or clear depending on background. No heavy layering unless heavily augmented.
- **Delivery Style:** Direct, pragmatic, emotionally expressive. Can sound hopeful, weary, determined, or ruthless depending on player actions and alignment.
- **Emotional Range:** Full human spectrum. Voice can gain subtle harmonic resonance if deeply attuned to Quellorians, or become colder and more distorted if heavily exposed to Draek influence.
- **Example Lines:**
  - "I’ve seen what both sides do. I’m done being a pawn."
  - "The Resonance feels... right. Like coming home."
  - "The hivemind offers power. But at what cost?"
- **Technical Notes:** Voice can dynamically gain subtle harmonic layers or slight distortion based on `crownstone_corruption_level` and alignment meters.

### Music & Audio Identity
- **Theme:** Versatile and emotional. Can shift between hopeful orchestral, gritty industrial, or haunting ambient depending on player alignment and current events.
- **Key Instruments:** Acoustic + electric guitars, powerful drums, human choirs, and subtle electronic elements that blend with faction music when aligned.
- **In-Game Usage:** Personal theme that evolves with player choices. Strongest emotional stingers during moral decisions (especially Crownstone Trilemma).

### Unique Features & Attributes
- **Adaptability:** Can gain bonuses from either Resonance or Hivemind systems (but not both at full strength).
- **Moral Compass:** Strongest narrative reactivity to RBE standing and Crownstone decisions.
- **Versatile Playstyle:** Excellent for players who want to switch alignments or remain independent.
- **Vulnerability:** Can be corrupted or overwhelmed more easily than the other races if not careful.

### Playable Role
Humans are the most flexible playable race. They can fully commit to Quellorian harmony, fall into Draek domination, or walk a dangerous independent path that affects the larger simulation.

### Technical Implementation Notes
- Use existing `WorldSimulationState` + new `HumanAlignment` component.
- Strong hooks into RBE moral layer and Crownstone Trilemma consequences (Humans feel the personal and societal cost most acutely).

---

## 5. Cydruids

**Cite:** steward C0 · D0 Eden @ `2afff36` · this Cydruid block · X status 1998585780420427986 (cite only; do not fetch binaries). **Budget:** 0 meshes.

**Law:** A Cydruid is a **human housed in a cyborg frame** — not a treant, not a bark creature, not tree-people. Nature is **practice / attunement**, not species. Heartwood Place may stay living-wood; **the person is not a tree.**

**STRIKE (do not draw / do not write as the People):** bark-skin · vines-as-body · heartwood-core-as-body · living-wood limbs as the People · treant-lean / tree-like species wording.

**Symbolism:** Human body kept or housed; machine worn or walked; nature as practice; guardianship without becoming the grove.
**Core Theme:** "The frame is worn. The grove is practiced. The person remains."

### Two paths (same People)

| Path | Body | Frame |
|---|---|---|
| **Chakra** | Human body kept | Machine worn (harness, plates, teal / leaf-metal trim) |
| **Vessel** | Human head / brain housed | Robot body walked |

Neither path grows bark, vines, or a heartwood-core-as-body. Teal / leaf-metal is **trim / metal accents**, not bark-skin (`ART_BIBLE`).

### Visual Design & Aesthetic
- **Overall Form:** Human proportions in a cyborg frame. Chakra path: visible human skin and posture, machine worn. Vessel path: human head / brain in a robot body. Not tree-like. Not vine-covered.
- **Color Palette:** Human skin + metal greys; teal / leaf-metal **trim and accents only** (not bark-skin). Soft gold fasteners. Instrument lights (cyan or amber) on the frame — not a glowing chest-core-as-heartwood.
- **Armor & Clothing:** Worn machine — plates, harness, mesh. Repair is mechanical / practiced, not bark that grows back. Technology is bolted, fitted, or worn; it does not grow wooden limbs.
- **Distinctive Features:** Human face (Chakra) or human head in a neck-ring (Vessel). Frame seams and teal / leaf-metal trim. No vines, no leaves that move with emotion, no heartwood-core-as-body.
- **Ships:** Frame-tech vessels with teal / leaf-metal trim. Not living-tree people. Heartwood Place may still read living-wood; the pilot is not the timber.

### Voice Acting Direction
- **Vocal Quality:** Human voice, close-miked. Frame may add a slight metallic or filtered edge (more on Vessel). No woody reverb. No rustling-as-speech.
- **Delivery Style:** Measured, practiced, human. Thoughtful without sounding ancient-as-oak.
- **Emotional Range:** Human spectrum — patient, sorrowful at wasted places, sharp when defending a practiced grove. Not "the forest speaking."
- **Example Lines:**
  - "I keep a human body. The machine is worn."
  - "The Place may be living-wood. I am not the tree."
  - "Attunement is a practice. It is not a species."
- **Technical Notes:** Voice may take a light frame filter near heavy machine rooms. Strain stays human under Draek corruption — dry throat, not dry bark.

### Music & Audio Identity
- **Theme:** Human pulse under practiced metal. Grounded, hopeful. Tribal percussion and flutes may mark **Heartwood Place** or nature-as-practice, not a tree-people anthem.
- **Key Instruments:** Hand drums, wooden flutes (Place / practice), singing bowls, soft synth pads, metal ticks from the frame. Natural Place beds (wind, water) belong to the room, not the skin.
- **In-Game Usage:** Heartwood exploration can stay living-wood ambient. The Cydruid theme is human + frame, not bark-choir.

### Unique Features & Attributes
- **Frame technology:** Can mend ships and structures as practiced machine-work. Strong defensive and sustainability bonuses.
- **Attunement mastery:** Nature as practice — excellent at bridging Quellorian resonance and certain neutral technologies without becoming a grove-species.
- **Place awareness:** Bonuses in natural or living-wood **Places** (Heartwood may stay living-wood). The bonus is attunement, not bark-blood.
- **Vulnerability:** Slower to adapt to pure hivemind environments; Vessel path can feel the frame more than the room.

### Playable Role
Cydruids often act as mediators or defenders of practiced balance. They can ally strongly with Quellorians (especially via Ambrosian attunement) or remain independent guardians. They are **humans in frames**, not a sixth vegetal species.

### Technical Implementation Notes
- Existing resource/component name `LivingSymbiosisState` (if kept) means practiced attunement + frame bond — **not** a tree-body.
- Strong integration potential with boarding (they can mend captured ships) and world simulation (Place / ecological impact tracking).
- This stamp does not authorize `client/**`, race lobby, or L2 cook.

---

## 6. Ambrosians

**Symbolism:** Pure resonance, crystalline harmony, attunement, the living song of the universe.
**Core Theme:** "We are the note. We are the chord. We are the song."

### Visual Design & Aesthetic
- **Overall Form:** Crystalline-organic humanoids. Bodies appear partially translucent with visible internal lattice structures that glow and shift.
- **Color Palette:** Pure luminous blues, whites, golds, and soft rainbows when in harmony. Turns cracked purple-red when Discordant.
- **Armor & Clothing:** Minimal physical armor — their bodies are living crystal that can harden or resonate. They often wear flowing energy mantles.
- **Distinctive Features:** Entire body can glow brighter with attunement strength. Cracks and discoloration appear when corrupted.
- **Ships:** Elegant crystalline vessels that resonate and sing when moving in formation.

### Voice Acting Direction
- **Vocal Quality:** Ethereal, multi-layered, bell-like and choral. Can sound like multiple voices singing in perfect harmony when in groups.
- **Delivery Style:** Flowing, melodic, almost sung rather than spoken. Very little individual "personality" in voice — more like a living instrument.
- **Emotional Range:** Limited traditional emotion. Expresses through harmonic intensity, pitch, and resonance strength. Becomes dissonant, cracked, and painful-sounding when Discordant.
- **Example Lines:**
  - "The Lattice sings. We answer."
  - "Your discord offends the song. It will be corrected."
  - (Discordant) "It hurts... the song is broken... make it stop..."
- **Technical Notes:** Voice is heavily processed with real-time harmonic layering. Discordant state applies heavy distortion, pitch instability, and painful overtones.

### Music & Audio Identity
- **Theme:** Pure, crystalline, choral, and transcendent. The most "musical" of all races.
- **Key Instruments:** Crystal singing, layered choirs, resonant bells, and pure sine-wave harmonics.
- **In-Game Usage:** Ambient "singing" from Ambrosian choirs on Quellorian ships. Beautiful swelling during attunement or Resonance Burst. Horrible dissonant shrieking when Discordant outbreaks occur.

### Unique Features & Attributes
- **Attunement Specialists:** Can greatly amplify Quellorian Resonance Network and Resonance Burst.
- **Discordant Risk:** Can fall into terrifying corrupted states that boost Draek forces.
- **Redemption Potential:** Strongest narrative and mechanical support for all three Discordant Redemption paths.
- **Vulnerability:** Extremely vulnerable to Crownstone influence and Hivelord direct corruption.

### Playable Role
Ambrosians are high-skill, high-reward. They excel at supporting Quellorian forces but carry the constant risk (and narrative weight) of falling into Discordant corruption — with powerful redemption arcs available.

### Technical Implementation Notes
- Direct integration with `AmbrosianAttunementState`, `DiscordantAmbrosianState`, and all three redemption path systems (already documented).
- One of the most mechanically and narratively rich playable races.

---

## 7. Enslaved Minion Species (Non-Playable / Limited)

The Draek Dominion has conquered and enslaved **dozens of sentient species** over millennia. These races serve as:

- Mindless or semi-mindless drone troops
- Broken-willed mercenary shock forces
- Living resources and biomass
- Tragic narrative background elements

**Examples of Enslaved Races (for world-building):**
- **The Vesh’kar** — Once proud warrior reptiles, now feral shock troops.
- **The Hollow Singers** — Former musical/psychic species reduced to screaming psychic amplifiers.
- **The Rootbound** — Ancient plant-like people turned into living siege engines.
- Many others exist as unnamed hordes.

These enslaved races are **not playable** in their current broken state. However, successful Crownstone Trilemma paths (especially Capture & Repurpose or Sabotage leading to civil war) can create opportunities for partial redemption or rebellion of these species in future content.

---

## 8. Technical Integration Summary

All five playable races should integrate with:
- `WorldSimulationState`
- `CrownstoneState` + Trilemma paths
- `ResonanceNetworkState` and `DraekHivemindState`
- Boarding, dogfight, and mothership mechanics
- RBE moral and abundance systems
- VoiceDirector, audio, VFX, and UI systems (as defined in prior documents)

Each race has clear mechanical asymmetry while remaining narratively and visually cohesive.

---

**End of Document**

*This document completes the core playable race foundation for Powrush-MMO. All future gameplay, narrative, audio, and visual systems should reference these five races symmetrically and with maximal immersion.*

`stamp: D0-2026-09-19-EDEN-PLANE-LAW | 5 doors = 5 Peoples · landings = 4 Places only (Human Sanctuary yard · Cydruid Heartwood · Quellorian Threshold · Draek Depths teal way-home · Ambrosian Sanctuary well-from-above, same Place as Human); no sixth People; no fifth Place; 0 meshes; Title Online grey; cite steward 2026-09-19 God-plane + this Peoples list; Garden law in PLACE_DRESS_SPEC §2.1; no client/shared/L2`

`stamp: C0-2026-09-19-CYDRUID-NOT-TREANT | Cydruid = human housed in a cyborg frame (Chakra: human body kept, machine worn · Vessel: human head/brain in robot body); nature = practice / attunement, not species; Heartwood Place may stay living-wood — the person is not a tree; STRIKE bark · vines · heartwood-core-as-body · living-wood limbs as the People · treant-lean / tree-like species; ART_BIBLE teal / leaf-metal = trim / metal accents, not bark-skin; 0 meshes; Title Online grey; cite steward C0 · D0 Eden @ 2afff36 · this Cydruid block · X status 1998585780420427986 (cite only); no client/shared/L2 / #459`