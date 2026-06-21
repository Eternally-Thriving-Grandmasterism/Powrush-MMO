use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::thread;

// Simulate a small Council-like event
#[derive(Clone, Copy)]
struct CouncilEvent {
    id: u64,
    proposer: u64,
    mercy_factor: f32,
}

fn bench_flume(c: &mut Criterion) {
    let mut group = c.benchmark_group("flume");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("bounded_100k_msgs", |b| {
        b.iter(|| {
            let (tx, rx) = flume::bounded::<CouncilEvent>(1024);

            let handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_msg) = rx.recv() {
                    count += 1;
                    if count >= 100_000 {
                        break;
                    }
                }
                count
            });

            for i in 0..100_000 {
                let event = CouncilEvent {
                    id: i,
                    proposer: i % 128,
                    mercy_factor: 0.72,
                };
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

    group.bench_function("bounded_100k_msgs", |b| {
        b.iter(|| {
            let (tx, rx) = kanal::bounded::<CouncilEvent>(1024);

            let handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_msg) = rx.recv() {
                    count += 1;
                    if count >= 100_000 {
                        break;
                    }
                }
                count
            });

            for i in 0..100_000 {
                let event = CouncilEvent {
                    id: i,
                    proposer: i % 128,
                    mercy_factor: 0.72,
                };
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

    group.bench_function("bounded_100k_msgs", |b| {
        b.iter(|| {
            let (tx, rx) = crossbeam_channel::bounded::<CouncilEvent>(1024);

            let handle = thread::spawn(move || {
                let mut count = 0u64;
                while let Ok(_msg) = rx.recv() {
                    count += 1;
                    if count >= 100_000 {
                        break;
                    }
                }
                count
            });

            for i in 0..100_000 {
                let event = CouncilEvent {
                    id: i,
                    proposer: i % 128,
                    mercy_factor: 0.72,
                };
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

    group.bench_function("bounded_100k_msgs", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        b.iter(|| {
            rt.block_on(async {
                let (tx, mut rx) = tokio::sync::mpsc::channel::<CouncilEvent>(1024);

                let handle = tokio::spawn(async move {
                    let mut count = 0u64;
                    while let Some(_msg) = rx.recv().await {
                        count += 1;
                        if count >= 100_000 {
                            break;
                        }
                    }
                    count
                });

                for i in 0..100_000 {
                    let event = CouncilEvent {
                        id: i,
                        proposer: i % 128,
                        mercy_factor: 0.72,
                    };
                    tx.send(black_box(event)).await.unwrap();
                }

                drop(tx);
                handle.await.unwrap()
            })
        });
    });

    group.finish();
}

criterion_group!(benches, bench_flume, bench_kanal, bench_crossbeam, bench_tokio_mpsc);
criterion_main!(benches);
