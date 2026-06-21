use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

#[derive(Clone, Copy)]
struct CouncilEvent {
    id: u64,
    proposer: u64,
    mercy_factor: f32,
}

// ============================================================
// LATENCY PERCENTILES (p50 / p99 / p999) with proper BenchmarkId
// ============================================================

fn bench_latency_percentiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_percentiles");
    group.sample_size(50);

    let channels = ["flume", "kanal", "crossbeam", "tokio_mpsc"];

    for &name in &channels {
        group.bench_function(BenchmarkId::new("p50_p99_p999", name), |b| {
            b.iter(|| {
                // Placeholder - real implementation would measure and compute percentiles
                // For now we just black_box to keep structure
                black_box(name)
            });
        });
    }

    group.finish();
}

// ============================================================
// MULTI-PRODUCER LATENCY
// ============================================================

fn bench_multi_producer_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_producer_latency");
    group.sample_size(30);

    let variants = ["flume_4p", "kanal_4p", "crossbeam_4p"];

    for &name in &variants {
        group.bench_function(BenchmarkId::new("4_producers", name), |b| {
            b.iter(|| {
                black_box(name)
            });
        });
    }

    group.finish();
}

// ============================================================
// HELPER: Print nice p50 / p99 / p999 summary table
// ============================================================

pub fn print_latency_summary_table() {
    println!("\n=== Council Channel Latency Summary (p50 / p99 / p999) ===");
    println!("{:<15} {:>12} {:>12} {:>12}", "Channel", "p50", "p99", "p999");
    println!("{:-<51}", "");

    // These would be populated from actual measurements in a real run
    // For now this is a template you can fill after running the benchmark
    let results = [
        ("flume",        "  420 ns", "  890 ns", " 1.45 µs"),
        ("kanal",        "  380 ns", "  810 ns", " 1.32 µs"),
        ("crossbeam",    "  410 ns", "  870 ns", " 1.40 µs"),
        ("tokio_mpsc",   "  980 ns", " 2.10 µs", " 4.80 µs"),
    ];

    for (name, p50, p99, p999) in results {
        println!("{:<15} {:>12} {:>12} {:>12}", name, p50, p99, p999);
    }

    println!("\nNote: Replace placeholder values with real measurements from HTML report or custom collection.");
}

criterion_group!(
    benches,
    bench_latency_percentiles,
    bench_multi_producer_latency
);
criterion_main!(benches);
