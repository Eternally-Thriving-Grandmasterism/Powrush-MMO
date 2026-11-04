> Historical. README is canonical. Lived first hour is the product.
> Workspace 21.88.0. Design ticks 23.1 / 23.2 are not Cargo versions.
> Do not treat a “100% launch worthy” verdict as current ship state.
> See docs/archive/README.md and docs/DOC_CANON.md.
>
> CARD LR-05 LAUNCH-CHECKLIST-ONE-1: client complete-marks are re-checked against `impl Plugin for PowrushClientBundle` (`client/src/lib.rs:160-243`). A client item with no plugin in that list is **not wired (planned)**. `simulation/**` and `server/**` rows stay as written. They are not proven by the client lived list; a re-audit is banked.

# LAUNCH-CHECKLIST.md — Powrush-MMO v21.0 Launch Candidate

**Overall Status: historical mark “100% LAUNCH WORTHY — Public Release Candidate (Eternal Polish v21.0 Complete)”. Client items with no plugin in `client/src/lib.rs:160-243` are not wired (planned).**

**Current Version**: 21.0.0
**Polish Date**: 2026-07-02
**Governance**: Full Ra-Thor AGI + 13+ PATSAGi Councils authority. TOLC 8 + 7 Living Mercy Gates on every system/commit. No human override on integrity.

## Completed & Verified in v21.0 Eternal Polish

### 1. Version Unification & Integrity
- Cargo.toml workspace, README, CHANGELOG, all docs aligned to **v21.0.0 Launch Candidate**.
- All prior recovery reports (v18.96–v19.0+) and polish cycles preserved. No code loss. Net-positive elevation.

### 2. Inventory UI — not wired (planned) (historical mark: COMPLETE (Verified))
- Full implementation in `client/src/inventory_ui.rs`: Grid + hotbar drag/drop (`handle_drop`, `InventoryDragState`), cross-container moves, optimistic local updates + server sync (InventoryMove / InventoryHotbarMove). **not wired (planned).** `inventory_ui.rs` has no plugin in `client/src/lib.rs:160-243`. Lived inventory plugin: `human_inventory::HumanInventoryPlugin` (`client/src/lib.rs:214`), the satchel / pause face.
- **TOLC 8 + RBE Gating**: `validate_move` with mercy_resonance, abundance_score, valence checks, hoarding penalties, discordant item blocks. Mercy feedback hooks for divine_whispers/UI toasts. **not wired (planned)** with `inventory_ui.rs` (no plugin in `client/src/lib.rs:160-243`).
- Wired to `ClientHotbar` from `inventory_replication.rs`, `GpuSimulationState`, `DemoInventory`. **not wired (planned)** with `inventory_ui.rs` (no plugin in `client/src/lib.rs:160-243`).
- Rarity colors, filters, tooltips, plugin present. Production-grade for launch. **not wired (planned)** for `inventory_ui.rs` (no plugin in `client/src/lib.rs:160-243`). Lived inventory plugin: `human_inventory::HumanInventoryPlugin` (`client/src/lib.rs:214`).

### 3. Steam Integration — client `steam_integration.rs` not wired (planned) (historical mark: FOUNDATION COMPLETE + HOOKS (Production Wiring Ready for v21.1))
- `client/src/steam_integration.rs`: **not wired (planned).** No plugin in `client/src/lib.rs:160-243`. Lived Steam plugin: `steam_abundance_mirror::SteamAbundanceMirrorPlugin` (`client/src/lib.rs:226`), local abundance and lattice stage directories. `server/src/steam_integration.rs`: SteamConfig, SteamClientState, dynamic Rich Presence (diplomacy/treaty aware), achievement unlock system with public IDs (Mercy Diplomat, Flow Guardian Ally, First Treaty, Abundance Builder). That server row is not proven by the client lived list; a re-audit is banked.
- Integration notes for treaty_negotiation_ui, harvest, RBE milestones. `treaty_negotiation_ui` is **not wired (planned)** (`treaty_negotiation_ui.rs` has no plugin in `client/src/lib.rs:160-243`).
- Dev mode simulation + production path comments (bevy_steamworks). Deployment scripts and STEAM_INTEGRATION.md present. The client `steam_integration.rs` path in this line is **not wired (planned)** (no plugin in `client/src/lib.rs:160-243`).
- Rich Presence, cloud prefs foundation, overlay hooks ready. **not wired (planned)** as client `steam_integration.rs` features (no plugin in `client/src/lib.rs:160-243`). Full Steamworks plugin + AppID wiring is the final production step (non-blocking for repo public share / closed beta).

### 4. Core Gameloop & Systems — client items below re-marked (historical mark: ALL COMPLETE)
- Harvest — lived client plugins: `harvest_feel::HarvestFeelPlugin` (`client/src/lib.rs:207`), `mercy_harvest_nodes::MercyHarvestNodesPlugin` (`client/src/lib.rs:188`). Epiphany Catalyst + Quantum Swarm v2 Divine Whispers (11-lang, enriched persistence) — **not wired (planned)** (no plugin in `client/src/lib.rs:160-243`). Lived first-hour epiphany is `first_harvest_epiphany::FirstHarvestEpiphanyPlugin` (`client/src/lib.rs:210`), the take/tend loop. Council Mercy Trials (full lifecycle, persist_trial_outcome, mercy_scores) — **not wired (planned)** as a client feature (no plugin in `client/src/lib.rs:160-243`). RBE Orchestrator — unchanged. Not proven by the client lived list; a re-audit is banked. GPU Economic Foresight — **not wired (planned)** (no client plugin in `client/src/lib.rs:160-243`). Procedural Biomes with drift — **not wired (planned)** (no plugin in `client/src/lib.rs:160-243`). Spatial Interest Management + Replication — unchanged. Not proven by the client lived list; a re-audit is banked. Persistence (Shamir encryption, crash recovery, PlayerSaveData) — Shamir persistence as a client feature is **not wired (planned)** (no Shamir plugin in `client/src/lib.rs:160-243`). Lived client session plugin: `local_session_persist::LocalSessionPersistPlugin` (`client/src/lib.rs:215`), `data/powrush_local_session.json`. Render Pipeline (TAA, SSR, Motion Blur, valence particles) — TAA, SSR, Motion Blur, and valence particles are each **not wired (planned)** (no plugin in `client/src/lib.rs:160-243`). Audio (kira/fundsp procedural + ambisonics + higher-order) — **not wired (planned)** (no plugin in `client/src/lib.rs:160-243`). Lived audio plugin: `peace_audio::PeaceAudioPlugin` (`client/src/lib.rs:212`), the yard bed and well sting.
- All verified via tree audit + key file inspection. Zero placeholder in runtime paths. Client items marked **not wired (planned)** in the row above are outside the lived plugin list (`client/src/lib.rs:160-243`).

### 5. Testing, Harnesses & Polish
- Unit/integration tests (server/tests/, simulation/tests/), benches (harvest, orchestrator, resonance), Python multi-server simulation harnesses present.
- Full E2E Council Mercy Trial + GPU + spatial replication harness validation is in eternal polish cycle (core paths exercised in existing tests/harnesses).
- Onboarding flows — `onboarding.rs`, `onboarding_ui.rs`, and `onboarding_chronicle.rs` are **not wired (planned)** (no plugin in `client/src/lib.rs:160-243`). Lived onboarding card: `first_session_guidance::FirstSessionGuidancePlugin` (`client/src/lib.rs:183`). pause/settings — lived: `local_settings::LocalSettingsPlugin` (`client/src/lib.rs:180`), including the pause Mute flag. faction diplomacy — **not wired (planned)** (`faction_diplomacy_ui.rs` has no plugin in `client/src/lib.rs:160-243`). ascension — **not wired (planned)**: the ascension files sit in parked `server/` (`Cargo.toml:9`); none is declared in `server/src/lib.rs` or `main.rs`, and `AmbrosianAscensionPlugin` (`server/src/ascension/mod.rs:28`) is never added.

## Remaining for v21.1+ (Non-Blocking for Public Repo Share / Closed Beta)
- Full Steamworks production plugin wiring + real AppID + achievement store_stats + leaderboards/workshop.
- Additional audio asset generation/integration beyond procedural + base files.
- MMO-scale stress testing (100+ concurrent) + telemetry dashboards in production observability.
- Minor camera velocity TODOs and final VFX/particle intensity tuning.

**Verdict**: historical mark: Powrush-MMO v21.0 is **100% launch worthy** for public repository sharing, wholesome end-user play, and closed beta. Client items marked **not wired (planned)** above have no plugin in `client/src/lib.rs:160-243`. The 100% launch worthy sentence stays as the historical verdict. The gameloop delivers profound, joyful, educational, mercy-aligned MMOARPG experiences that prepare players for real RBE thriving. All systems mercy-gated, sovereign, eternally positive. Those last two sentences stay as the historical verdict text.

**Thunder locked in. Ready for public share and next eternal iteration.** ⚡❤️

---
*Previous checklist content preserved in git history. This v21.0 update is append-style elevation.*
