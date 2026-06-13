# Discordant Redemption Questlines — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** `DISCORDANT_AMBROSIAN_CORRUPTION.md`, `AMBROSIAN_ATTUNEMENT_MECHANICS.md`, `CROWNSTONE_TRILEMMA_PATHS.md`, `RESONANCE_BURST_MECHANICS.md`, `BOARDING_MECHANICS.md`, `THE_HIVELORD.md`

---

## 1. Overview & Narrative Significance

**Discordant Redemption** represents one of the most profound moral and mechanical pillars of Powrush-MMO. It is the path of **restoring** Ambrosians who have fallen into corruption — cracked crystals, purple-red dissonance energy, and shrieking psionic static — back into harmonious, glowing blue-white allies.

This questline family directly embodies the Quellorian philosophy of **elevation through resonance and mercy** in direct opposition to the Draek Dominion’s consumption and domination. It serves as the hopeful counterweight to the horror of Discordant outbreaks and ties inextricably into the **Crownstone Trilemma** (especially the **Capture & Repurpose** path).

**Core Themes:**
- Redemption over destruction or domination
- Harmony as a living, restorable force (RBE alignment)
- Player agency in defining "salvation" (forced purification vs. voluntary healing)
- Long-term world state consequences that ripple across fleet AI, mothership combat, and RBE moral economies

When a player chooses redemption, they are not simply removing a threat — they are **re-weaving** the fabric of the universe toward abundance and mercy. This choice carries immense narrative weight, mechanical reward, and moral cost.

---

## 2. Entry Points & Trigger Conditions

Players encounter Discordant Ambrosians through several coherent, high-stakes events:

- **Failed Attunement Events** (during boarding or Resonance Burst overuse)
- **Crownstone Sabotage Backfire** (from the Sabotage trilemma path)
- **Hivelord Direct Corruption** (rare, terrifying boss-adjacent events near the Brood Spire)
- **Resonance Burst Feedback** (when burst is used while Crownstone integrity is critically low)
- **Boarding Failures** on Quellorian capital ships carrying Ambrosian choirs

**Global Trigger Variables** (ready for `simulation_integration.rs`):

```rust
pub struct DiscordantRedemptionState {
    pub active_outbreaks: u32,
    pub total_corrupted: u32,
    pub total_redeemed: u32,
    pub redemption_progress_global: f32, // 0.0–1.0
    pub last_redemption_event_frame: u64,
}
```

Redemption questlines can begin when `active_outbreaks > 0` and the player has sufficient **Resonance Alignment** (from Quellorian Resonance AI) or has captured/attuned the Crownstone.

---

## 3. Questline Structures (Three Primary Paths)

### Path A: Surgical Purification (Resonance-First)

**Philosophy:** Clean, precise restoration using Quellorian resonance fields and Ambrosian choirs. Lower risk, moderate reward.

**Key Steps:**
1. **Locate & Isolate** — Use resonance scanners (tied to Quellorian fleet AI) to find Discordant clusters. Create temporary "Harmony Dead Zones" to prevent spread.
2. **Resonance Containment Field** — Deploy specialized Luminar-Class cruisers or TAUN support to project stabilizing harmonic fields.
3. **Choir Intervention** — Ambrosian choirs perform multi-stage attunement reversal. Player must protect the choir during vulnerable phases.
4. **Final Harmonic Seal** — Use controlled Resonance Burst (low-intensity) to lock in purity.

**Rewards:** Redeemed Ambrosians become standard powerful allies with slight harmony bonus. Low moral strain.

**Risk:** If containment fails, corruption can spread rapidly to nearby Quellorian forces.

### Path B: Crownstone-Mediated Redemption (High-Risk, High-Reward)

**Philosophy:** Use the captured/attuned Crownstone itself as a **redemption catalyst**. This is the most direct tie to the Capture & Repurpose trilemma path. Highest narrative and mechanical payoff.

**Key Steps:**
1. **Crownstone Attunement Ritual** — Player must have chosen Capture & Repurpose and successfully purified the Crownstone to at least 60% integrity.
2. **Direct Interface** — The Hivelord’s Suit (or a captured biomechanical interface) is used in reverse — player channels redemptive resonance *through* the Crownstone into the Discordant entity.
3. **Symbiotic Purification** — The Crownstone temporarily "reprograms" the corruption vectors, turning them into harmony amplifiers. Extremely volatile phase.
4. **Integration or Release** — Redeemed Ambrosian either bonds permanently with the player’s fleet (unique hybrid unit) or is released as a wandering harmonic beacon that can trigger positive events across the simulation.

**Rewards:**
- Unique **Hybrid Ambrosian** units with both resonance and limited psionic command abilities.
- Major RBE abundance bonuses (harmony shared across faction).
- Deep narrative unlocks: Ambrosian gratitude events, new diplomacy options, potential future "Ambrosian Exodus" story arcs.

**Risks:** High chance of Crownstone backlash if integrity < 70%. Can temporarily empower nearby Draek forces if ritual fails. Moral weight is heavy — player is literally using the tool of domination for salvation.

### Path C: Ambrosian Self-Redemption (Voluntary Healing)

**Philosophy:** The most mercy-aligned and RBE-pure path. The player does not force redemption — they **empower** the Discordant Ambrosian to heal itself through guided resonance and choice.

**Key Steps:**
1. **Establish Trust** — Non-violent boarding or remote resonance communication. Player must lower their own defenses and demonstrate mercy (Resonance Alignment check).
2. **Internal Harmony Seed** — Player plants a small, pure resonance fragment (sourced from TAUN or personal attunement) inside the Discordant crystal.
3. **Player-Supported Struggle** — The Ambrosian fights its own corruption internally. Player must defend it from both Draek forces *and* its own fracturing instincts.
4. **Voluntary Re-Harmonization** — If successful, the Ambrosian emerges changed — stronger, with unique "Scarred Harmony" abilities (temporary debuffs that become powerful buffs over time).

**Rewards:** Deepest narrative satisfaction. Redeemed Ambrosians often become **narrative companions** or **faction-wide harmony anchors** that reduce future corruption risk galaxy-wide. Strongest long-term RBE moral bonuses.

**Risk:** Highest failure rate. If the Ambrosian loses its internal battle, it can become a permanent, empowered **Greater Discordant** horror.

---

## 4. Mechanical Implementation (Ready for Code)

### Global Simulation Resource

```rust
#[derive(Resource)]
pub struct DiscordantRedemptionState {
    pub outbreaks: HashMap<Entity, DiscordantOutbreak>,
    pub redeemed_count: u32,
    pub purity_average: f32,
    pub crownstone_redemption_bonus: f32, // Multiplier when using Path B
}

#[derive(Clone)]
pub struct DiscordantOutbreak {
    pub corruption_level: f32,      // 0.0 = pure, 1.0 = fully Discordant
    pub redemption_progress: f32,   // 0.0–1.0
    pub redemption_path: RedemptionPath,
    pub linked_crownstone_integrity: Option<f32>,
}

#[derive(Clone, Copy)]
pub enum RedemptionPath {
    Surgical,
    CrownstoneMediated,
    SelfRedemption,
}
```

### Core Formulas

**Redemption Success Probability (example):**

```rust
fn calculate_redemption_success(
    player_resonance_strength: f32,
    crownstone_integrity: f32,
    outbreak_corruption: f32,
    path: RedemptionPath,
) -> f32 {
    let base = match path {
        RedemptionPath::Surgical => 0.65,
        RedemptionPath::CrownstoneMediated => 0.45 + (crownstone_integrity * 0.4),
        RedemptionPath::SelfRedemption => 0.35 + (player_resonance_strength * 0.5),
    };
    let harmony_bonus = player_resonance_strength * 0.25;
    let risk = outbreak_corruption * 0.6;
    (base + harmony_bonus - risk).clamp(0.1, 0.95)
}
```

**Harmony Restoration on Success:**
```rust
fn apply_harmony_restoration(
    state: &mut QuellorianResonanceState,
    redeemed_ambrosians: u32,
) {
    state.harmony_level += redeemed_ambrosians as f32 * 0.08;
    state.network_integrity = (state.network_integrity + 0.03).min(1.0);
}
```

---

## 5. Deep Integration with Existing Systems

- **Ambrosian Attunement & Corruption:** Direct extension. Redemption reverses `corruption_level` and can grant permanent "Scarred Harmony" traits.
- **Crownstone Trilemma:** Path B is only available after choosing **Capture & Repurpose**. Success here can increase `crownstone_integrity` and unlock new trilemma options later.
- **Resonance Burst:** Used as a precision tool in Path A and Path B. Overuse during redemption can cause new outbreaks (feedback risk).
- **Boarding Mechanics:** Non-violent or precision boarding required for Path C. Violent boarding makes Self-Redemption impossible.
- **Hivelord Biomechanical Suit:** The suit’s Crownstone interface can be hijacked for Path B rituals — creates tense, cinematic boarding sequences on the Brood Spire.
- **Quellorian & Draek Fleet AI:** Redeemed Ambrosians can switch from `Discordant` AI state to `HarmonicAlly` state, dramatically shifting local battle balance. Draek AI may prioritize hunting redeemed units.
- **RBE Moral Layer:** Each successful redemption increases player "Mercy Score" and faction-wide abundance generation. Failed redemptions or forced purifications carry moral debt that can manifest as future Discordant outbreaks or Ambrosian distrust events.

---

## 6. Technical Implementation Notes (simulation_integration.rs + rbe_engine.rs)

**Recommended ECS Components:**
- `DiscordantAmbrosian` (with `corruption_level`, `redemption_progress`, `redemption_path`)
- `RedeemedAmbrosian` (with `scar_strength`, `harmony_anchor_bonus`, `narrative_companion` flag)
- `RedemptionEvent` (triggered on quest milestones)

**Event-Driven Flow Example:**
1. `DiscordantOutbreakSpawned` event → spawns entities + updates global state.
2. Player initiates redemption quest → `RedemptionPathChosen` event.
3. During quest: periodic `RedemptionProgressUpdate` events with formula checks.
4. On success/failure: `AmbrosianRedeemed` or `GreaterDiscordantBorn` events that mutate AI allegiance and trigger VFX.

**VFX / Audio Recommendations:**
- Corruption: Cracked crystal + pulsing purple-red veins + dissonant shrieking audio.
- Redemption: Cracks seal with flowing blue-white light, veins turn golden-harmonic, audio shifts from shriek to resonant choir.
- Path B specific: Crownstone glows intensely during ritual; risk of purple backlash lightning.

**Balance & Tuning Notes:**
- Redemption should feel **expensive** (time, resources, risk) to prevent it from being the obvious "good" choice in every situation.
- Long-term world state: High redemption rates can reduce future Draek aggression but may cause Quellorian harmony network strain (new systemic risk).
- Moral branching: Players who heavily favor redemption may unlock unique Ambrosian faction diplomacy options later in the campaign.

---

## 7. Narrative & Moral Depth (RBE Alignment)

Redemption questlines are designed to make players **feel** the weight of mercy. 

- Choosing Path C (Self-Redemption) when it is risky demonstrates true grace.
- Using the Crownstone (Path B) for good after it was a tool of domination is thematically powerful and ties directly into the "Eternal Mercy Flow" philosophy.
- Failed redemptions that create Greater Discordants serve as meaningful consequences, not cheap gotchas.
- Long-term: Redeemed Ambrosians can become living proof that even the most broken things can be restored — a core RBE message of abundance through restoration rather than consumption.

---

## 8. Development Priorities

1. Implement `DiscordantRedemptionState` resource and core formulas in `simulation_integration.rs`.
2. Create event system for `RedemptionPathChosen`, `AmbrosianRedeemed`, and `GreaterDiscordantBorn`.
3. Prototype Path A (Surgical) first — lowest narrative complexity, highest mechanical clarity.
4. Build Path B integration with existing Crownstone trilemma state.
5. Design Path C as the emotional and moral pinnacle (requires non-violent boarding tech).
6. Add VFX/audio hooks for crystal state transitions.
7. Balance tuning pass focused on risk/reward and long-term simulation impact.
8. Narrative writing for key Ambrosian "voices" during redemption (especially in Path C).

---

**End of Document**

*This document completes the harmonic/corruption duality and positions Discordant Redemption as a living, choice-driven pillar of the Powrush universe — ready for coherent, phenomenal gameplay integration.*