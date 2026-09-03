# CHANGELOG.md — Powrush-MMO

## [23.2.19] — 2026-09-02 — Skirmish well (Slice 15 / Wave 2)

Practice travelers hold the first well (Sanctuary ember). E Contest. After a short hold Mira answers. Dawn after loss — you still walk. Mercy aftercare, no corpse-grey, no F-key. Lives in Peace. Harvest still works while you hold.

## [23.2.18] — 2026-09-02 — Compass tells (Slice 14)

Live W at 20 and 60 speaks. Peace is silent. UI never shows the formula. Opting DeclaredLethal posts hunter x so the 20-tell can fire. No extra key. Last later-list item.

## [23.2.17] — 2026-09-02 — Hybrid Matrix v0 (Slice 13)

After a Sylvaris tend, E Attune. Stability 1. Double vision under the ledger. Not racial modules, not a second body. Dies in Peace. Compass ±20/60 is next.

## [23.2.16] — 2026-09-02 — Species redemption v0 (Slice 12)

After the stone is seen, E Offer a tend to the Sylvaris grove. One tick (0.25). Other four progresses stay 0. Not boarding, not Hivelord. Dies in Peace. Hybrid Matrix / Compass stay later.

## [23.2.15] — 2026-09-02 — Crownstone set-piece (Slice 11)

After the embassy seat, the stone is live. E Witness. Integrity 1, no owner, path Unset. Does not fire purify/destroy/sabotage. No F-key, no Peace PK, W stays walk. Species redemption / Hybrid / Compass stay later.

## [23.2.14] — 2026-09-02 — DeclaredLethal + blood tariff (Slice 10)

Opt-in Ledger clause. L open, **3** posts DeclaredLethal and a hunter blood tariff. Default win stays Bind. E still Bind/Escort. No F-key, no Peace PK, W stays walk. Last pack slice.

## [23.2.13] — 2026-09-02 — War week (Slice 9)

Tab Chart declares War week. Score is tons delivered + nodes restored (LaneCrate + MendSpool). Not loot. No combat key. Dies in Peace. DeclaredLethal stays last.

## [23.2.12] — 2026-09-02 — Embassy + blueprints (Slice 8)

After the Proof Pack unlocks, the embassy lamp is live. E Request seat. One local seat. Blueprints copy MendSpool and LaneCrate. Dies in Peace. War week is Slice 9.

## [23.2.11] — 2026-09-02 — Fabricator + Proof Pack (Slice 7)

After the crate arrives, Q plants a fabricator, then MendSpool (repair) and LaneCrate (logi). Proof Pack unlocks when both have run. Not +DPS. Dies in Peace. Embassy is Slice 8.

## [23.2.10] — 2026-09-02 — Ledger + Bind/Escort (Slice 6)

L opens The Ledger after Charter + Frontier. Default win is Bind, not lethal. Purse is flow + repair-rights. E Bind, then escort to the post. Dies in Peace. Server-auth Bind waits on the parked server.

## [23.2.9] — 2026-09-02 — Infra offline + spill (Slice 5)

An extractor is already Offline with visible spill (I2). E-slab shows life + pack hash. Dies in Peace. Does not teach attack. Repair/Ledger is Slice 6.

## [23.2.8] — 2026-09-02 — Co-op Voice (Slice 4)

Quorum card on the beacon. After Charter + Frontier, **G** opens Voice. **E** aye · **2** nay. One local seat; the tutorial card carries. Dies in Peace. Harvest E still works when the sash is closed. Live multi-seat AOI waits on the parked server.

## [23.2.7] — 2026-09-02 — Vertical factory (Slice 3)

Local House tutorial: Q on Frontier founds, then extractor → depot → hauler → two stops → arrival chime (*The machine exists*). Dies in Peace. Two-client AOI waits on the parked server.

## [23.2.6] — 2026-09-02 — Hex flags (Slice 2)

Peace / Frontier / War / Contestable. Industry and Ledger only where flagged. Contestable is opt-in. A Peace visitor on Frontier: E says *Not your charter / Peace visitor*. Peace hour E still harvests.

## [23.2.5] — 2026-09-02 — Identity persist (Slice 1)

`rsil-identity::IdentityPersist` keeps charter_id, W, kind, lethal_count, repair_ratio on the DID. Peace still reports live W = 0. No tradecraft in the JSON.

## [23.2.4] — 2026-09-02 — Hour sacred (Slice 0)

Peace hour stays one human / one machine:
- `shared::space_law` — HexFlag, WarrantWeight (live 0 in Peace), CharterKind, skin gate
- Tab / G / L / Q no-op without `charter_id` + Frontier. WASD / E / I / H / R untouched
- No hunt UI, no Currency3, no factory

## [23.2.3] — 2026-09-02 — Allocate by hand

R is earn-by-sharing, not a mouse-only panel:
- **1** flow outward · **2** steward reserve while the panel is open (satchel 1/2 yield)
- First share is its own thriving moment (does not steal harvest/inventory firsts)
- Guidance teaches R after the satchel

## [23.2.2] — 2026-09-02 — Harvest juice

First E at a glowing node is a reward, not a counter tick:
- Node emissive + child point light bloom with `pulse`
- Camera punch from `HarvestJuice` (first take stronger; never steals look)
- First-take rumble is longer; epiphany credit only on the first harvest

## [23.2.1] — 2026-09-02 — Lived-hour client bin

- Default `powrush-client` bin is `PowrushClientBundle` (walk, harvest, satchel, guidance), not GPU test spheres.
- Bundle no longer pulls networking, `simulation`, ambisonics, or egui GPU materials.
- Harvest reach uses `SoftPresence` (the human body). `RbeGlobalState` / `RbeUiSync` are local first-hour resources.

## [23.2.0] — 2026-09-02 — Standalone cargo (human game)

Lived first hour is the product. This slice unblocks building *this* repo alone:

- `shared` no longer path-deps sibling `../../Ra-Thor/crates/mercy_tolc_operator_algebra`. NEVC **Mode B** (`nevc_adapter`) is the default. Mode A is a local overlay when the lattice is co-located — not a CI feature.
- Workspace Core members: `shared`, `crates/rsil-identity`. Parked `client`, `server`, `simulation` (unpublished `shamirs-secret-sharing`), `host`, `game`, `powrush-divine-module`. Lived hour stays `client/src`.
- CI Core: `cargo test -p shared -p rsil-identity`. Parked workspace fmt/clippy/`--workspace` test, WASM/Trunk, Docker, and cargo-deny until those surfaces compile.
- README no longer claims 100% launch-worthy.

Next: default `powrush-client` bin is `PowrushClientBundle`, not GPU test spheres.

## [23.1.0] — 2026-08-26 — Lived tick

Launch-worthy was the *sim*. This is the seam into the hour:
- `data/powrush_lived_tick.json` ~1 Hz (lineage, pool, climate, flow, pocket)
- Classic ↔ sim race map documented
- Ra-Thor may read; it may not drive the keys

## [23.0.0] — Lineage (C)
## [22.15.0] — Practice travelers

Contact: **info@Rathor.ai**. Thunder locked in. Yoi ⚡
