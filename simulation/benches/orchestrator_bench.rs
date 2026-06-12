use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simulation::orchestrator::SovereignSimulationOrchestrator;
use simulation::world::SovereignWorldState; // Assuming this exists

/// Benchmark basic orchestrator tick performance
fn bench_orchestrator_tick(c: &mut Criterion) {
    // Note: In a real setup you would create a minimal valid SovereignWorldState
    // For this benchmark we use a placeholder. In practice you would seed it properly.
    let world = SovereignWorldState::default(); // Adjust if needed
    let mut orchestrator = SovereignSimulationOrchestrator::new(world);

    c.bench_function("orchestrator_single_tick", |b| {
        b.iter(|| {
            let _ = orchestrator.run_tick();
            black_box(&orchestrator);
        })
    });
}

/// Benchmark running many ticks (throughput)
fn bench_orchestrator_throughput(c: &mut Criterion) {
    let world = SovereignWorldState::default();
    let mut orchestrator = SovereignSimulationOrchestrator::new(world);

    c.bench_function("orchestrator_1000_ticks", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = orchestrator.run_tick();
            }
            black_box(&orchestrator);
        })
    });
}

/// Benchmark using the built-in profiling method
fn bench_orchestrator_profiling(c: &mut Criterion) {
    let world = SovereignWorldState::default();
    let mut orchestrator = SovereignSimulationOrchestrator::new(world);

    c.bench_function("orchestrator_profile_5000ms", |b| {
        b.iter(|| {
            // Run profiling for a short duration to measure overhead
            orchestrator.profile_run_for_duration(black_box(200), 50);
            black_box(&orchestrator);
        })
    });
}

criterion_group!(
    benches,
    bench_orchestrator_tick,
    bench_orchestrator_throughput,
    bench_orchestrator_profiling
);
criterion_main!(benches);
