use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::thread;
use std::time::{Duration, Instant};

// Simulate a small Council-like event
#[derive(Clone, Copy)]
struct CouncilEvent {
    id: u64,
    proposer: u64,
    mercy_factor: f32,
}

fn format_duration(d: Duration) -> String {
    if d.as_nanos() < 1_000 {
        format!("{:.1}ns", d.as_nanos())
    } else if d.as_micros() < 1_000 {
        format!("{:.2}µs", d.as_micros() as f64 / 1000.0)
    } else {
        format!("{:.2}ms", d.as_secs_f64() * 1000.0)
    }
}

// ============================================================
// SINGLE PRODUCER - BOUNDED - THROUGHPUT
// ============================================================

fn bench_flume(c: &mut Criterion) { /* ... existing ... */ }

fn bench_kanal(c: &mut Criterion) { /* ... existing ... */ }

fn bench_crossbeam(c: &mut Criterion) { /* ... existing ... */ }

fn bench_tokio_mpsc(c: &mut Criterion) { /* ... existing ... */ }

// ============================================================
// MULTI-PRODUCER + UNBOUNDED + BASIC LATENCY (kept for compatibility)
// ============================================================

fn bench_multi_producer(c: &mut Criterion) { /* ... existing ... */ }

fn bench_unbounded(c: &mut Criterion) { /* ... existing ... */ }

// ============================================================
// ADVANCED LATENCY: p50 / p99 Percentiles
// ============================================================

fn bench_latency_percentiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_p50_p99");

    // Flume
    group.bench_function("flume", |b| {
        b.iter(|| {
            let (tx, rx) = flume::bounded::<u64>(1024);
            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);

            for i in 0..10_000 {
                let start = Instant::now();
                tx.send(i).unwrap();
                let _ = rx.recv().unwrap();
                latencies.push(start.elapsed());
            }

            latencies.sort_unstable();
            let p50 = latencies[latencies.len() / 2];
            let p99_idx = (latencies.len() as f64 * 0.99) as usize;
            let p99 = latencies[p99_idx.min(latencies.len() - 1)];

            black_box((p50, p99))
        });
    });

    // Kanal
    group.bench_function("kanal", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::bounded::<u64>(1024);
            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);

            for i in 0..10_000 {
                let start = Instant::now();
                tx.send(i).unwrap();
                let _ = rx.recv().unwrap();
                latencies.push(start.elapsed());
            }

            latencies.sort_unstable();
            let p50 = latencies[latencies.len() / 2];
            let p99_idx = (latencies.len() as f64 * 0.99) as usize;
            let p99 = latencies[p99_idx.min(latencies.len() - 1)];

            black_box((p50, p99))
        });
    });

    // Crossbeam-channel
    group.bench_function("crossbeam", |b| {
        b.iter(|| {
            let (tx, rx) = crossbeam_channel::bounded::<u64>(1024);
            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);

            for i in 0..10_000 {
                let start = Instant::now();
                tx.send(i).unwrap();
                let _ = rx.recv().unwrap();
                latencies.push(start.elapsed());
            }

            latencies.sort_unstable();
            let p50 = latencies[latencies.len() / 2];
            let p99_idx = (latencies.len() as f64 * 0.99) as usize;
            let p99 = latencies[p99_idx.min(latencies.len() - 1)];

            black_box((p50, p99))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_flume,
    bench_kanal,
    bench_crossbeam,
    bench_tokio_mpsc,
    bench_multi_producer,
    bench_unbounded,
    bench_latency_percentiles
);
criterion_main!(benches);
