#!/bin/bash
#
# Powrush-MMO Simulation Profiling Helper
# Provides convenient commands for running benchmarks and generating flame graphs.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIMULATION_DIR="$(dirname "$SCRIPT_DIR")"

cd "$SIMULATION_DIR"

case "$1" in
  bench)
    echo "Running all simulation benchmarks..."
    cargo bench
    ;;
  bench-orchestrator)
    echo "Running orchestrator benchmarks..."
    cargo bench --bench orchestrator_bench
    ;;
  bench-resonance)
    echo "Running resonance decay/recovery benchmarks..."
    cargo bench --bench resonance_decay_recovery_bench
    ;;
  flamegraph-orchestrator)
    echo "Generating flamegraph for orchestrator..."
    cargo flamegraph --bench orchestrator_bench -- --bench
    ;;
  flamegraph-resonance)
    echo "Generating flamegraph for resonance simulation..."
    cargo flamegraph --bench resonance_decay_recovery_bench -- --bench
    ;;
  profile)
    echo "Running orchestrator with built-in profiling..."
    # This would typically be called from within a test or example
    echo "Use 'cargo test' or integrate profile_run_for_duration in your code."
    ;;
  *)
    echo "Powrush-MMO Simulation Profiling Helper"
    echo ""
    echo "Usage: $0 {bench|bench-orchestrator|bench-resonance|flamegraph-orchestrator|flamegraph-resonance}"
    echo ""
    echo "  bench                  - Run all Criterion benchmarks"
    echo "  bench-orchestrator     - Run orchestrator performance benchmarks"
    echo "  bench-resonance        - Run resonance decay/recovery benchmarks"
    echo "  flamegraph-orchestrator- Generate flamegraph for orchestrator"
    echo "  flamegraph-resonance   - Generate flamegraph for resonance sim"
    exit 1
    ;;
esac
