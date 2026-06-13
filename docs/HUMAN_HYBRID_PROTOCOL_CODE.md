# Human Hybrid Protocol Code — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Production-Ready Design  
**Related Documents:** `HUMAN_RACE_MECHANICS.md`, `DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md`, `CYDRUID_ECOLOGICAL_DEFENSE_ROLES.md`, `CROWNSTONE_TRILEMMA_PATHS.md`, `MIRROR_RECKONING_EVENT.md`

---

## 1. Lore Context & Philosophical Foundation

The **Human Hybrid Protocol** is the signature mechanical identity of the Human race in Powrush-MMO. It embodies humanity’s greatest strength and greatest danger: **radical adaptability through synthesis**.

### Historical Tie-in (Draek Origin & Great Betrayal)
- Pre-fall Draeks worked peacefully with early human civilizations and Cydruids.
- After the loss of Draek females and the cloning catastrophe, Draeks stole Quellorian resonance technology.
- Humans, caught between the emerging Draek threat and Quellorian alliance on Earth, became masters of scavenging, reverse-engineering, and temporary fusion of foreign technologies.
- The Hybrid Protocol is not "clean" mastery — it is desperate, brilliant, and unstable innovation born from survival.

**Core Theme:**
> "We do not belong to any one song. We steal the notes from every choir and make them sing together — even if the music sometimes breaks us."

---

## 2. Core Concept

The **Hybrid Protocol** allows a Human player (or Human-controlled unit) to temporarily **fuse modules** from other races into a single hybrid loadout.

- **Duration:** Short to medium (30s – 4min depending on quality).
- **Cost:** Instability accumulation + moral drift + potential backlash.
- **Power:** Extremely high when successful — often exceeding pure-race specialists for a brief window.
- **Risk:** System collapse, friendly fire, Crownstone corruption feedback, or permanent module damage.

This makes Humans the ultimate "wildcard" race — capable of adapting to almost any situation, but never as cleanly or safely as a dedicated Quellorian, Cydruid, or Ambrosian.

---

## 3. Module Categories

### 3.1 Quellorian Resonance Modules
- **Resonance Juke** — Evasive blink with harmonic trail.
- **Harmony Overcharge** — Temporary damage + healing aura.
- **Choir Shield** — Shared damage absorption for nearby allies.

### 3.2 Draek Hivemind / Corruption Modules
- **Consumption Link** — Leech health from nearby enemies (risk of self-corruption).
- **Swarm Coordination** — Temporary control of nearby drone wreckage.
- **Pain Conditioning** — Boost to self and nearby minions at cost of sanity.

### 3.3 Cydruid Ecological Modules
- **Grove Healing Pulse** — Area heal + root snare.
- **Symbiotic Overgrowth** — Temporary terrain control and vision denial.
- **Reclamation Field** — Convert enemy biomass into friendly resources.

### 3.4 Ambrosian Attunement Modules
- **Attunement Surge** — Massive single-target harmony damage + cleanse.
- **Discordant Reflection** — Reflect incoming psionic damage (risk of self-Discordance).
- **Crystal Lattice** — Temporary invulnerability bubble (long cooldown).

### 3.5 Human Baseline Modules (Always Available)
- **Scavenger Instinct** — Bonus resources from wrecks.
- **Innovation Surge** — Temporary crafting speed + reverse-engineering insight.
- **Last Stand Protocol** — Increasing power as health drops (Earth Defense synergy).

---

## 4. How the Hybrid Protocol Works

### 4.1 Activation
1. Player opens **Hybrid Matrix** UI (radial or grid).
2. Selects 1–3 foreign modules + keeps 1–2 Human baseline modules.
3. Confirms fusion.
4. System calculates **Hybrid Stability Score**.

### 4.2 Hybrid Stability Formula

```rust
fn calculate_hybrid_stability(
    quellorian_count: u8,
    draek_count: u8,
    cydruid_count: u8,
    ambrosian_count: u8,
    player_mercy_alignment: f32,      // -100.0 to +100.0
    current_crownstone_corruption: f32, // 0.0 to 100.0
) -> f32 {
    let diversity_penalty = (quellorian_count + draek_count + cydruid_count + ambrosian_count) as f32 * 8.0;
    let alignment_bonus = player_mercy_alignment.abs() * 0.15; // Extremes are more stable
    let corruption_penalty = current_crownstone_corruption * 0.8;

    100.0 - diversity_penalty + alignment_bonus - corruption_penalty
}
```

### 4.3 Instability & Backlash
- Every second while hybrid is active, instability ticks up.
- At 100 instability → **Hybrid Collapse** (random negative effect + cooldown).
- High instability can trigger **Crownstone Feedback** (temporary Draek corruption on self/allies).

---

## 5. Specific Hybrid Examples

### Example A: "Resonant Reclaimer" (Quellorian + Cydruid)
- **Modules:** Resonance Juke + Grove Healing Pulse + Human Scavenger Instinct
- **Effect:** Blink-heal hybrid that leaves behind temporary root fields.
- **Instability:** Low–Medium
- **Moral Impact:** Strongly positive (healing + harmony)

### Example B: "Corrupted Innovator" (Draek + Human)
- **Modules:** Consumption Link + Innovation Surge + Last Stand Protocol
- **Effect:** Leech health while rapidly reverse-engineering enemy tech on the fly.
- **Instability:** High
- **Moral Impact:** Negative drift (risk of Crownstone corruption)

### Example C: "Mirror of Harmony" (Ambrosian + Quellorian)
- **Modules:** Attunement Surge + Choir Shield + Human Moral Compass
- **Effect:** Reflect psionic damage while shielding allies — powerful but risks Discordant outbreak on self.

---

## 6. Technical Implementation (Production-Ready)

### 6.1 New Resources & Components

```rust
// In simulation_integration.rs or human_race.rs

#[derive(Resource)]
pub struct HumanHybridProtocolState {
    pub active_hybrids: HashMap<Entity, ActiveHybrid>,
    pub global_instability_modifier: f32, // Server-wide from Mirror Reckoning
}

#[derive(Clone, Debug)]
pub struct ActiveHybrid {
    pub modules: Vec<HybridModule>,
    pub stability: f32,
    pub instability: f32,
    pub time_remaining: f32,
    pub moral_drift_per_second: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HybridModule {
    QuellorianResonanceJuke,
    QuellorianHarmonyOvercharge,
    DraekConsumptionLink,
    CydruidGroveHealingPulse,
    AmbrosianAttunementSurge,
    // ... more
}

#[derive(Component)]
pub struct HybridCapable {
    pub current_hybrid: Option<ActiveHybrid>,
}
```

### 6.2 Core System (Simplified)

```rust
pub fn hybrid_protocol_update_system(
    mut query: Query<(&mut HybridCapable, &MercyAlignment, &CrownstoneCorruptionLevel)>,
    time: Res<Time>,
    mut hybrid_state: ResMut<HumanHybridProtocolState>,
) {
    for (mut hybrid_capable, alignment, corruption) in query.iter_mut() {
        if let Some(active) = &mut hybrid_capable.current_hybrid {
            active.instability += calculate_instability_tick(active, alignment.0, corruption.0);
            active.time_remaining -= time.delta_seconds();

            if active.instability >= 100.0 || active.time_remaining <= 0.0 {
                trigger_hybrid_collapse(&mut hybrid_capable, &mut hybrid_state);
            }
        }
    }
}
```

### 6.3 Integration Hooks
- **Mirror Reckoning:** High hybrid usage during the week increases "Chaotic Innovator" shadow strength on weekend.
- **Crownstone Trilemma:** Using too many Draek modules increases corruption → harder Capture & Repurpose path.
- **Hivelord Counter-Strategies:** Hivelord prioritizes targeting high-instability hybrid Humans.
- **RBE Layer:** Successful clean hybrids generate small server-wide abundance pulses.

---

## 7. Voice, VFX & Audio Direction

- **Activation:** Grinding mechanical + harmonic chime (human innovation sound).
- **While Active:** Layered, unstable audio — Quellorian harmonics glitching with Draek distortion or Cydruid organic creaks.
- **Collapse:** Painful, broken sound + visual static + possible Crownstone purple flash.
- **Voice Lines (Human Pilot):** 
  - "Pushing the limit... come on, hold together!"
  - "I can feel all of them... it’s beautiful... and terrifying."
  - On collapse: "No— not like this!"

---

## 8. Development Priorities

1. Implement `HybridModule` enum + `ActiveHybrid` struct.
2. Build Hybrid Matrix UI (radial selection with instability preview).
3. Create `hybrid_protocol_update_system` + collapse logic.
4. Wire moral drift into `MercyAlignment` resource.
5. Add Mirror Reckoning ecological/moral feedback from hybrid usage.
6. Balance pass on instability formulas.

---

**End of Document**

*This system makes Humans the most replayable and strategically deep race while teaching the cost of unchecked adaptability and the beauty of temporary, risky cooperation.*