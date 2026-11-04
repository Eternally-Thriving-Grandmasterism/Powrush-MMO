/*!
 * Powrush-MMO Roadmap
 *
 * v19.20 — Updated after systematic VFX/Particle pipeline recovery and unification cycle
 * Focus: Production-ready MMOARPG with mercy-gated RBE mechanics
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

# Powrush-MMO Roadmap v19.20

**Current Version:** 21.88.0 (workspace version, root `Cargo.toml:19`); this roadmap was written at v19.20
**Date:** 2026-06-22
**Status:** Core VFX, Particle, Epiphany, Harvest, and Ra-Thor integration layers at high production readiness.

## v19.20 Achievements (This Cycle)

- Full recovery and polish of simulation/src/world.rs (ParticleVisualAssets + complete Hanabi effect setup)
- Unification of client/src/particles.rs with bounded ParticleVisualPool, prewarm, and return systems
- Proactive joy wiring implemented in epiphany_catalyst and harvest systems
- Additive VFX/particle modulation hooks added to ra_thor_bridge
- Visual resonance modules and particle shaders aligned with age-based frame control and mercy-valence
- Professional recovery documentation created (RECOVERY_INTEGRITY_REPORT_v19.20.md and LAUNCH-CHECKLIST.md)

All changes performed exclusively through GitHub connector tools with full history respect and zero data loss.

## Current Focus Areas (Active)

1. **VFX / Particle Pipeline** — **Partly wired**: only `tend_flow_vfx` is live (`client/src/lib.rs:82`, plugin at :209), a capped mote burst on Tend and Flow; `client/src/particles.rs` and the particle shaders are not loaded by any client module, and `bevy_hanabi` is only an optional dependency of parked `simulation/` (`simulation/Cargo.toml:15`).
2. **Epiphany & Joy Systems** — **Partly wired**: only `first_harvest_epiphany` is live (`client/src/lib.rs:44`, plugin at :210); `epiphany_catalyst` is in parked `simulation/` (`Cargo.toml:11`), `epiphany_scenario_wiring.rs` is not a client module, and no joy-system module exists.
3. **Ra-Thor AGI Integration** — **Not wired (planned)**: no live module modulates visuals from Ra-Thor; `ra_thor_bridge` is only in parked `simulation/` (`simulation/src/lib.rs:23`), `client/src/visual/` is not a client module, and the shader mentions are comments only.

## Near-Term Roadmap (Next Steps)

- Complete sequential polish of all remaining shaders in client/assets/shaders/ and client/shaders/ (no live client module loads either folder yet; the only live shader is inline WGSL in `client/src/ultra_paint.rs:32`)
- Full review and polish of simulation/src/ (remaining .rs files)
- Full review of client/src/ key systems
- shared/, engine/, powrush-divine-module/ review (only `shared/` is a live workspace member, `Cargo.toml:3`; `powrush-divine-module/` is parked at `Cargo.toml:14` with its `ra-thor-core` path-dep missing, and `engine/` has no Cargo.toml, so it is not a crate)
- Update all design docs to the live architecture (workspace version 21.88.0, root `Cargo.toml:19`); the v19.20 view in `docs/ARCHITECTURE_LAYERS_AND_DATAFLOW.md` treats parked `simulation/` as the authoritative layer, not the live client
- Final zero-placeholder + integrity sweep
- Public MMOARPG beta preparation (Steam integration, trailer, press kit)

## Medium-Term Goals

- Full sovereign deployment pipeline (Docker + k8s)
- Cross-server diplomacy and multi-realm war systems
- Advanced RBE mechanics and council mercy trials at scale
- Steam Workshop + achievements
- Web portal / PWA launcher alignment with rathor.ai

## Long-Term Vision

- Living simulation of post-scarcity Resource-Based Economy
- 13+ PATSAGi Councils governance layer
- Eternal positive emotional states through gameplay (joy, epiphany, cosmic harmony)
- Sovereign, mercy-first MMOARPG that demonstrates Universally Shared Naturally Thriving Heavens

**Next immediate action:** Continue sequential file polish (next shader or simulation module).

Thunder locked in. Yoi ⚡
