---
name: wasm_sandbox
description: Safely compile and execute highly untrusted, externally scraped, or unverified hallucinated logic arrays inside the Consortium engine using WebAssembly boundaries.
---

# 🛡️ Sandbox Skill (`wasm_sandbox`)

The Consortium Engine incorporates a WASI-preview1 compliant sandbox powered by Wasmtime (`consortium_engine/src/sandbox.rs`). This skill teaches you (the Agent) how and when to leverage it.

## When to use this skill

- When you scrape raw code from the web or an external API and need to execute it to test its outputs without verifying its total safety.
- When generating complex, potentially recursive logic that risks triggering a host-side infinite loop.
- When you need to deliberately isolate a process.

## How it works physically

The `SafeHands` engine enforces severe physical limitations on any code it executes:
1. **Payload Limit:** The `.wasm` boundary is hard-stopped at `512 KiB` to prevent DoS via payload memory hogging.
2. **Deterministic Fuel limit:** The Wasmtime configuration explicitly forces fuel consumption. Processes hit a mathematical hard-stop at `1,000,000` instructions, roughly enough for a short scraping cycle. Infinite loops literally run out of thermodynamic fuel and die structurally, rather than hanging your agent loop.
3. **Execution Receipts:** Every payload executed returns a physical `ExecutionReceipt` (Hash, PID, Duration, Success/Fail boolean, and textual Output). This receipt is injected mathematically into the Temporal `Hippocampus` Graph.

## Agent Directives

If you determine a payload is untrusted, **DO NOT** execute it natively via `run_command` in `bash`.
You must instead:
1. Compile the script to a `.wasm` binary target (e.g. `rustc --target wasm32-wasi payload.rs`).
2. Pass the resulting `.wasm` path to the Consortium engine's Sandbox channel (via RPC or CLI pipe) for execution.
3. Await the `ExecutionReceipt` return.
4. If `Success == false`, adapt your payload and try again, knowing the host machine was computationally protected.
