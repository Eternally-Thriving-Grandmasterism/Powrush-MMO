# Human Patchwork Ship Designs

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development  
**Related Documents:** `HUMAN_RACE_MECHANICS.md`, `HUMAN_HYBRID_PROTOCOL_CODE.md`, `HYBRID_INSTABILITY_MECHANICS.md`, `DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md`, `SHIP_VISUAL_GUIDELINES.md`

---

## 1. Lore Integration: The Phoenix Forge

Human ships are not born from a single coherent technological tradition. They are **forged in the fire of survival**.

After the **Draek Origin & The Great Betrayal**, when the Draeks stole Quellorian resonance technology and turned it into instruments of consumption, Humanity found itself on the front lines defending Earth alongside the Cydruids and with distant Quellorian support. 

Humans became master scavengers. They pulled elegant radial Quellorian hull plating from wrecks, grafted Draek biomechanical tendrils onto their own vessels for raw power, integrated Cydruid living wood for self-repair, and even risked Ambrosian crystal lattices for high-risk sensor arrays.

The result is the **Patchwork aesthetic** — chaotic, beautiful, scarred, and deeply personal. Every Human ship tells the story of the wars it has survived.

**Core Symbolism**
- **The Phoenix**: Rebirth through fire and adaptation.
- **The Mirror**: Reflects both the best and worst of the server’s moral state.
- **The Forge**: Constant transformation and innovation under pressure.

---

## 2. Visual Philosophy

Human ships embrace **asymmetric beauty**. They are not elegant like Quellorian vessels, nor terrifyingly organic like Draek ships. They are *alive with contradiction*.

- **Jury-rigged elegance**: Clean Quellorian lines interrupted by crude but effective Draek armor plates.
- **Living scars**: Hulls show battle damage that has been repaired with whatever was available — glowing green Cydruid bark patches, pulsing purple Draek veins, iridescent Ambrosian crystal shards.
- **Moral Reactivity**: The visual state of a Human ship changes based on the player’s (and server’s) moral alignment and RBE standing. High-mercy ships develop more auroral blue-white energy and harmonious Quellorian elements. Greedy or chaotic ships grow more Draek-like spikes, purple energy veins, and unstable hybrid modules.

**Color Palette**
- Base: Weathered gunmetal, scorched bronze, and matte black.
- Mercy-aligned accents: Soft auroral blue-white and warm gold.
- Greedy/Chaotic accents: Aggressive purple-red and sickly green.
- Hybrid energy: Iridescent shifting between multiple faction colors.

---

## 3. Core Ship Classes

### 3.1 Scavenger Frigate (Light / Boarding Specialist)

**Role**: Fast insertion, boarding actions, resource recovery.

**Visual Description**:
- Sleek Quellorian-inspired hull core with heavy Draek biomechanical "claws" grafted onto the front for boarding.
- Exposed wiring and Cydruid root patches where damage was repaired.
- Side-mounted Ambrosian crystal sensor spikes (high risk of instability).
- Paint and markings are a chaotic mix of old Earth military, Quellorian glyphs, and personal tags.

**Special Visual Effect**: When performing a successful boarding, the ship briefly "lights up" with stolen energy signatures from the boarded vessel.

### 3.2 Hybrid Cruiser (Core Combat Vessel)

**Role**: Mainline combat + Hybrid Protocol platform.

**Visual Description**:
- The most "Human" looking ship — a deliberate fusion of all available technologies.
- Central Quellorian radial core surrounded by asymmetrical Draek armor "wings" and Cydruid living hull sections.
- Multiple hardpoints for temporary hybrid modules (these visibly change the ship’s silhouette and energy signature when mounted).
- When the Hybrid Protocol is active, the ship pulses with unstable multi-colored energy and develops temporary crystalline growths or biomechanical tendrils.

**Mirror Reckoning Interaction**: The visual "purity" of the Hybrid Cruiser degrades the more unstable or morally compromised the server becomes.

### 3.3 Mirror Corvette (Stealth / Recon)

**Role**: Electronic warfare, signal reflection, deep reconnaissance.

**Visual Description**:
- Extremely irregular shape — looks like it was built from the salvaged parts of many different ships.
- Large external "mirror arrays" (repurposed Quellorian resonance dishes mixed with Draek psionic amplifiers).
- When cloaked or reflecting enemy signals, the ship’s hull becomes semi-transparent with shifting distorted reflections of nearby vessels.

**Special Effect**: In Mirror Reckoning events, these ships can temporarily take on the visual appearance of the server’s own Shadow fleet.

### 3.4 Forge Carrier (Support / Production)

**Role**: Deploys temporary hybrid drones, field repairs, resource processing.

**Visual Description**:
- Bulky, industrial appearance with large open "forge bays" that glow with intense orange-white light.
- Cydruid living wood scaffolding supports the bays.
- Constant emission of small repair drones that look like mechanical insects fused with plant matter.
- When producing hybrid units, the carrier grows temporary biomechanical or crystalline structures.

### 3.5 Last Stand Capital (Heavy Command Vessel)

**Role**: Fleet anchor, "Never Again" protocol, high-risk high-reward.

**Visual Description**:
- Massive, heavily scarred vessel that looks like it has survived multiple apocalypses.
- The more allies are destroyed around it, the more the ship "grows" — adding new armor plates, weapon hardpoints, and energy conduits from nearby wrecks in real-time.
- At critical health, it can activate the **Last Stand Protocol**, causing dramatic visual transformation: hull integrity fields flare white-gold, and the ship becomes a beacon of defiant light.

---

## 4. Alignment & Moral Reactivity

Human ships are the most visually reactive to moral state in the game.

| Moral State       | Visual Changes                              | Energy Signature          | Sound Design                     |
|-------------------|---------------------------------------------|---------------------------|----------------------------------|
| **High Mercy**    | More auroral blue-white, clean Quellorian lines re-emerge | Harmonious multi-layered  | Clean resonant chimes + human voices |
| **Neutral**       | Classic patchwork, balanced mix             | Mixed chaotic             | Industrial hum + occasional glitches |
| **Greedy/Chaotic**| More Draek spikes, purple veins, unstable modules | Aggressive distorted      | Grinding metal + corrupted Quellorian harmonics |

This makes Human ships powerful storytelling tools during **Mirror Reckoning** weekends.

---

## 5. Integration with Hybrid Protocol

The visual design of Human ships is built around the **Hybrid Protocol**:

- When a player activates Hybrid Protocol, the ship’s model dynamically morphs.
- Quellorian modules add elegant glowing fins and auroral trails.
- Draek modules add aggressive spikes and tendrils.
- Cydruid modules cause living wood and root growths.
- Ambrosian modules cause crystalline growths that can crack if instability rises.

Instability is **visually obvious** — the ship begins to visibly break apart at the seams with energy feedback and module rejection effects.

---

## 6. Technical Implementation Notes (Bevy)

- Use **velocity_prepass + TAA** pipeline for clean motion on these highly detailed, asymmetrical models.
- **Custom shader** for moral reactivity (blend between multiple texture sets and emissive maps based on `human_moral_alignment` variable).
- **Skeletal animation** or **morph targets** for dynamic module attachment/detachment during Hybrid Protocol.
- **Particle systems** for repair drones, hybrid energy feedback, and Last Stand Protocol explosion of light.
- **Audio hooks** from VoiceDirector for ship-specific voice lines that change with moral state.

**Recommended ECS Components**:
- `HumanShipVisualState`
- `HybridModuleVisuals`
- `MoralReactivity`

---

## 7. Development Priorities

1. Create base Human ship models with modular hardpoints.
2. Implement moral reactivity shader and texture blending system.
3. Build Hybrid Protocol visual morphing system.
4. Integrate with Mirror Reckoning visual feedback.
5. Polish Last Stand Protocol dramatic transformation effect.

---

**End of Document**