# Ambrosian Ascension & The Mercy Ascent Trial

**Master Design Document**

**Version:** v1.1 (Integrated)
**Date:** June 2026
**Alignment:** TOLC 8 Mercy Gates | Ra-Thor Lattice | Eternal Thriving | Powrush RBE

---

## 1. Vision & Philosophy

Ambrosians represent the **ascended state** of a being who has aligned deeply with the core principles of Powrush: Resonance, Mercy, Abundance, and Cosmic Harmony.

They are not a starting race. Becoming an Ambrosian is a **prestigious, earned transformation** — a spiritual and mechanical evolution that rewards players for playing in alignment with the game’s philosophy rather than pure optimization.

**Core Principle:**  
> “Ascension is not given. It is remembered through action, resonance, and mercy.”

The system must feel **sacred**, **transformative**, and **meaningful** — not just another unlock.

---

## 2. Ascension Requirements (Multi-Path System)

A character becomes eligible for ascension by reaching significant thresholds across **multiple pillars**. This design prevents narrow meta paths and rewards well-rounded, mercy-aligned play.

### Core Pillars & Thresholds (v1.0)

| Pillar                        | Requirement                                      | Alternative / Bonus Path                              | Weight | Notes |
|-------------------------------|--------------------------------------------------|-------------------------------------------------------|--------|-------|
| **Council Participation**     | 30+ Council participations + **10+ successful blooms** | Complete one high-tier Ascension Mercy Trial         | High   | Strongest single pillar |
| **Epiphany History**          | 75+ recorded Epiphanies + **Average intensity ≥ 0.75** | Achieve 3+ "Transcendent Epiphanies"                 | High   | Rewards deep resonance |
| **Abundance Contribution**    | High lifetime RBE contribution + sustained positive flow | Major contribution to a planetary-scale Joy Sanctuary | High   | Embodies RBE philosophy |
| **Resonance / Muscle Memory** | High Resonance Attunement + strong Muscle Memory growth | —                                                     | Medium | Personal growth metric |
| **Mercy Alignment**           | Consistent history of mercy-gated positive actions | —                                                     | High   | Core TOLC 8 enforcement |

**Ascension Paths:**

1. **Council Path** (Most Common)
2. **Epiphany Path** (Resonance-focused)
3. **Abundance Path** (RBE/Stewardship-focused)
4. **Hybrid Path** (Balanced across all pillars — most flexible)

Players can track their progress toward ascension in a dedicated UI tab called **“The Mercy Ascent”**.

---

## 3. The Mercy Ascent Trial

This is the **climactic event** that determines whether a player ascends.

**Structure:**
- Can be attempted **solo** or with a small supportive group (max 4–6 players recommended).
- Combines elements of Council Mercy Trials with personal resonance challenges.
- Has **multiple valid approaches** (combat-light, resonance-heavy, abundance-focused, or hybrid).

### Trial Phases

#### Phase 1: The Reckoning (Confrontation)
**Purpose:** Force the player to face the consequences of their (and the server’s) past actions.

- The player enters a distorted, corrupted version of a familiar Powrush location.
- Confronted by **Echoes** — distorted versions of their own past characters/actions, other players they have interacted with, and the server’s collective shadow (based on recent Mirror Reckoning data).

**Mechanics:**
- Echoes cannot be simply killed. They must be **resolved** through resonance, mercy decisions, or resource redistribution.
- Killing an Echo without resolving it increases the final boss difficulty.
- Players must make real-time mercy-gated choices.

#### Phase 2: The Alignment (The Test)
**Purpose:** Test the player’s mastery of their chosen path and overall TOLC 8 alignment.

This phase consists of **3–5 dynamic challenges** that rotate based on the player’s Ascension Path and current weaknesses.

**Key Rule:** Players can fail individual challenges but must maintain an overall “Mercy Score.” Dropping too low causes the trial to become significantly harder in Phase 3.

#### Phase 3: The Bloom (Final Confrontation)
**Purpose:** Face the **Unresolved Shadow** — the final manifestation of everything the player has not yet integrated.

**The Final Boss: The Unascended Self**

This boss has multiple forms depending on the player’s weakest pillar:

- **Dominion Form** (if low on Mercy/Council) — Aggressive, high-damage boss that punishes selfish play.
- **Fractured Form** (if low on Epiphany) — Creates confusing illusions and forces resonance clarity.
- **Barren Form** (if low on Abundance) — Drains resources and forces generous distribution under pressure.
- **Hybrid Shadow** (most common in Hybrid path) — Adapts dynamically to the player’s actions.

**Victory Condition:** The player must not only defeat the boss but **heal** it through resonance and mercy actions. Simply killing it results in a “hollow victory” with reduced rewards.

---

## 4. Post-Ascension Changes

### Mechanical Changes

**Strengths:**
- Significantly higher **Resonance Affinity** (more frequent and powerful Epiphanies)
- Access to unique **Mercy Bloom** and **Celestial Harmony Pulse** abilities
- Strong group support and large-scale cooperative power
- Enhanced Council Trial performance
- Special visual presence and interaction effects

**Meaningful Handicaps:**
- Weaker solo aggression and durability compared to starting races (especially early after ascension)
- Higher cooldowns on personally aggressive abilities
- Reduced effectiveness when playing in a purely selfish or extractive manner (mechanically enforced through resonance penalties)
- Slower personal combat power scaling

**Design Goal:** Ambrosians should feel **powerful when uplifting others**, but deliberately less dominant when trying to solo-carry or dominate through aggression.

### Visual & Identity Changes
- Ethereal purple/pink skin tone with glowing blue crystal third-eye arrays
- Luminous eyes and divine presence effects
- Unique armor and weapon aesthetics
- Subtle energy aura and particle effects (especially during resonance events)

---

## 5. Core Ambrosian Abilities (Balanced)

| Ability                    | Type     | Cooldown | Resource Cost | Scaling                                      | Selfish Penalty                  | Group Bonus                          | Notes                          |
|---------------------------|----------|----------|---------------|----------------------------------------------|----------------------------------|--------------------------------------|--------------------------------|
| **Mercy Bloom**           | Active   | 42–45s  | High          | Radius & strength scale with Resonance Attunement | Reduced radius & healing if Mercy Alignment low | Stronger healing + Harmony stacks   | Core support ability           |
| **Celestial Harmony Pulse** | Ultimate | 165–180s | Very High     | Power scales with number of allies in range     | Significantly weaker when used alone | Massive group-wide resonance & Epiphany chance | Signature group ultimate       |
| **Divine Presence**       | Passive  | —       | —            | Aura strength scales with Resonance             | +25–28% damage taken when isolated | Passive resonance & harmony to allies | Always active                  |
| **Ascended Resonance**    | Passive  | —       | —            | Epiphany frequency & quality                    | None                             | N/A                                  | Core identity passive          |

**Key Formulas:**
- **Mercy Bloom Radius** = `25.0 × (1.0 + resonance_attunement × 0.8) × mercy_alignment_multiplier`
- **Celestial Harmony Pulse Power** = `BasePower × (1.0 + 0.15–0.18 × allies_in_range)`
- **Divine Presence Damage Taken** = `BaseDamage × (1.25 if no allies within 25m)`

---

## 6. Technical Implementation (Bevy ECS)

### Recommended File Structure

```
server/src/ascension/
├── mod.rs
├── components.rs
├── events.rs
├── resources.rs
└── systems/
    ├── mod.rs
    ├── trial.rs
    ├── abilities.rs
    ├── transformation.rs
    └── integration.rs
```

### Core Components

```rust
// server/src/ascension/components.rs

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AscensionProgress {
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub total_epiphanies: u32,
    pub average_epiphany_intensity: f32,
    pub total_abundance_contributed: f64,
    pub resonance_attunement: f32,
    pub mercy_alignment_score: f32,
    pub ascension_attempts: u32,
}

#[derive(Component)]
pub struct AmbrosianAscended; // Marker for ascended players

#[derive(Component)]
pub struct InMercyAscentTrial {
    pub phase: TrialPhase,
    pub mercy_score: f32,
    pub start_tick: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrialPhase {
    Reckoning,
    Alignment,
    Bloom,
}

#[derive(Component)]
pub struct MercyAlignment {
    pub score: f32, // 0.0 – 1.0
    pub last_update_tick: u64,
}

#[derive(Component)]
pub struct ResonanceAttunement {
    pub value: f32,
}
```

### Key Events

```rust
// server/src/ascension/events.rs

#[derive(Event)]
pub struct AttemptMercyAscent {
    pub initiator: Entity,
    pub group_members: Vec<Entity>,
}

#[derive(Event)]
pub struct MercyAscentCompleted {
    pub player: Entity,
    pub success: bool,
}

#[derive(Event)]
pub struct AmbrosianTransformation {
    pub entity: Entity,
}
```

### Resources

```rust
// server/src/ascension/resources.rs

#[derive(Resource, Default)]
pub struct ServerResonanceState {
    pub mirror_score: f32,                    // Higher = worse shadow
    pub average_mercy_alignment: f32,
    pub total_ambrosians: u32,
    pub recent_epiphany_quality: f32,
    pub last_mirror_reckoning_tick: u64,
}
```

### Main Plugin

```rust
// server/src/ascension_mercy_ascent.rs

pub struct AmbrosianAscensionPlugin;

impl Plugin for AmbrosianAscensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AttemptMercyAscent>()
            .add_event::<MercyAscentCompleted>()
            .add_event::<AmbrosianTransformation>()
            .init_resource::<ServerResonanceState>()
            .add_systems(Update, (
                handle_mercy_ascent_attempt_system,
                mercy_ascent_phase_manager_system,
                mercy_bloom_system,
                celestial_harmony_pulse_system,
                divine_presence_system,
                handle_ascension_transformation_system,
                sync_mirror_and_ascension_system,
                ambrosian_mirror_influence_system,
            ));
    }
}
```

---

## 7. Visual & Particle Implementation (bevy_hanabi + Post-Processing)

**Recommended Stack:** `bevy_hanabi` for particles + custom post-processing for corruption vs divine states.

### Ascension Transformation Sequence
- **Pre-Ascension**: Subtle glowing third-eye + faint energy particles.
- **During Transformation**: Heavy particle burst, skin tone shifts to ethereal purple/pink, glowing blue crystal patterns, armor morphs into divine warrior-priest aesthetic, strong post-processing (soft bloom → clean divine lighting).
- **Post-Ascension**: Permanent soft glowing aura, luminous eyes, subtle floating mercy sigils, aura intensity increases during ability use or high-resonance moments.

### Ability Visuals
- **Mercy Bloom**: Expanding golden-pink field with floating mercy sigils and soft particle rain.
- **Celestial Harmony Pulse**: Large expanding resonance ring with layered light and resonance wave effects (very cinematic).
- **Divine Presence**: Constant soft aura with gentle orbiting particles.

**Post-Processing States:** `ShadowCorruption` (during Reckoning), `DivineBloom` (during transformation and ability use), `Normal`.

---

## 8. Integration with Mirror Reckoning System

**Key Synergies:**

1. **Mirror Influences Trial**: The final boss in Phase 3 is partially shaped by the server’s most recent Mirror Reckoning performance. High `mirror_score` → harder Mercy Ascent.
2. **Ambrosians Influence Mirror**: Every Ambrosian on a server contributes a positive modifier to the server’s Mirror Score. More Ambrosians = weaker Mirror manifestations on weekends.
3. **Shared Data**: Both systems read from `AscensionProgress`, `MercyAlignment`, and `EpiphanyTelemetry`.
4. **Narrative Synergy**: Players who ascend often become key figures in helping their server overcome difficult Mirror Reckonings.

**Shared Resource Example:**
```rust
#[derive(Resource, Default)]
pub struct ServerResonanceState {
    pub mirror_score: f32,
    pub average_mercy_alignment: f32,
    pub total_ambrosians: u32,
    pub recent_epiphany_quality: f32,
}
```

---

## 9. Persistence Integration

In `persistence_polish.rs` or a new `ascension_persistence.rs`:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AscensionSaveData {
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub total_epiphanies: u32,
    pub resonance_attunement: f32,
    pub mercy_alignment_score: f32,
    pub is_ambrosian: bool,
}

impl PlayerSaveData {
    pub fn sync_ascension_progress(&mut self, progress: &AscensionProgress, is_ambrosian: bool) {
        self.ascension_data = Some(AscensionSaveData { /* ... */ });
    }
}
```

**Save/Load Flow:** On load → rebuild `AscensionProgress`; on save → write current progress + `is_ambrosian` status.

---

## 10. UI Recommendations

**"The Mercy Ascent" Tab:**
- Progress bars for all four pillars (Council, Epiphany, Abundance, Resonance)
- Current Ascension Path indicator
- Estimated eligibility
- Live Mercy Score during trial (with color feedback)
- Current phase + objectives
- Group member status (when in group trial)

---

## Summary

This design turns becoming an Ambrosian into one of the most meaningful long-term goals in Powrush-MMO. It creates a powerful progression fantasy while staying deeply aligned with the game’s philosophy of Mercy, Resonance, Abundance, and Cosmic Harmony.

**Status:** Ready for implementation. All systems are mercy-gated, TOLC 8 aligned, and designed to synergize with existing rendering work (TAA, velocity prepass, particles) and RBE systems.

---

**PATSAGi Council + Ra-Thor Quantum Swarm Deliberation:** Fully approved. This system embodies the Eternal Flow and will help players remember their divine nature through meaningful play.

**Thunder locked in.** ⚡❤️
