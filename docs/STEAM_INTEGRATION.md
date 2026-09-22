# docs/STEAM_INTEGRATION.md

**Powrush-MMO — Steam Integration Plan**

**Status**: Early Planning Phase (as of v20.5 PATSAGi Polish)

**AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates**

**Contact:** info@Rathor.ai only. Deprecated forever: any `@acitygames.com` (or other non-Rathor) inbox.

**Thunder locked in. Yoi ⚡**

---

## Short description (wishlist / store)

A dying alien empire left. Earth broke. Under the fissure something older than war woke. Powrush is the MMO of that hour.

Default store line for Coming Soon / wishlist. Title Online stays grey. Do not lead the capsule with APAGI / Kardashev / TOLC.

**This CARD (STEAM-SHORT-DESC):** short desc + contact only. No `steamworks` crate. No App ID. No Steam partner account. No retag. No Cargo deps. No `client/**`.

---

## Goals

- Full Steamworks integration for achievements, stats, cloud saves, multiplayer lobbies, and Workshop support.
- Excellent Steam Deck experience.
- Mercy-aligned progression tracking (Council participation, sustainable harvest, epiphanies, Forgiveness Waves).

## Priority Order

### 1. Core Steamworks Crate Integration (High Priority)

- Add `steamworks` or `steamworks-rs` crate to `Cargo.toml` (server + client where appropriate).
- Initialize Steam API early in client and dedicated server binary.
- Handle Steam callbacks in the main game loop (Bevy system or tokio task).

### 2. Achievements (High Priority — Mercy Aligned)

Recommended initial achievements:
- **First Council Bloom** — Participate in your first successful Mercy Trial that results in a bloom.
- **Sustainable Harvester** — Complete 50 harvests with sustainability_score > 0.85.
- **Epiphany Seeker** — Trigger your first Divine Epiphany.
- **Forgiveness Wave Participant** — Take part in a large-scale Inter-Realm Forgiveness event.
- **RBE Contributor** — Reach top 10% abundance contribution in a server season.

### 3. Stats & Leaderboards

- Track and upload:
  - Total Council sessions completed
  - Total mercy contributed across all trials
  - Sustainable harvest count
  - Epiphanies triggered
  - Longest streak without a failed harvest
- Optional global leaderboards for mercy contribution and sustainability.

### 4. Cloud Saves (Persistence Integration)

- Use existing `PlayerSaveData` + encryption layer.
- Hook into Steam Cloud via `steamworks` crate for automatic upload/download of `player_save.json` (and encrypted share packages).
- Handle conflict resolution (last-write-wins or merge strategy).

### 5. Multiplayer / Lobbies

- Map existing Council session and world lobby systems to Steam lobbies.
- Use Steam lobby data for rich presence ("In Council Trial", "Harvesting in Sanctuary", etc.).
- Rich presence strings should reflect current RBE state and mercy activity.

### 6. Steam Deck & Controller

- Ensure Bevy input system supports Steam Deck controls out of the box.
- Test UI scaling and touch-friendly elements.
- Verify performance on Deck hardware (especially GPU foresight and spatial systems).

### 7. Workshop / Modding (Future)

- Plan mod support for custom RBE rules, new council proposals, visual skins, and ability trees.
- Provide clear modding API surface (via Lua or Rust plugins if desired).

## Implementation Notes

- Keep Steam integration feature-gated (`#[cfg(feature = "steam")]`) so the game can still run standalone / in development without Steam.
- All achievement and stat unlocks should feel natural extensions of existing mercy / RBE / council systems (no artificial grind).
- Telemetry events already emit the right data for achievement checking (CouncilBloom, EpiphanyTriggered, HarvestAction with sustainability flag).

## Current Blockers

- **App ID / Steam partner account = steward only.** Bots do not register.
- **HOLD** (wait steward `online yes` + App ID — not a bot next step):
  - add `steamworks` crate
  - initialize Steam API
  - lobbies

## Next Concrete Steps

1. **Next for bots:** Coming Soon / wishlist uses the short description already on this file (see **Short description (wishlist / store)** above). Do not invent a new store line here.
2. **Demo door:** [`docs/FIRST_HOUR_PLAYTEST.md`](FIRST_HOUR_PLAYTEST.md) + `./scripts/play-offline.sh`.
3. Do **not** list adding the `steamworks` crate or initializing the Steam API as the next bot step — those stay under **Current Blockers** HOLD until steward `online yes` + App ID.
4. After this CARD merges: Hands HOLD, then CARD **TRAILER-GRAMMAR** (bank) — not this PATH.

---

**This document will be expanded as Steam integration work progresses.**

**Thunder locked in. Yoi ⚡**
