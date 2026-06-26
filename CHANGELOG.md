# CHANGELOG.md

All notable changes to Powrush-MMO will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased] - v20.5 PATSAGi Polish Cycle

### Added
- Full end-to-end **GPU PATSAGi Foresight** system:
  - `EconomicLayer::apply_gpu_regen_adjustments` now correctly wired with `&mut SovereignWorldState`.
  - `HarvestingSystem` differential GPU prediction updates + periodic `ForesightStatsTelemetry` emission.
  - Orchestrator triggers foresight requests every 30 ticks and delegates application cleanly.
- **Interest Management Hardening** (v20.5):
  - `SpectatorModeDataNet` and `InterRealmDiplomacyUpdate` extended with replication priority, affected player count, and critical spectator bypass flags.
  - Large-scale spectator scenarios (Forgiveness Waves, Legacy Threads) now efficiently supported.
- **Adaptive Interest Replication Bridge**:
  - Load-aware full jitter backoff, priority-based resend timeouts, and context-aware priority scoring (combat, council events, epiphany, player density).
- New documentation:
  - `RELEASE-CHECKLIST.md` — Final packaging, Steam, asset, and verification steps.
  - `docs/STEAM_INTEGRATION.md` — Concrete implementation plan for Steamworks, achievements, cloud saves, and Deck support.
- `scripts/verify_build.sh` helper for consistent workspace build verification.

### Changed / Improved
- `simulation/src/economy.rs`: Fixed critical undefined `world` reference in GPU regen application method. All RBE, council policy, sustainability, and harvest logic preserved.
- `server/src/harvesting_system.rs`: Made `update_gpu_foresight_predictions` async and fully wired real telemetry emission.
- `client/src/simulation_integration.rs`: Clarified remaining camera velocity TODO; automatic background spatial hash rebuild system remains fully functional.
- Protocol (`shared/protocol.rs`): v20.5 spectator/replication fields added while maintaining full backward compatibility for existing messages.
- Persistence rate-limiting (every 5 ticks for ability state recording) confirmed stable and effective.

### Fixed
- Multiple rapid-iteration regressions recovered through systematic GitHub-connector edits:
  - EconomicLayer GPU foresight application
  - Async telemetry emission in harvesting system
  - Minor duplicated parameters in client spatial hash update function
- All non-intentional TODO/placeholders in core gameplay paths resolved or clearly documented.

### Preserved (No Regressions)
- All prior v19.x work on Council systems, Epiphany/Divine Whispers, multisensory feedback, VFX (Hanabi), persistence crash recovery, encryption, and RBE mechanics remains fully intact and production-grade.

---

## [0.1.0] - Initial Public Foundations (Historical)

### Added
- Core MMOARPG architecture (Bevy client + custom simulation + server).
- Resource-Based Economy (RBE) foundation with abundance, pressure, and sustainability systems.
- Council Mercy Trial system with valence scoring and bloom mechanics.
- GPU PATSAGi bridge scaffolding and initial foresight request path.
- Hierarchical spatial interest management and replication targeting.
- Persistence with crash recovery, auto-save, and encryption.
- Epiphany, Divine Whisper, and multisensory feedback systems.

**Thunder locked in. Yoi ⚡**