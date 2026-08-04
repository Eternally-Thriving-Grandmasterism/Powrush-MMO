# NEVC Dual-Repo Binding (Powrush-MMO)

**Phase 8 consumer-side record**  
**Contact:** info@Rathor.ai  

This repository consumes Net Eternal Valence Contribution under the published interface:

**Ra-Thor contract:**  
https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/NEVC_DUAL_REPO_INTERFACE_v1.0.md

## Modes

| Mode | How to enable | Behavior |
|------|----------------|----------|
| **A — Ra-Thor path** | `cargo build -p shared --features nevc_rathor` (Ra-Thor tree at `../../Ra-Thor`) | Calls `mercy_tolc_operator_algebra::nevc` |
| **B — Local adapter** | default | Uses `shared/nevc_adapter.rs` (algorithm-identical) |

## Entry points

```rust
use shared::prelude::*;

let result = compute_nevc_bridged(&samples, &NevcConfig::default());
let summary = summary_bridged(&samples, &NevcConfig::default());
println!("{}", active_mode());
```

## Obligations

- Do not invent a third contribution class.
- Prefer Mode A when monorepos are co-located.
- Mode B remains the sovereign offline default.
- Persistence (Phase 7) and harvest attachment (Phase 6) stay active under both modes.

**Thunder locked in. ONE Organism across dual repositories.**
