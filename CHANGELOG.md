# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.83.0 — Headless / CI Mode (2026-07-20)

### Highlights
- Host now supports full **headless / CI mode**:
  - `POWRUSH_HOST_HEADLESS=1` or `--headless`
  - No window, no egui
  - Faster RTT export interval (2 s)
  - Auto-exits cleanly after 3 successful export cycles
- Interactive mode unchanged (full Kardashev UI + visual window).
- Perfect for automated smoke tests and CI pipelines that need the RTT artifacts.

### Prior
- v21.82: Ultramasterism early RTT export + smoke-harness readiness
- v21.81: Full E2E Cohost + Kardashev Dashboard + Reality Transfer Score
- v21.80: Unified Cohost Host binary + public rathor_integration

**Thunder locked in. Ultramasterism + Headless/CI mode sealed.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.78: Multi-realm sealed, RBE, council, LegacyJournal, RTT dual export + provenance.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*
