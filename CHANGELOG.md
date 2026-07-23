# CHANGELOG.md — Powrush-MMO

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
- Never blocks movement, harvest, or joy
- Module: `client/src/first_session_guidance.rs`

#### Input comfort
- Gamepad left-stick movement with deadzone
- Normalized diagonal movement
- Interact on **Space** or gamepad **South** (A / Cross)
- Ability slots 1–4

#### Closed-beta path restored
- Invite UI systems that were empty stubs are fully restored:
  - Panel visibility
  - Text input + Enter submission
  - Live status / error messaging

Contact: **info@Rathor.ai**. Permanent PATSAGi. Thunder locked in. Yoi ⚡

---

## [21.89.5] — 2026-07-21 — Steamworks RemoteStorage

### Highlights
- **SteamRemoteStorageBackend**: `FileWrite` / `FileRead` with account+app cloud checks and quota guard
- **SteamworksRemoteStoragePlugin**: `Client::init` / `init_app`, per-frame `run_callbacks`, graceful Null fallback
- Feature flag: `--features steam` (workspace `steamworks = 0.11`)
- Dev: `steam_appid.txt` (480) + `STEAM_APP_ID` env
- Docs: `docs/STEAM_CLOUD_AUDIO.md`

## [21.89.4] — Steam Cloud stage + premade stems + Bevy drain
## [21.89.3] — TransportCommandSender + try_recv
## [21.89.2] — Protocol unified + server audio ingress
## [21.89.1] — Council/Epiphany synth wiring
## [21.89.0] — Real-time audio synthesis + persistent recall

**Thunder locked in. Permanent PATSAGi. Eternal forward.** Yoi ⚡
