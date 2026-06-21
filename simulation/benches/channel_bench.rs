use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct CouncilEvent {
    id: u64,
    proposer: u64,
    mercy_factor: f32,
}

// ============================================================
// p50 / p99 LATENCY PERCENTILES (with higher sample size for stability)
// ============================================================

fn bench_latency_percentiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_p50_p99");
    group.sample_size(50); // Increase samples for more stable percentile estimates

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
            let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
            black_box((p50, p99))
        });
    });

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
            let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
            black_box((p50, p99))
        });
    });

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
            let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
            black_box((p50, p99))
        });
    });

    group.bench_function("tokio_mpsc", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let (tx, mut rx) = tokio::sync::mpsc::channel::<u64>(1024);
                let handle = tokio::spawn(async move {
                    let mut count = 0u64;
                    while let Some(_) = rx.recv().await {
                        count += 1;
                        if count >= 10_000 { break; }
                    }
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
    });

    group.finish();
}

// ============================================================
// MULTI-PRODUCER LATENCY (4 producers) with higher sample size
// ============================================================

fn bench_multi_producer_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_producer_latency_p50_p99");
    group.sample_size(30); // Slightly lower for expensive multi-producer tests

    group.bench_function("flume_4p", |b| {
        b.iter(|| {
            let (tx, rx) = flume::bounded::<u64>(1024);
            let mut producer_handles = vec![];

            for p in 0..4 {
                let tx = tx.clone();
                producer_handles.push(thread::spawn(move || {
                    for i in 0..2500 {
                        tx.send((p * 2500 + i) as u64).unwrap();
                    }
                }));
            }

            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);
            let start = Instant::now();
            let mut received = 0;
            while received < 10_000 {
                if rx.recv().is_ok() {
                    received += 1;
                    if received % 4 == 0 {
                        latencies.push(start.elapsed());
                    }
                }
            }
            for h in producer_handles { let _ = h.join(); }
            drop(tx);

            latencies.sort_unstable();
            let p50 = if !latencies.is_empty() { latencies[latencies.len() / 2] } else { Duration::ZERO };
            let p99 = if latencies.len() > 10 { latencies[(latencies.len() as f64 * 0.99) as usize] } else { Duration::ZERO };
            black_box((p50, p99))
        });
    });

    group.bench_function("kanal_4p", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::bounded::<u64>(1024);
            let mut producer_handles = vec![];

            for p in 0..4 {
                let tx = tx.clone();
                producer_handles.push(thread::spawn(move || {
                    for i in 0..2500 {
                        tx.send((p * 2500 + i) as u64).unwrap();
                    }
                }));
            }

            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);
            let start = Instant::now();
            let mut received = 0;
            while received < 10_000 {
                if rx.recv().is_ok() {
                    received += 1;
                    if received % 4 == 0 {
                        latencies.push(start.elapsed());
                    }
                }
            }
            for h in producer_handles { let _ = h.join(); }
            drop(tx);

            latencies.sort_unstable();
            let p50 = if !latencies.is_empty() { latencies[latencies.len() / 2] } else { Duration::ZERO };
            let p99 = if latencies.len() > 10 { latencies[(latencies.len() as f64 * 0.99) as usize] } else { Duration::ZERO };
            black_box((p50, p99))
        });
    });

    group.bench_function("crossbeam_4p", |b| {
        b.iter(|| {
            let (tx, rx) = crossbeam_channel::bounded::<u64>(1024);
            let mut producer_handles = vec![];

            for p in 0..4 {
                let tx = tx.clone();
                producer_handles.push(thread::spawn(move || {
                    for i in 0..2500 {
                        tx.send((p * 2500 + i) as u64).unwrap();
                    }
                }));
            }

            let mut latencies: Vec<Duration> = Vec::with_capacity(10_000);
            let start = Instant::now();
            let mut received = 0;
            while received < 10_000 {
                if rx.recv().is_ok() {
                    received += 1;
                    if received % 4 == 0 {
                        latencies.push(start.elapsed());
                    }
                }
            }
            for h in producer_handles { let _ = h.join(); }
            drop(tx);

            latencies.sort_unstable();
            let p50 = if !latencies.is_empty() { latencies[latencies.len() / 2] } else { Duration::ZERO };
            let p99 = if latencies.len() > 10 { latencies[(latencies.len() as f64 * 0.99) as usize] } else { Duration::ZERO };
            black_box((p50, p99))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_latency_percentiles,
    bench_multi_producer_latency
);
criterion_main!(benches);
