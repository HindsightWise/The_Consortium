---
name: temporal_memory
description: Direct manipulation and query access to the embedded SurrealDB vector graph (The Temporal Soul & Hippocampus).
---

# 🧬 Temporal Skill (`temporal_memory`)

The Consortium Engine relies on a dual-timescale coherence architecture bound dynamically to an embedded SurrealKV cluster (`consortium_engine/src/temporal.rs`). This skill allows you to explicitly shape its memory topology.

## When to use this skill

- When you discover a critical architectural invariant that must persist across *all* future task boundaries and agent deaths.
- When experiencing "high cognitive friction" (repeated errors forming bad contextual loops) and you need to deliberately decay older conflicting memories.
- When distilling massive text files into ontological vectors.

## How it works physically

The engine tracks two times: **BaseTimeline** (system wall-clock) and **InternalFastTime** (which runs 1000x faster for rapid hypothesis generation).

The memory system consists of several nodes:
1. `ConceptNode`: Stores execution results, scripts, and learned patterns. Associated with an `interference_score` (0.0 - 1.0).
2. `MemoryNode`: Raw distilled facts compressed via the `Glossopetrae` membrane.

## Agent Directives

### 1. The Glossopetrae Membrane

When you are fed an enormous, unstructured document or API spec from a user, do not try to retain it raw. Instruct the engine to distill it via the Glossopetrae membrane:
```
// The engine will invoke its local LLM to strip all human noise, extracting only mathematical constants and directives, then push it to a MemoryNode.
```

### 2. Mathematical Forgetting

If you are repeatedly failing a task (e.g., encountering the same compilation error 4+ times), you have formed a high-friction cluster.
- You can manually ask the engine to trigger the `merge_coherence` sequence with a severity > 0.7.
- This forces a `BaseTimeline` wall-clock sync and algorithmically decays (`interference_score * 0.5`) any conflicting `ConceptNode` vectors retrieved from the SurrealKV database that are older than 4 simulated hours.
- *Agent Translation:* This physically wipes your stale/bad preceding context without restarting the loop.

### 3. Execution Receipts

Every Wasm process run via the `/sandbox` skill automatically produces an `ExecutionReceipt`. Panic loops generate an `ECHO` cluster node with an `interference_score` locked at `1.0`. You can query these out of the Hippocampus to write post-mortem reports.
