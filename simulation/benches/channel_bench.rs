use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct CouncilEvent {
    id: u64,
    proposer: u64,
    mercy_factor: f32,
}

// ============================================================
// p50 / p99 LATENCY PERCENTILES
// ============================================================

fn bench_latency_percentiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_p50_p99");
    group.sample_size(50);

    for (name, bench_fn) in [
        ("flume", bench_flume_latency as fn(&mut Criterion)),
        ("kanal", bench_kanal_latency as fn(&mut Criterion)),
        ("crossbeam", bench_crossbeam_latency as fn(&mut Criterion)),
        ("tokio_mpsc", bench_tokio_latency as fn(&mut Criterion)),
    ] {
        group.bench_function(BenchmarkId::new("p50_p99", name), |b| {
            bench_fn(b);
        });
    }

    group.finish();
}

// Individual latency benchmark functions for cleaner Criterion comparison
fn bench_flume_latency(b: &mut criterion::Bencher) {
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
        let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
        black_box((p50, p99))
    });
}

fn bench_kanal_latency(b: &mut criterion::Bencher) {
    b.iter(|| { /* same pattern as above */ });
}

fn bench_crossbeam_latency(b: &mut criterion::Bencher) {
    b.iter(|| { /* same pattern as above */ });
}

fn bench_tokio_latency(b: &mut criterion::Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| {
        rt.block_on(async {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<u64>(1024);
            let handle = tokio::spawn(async move {
                let mut count = 0u64;
                while let Some(_) = rx.recv().await { count += 1; if count >= 10_000 { break; } }
            });
            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);
            for i in 0..10_000 {
                let start = Instant::now();
                tx.send(i).await.unwrap();
                latencies.push(start.elapsed());
            }
            drop(tx);
            handle.await.unwrap();
            latencies.sort_unstable();
            let p50 = latencies[latencies.len() / 2];
            let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
            black_box((p50, p99))
        })
    });
}

// ============================================================
// MULTI-PRODUCER LATENCY
// ============================================================

fn bench_multi_producer_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_producer_latency_p50_p99");
    group.sample_size(30);

    group.bench_function("flume_4p", |b| { /* existing implementation */ });
    group.bench_function("kanal_4p", |b| { /* existing implementation */ });
    group.bench_function("crossbeam_4p", |b| { /* existing implementation */ });

    group.finish();
}

criterion_group!(
    benches,
    bench_latency_percentiles,
    bench_multi_producer_latency
);
criterion_main!(benches);
