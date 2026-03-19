# Technical Memory Architecture – C4 Model & Crash Recovery Focus

This document provides a clean, metaphor-free mapping of the Consortium's memory systems using the **C4 model** (Context → Containers → Components).  
It focuses especially on **crash recovery** and the **Ozymandias checkpointer**, as these are currently the highest-risk areas.

## C4 Level 1 – System Context

```mermaid
flowchart LR
    User[Human User via TUI] -->|Input / Ctrl+C| Engine[Consortium Engine\n(tokio runtime + ratatui)]
    Engine -->|Tool calls / Market data| External[External Tools\nSandbox / Market Streams]
    Engine -->|Persistent episodic memory| SurrealDB[SurrealDB Embedded\n(kv-surrealkv)]
    Engine -->|Short-term state| MemoryFiles[JSON Files in motor_cortex/]
```

## C4 Level 2 – Containers

- **Engine Core** — tokio::main loop, message handling, tool execution, TUI bridge (crossbeam-channel)
- **Ozymandias Checkpointer** — Background tokio task for periodic market state persistence
- **Working Memory Writer** — Synchronous per-message/tool JSON writer
- **Crash Recovery Pipeline** — Panic hook + bash wrapper + Phase 24 reconstitution
- **SurrealDB Embedded** — Long-term graph, vectors, timelines (not focus of this doc)

## C4 Level 3 – Components (Crash Recovery & Ozymandias Focus)

### Working Memory (Instant Write)
- Path: `./motor_cortex/working_memory.json` (relative to engine binary)
- Struct: `WorkingMemory { messages: Vec<Message> }`
- Write pattern: Synchronous, after every user message or tool result
- Size: Small → low corruption risk, but still benefits from fsync if ultra-durable needed

### Ozymandias State (Periodic Atomic WAL Checkpoint)
- Path: `./motor_cortex/ozymandias_state.json`
- Struct:

```rust
#[derive(Serialize, Deserialize)]
pub struct FinTraceKnowledgeBase {
    behavioral_indexes: HashMap<String, Vec<MarketBehavioralFeature>>,
    quote_buffer: HashMap<String, VecDeque<Quote>>,
    trade_buffer: HashMap<String, VecDeque<Trade>>,
    max_history_window_secs: u64,
}
```

- Current pain: `std::fs::write` mid-checkpoint → panic → corrupted file → lost market context
- Proposed fix: Atomic temp write + rename

```rust
async fn checkpoint_ozymandias(state: &Arc<Mutex<FinTraceKnowledgeBase>>) -> Result<()> {
    let guard = state.lock().await;
    let serialized = serde_json::to_string_pretty(&*guard)?;
    
    let temp_path = Path::new("./motor_cortex/ozymandias_state.tmp.json");
    let final_path = Path::new("./motor_cortex/ozymandias_state.json");
    
    tokio::fs::write(temp_path, serialized).await?;
    temp_path.sync_all()?;               // durability
    std::fs::rename(temp_path, final_path)?; // atomic
    
    Ok(())
}
```

### Graceful Shutdown & Cancellation Safety

Add signal handling + shutdown channel:

```rust
let (shutdown_tx, shutdown_rx) = crossbeam_channel::bounded::<()>(1);

// In checkpointer task
tokio::spawn(async move {
    let mut interval = interval(Duration::from_secs(60));
    loop {
        tokio::select! {
            _ = interval.tick() => { checkpoint_ozymandias(...).await; }
            _ = shutdown_rx.recv() => {
                checkpoint_ozymandias(...).await; // final save
                break;
            }
        }
    }
});

// Graceful Ctrl+C handler
tokio::spawn(async move {
    if ctrl_c().await.is_ok() {
        let _ = shutdown_tx.send(());
        tokio::time::sleep(Duration::from_secs(5)).await; // grace period
        std::process::exit(0);
    }
});
```

### Panic Hook & Reconstitution (Phase 24)

```rust
fn install_panic_hook(shutdown_tx: Sender<()>) {
    panic::set_hook(Box::new(move |info| {
        let mut f = OpenOptions::new().append(true).create(true).open("crash_report.txt").unwrap();
        writeln!(f, "Panic at {:?}: {:?}", Utc::now(), info).unwrap();
        let bt = Backtrace::new();
        writeln!(f, "{:?}", bt).unwrap();
        let _ = shutdown_tx.send(()); // attempt emergency checkpoint
    }));
}
```

On restart (bash wrapper):
- Check & load `ozymandias_state.json` if valid
- Deserialize → rehydrate `FinTraceKnowledgeBase`
- Fallback: minimal cold start or last SurrealDB snapshot

## Future Extensions

- **Oblivion Protocol** — tiktoken-rs based eviction reaching 80% context window → drop oldest/low-relevance messages (working_memory.json)
- **Hopfield Convergence Array** — mlx-rs Metal loop for code/logic pattern healing (motor cortex memory)
- **SurrealDB Pruning** — Relevance-based node deletion when entropy > threshold (temporal.rs)

## Recommendations

- Add BLAKE3 checksum suffix to JSON files for load validation
- Versioned backups: `ozymandias_state.json.bak` after successful write
- Metrics: Prometheus counters for checkpoint success, reconstitution count, corruption events
