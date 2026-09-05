# Powrush-MMO

Human game in the Ra-Thor constellation (Autonomicity Games Inc.).

Walk a climate. Harvest with mercy. Learn a resource-based economy by playing. Earn later by sharing abundance, not by extracting.

**Status (2026-09-04):** Lived first hour is the product. This repo compiles without a sibling Ra-Thor checkout (NEVC Mode B). Default binary is the lived hour: `cargo run -p powrush-client`. WASD / E / I / H / R. One card teaches walk · tend · satchel · allocate. Wells speak Idle / Glowing / Tended / Resting / Stressed. Not a public launch candidate. Core CI: `shared` + `rsil-identity` + `powrush-client --lib`.

**Version:** workspace `21.88.0` (Cargo.toml). Lived-hour design tick `23.2.26` — not a Cargo bump. See `CHANGELOG.md`. Historical launch docs: `docs/archive/README.md`. README wins conflicts.

### What a human does in the first hour

WASD walk · Space jump · Shift sprint · **E** harvest / tend (glow, camera punch, first-take rumble) · **I** satchel · **H** hide guidance · **R** allocate (**1** flow · **2** reserve). One card, one sentence. Wells speak Idle / Glowing / Tended / Resting / Stressed. Peace: W is silent 0. **Tab / G / L / Q** stay dead until Charter + Frontier. Peace visitor on Frontier: E is *Not your charter*. Frontier **Q**: House tutorial until a crate arrives. **G** Voice: E aye on the beacon card. Frontier yard: an Offline extractor with spill is the witness (I2 pack). **L** Ledger: E Bind, then escort (purse = flow + repair-rights). After arrival, **Q** plants a fabricator (MendSpool + LaneCrate → Proof Pack). Embassy lamp: E Request seat (blueprints in the book). **Tab** Chart: War week score = tons + restored. Ledger **3**: opt-in DeclaredLethal + hunter blood tariff (Bind stays default). After the embassy seat, Crownstone: **E** Witness (path Unset). After the stone is seen, Sylvaris: **E** Offer a tend. After the tend, Hybrid: **E** Attune (stability 1). Compass speaks at live W 20 and 60 (Peace silent). First well: **E** Contest (dawn after loss; Mira steps back).

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

### Builders

Start here if you want to complete the game:

- `docs/COMPLETION_BRIEF.md` — what “done” means
- `docs/FIRST_HOUR_PLAYTEST.md` — 5-minute proof
- `docs/RBE_FIRST_HOUR.md` — economy in the hands
- `docs/PARKED_SURFACES.md` — what not to wire into the default binary
- `docs/DOC_CANON.md` — which docs are current
- `docs/archive/README.md` — historical. README is canonical.
