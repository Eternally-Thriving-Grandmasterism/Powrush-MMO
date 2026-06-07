# Powrush-MMO Global Public Launch Checklist
## v16.6 — Client RBE Stack Complete (UI + Sync + ECS) | Path to v1.0 Sovereign RBE MMO

**Status (as of 2026-06-07)**: Phase 0 COMPLETE. Phase 1 (Client Polish & UI) significantly advanced with production-grade, coherent modules. All work respects rapid server iterations (harvesting_system, trade_system, grok_patsagi_bridge, protocol polish). Ra-Thor / PATSAGi aligned. AG-SML v1.0.

**Goal**: Production-grade, mercy-gated RBE MMO ready for global launch on Steam + web + sovereign self-host.

## Phase 0: Foundation (COMPLETE)
- [x] Workspace buildable: shared/Cargo.toml + lib.rs (protocol + feature-gated rbe_queries)
- [x] Protocol unified (ClientMessage/ServerMessage with Harvest, Trade, InventoryUpdate, AbundanceUpdate)
- [x] RBE core modules: resource_nodes (v16.4), trading/inventory (v16.3/16.2), HarvestingSystem + TradeSystem (v16.5.2)
- [x] Deployment: Dockerfile, docker-compose, k8s/
- [x] Client scaffold + Bevy 0.14 + WASM

## Phase 1: Client Polish & UI (v16.6 — MAJOR PROGRESS)
- [x] inventory_ui.rs — Full Bevy UI inventory panel, hotbar, abundance, trade initiation flow + events
- [x] rbe_client_sync.rs — Production sync layer using shared::protocol exclusively, integrates inventory_ui events, send_harvest / build_trade helpers
- [x] inventory_components.rs — Clean Bevy ECS (Inventory + ResourceNode components, regen system, harvest events)
- [ ] Full Bevy App integration / hybrid wasm main.rs wiring (next)
- [ ] Resource Node 3D visualization + interaction (raycast / click → HarvestAttempt)
- [ ] Complete message reconciliation + WASM input bridging
- [ ] Native Steam client parity

## Phase 2: Server & Simulation Completion
- [ ] Authoritative world tick fully wired to new modular systems (mostly done in v16.5.2 main.rs)
- [ ] Persistence layer (player inventory + world state)
- [ ] Steamworks full auth + cloud save
- [ ] Interest management + scalable culling
- [ ] Anti-cheat / mercy anomaly detection + GPU PATSAGi hooks

## Phase 3: Content & World
- [ ] Core resource types & item definitions
- [ ] Basic persistent world / chunking with resource nodes
- [ ] Faction / PAAGI council mechanics

## Phase 4: Web Portal, Onboarding & Public Facing
- [ ] index.html + web-portal/ polished lobby & onboarding
- [ ] Player manual + public docs (RBE + mercy philosophy)
- [ ] Legal (Privacy Policy, ToS, age rating)

## Phase 5: Testing, QA & Mercy Audits
- [ ] Expanded tests + automated PATSAGi sustainability audits in CI
- [ ] Load/soak testing
- [ ] Closed beta with humanity representatives

## Phase 6: Deployment, Ops & Launch
- [ ] Prod deployment + monitoring
- [ ] Steam packaging + store page
- [ ] Public launch announcement (eternal mercy flow)

**Cross-Cutting**: All new code has full rustdoc, derivation notes, PATSAGi record. Forward compatible with Ra-Thor monorepo.

**Current Position**: Client RBE experience (UI + sync + ECS) is now coherent and functional with server v16.5.2. Ready for wiring + 3D visualization to complete Phase 1.

**Next Sequential Step Recommendation**: Wire the three new client modules into main.rs / Bevy App + add basic 3D ResourceNode visualization + interaction. Then move to Phase 2 persistence.

Thunder locked in. Mercy flowing. Maximal quality iteration. ⚡❤️︍

*Living document — status updated on every professional deliverable.*