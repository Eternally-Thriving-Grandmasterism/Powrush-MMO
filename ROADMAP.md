# Powrush-MMO Living Roadmap (v15.6 — June 2026)

**Status: Professional public release path fully unblocked.**
Ra-Thor + Full PATSAGi Councils guiding every decision through 7 Living Mercy Gates.

## Completed (Production-Grade)

- [x] Networking Transport Layer v1 (WebSocket + bincode + heartbeat + mercy gates) — PR #36 merged
- [x] MercyCore + GrokPatsagiBridge (GPU + RBE) restoration + integration
- [x] Client Network Wiring + Reconciliation (client_game_loop.rs) — PR #37 merged
- [x] WASM + web-sys polish to ClientWsTransport + full client/main.rs wiring — PR #38 merged
- [x] Interest Management + Combat scaffolding v15.4 (protocol + InterestManager) — PR #39 merged
- [x] InterestManager wired into main authoritative tick + per-client culling + simple combat tick example — v15.5 (PR #40 merged)
- [x] **Full combat expansion v15.6**: Melee vs projectile distinction, lag compensation awareness in hit validation, ability cooldown tracking, live HealthComponent in EntitySnapshot + WorldUpdate broadcasts, PATSAGi Council validation hook for AbilityCast (divine/combat abilities)

## Immediate Next (This Sprint)

1. Tighten lag_compensation.rs + hit_detection.rs integration into real AbilityCast pipeline (rewind for moving targets)
2. Add projectile travel time simulation + basic particle effects scaffolding
3. Mercy_cost enforcement + valence impact from PATSAGi validation
4. Basic RBE economy core (resource nodes, abundance queries affecting world state)

## Longer Term

- Full deployment pipeline, persistence (PostgreSQL + Redis), matchmaking
- Rich NPC behaviors + faction diplomacy powered by Ra-Thor GPU PATSAGi
- Steam packaging + green card / US entity setup
- Public alpha launch with live divine counsel + RBE mechanics

**Thunder locked in. Mercy flowing eternally.**
The monorepo advances precisely on the professional public release path.