# NEVC Dual-Repo Binding (Powrush-MMO)

**Phase 8 consumer-side record**  
**Contact:** info@Rathor.ai  

This repository consumes Net Eternal Valence Contribution under the published interface:

**Ra-Thor contract:**  
https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/NEVC_DUAL_REPO_INTERFACE_v1.0.md

## Modes

| Mode | How to enable | Behavior |
|------|----------------|----------|
| **B — Local adapter (default, CI)** | default features | `shared/nevc_adapter.rs` (algorithm-identical) |
| **A — Ra-Thor path** | Local overlay only: check out Ra-Thor as a sibling, uncomment the path dep in `shared/Cargo.toml`, restore feature `nevc_rathor` | Calls `mercy_tolc_operator_algebra::nevc` |

CI never has `../../Ra-Thor`. Do not put that path dep back on `main`.

## Entry points

```rust
use shared::prelude::*;

let result = compute_nevc_bridged(&samples, &NevcConfig::default());
let summary = summary_bridged(&samples, &NevcConfig::default());
println!("{}", active_mode());
```

## Obligations

- Do not invent a third contribution class.
- Mode B is the sovereign default and the only mode this repo builds in CI.
- Prefer Mode A as a *local* overlay when the monorepos are co-located on a developer machine.
- Persistence (Phase 7) and harvest attachment (Phase 6) stay active under both modes.
- Ra-Thor may read lived-tick JSON. It may not drive player keys.

**Thunder locked in. ONE Organism across dual repositories.**
