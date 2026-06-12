use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Instant;
use simulation::resonance_decay_recovery_sim::SimulatedPlayer;

/// Detailed profiling of individual operations with timing breakdown
fn profile_individual_operations() {
    println!("\n=== DETAILED BOTTLENECK PROFILING ===\n");

    let iterations = 100_000;

    // Profile apply_selfish_penalty
    let start = Instant::now();
    let mut player = SimulatedPlayer::new(4.0);
    for _ in 0..iterations {
        player.apply_selfish_penalty(black_box(1.0));
    }
    let duration = start.elapsed();
    println!("apply_selfish_penalty x{}: {:?} (avg: {:?})",
        iterations, duration, duration / iterations);

    // Profile apply_epiphany
    let start = Instant::now();
    let mut player = SimulatedPlayer::new(3.5);
    for _ in 0..iterations {
        player.apply_epiphany(black_box(2.2), black_box(0.85));
    }
    let duration = start.elapsed();
    println!("apply_epiphany x{}: {:?} (avg: {:?})",
        iterations, duration, duration / iterations);

    // Profile apply_council_bloom
    let start = Instant::now();
    let mut player = SimulatedPlayer::new(3.0);
    for _ in 0..iterations {
        player.apply_council_bloom(black_box(0.82));
    }
    let duration = start.elapsed();
    println!("apply_council_bloom x{}: {:?} (avg: {:?})",
        iterations, duration, duration / iterations);

    println!("\n=== ANALYSIS ===");
    println!("- All operations are extremely lightweight (nanosecond range).");
    println!("- No significant allocations or complex logic in resonance simulation.");
    println!("- Bottlenecks in broader system likely elsewhere (archetype, economy, GPU sync, world state).\n");
}

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

// Run detailed profiling when executing the benchmark binary directly
fn main() {
    // Run Criterion benchmarks
    criterion_main!();

    // Also run detailed bottleneck profiling
    profile_individual_operations();
}
