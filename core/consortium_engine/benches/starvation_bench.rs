use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tokio::runtime::Runtime;

// Simulates the synchronous blocking work
fn blocking_hash_work() {
    let mut hasher = DefaultHasher::new();
    for i in 0..10_000 {
        "thermodynamic_barnacle_grind".hash(&mut hasher);
        i.hash(&mut hasher);
    }
    let _hash_val = hasher.finish();
}

fn bench_sync_blocking(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("sync_blocking_hashing", |b| {
        b.to_async(&rt).iter(|| async {
            blocking_hash_work();
        });
    });
}

fn bench_spawn_blocking(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("spawn_blocking_hashing", |b| {
        b.to_async(&rt).iter(|| async {
            tokio::task::spawn_blocking(|| {
                blocking_hash_work();
            })
            .await
            .unwrap();
        });
    });
}

criterion_group!(benches, bench_sync_blocking, bench_spawn_blocking);
criterion_main!(benches);
