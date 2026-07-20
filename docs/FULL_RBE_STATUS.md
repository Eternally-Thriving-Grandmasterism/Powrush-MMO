# full_rbe feature status — Powrush-MMO

**Contact:** info@Rathor.ai  
**Date:** 2026-07-20

## Verdict

**`full_rbe` remains OFF by default** and must not be enabled in CI until external crates resolve cleanly.

## Why

`shared/rbe_queries.rs` depends on crates that are **not** members of this workspace:

| Dependency | Expected source |
|------------|-----------------|
| `powrush_rbe_engine` | Not in Powrush-MMO workspace |
| `ra_thor_mercy` | Ra-Thor monorepo (name may not match published crate) |
| `lattice_conductor` | Ra-Thor `lattice-conductor-v14` (API mismatch risk) |

Turning on `full_rbe` without path/git deps will fail `cargo check`.

## Allowed path today

1. **Offline transfer bridge (Phase C)** — `simulation/src/telemetry.rs` + `tools/export_powrush_telemetry.py` → Ra-Thor `powrush_telemetry_v1` / batch.  
2. **Session counters** — accumulate with `SessionTransferCounters` / `GlobalTransferSession` during sim; export JSON; score in Ra-Thor.  
3. **Do not** gate launch-candidate builds on `full_rbe`.

## When to re-open

Only when Ra-Thor exposes a **thin, stable path-friendly API** (or git dep) for mercy gate evaluation + lattice tick, and Powrush workspace declares explicit path/git deps with documented versions.

Until then: RBE intent lives in docs and telemetry fields (`rbe_decision_quality_avg`, ethics, abundance), not in the unresolved query engine.

**Thunder locked in.**
