# Simulation Performance Profiling & Flame Graph Analysis

This document explains how to generate and analyze flame graphs for the Powrush-MMO simulation crate, with focus on the `SovereignSimulationOrchestrator` and resonance systems.

## Quick Start

### 1. Install cargo-flamegraph

```bash
cargo install flamegraph
```

### 2. Generate a Flame Graph

```bash
# Profile the orchestrator benchmark
cargo flamegraph --bench orchestrator_bench -- --bench

# Profile resonance benchmarks
cargo flamegraph --bench resonance_decay_recovery_bench -- --bench

# Profile with more samples for better accuracy
cargo flamegraph --bench orchestrator_bench -- -F 10000 --bench
```

On Linux you may need:
```bash
sudo sysctl kernel.perf_event_paranoid=1
```

## What to Look For in Flame Graphs

### Expected Hotspots (as of June 2026)

Based on code structure, here is what flame graphs are likely to show:

| Area                        | Expected Width in Flame Graph      | Notes |
|----------------------------|------------------------------------|-------|
| `archetype_system.update`  | Medium to Wide                     | Dynamic archetype evolution is one of the heavier CPU components |
| `economic_layer.batch_update` | Often the widest                 | Contains GPU work + RBE calculations. Watch for GPU sync stalls |
| `MercyGate` validation     | Narrow                             | Should be very cheap |
| `run_tick` overhead        | Medium                             | Glue code + timing instrumentation |
| `SimulatedPlayer` methods  | Very narrow                        | Trivial math — should barely appear |

### Key Things to Investigate

1. **Wide flat areas** at the bottom → Indicates many small functions being called (possible inlining issue or many small updates).
2. **Deep stacks in `economic_layer`** → Often points to GPU command submission or data transfer between CPU/GPU.
3. **Repeated calls to the same function** across many ticks → Good candidate for optimization or caching.
4. **Time spent in `wgpu`** or driver code → GPU synchronization / memory transfer bottleneck.

## Recommended Workflow

1. Run `cargo flamegraph` on `orchestrator_bench`.
2. Open the generated `flamegraph.svg`.
3. Zoom into the widest sections.
4. Compare before/after changes to subsystems.
5. Use `--reverse` flag if you want to see functions sorted by total time.

```bash
cargo flamegraph --bench orchestrator_bench --reverse -- --bench
```

## Current Profiling Status

- Resonance simulation: Extremely lightweight (nanosecond range)
- Orchestrator tick: Dominated by Archetype + Economic Layer
- Profiling instrumentation added to `profile_run_for_duration`

## Next Steps

- Add `tracing` spans around major subsystems for correlated flame graphs + logs.
- Create automated benchmark + flamegraph CI job.
- Profile under different world sizes and time accelerations.

**Thunder locked in. Profile with purpose.**
