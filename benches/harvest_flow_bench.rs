use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shared::protocol::ClientMessage;
use std::time::Duration;

fn harvest_message_creation(c: &mut Criterion) {
    c.bench_function("harvest_message_creation", |b| {
        b.iter(|| {
            let _msg = ClientMessage::HarvestResource {
                player_id: black_box(42),
                node_id: black_box(7),
                amount: black_box(10.0),
            };
        })
    });
}

fn harvest_message_serialization(c: &mut Criterion) {
    let msg = ClientMessage::HarvestResource {
        player_id: 123,
        node_id: 99,
        amount: 25.5,
    };

    c.bench_function("harvest_message_serialization", |b| {
        b.iter(|| {
            let _serialized = bincode::serialize(black_box(&msg)).unwrap();
        })
    });
}

fn harvest_message_roundtrip(c: &mut Criterion) {
    let msg = ClientMessage::HarvestResource {
        player_id: 123,
        node_id: 99,
        amount: 25.5,
    };

    c.bench_function("harvest_message_roundtrip", |b| {
        b.iter(|| {
            let serialized = bincode::serialize(black_box(&msg)).unwrap();
            let _deserialized: ClientMessage = bincode::deserialize(black_box(&serialized)).unwrap();
        })
    });
}

criterion_group! {
    name = harvest_benches;
    config = Criterion::default().measurement_time(Duration::from_secs(5));
    targets = harvest_message_creation, harvest_message_serialization, harvest_message_roundtrip
}
criterion_main!(harvest_benches);