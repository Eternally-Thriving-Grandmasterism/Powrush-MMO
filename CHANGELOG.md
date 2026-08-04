# CHANGELOG.md — Powrush-MMO

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
