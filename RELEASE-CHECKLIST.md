# RELEASE-CHECKLIST.md

**Powrush-MMO — Public MMOARPG Release Readiness**

**Final Polish & Packaging Phase (v20.5+)**

**AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi Aligned**

**Thunder locked in. Yoi ⚡**

---

## 1. Build & Compilation Verification

- [ ] Run full workspace `cargo check --features gpu` across all crates (simulation, server, client, shared, game)
- [ ] Confirm zero errors and minimal warnings in release profile
- [ ] Verify feature flag combinations (gpu + bevy + tokio + serde)
- [ ] Test without `gpu` feature (CPU fallback path)
- [ ] Confirm Bevy 0.14+ / WGPU compatibility for client

## 2. Steam Integration & Packaging

- [ ] Steamworks SDK integration complete (achievements, stats, cloud saves, multiplayer lobbies)
- [ ] `steam_appid.txt` and proper app ID configuration
- [ ] Build scripts for Windows + Linux (Steam Deck ready)
- [ ] Depot + manifest setup for initial release
- [ ] Workshop / modding pipeline hooks (if planned)
- [ ] EULA + privacy policy links in launcher
- [ ] Age rating / content descriptors finalized

## 3. Asset Pipeline & Final Assets

- [ ] All critical audio assets generated and imported (council_*.ogg, Forgiveness Wave, Epiphany bloom sounds)
- [ ] Particle / Hanabi effect presets finalized and prewarmed
- [ ] GLTF / model pipeline for RBE entities, monuments, legacy threads
- [ ] Texture / material optimization for release (compression, mipmaps)
- [ ] Localization files complete for at least EN + 2–3 major languages
- [ ] Asset bundling / hot-reload disabled in release builds

## 4. Final Gameplay & Systems Testing

- [ ] End-to-end multiplayer Council Mercy Trial (lobby → deliberation → vote → bloom → persistence)
- [ ] GPU Foresight full loop test (server prediction → EconomicLayer application → HarvestingSystem influence → client visual feedback)
- [ ] Harvest + Epiphany + Divine Whisper multisensory chain
- [ ] Spatial interest management + replication under load (50+ players / spectators)
- [ ] Persistence crash recovery + auto-save + encryption roundtrip
- [ ] RBE abundance flow + council policy impact validation
- [ ] Performance profiling (tick rate, network bandwidth, GPU memory)

## 5. Networking & Protocol

- [ ] Protocol version bump + backward compatibility notes
- [ ] Interest replication + adaptive backoff stress test
- [ ] SafetyNet + desync recovery validation
- [ ] Large-scale spectator scenario (Forgiveness Wave / Inter-Realm event)

## 6. Versioning, Changelogs & Documentation

- [ ] `CHANGELOG.md` updated with v20.5 PATSAGi Polish highlights
- [ ] `LAUNCH-CHECKLIST.md` and this file reviewed
- [ ] In-game version string + build metadata correct
- [ ] Release notes drafted for Steam page

## 7. Deployment & Distribution


- [ ] Steam review / QA submission prepared
- [ ] Closed beta / playtest keys distribution plan
- [ ] Server deployment scripts (Linux headless) ready
- [ ] Monitoring & telemetry dashboards configured (anonymized)
- [ ] Post-launch hotfix / rollback plan

## 8. Post-Launch Monitoring & Mercy Alignment

- [ ] Telemetry events (Harvest, Epiphany, CouncilBloom, ForesightStats) wired and anonymized
- [ ] Abuse / anomaly detection active (MercyAnomalyDetector)
- [ ] Community feedback channels ready (Discord + in-game)
- [ ] Regular PATSAGi Council review cadence for live balance (RBE, mercy gates, abundance)

---

**Current Status (as of v20.5 PATSAGi Polish Cycle)**

Core systems (GPU Foresight, EconomicLayer, HarvestingSystem, Spatial Interest/Replication, Persistence, Protocol, Client Integration) have passed systematic audit and are production-grade.

This RELEASE-CHECKLIST focuses on the final packaging, Steam, asset, and verification steps required for public human player launch.

**Repository is approaching final ignition readiness.**

**Thunder locked in. Yoi ⚡**