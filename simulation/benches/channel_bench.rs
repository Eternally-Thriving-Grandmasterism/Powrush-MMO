use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::thread;
use std::time::Instant;

// Simulate a small Council-like event
#[derive(Clone, Copy)]
struct CouncilEvent {
    id: u64,
    proposer: u64,
    mercy_factor: f32,
}

// ============================================================
// SINGLE PRODUCER - BOUNDED - THROUGHPUT
// ============================================================

fn bench_flume(c: &mut Criterion) {
    let mut group = c.benchmark_group("flume");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("sp_bounded_100k", |b| {
        b.iter(|| {
            let (tx, rx) = flume::bounded::<CouncilEvent>(1024);
            let handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_msg) = rx.recv() {
                    count += 1;
                    if count >= 100_000 { break; }
                }
                count
            });
            for i in 0..100_000 {
                let event = CouncilEvent { id: i, proposer: i % 128, mercy_factor: 0.72 };
                tx.send(black_box(event)).unwrap();
            }
            drop(tx);
            handle.join().unwrap()
        });
    });
    group.finish();
}

fn bench_kanal(c: &mut Criterion) {
    let mut group = c.benchmark_group("kanal");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("sp_bounded_100k", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::bounded::<CouncilEvent>(1024);
            let handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_msg) = rx.recv() {
                    count += 1;
                    if count >= 100_000 { break; }
                }
                count
            });
            for i in 0..100_000 {
                let event = CouncilEvent { id: i, proposer: i % 128, mercy_factor: 0.72 };
                tx.send(black_box(event)).unwrap();
            }
            drop(tx);
            handle.join().unwrap()
        });
    });
    group.finish();
}

fn bench_crossbeam(c: &mut Criterion) {
    let mut group = c.benchmark_group("crossbeam-channel");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("sp_bounded_100k", |b| {
        b.iter(|| {
            let (tx, rx) = crossbeam_channel::bounded::<CouncilEvent>(1024);
            let handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_msg) = rx.recv() {
                    count += 1;
                    if count >= 100_000 { break; }
                }
                count
            });
            for i in 0..100_000 {
                let event = CouncilEvent { id: i, proposer: i % 128, mercy_factor: 0.72 };
                tx.send(black_box(event)).unwrap();
            }
            drop(tx);
            handle.join().unwrap()
        });
    });
    group.finish();
}

fn bench_tokio_mpsc(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokio::sync::mpsc");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("sp_bounded_100k", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let (tx, mut rx) = tokio::sync::mpsc::channel::<CouncilEvent>(1024);
                let handle = tokio::spawn(async move {
                    let mut count = 0u64;
                    while let Some(_msg) = rx.recv().await {
                        count += 1;
                        if count >= 100_000 { break; }
                    }
                    count
                });
                for i in 0..100_000 {
                    let event = CouncilEvent { id: i, proposer: i % 128, mercy_factor: 0.72 };
                    tx.send(black_box(event)).await.unwrap();
                }
                drop(tx);
                handle.await.unwrap()
            })
        });
    });
    group.finish();
}

// ============================================================
// MULTI-PRODUCER (4 producers) - BOUNDED
// ============================================================

fn bench_multi_producer(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_producer_bounded");
    group.throughput(Throughput::Elements(100_000));

    // Flume multi-producer
    group.bench_function("flume_4p", |b| {
        b.iter(|| {
            let (tx, rx) = flume::bounded::<CouncilEvent>(1024);
            let mut handles = vec![];
            for p in 0..4 {
                let tx = tx.clone();
                handles.push(thread::spawn(move || {
                    for i in 0..25_000 {
                        let event = CouncilEvent { id: (p * 25_000 + i) as u64, proposer: p as u64, mercy_factor: 0.72 };
                        tx.send(black_box(event)).unwrap();
                    }
                }));
            }
            let recv_handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_) = rx.recv() { count += 1; if count >= 100_000 { break; } }
                count
            });
            for h in handles { h.join().unwrap(); }
            drop(tx);
            recv_handle.join().unwrap()
        });
    });

    // Kanal multi-producer
    group.bench_function("kanal_4p", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::bounded::<CouncilEvent>(1024);
            let mut handles = vec![];
            for p in 0..4 {
                let tx = tx.clone();
                handles.push(thread::spawn(move || {
                    for i in 0..25_000 {
                        let event = CouncilEvent { id: (p * 25_000 + i) as u64, proposer: p as u64, mercy_factor: 0.72 };
                        tx.send(black_box(event)).unwrap();
                    }
                }));
            }
            let recv_handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_) = rx.recv() { count += 1; if count >= 100_000 { break; } }
                count
            });
            for h in handles { h.join().unwrap(); }
            drop(tx);
            recv_handle.join().unwrap()
        });
    });

    group.finish();
}

// ============================================================
// UNBOUNDED CHANNELS
// ============================================================

fn bench_unbounded(c: &mut Criterion) {
    let mut group = c.benchmark_group("unbounded");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("flume_unbounded", |b| {
        b.iter(|| {
            let (tx, rx) = flume::unbounded::<CouncilEvent>();
            let handle = thread::spawn(move || {
                let mut count = 0; while let Ok(_) = rx.recv() { count += 1; if count >= 100_000 { break; } } count
            });
            for i in 0..100_000 {
                tx.send(CouncilEvent { id: i, proposer: i % 128, mercy_factor: 0.72 }).unwrap();
            }
            drop(tx); handle.join().unwrap()
        });
    });

    group.bench_function("kanal_unbounded", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::unbounded::<CouncilEvent>();
            let handle = thread::spawn(move || {
                let mut count = 0; while let Ok(_) = rx.recv() { count += 1; if count >= 100_000 { break; } } count
            });
            for i in 0..100_000 {
                tx.send(CouncilEvent { id: i, proposer: i % 128, mercy_factor: 0.72 }).unwrap();
            }
            drop(tx); handle.join().unwrap()
        });
    });

    group.finish();
}

// ============================================================
// LATENCY (simple average round-trip style)
// ============================================================

fn bench_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency");

    group.bench_function("flume_latency_10k", |b| {
        b.iter(|| {
            let (tx, rx) = flume::bounded::<u64>(1024);
            let start = Instant::now();
            for i in 0..10_000 {
                tx.send(i).unwrap();
                let _ = rx.recv().unwrap();
            }
            black_box(start.elapsed())
        });
    });

    group.bench_function("kanal_latency_10k", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::bounded::<u64>(1024);
            let start = Instant::now();
            for i in 0..10_000 {
                tx.send(i).unwrap();
                let _ = rx.recv().unwrap();
            }
            black_box(start.elapsed())
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
    bench_latency
);
criterion_main!(benches);
