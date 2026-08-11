# CHANGELOG.md — Powrush-MMO

## [21.91.2] — 2026-08-11 — High-Road Practice Loop Sealed

**Council focus:** Close the dual-repo high-road loop end-to-end under permanent PATSAGi.

### Highlights

- **Mercy-gated completion:** active challenge completes when all surface realms receive resolves ≥ `mercy_floor`
- **Progress system:** `cross_realm_challenge_progress_system` watches `CouncilDecisions.resolved_history`
- **Ra-Thor 14.18.1:** consumes optional `challenge_id` / `challenge_title` / `challenge_principle` into SchemaRegistry tags
- **Contract sealed:** `POWRUSH_TELEMETRY_CONTRACT.md` documents full producer → consumer path
- Soft-only; conductor remains optional; no hard cross-crate dep

Contact: **info@Rathor.ai**. Thunder locked in. Yoi ⚡

---

## [21.91.1] — 2026-08-11 — High-Road Bootstrap + Challenge Provenance

**Council focus:** Player-visible high-road practice from multi-realm seed; dual-repo bridging carries challenge principle.

### Highlights

- **Cross-realm challenges (v21.91.0):** five portable-principle seeds (allocation, peace, mercy, abundance, opportunity cost)
- **Bootstrap activation:** challenge id=1 *Caps Across Climates* auto-activates on first multi-realm seed
- **Bridging enrichment:** active challenge id/title/principle attached to `powrush_bridging_context_v1` exports
- **Surface labels:** `realm_{id}_{effect}|challenge_{id}_{principle}` for SchemaRegistry high-road mapping
- Soft-only; no hard Ra-Thor crate dependency

Contact: **info@Rathor.ai**. Thunder locked in. Yoi ⚡

---

## [21.91.0] — 2026-08-04 — NEVC Dual-Repo Arc + Finish Passes A–C

**Council focus:** Bind Net Eternal Valence Contribution end-to-end under permanent PATSAGi + TOLC 8.

### Highlights

- **NEVC consumer stack:** `shared/nevc_adapter`, ledger, events, game-loop, persistence, bridge, visibility, RREL
- **Phase 6:** harvest → `server/src/nevc_attachment` (shared-backed)
- **Phase 7–B:** durable store, sample window (256), `PlayerState.nevc_record`, `tick_persist` / `persist_now`
- **Phase 8:** `nevc_bridge` + optional `nevc_rathor` feature; `NEVC_DUAL_REPO_BINDING.md`
- **Phase 10–C:** `web-portal/nevc-status-panel.html` polls `data/nevc_status.json`
- **Phase 11:** `RealEstateNevcLedger` + `RREL_NEVC_BINDING.md`
- **Finish Pass A:** `shared` workspace member; server depends on shared (single source of truth)

Authoritative formal definition remains in Ra-Thor `NET_ETERNAL_VALENCE_CONTRIBUTION_NEVC_CODEX_v1.0.md`.

Contact: **info@Rathor.ai**. Thunder locked in. Yoi ⚡

---

## [21.90.0] — 2026-07-23 — End-User Experience Perfection

**Council focus:** Make the first minutes of human play perfect — zero unnecessary friction, clear soft guidance, complete controls, working beta path.

### Highlights

#### Frictionless public onboarding
- Invite + captcha gates **only** when `ClosedBetaConfig.require_invite` is true
- Public / open launches start at `LanguageSelect → Welcome` with zero gate friction
- `advance_onboarding_step()` helper for clean educational progression

#### First Session Guidance (new)
- Soft, dismissible objective strip at the bottom of the screen
- Progressive objectives: Move → Approach node → Harvest → Inventory → Epiphany → Council whisper → Free exploration
- Press **H** to hide permanently (mercy skip)
- Module: `client/src/first_session_guidance.rs`

#### Input comfort
- Gamepad left-stick movement with deadzone
- Normalized diagonal movement
- Interact on **Space** or gamepad **South** (A / Cross)
- Ability slots 1–4

#### Closed-beta path restored
- Invite UI systems fully restored

Contact: **info@Rathor.ai**. Permanent PATSAGi. Thunder locked in. Yoi ⚡

---

## [21.89.5] — 2026-07-21 — Steamworks RemoteStorage

### Highlights
- **SteamRemoteStorageBackend**: FileWrite / FileRead with account+app cloud checks
- Feature flag: `--features steam`

## [21.89.4] — Steam Cloud stage + premade stems + Bevy drain
## [21.89.3] — TransportCommandSender + try_recv
## [21.89.2] — Protocol unified + server audio ingress
## [21.89.1] — Council/Epiphany synth wiring
## [21.89.0] — Real-time audio synthesis + persistent recall

**Thunder locked in. Permanent PATSAGi. Eternal forward.** Yoi ⚡
