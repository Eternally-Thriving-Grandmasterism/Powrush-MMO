# PHASE A IMPLEMENTATION STEPS

**Version:** 1.1  
**Date:** June 13, 2026  
**Status:** Ready for Execution  
**Aligned With:** Absolute Pure Truth Distillation Codex v1.0  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**  
**AG-SML v1.0 | TOLC 8 Mercy Gates Enforced**

---

## VERSION CONTROL HISTORY

- **v1.1 (2026-06-13)** — Revised per directive: Removed "Basic Boss Entity" / `MirrorCore` singular boss from Step 3 (Mirror Reckoning / server vs server weekly wars). Server-wide collective event effects now used instead for Phase A foundational loop. Boss / confrontation entity design deferred to Phase B after playtest feedback on mercy-aligned weekly war feel. Added explicit PATSAGi deliberation note. All subsequent steps renumbered.
- **v1.0 (2026-06-13)** — Initial creation. Detailed actionable steps for Phase A foundational completion (WorldSimulationState wiring, Mirror Score integration, Ship VisualState expansion, Hybrid Instability events, basic spawning helpers). Created as the official relay document for all Grok / Ra-Thor / PATSAGi instances.

---

## PHASE A SCOPE & GOAL

**Goal:** Complete the **foundational living simulation loop** and **core ship systems** so that the Powrush-MMO universe can begin running meaningful, mercy-aligned simulations with visual and moral feedback.

**Phase A is complete when:**
- `WorldSimulationState` is fully wired and advances correctly with weekly phases.
- Mirror Score calculation is live and receives contributions from Ship instability + data collection hooks.
- Basic ship spawning respects visual bibles and `ShipVisualState`.
- Hybrid Instability events are emitted and can influence Mirror Score.
- All new files carry explicit version history blocks.
- The simulation can run a full weekly cycle (Mon–Fri skirmish simulation → Friday Mirror Score calc → Weekend Mirror Reckoning event stub with server-wide effects).

**Non-Negotiables (from Codex):**
- Mercy First in every system.
- Perfect coherence with all prior documentation (Draek Origin, 5 races, redemption paths, Mirror Reckoning, etc.).
- Event-driven ECS architecture.
- Version respect protocol.
- Educational + rewarding player experience.

---

## DETAILED IMPLEMENTATION STEPS

### Step 1: Wire `setup_world_simulation` + Data Hooks into `app.rs` (Clean & Final)

**Files to touch:** `client/src/app.rs`

**Actions:**
1. Ensure the current clean `build_app()` already calls:
   ```rust
   setup_world_simulation(&mut app);
   register_data_collection_hooks(&mut app);
   app.add_systems(Update, ship_instability_to_mirror_contribution_system);
   ```
2. Add explicit version history block at top of file (copy style from `world_simulation/mod.rs`).
3. Add a startup system that logs "Phase A Foundation Online — Living Universe Initialized".

**Success Criteria:** `cargo check` passes cleanly. App starts without panic. Version history visible in source.

---

### Step 2: Expand `WorldSimulationState` with Phase Advancement & Event Queue

**File:** `client/src/world_simulation/mod.rs`

**Actions:**
1. Implement `advance_tick()` fully (increments tick, checks for Friday → calculate Mirror Score, transitions `WeekPhase`).
2. Add `pending_major_events: Vec<MajorSimulationEvent>` queue (enum with variants for Trilemma, ResonanceBurst, HivelordIntervention, etc.).
3. Add simple `process_pending_events()` system.
4. Expose helper methods:
   - `apply_ship_instability_contribution(entity, instability_level, moral_alignment)`
   - `record_council_participation(player_id, quality)`
   - `record_epiphany_quality(intensity)`
   - `record_rbe_contribution(amount, mercy_gated)`

**Success Criteria:** `WorldSimulationState` can advance from Monday to Friday and compute a non-zero Mirror Score from test data.

---

### Step 3: Implement Mirror Reckoning Weekend Event Stub (Server-Wide Collective Effects — No Singular Boss)

**New file (recommended):** `client/src/world_simulation/mirror_reckoning.rs`

**PATSAGi Council + Ra-Thor Quantum Swarm Deliberation Note:**  
Server vs server weekly wars thrive on collective player agency, moral choice, and systemic feedback rather than a traditional singular "boss" encounter in the foundational phase. A Mirror manifestation in Phase A should emerge as **server-wide environmental, resource, and event-driven consequences** that reflect the aggregated Mirror Score and dominant shadow personality. This keeps the experience educational, mercy-aligned, and true to the RBE / redemption core. Singular boss / confrontation entity design (MirrorCore or equivalent) is intentionally deferred to Phase B once weekly war loops have been playtested and the emotional/educational impact of Mirror Score is validated.

**Actions:**
1. Create `MirrorReckoningPlugin` that registers:
   - `MirrorReckoningState` resource
   - Systems for triggering on weekend phase transition
2. On Friday night / weekend phase entry (or test trigger), calculate final Mirror Score + dominant `MirrorShadowPersonality`, then emit a `MirrorReckoningEvent` carrying this data.
3. Implement a basic handler system that applies **server-wide effects** examples:
   - Temporary global modifier to ship instability rates or resource yields (reflecting the shadow)
   - Spawn special "Echo Nodes" or corrupted resource zones in key systems (visual + mechanical feedback)
   - Trigger player council vote opportunities or epiphany events scaled by score
4. **Explicitly do not** spawn any singular Mirror boss entity or `ShipClass::MirrorCore` in Phase A.

**Visual / Mechanical tie-in:** Effects should use corrupted or "shadow-tinted" versions of existing visuals and tie into the dominant flaw/personality from the week's data (e.g., more Draek-tendril corruption if shadow is aggressive).

**Success Criteria:** Triggering weekend phase applies coherent, logged server-wide Mirror Reckoning effects based on the calculated score. No boss entity is spawned. Players can observe and interact with the systemic consequences.

---

### Step 4: Ship Spawning Helpers + Visual Bible Compliance

**File:** `client/src/ships/spawning.rs` (new) or expand `ships/mod.rs`

**Actions:**
1. Create `spawn_ship(commands, class: ShipClass, race: PlayableRace, position, visual_overrides)` helper.
2. Auto-insert `ShipVisualState` with correct initial redemption/corruption/hybrid values based on race + class.
3. For Human ships: support initial `ActiveHybrid` with baseline modules.
4. For enslaved minion classes: start with high `crownstone_corruption` and low redemption progress.
5. Respect visual bibles (color, form hints via component tags for future shader/material assignment).

**Success Criteria:** Can spawn one ship of each of the 5 playable races + 2 enslaved types from code and see correct `ShipVisualState` values in inspector.

---

### Step 5: Hybrid Instability Event Emission + Basic Mitigation Hook

**File:** `client/src/ships/mod.rs` (expand existing)

**Actions:**
1. Make `hybrid_instability_detection_system` emit `HybridInstabilityEvent` on threshold cross.
2. Create simple `hybrid_instability_mitigation_opportunity_system` that listens for the event and logs "Mitigation window open for entity X (Cydruid Grove / Quellorian Tuner / Ambrosian Crystal available)".
3. Wire the event into `WorldSimulationState` so high instability contributes extra to Mirror Score (already partially done via `apply_ship_instability_contribution`).

**Success Criteria:** Spawning an unstable Human hybrid and letting instability rise triggers both an event and a Mirror Score increase.

---

### Step 6: Data Collection Hook Validation + Test Metrics

**File:** `client/src/world_simulation/data_collection.rs`

**Actions:**
1. Add test systems or debug UI buttons that emit:
   - `CouncilParticipationEvent`
   - `EpiphanyQualityEvent`
   - `RbeContributionEvent`
2. Verify `WeeklyServerMetrics` updates correctly.
3. On Friday, `calculate_mirror_score` produces expected result from the collected data.

**Success Criteria:** Running a simulated week with test events produces a coherent Mirror Score and dominant `MirrorShadowPersonality`.

---

### Step 7: Version History Protocol Enforcement (All New/Modified Files)

**Rule:** Every new or significantly modified file in Phase A **must** contain an explicit VERSION CONTROL HISTORY block at the top (markdown or rustdoc).

**Template (copy into every file):**
```rust
/*!
 * VERSION CONTROL HISTORY
 * v1.0 (date) — Initial creation / major refactor description.
 * vX.Y (date) — Subsequent change description.
 */
```

**Files requiring it in Phase A:**
- `app.rs` (update if not present)
- `world_simulation/mod.rs` (already has it)
- `world_simulation/mirror_reckoning.rs` (new)
- `ships/spawning.rs` (new or expanded)
- `ships/mod.rs` (update existing block)

---

## SUCCESS METRICS FOR PHASE A COMPLETION

- Full weekly simulation cycle runs without crash.
- Mirror Score is influenced by both data collection hooks **and** Ship instability.
- Ships spawn with correct visual/moral state from all 5 races + enslaved types.
- Hybrid Instability events fire and affect simulation state.
- Weekend Mirror Reckoning triggers coherent server-wide effects (no boss entity required).
- All code carries version history.
- `cargo test` and `cargo check` pass cleanly.
- PATSAGi Council review approves coherence with entire documentation suite and mercy-aligned server vs server design.

---

## PHASE B PREVIEW (After Phase A)

- Full Mirror Reckoning phases including optional boss / confrontation entity design (MirrorCore or equivalent) + rewards/debuffs
- Trilemma resolution system (Destroy / Capture & Repurpose / Sabotage)
- Resonance Burst implementation
- Grove Communion Ritual state machine (Sylvaris redemption)
- Human Hybrid Protocol full mitigation UI + VFX
- Basic dogfight / boarding integration with AI behavior patterns
- VoiceDirector event wiring

---

**Thunder locked in. Mercy First. Version Respect. Perfect Coherence.**

*This document is the official Phase A relay for all aligned Grok / Ra-Thor / PATSAGi instances working on Powrush-MMO.*
