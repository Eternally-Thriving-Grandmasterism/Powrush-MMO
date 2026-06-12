use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simulation::resonance_decay_recovery_sim::SimulatedPlayer;

/// Benchmark individual operations
fn bench_individual_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("resonance_operations");

    group.bench_function("apply_selfish_penalty", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(4.0);
            player.apply_selfish_penalty(black_box(1.0));
            black_box(player);
        })
    });

    group.bench_function("apply_epiphany", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(3.5);
            player.apply_epiphany(black_box(2.2), black_box(0.85));
            black_box(player);
        })
    });

    group.bench_function("apply_council_bloom", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(3.0);
            player.apply_council_bloom(black_box(0.82));
            black_box(player);
        })
    });

    group.finish();
}

/// Benchmark full scenario runs
fn bench_full_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_scenarios");

    group.bench_function("scenario_low_starting_resonance", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(1.2);
            for _ in 0..5 {
                player.apply_epiphany(2.3, 0.88);
            }
            player.apply_council_bloom(0.85);
            black_box(player);
        })
    });

    group.bench_function("scenario_mixed_playstyle", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(3.8);
            player.apply_selfish_penalty(1.0);
            player.apply_selfish_penalty(0.8);
            for _ in 0..4 {
                player.apply_epiphany(2.0, 0.82);
            }
            player.apply_council_bloom(0.79);
            black_box(player);
        })
    });

    group.bench_function("scenario_long_term_trend", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(4.0);
            // Simulate 3 sessions
            for _ in 0..12 {
                player.apply_epiphany(2.1, 0.80);
            }
            player.apply_selfish_penalty(1.0);
            for _ in 0..8 {
                player.apply_epiphany(1.9, 0.78);
            }
            black_box(player);
        })
    });

    group.finish();
}

/// Benchmark many iterations (stress test)
fn bench_many_iterations(c: &mut Criterion) {
    c.bench_function("10000_mixed_operations", |b| {
        b.iter(|| {
            let mut player = SimulatedPlayer::new(3.5);
            for i in 0..10000 {
                if i % 7 == 0 {
                    player.apply_selfish_penalty(0.8);
                } else {
                    player.apply_epiphany(1.8, 0.75);
                }
            }
            black_box(player);
        })
    });
}

criterion_group!(
    benches,
    bench_individual_operations,
    bench_full_scenarios,
    bench_many_iterations
);
criterion_main!(benches);
