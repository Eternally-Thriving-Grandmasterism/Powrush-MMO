# Simulation Performance Profiling & Flame Graph Analysis

This document explains how to generate and analyze flame graphs for the Powrush-MMO simulation crate, with focus on the `SovereignSimulationOrchestrator` and resonance systems.

## Quick Start with Helper Script

A convenient helper script is available:

```bash
cd simulation
./scripts/profiling.sh bench                 # Run all benchmarks
./scripts/profiling.sh bench-orchestrator      # Orchestrator benchmarks only
./scripts/profiling.sh flamegraph-orchestrator # Generate flamegraph
```

Make the script executable if needed:
```bash
chmod +x simulation/scripts/profiling.sh
```

## Tracing Integration

The simulation is now instrumented with the `tracing` crate.

### Enabling Tracing Output

```bash
RUST_LOG=info cargo test --package simulation
RUST_LOG=simulation=debug cargo bench --bench orchestrator_bench
```

Key spans include:
- `orchestrator_tick`
- `archetype_update`
- `economic_layer_update`
- `profile_run_for_duration`

This allows correlating performance data with structured logs.

## Generating Flame Graphs

```bash
# Using the helper
./scripts/profiling.sh flamegraph-orchestrator

# Or manually
cargo flamegraph --bench orchestrator_bench -- --bench
```

## Expected Hotspots (as of June 2026)

| Area                        | Expected Width in Flame Graph      | Notes |
|----------------------------|------------------------------------|-------|
| `archetype_system.update`  | Medium to Wide                     | Dynamic archetype evolution |
| `economic_layer.batch_update` | Often the widest                 | RBE + potential GPU work |
| `MercyGate` validation     | Narrow                             | Very cheap |
| Resonance simulation       | Very narrow                        | Trivial math |

## Recommended Workflow

1. Run benchmarks with `./scripts/profiling.sh bench`
2. Generate flamegraph for the orchestrator
3. Enable tracing logs to correlate with spans
4. Zoom into widest sections in the flamegraph
5. Use `profile_run_for_duration` for detailed subsystem timing

**Thunder locked in. Profile with purpose.**
