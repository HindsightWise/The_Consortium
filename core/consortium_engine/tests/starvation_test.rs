use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};
use tokio::task;

// Simulates the synchronous blocking work
fn blocking_hash_work() {
    let mut hasher = DefaultHasher::new();
    for i in 0..10_000 {
        "thermodynamic_barnacle_grind".hash(&mut hasher);
        i.hash(&mut hasher);
    }
    let _hash_val = hasher.finish();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_executor_starvation() {
    println!("--- Testing Executor Starvation ---");
    // 1. Measure latency of a simple async task when NO blocking work is running
    let start = Instant::now();
    let pure_async = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
    });
    pure_async.await.unwrap();
    let baseline_latency = start.elapsed();
    println!("Baseline async sleep latency: {:?}", baseline_latency);

    // 2. Measure latency when there is synchronous blocking work
    // We launch a background task that does the blocking hash work
    let blocking_task = tokio::spawn(async {
        // Run it synchronously - this blocks the executor thread
        for _ in 0..100 {
            blocking_hash_work();
        }
    });

    // Slight yield to ensure the blocking task has started
    tokio::time::sleep(Duration::from_millis(1)).await;

    let start = Instant::now();
    let starved_async = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
    });
    starved_async.await.unwrap();
    let starved_latency = start.elapsed();
    println!(
        "Starved async sleep latency (sync blocking): {:?}",
        starved_latency
    );

    blocking_task.await.unwrap();

    // 3. Measure latency when we use `spawn_blocking`
    let spawn_blocking_task = tokio::spawn(async {
        tokio::task::spawn_blocking(|| {
            for _ in 0..100 {
                blocking_hash_work();
            }
        })
        .await
        .unwrap();
    });

    // Slight yield to ensure the blocking task has started
    tokio::time::sleep(Duration::from_millis(1)).await;

    let start = Instant::now();
    let offloaded_async = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
    });
    offloaded_async.await.unwrap();
    let offloaded_latency = start.elapsed();
    println!(
        "Offloaded async sleep latency (spawn_blocking): {:?}",
        offloaded_latency
    );

    spawn_blocking_task.await.unwrap();
}
