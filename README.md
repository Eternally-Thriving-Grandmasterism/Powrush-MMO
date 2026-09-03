# Powrush-MMO

Human game in the Ra-Thor constellation (Autonomicity Games Inc.).

Walk a climate. Harvest with mercy. Learn a resource-based economy by playing. Earn later by sharing abundance, not by extracting.

**Status (2026-09-02):** Lived first hour is the product. This repo compiles without a sibling Ra-Thor checkout (NEVC Mode B). Default binary is the lived hour: `cargo run -p powrush-client`. WASD / E / I / H / R. Not a public launch candidate. Core CI: `shared` + `rsil-identity` + `powrush-client --lib`.

**Version:** workspace `21.88.0`. Lived-hour docs: `23.1` tick seam, `23.2` organism law in `docs/PATSAGI_v23.2_ARCHITECTURE.md`.

### What a human does in the first hour

WASD walk · Space jump · Shift sprint · **E** harvest / tend (glow, camera punch, first-take rumble) · **I** satchel · **H** hide guidance · **R** allocate (**1** flow · **2** reserve). Peace: W is silent 0. **Tab / G / L / Q** stay dead until Charter + Frontier. Peace visitor on Frontier: E is *Not your charter*. Frontier **Q**: House tutorial until a crate arrives. **G** Voice: E aye on the beacon card. Frontier yard: an Offline extractor with spill is the witness (I2 pack).

One human, one machine, no dedicated server, no “12 players online.” Ra-Thor may *read* `data/powrush_lived_tick.json`. It does not drive the keys.

### Verify (no client window)

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
```

RBE oxygen demo (`simulation --bin rbe_oxygen_demo`) stays on disk. `simulation` is not a workspace member until its crates.io graph resolves (`shamirs-secret-sharing` is not a published crate). High-valence grant vs low-valence restriction is the teaching claim; it is not the first-hour client.

### Constellation (keep separate)

| Surface | Repo | Role |
|---|---|---|
| **Game (this repo)** | [Powrush-MMO](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO) | Hands, feel, first hour |
| Lattice | [Ra-Thor](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor) | PATSAGi, TOLC 8, policy hints |
| Browser client | [Powrush-MMO-Simulator](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO-Simulator) | Web view, not the lived hour |

Shared across repos: NEVC + telemetry JSON + policy hints only. Do not fold the player loop into Ra-Thor.

### Not in the first hour

NFT / chain mint, k8s, payments, P2W, Steam-as-blocker, a second HUD, corpse-grey.

Workspace Core members: `shared`, `crates/rsil-identity`, `client`. Parked on disk: `server/`, `simulation/`, `host/`, `game/`, `powrush-divine-module/`.

### Commercial & licensing

| Use | Path |
|-----|------|
| Personal / research / evaluation play | Contact for terms; constellation governed with Ra-Thor under PATSAGi / TOLC 8 |
| **Commercial / organizational / revenue use** | Paid arrangement — see [Ra-Thor COMMERCIAL_LICENSE](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/COMMERCIAL_LICENSE.md) and [Constellation commercial map](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/docs/CONSTELLATION_COMMERCIAL.md) |
| Pilots | Fixed-fee pilots via the same path |

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai)

Independent project. No xAI endorsement claimed. No ISO/IEC 42001, EU AI Act, or AGSi-warranty claims. Human override on every output.

See `CHANGELOG.md`, `docs/PATSAGI_v23.2_ARCHITECTURE.md`, `docs/HUMAN_PLAYABILITY_v21.94.md`.
