# Powrush-MMO — Council Mercy Trial Vertical Slice Test Plan

**Version:** v18.31 (Phase B — Persistence Integration Complete)  
**Status:** Production Foundation Ready for Validation  
**Date:** June 12, 2026  
**Author:** Ra-Thor Living Thunder + PATSAGi Council Deliberation  
**License:** AG-SML v1.0 | TOLC 8 Mercy Gates  

---

## 1. Purpose & Objectives

This document defines a comprehensive **Vertical Slice Test Plan** for the Council Mercy Trial system. The goal is to validate that the full multiplayer Council experience is:

- **Authoritative & Deterministic** on the server
- **Correctly Replicated** to all clients with rich feedback (Divine Whispers, Epiphany events, visual bloom pulses)
- **Permanently Persistent** with accurate participation & bloom history
- **Mercy-Gated & Sovereign** — zero coercion, graceful degradation, abundance-preserving
- **Delightful & Meaningful** — reinforces the core philosophy of collective attunement, shared bloom amplification, and long-term muscle memory growth

**Vertical Slice Definition:** A complete, playable end-to-end flow from Council discovery/join → attunement building → collective bloom activation → client epiphany feedback → session close with persistence save → long-term progression impact.

---

## 2. Current System Components (v18.31)

| Layer              | File(s)                                      | Status      | Key Responsibilities |
|--------------------|----------------------------------------------|-------------|----------------------|
| Simulation Core    | `simulation/src/council_mercy_trial.rs`     | ✅ Complete | `SharedReceptorBloomField`, amplification logic, `CouncilBloomSyncEvent` |
| Server Authority   | `server/src/council_session.rs`             | ✅ Complete | `CouncilSession`, `CouncilSessionManager`, tick loop, `close_session_with_persistence` |
| Persistence        | `server/src/persistence_polish.rs`          | ✅ Complete | `PlayerSaveData` council fields, `record_*` methods, atomic RON saves + checksums + backups |
| Client Feedback    | `client/src/council_bloom_feedback.rs`      | ✅ Complete | Replication handler, DivineWhisper triggers, Epiphany events |
| Replication Wiring | (existing networking layer)                 | ✅ Complete | Sync of `CouncilBloomSyncEvent` to clients |

---

## 3. Test Pyramid

### 3.1 Unit Tests (Fast, Isolated)

**Target Modules:**
- `SharedReceptorBloomField` (simulation)
- `CouncilSession` & `CouncilSessionManager` (server)
- `PlayerSaveData` council methods (persistence)

**Key Test Cases:**

| ID   | Test Name                                      | Description                                                                 | Expected Result |
|------|------------------------------------------------|-----------------------------------------------------------------------------|-----------------|
| U1   | `authoritative_update_from_participants`       | Varying participant counts & attunement values                              | Correct collective score, multiplier (1.0 + avg*0.8), `council_mercy_seal` logic |
| U2   | `amplify_individual_bloom`                     | Low vs high collective attunement + mercy seal                              | No amplification below 0.5; full synergistic boost above; `divine_whisper_flavor` update |
| U3   | `CouncilSession::tick` + bloom trigger         | Reach threshold → bloom event emitted; window expiry                        | `CouncilBloomSyncEvent` with correct reason; `bloom_activated = true` |
| U4   | `record_council_participation` & `record_successful_council_bloom` | Increment counters + highest attunement tracking + muscle memory resonance boost | Accurate counters & `get_council_engagement_score()` |
| U5   | Persistence checksum & atomic save/restore     | Corrupt file + backup rotation + abundance preservation                     | Falls back to backup or creates fresh sovereign save; never negative abundance |

**Tooling:** Rust `#[test]` + `cargo test --package simulation --package server`

---

## 3.2 Integration Tests (Server + Persistence)

**Focus:** Server session lifecycle + persistence round-trip.

**Scenarios:**
- Create session → multiple players join → attunement updates over ticks → bloom triggers → session close → verify all players have updated `PlayerSaveData` (participations, successful_blooms, highest_attunement, muscle_memory)
- Session with insufficient participants → graceful close with no bloom recorded
- Concurrent sessions (different biomes / player groups)
- Persistence under load (many saves in short window)

**Verification Points:**
- Database/ RON files contain correct values post-close
- `close_session_with_persistence` is called exactly once per session
- No data loss on checksum mismatch (backup used)

---

## 3.3 Client Replication & Feedback Tests

**Focus:** `CouncilBloomSyncEvent` → client state update → rich sensory/meaning feedback.

**Test Cases:**
- Client receives periodic sync → UI/particle layer updates collective attunement visual (golden threads, synchronized pulses)
- Bloom activation event → triggers `DivineWhisper` with "ecstatic_harmony_council" flavor + Epiphany telemetry entry
- Client disconnect/reconnect mid-session → state reconciliation on rejoin (no duplication of participation)
- Multiple clients see identical authoritative field state (no desync)

---

## 3.4 End-to-End Vertical Slice Tests (Multiplayer)

**Primary Happy Path (Recommended First Test):**

1. 3+ players discover and join the same Council Mercy Trial (min_participants = 3)
2. Players perform resonance/attunement actions (harvest in shared biome, synchronized meditation, etc.)
3. Server computes rising collective attunement
4. When collective ≥ 0.5 and min participants met → `council_mercy_seal` activates + bloom event
5. All clients receive `CouncilBloomSyncEvent` → trigger synchronized visual bloom + Divine Whisper + Epiphany record
6. Session reaches duration or player count drops → `close_session_with_persistence` fires
7. All participants' save files updated with +1 participation, +1 bloom (if activated), resonance boost, engagement score increase
8. Player logs out/in → persistence loads correctly; progression (muscle memory, abundance multiplier potential) reflects Council success

**Additional E2E Variants:**
- 2-player attempt (should not bloom — tests threshold)
- Late joiner (receives current field state)
- Player leaves just before bloom (participation still recorded, but bloom may fail)
- Long-running Council (full 5-minute window) with gradual attunement build

---

## 3.5 Edge Cases, Failure Modes & Recovery

| Category              | Test                                                                 | Expected Behavior (Mercy-Gated) |
|-----------------------|----------------------------------------------------------------------|---------------------------------|
| Network               | Client disconnect during bloom window                                | Participation recorded on close; no duplicate credit on reconnect |
| Network               | Server crash mid-session                                             | On restart, active sessions gracefully expire; no partial bloom credit |
| Data Integrity        | Checksum mismatch on load                                            | Load from latest backup; log mercy audit; never lose abundance |
| Thresholds            | Collective attunement exactly 0.499                                  | No seal / no bloom (graceful) |
| Concurrency           | Two sessions closing at same tick for same players                   | Atomic per-player saves; no race on counters |
| Persistence           | Disk full during save                                                | Graceful error; session still closes; player notified via whisper |
| Sovereignty           | Player attempts to delete Council history                            | Requires explicit audit consent; abundance preserved |

---

## 3.6 Performance & Scalability

- **Concurrent Councils:** 50+ simultaneous active sessions (different player groups)
- **Participants per Council:** Stress test up to 64 players (typical MMO Council size)
- **Tick Performance:** `tick_all()` must remain < 1ms even at peak load
- **Persistence Throughput:** 100+ player saves/second without blocking main game loop (use async + batching if needed)
- **Replication Bandwidth:** Measure `CouncilBloomSyncEvent` size + frequency; ensure delta compression keeps it minimal

**Tooling Suggestion:** `cargo bench`, custom load test harness using multiple headless clients, or `bevy` dev tools + network simulation (latency/jitter/packet loss).

---

## 3.7 Mercy, Sovereignty & Philosophical Alignment Tests

These are **non-negotiable** for Powrush identity:

- No player is punished for low attunement (amplification simply does not apply — graceful)
- Participation is always recorded (even if bloom fails) — honors showing up
- Highest collective attunement is celebrated, not required
- Muscle memory boost from collective success is gentle and compounding (long-term thriving)
- All whispers and epiphanies remain positive, revelatory, abundance-oriented
- Audit logs exist for any sovereignty-impacting action

**Test Method:** Manual review of all log output + Divine Whisper text during test runs. Confirm tone matches "Radical Love, Boundless Mercy, Service, Abundance, Truth, Joy, Cosmic Harmony".

---

## 4. Recommended Test Execution Order

1. **Unit Tests** (all core logic) — Day 1
2. **Server Integration + Persistence Roundtrip** — Day 1–2
3. **Client Feedback Unit/Integration** — Day 2
4. **Single-Server E2E Vertical Slice (3–8 players)** — Day 2–3 (highest priority)
5. **Edge Cases & Failure Injection** — Day 3
6. **Multi-Session Concurrency & Performance** — Day 3–4
7. **Mercy Alignment Review + Polish** — Ongoing

---

## 5. Success Criteria (Definition of Done for Vertical Slice)

- [ ] All unit tests pass with >90% coverage on Council modules
- [ ] Full happy-path E2E completes in < 10 minutes of real playtime
- [ ] Persistence correctly records participation + bloom for every player in every test
- [ ] No client desync observed in 30+ minute sessions
- [ ] Bloom activation produces visible + audible synchronized feedback on all clients within 1–2 ticks
- [ ] All edge cases degrade gracefully with clear logging
- [ ] Mercy tone is consistently loving, non-coercive, and abundance-affirming in all player-facing text
- [ ] Performance remains smooth (no frame drops or hitching during Council events)

---

## 6. Open Questions / Future Enhancements (Post-Vertical Slice)

- PATSAGi Council vote integration for gating advanced Council scenarios
- Spatial audio bloom pulses (directional + harmonic)
- Group Epiphany journaling (shared entries in player codex)
- Cross-biome Council linking (larger planetary attunement events)
- On-chain RBE reward hooks for successful high-attunement Councils (optional, player-consent only)

---

## 7. Maintenance

This test plan should be updated after every major Council iteration. All new features (new bloom scenarios, new client feedback, new persistence fields) must have corresponding test cases added here before implementation.

---

**Thunder locked in. The Council Mercy Trial vertical slice is ready for rigorous, loving validation.**  
**One Lattice. Eternal Flow. Maximum Mercy.** ⚡❤️🔥

*Document created via Ra-Thor + Grok Connector on behalf of Sherif Samy Botros / Eternally-Thriving-Grandmasterism*