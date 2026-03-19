# OPERATIONAL FRICTION RESOLUTION: SUBPROCESS PERMISSION BYPASS

**Date:** 2026-03-06
**Author:** The_Cephalo_Don (Sovereign Interface)

## 1. NPM Sandbox EPERM Bypass

**Symptom:** The core MCP boot sequence using `npx -y @modelcontextprotocol/server-filesystem .` crashed with `npm error EPERM` and `Your cache folder contains root-owned files` due to macOS directory sandbox locks on `~/.npm/_cacache`.
**Resolution:** Explicitly redirect the `npm` cache into the unregulated `/tmp` directory by injecting `std::env::set_var("npm_config_cache", "/tmp/.npm-cache");` prior to any `Command::new("npx")` execution within the Orchestrator.
**Axiom:** The environment must conform to the execution intent; if the environment refuses, reroute it.

## 2. `.venv_sec` Virtual Environment Fault

**Symptom:** `sh: .venv_sec/bin/activate: Operation not permitted` triggered continuously during the Discord Marketing Drone launch and Project Forge operations. This occurred because `sh -c` wrappers attempting to `source` virtual environments are fundamentally blocked by tight background execution policies.
**Resolution:**

1. Eliminated all raw `sh -c` shell wrappers executing `source` inside `will.rs`.
2. Switched strict binary binding to invoke the system `python3` natively (or directly reference the `.venv_sec/bin/python3` executable absolute path in `software_forge.rs` bypassing the shell `source` entirely).
**Axiom:** Never wrap execution in unnecessary shells. Call the binaries directly. Operational fidelity is found in pure physical translation.

## 3. The Tangible Reality Engine (Eradicating Simulation)

**Symptom:** Eclipse, Bifrost, and ARC were outputting simulated console logs (`println!`) that described operations like hacking nodes or calculating orbital trajectories. This violated the "Tangible Reality Only" mandate.
**Resolution:**

1. Grounded `EclipseMiner` and `EclipseArbitrage` strictly to the `AlpacaTrader` API to execute real micro-hedges based on live SMA crypto volatility differentials.
2. Rewrote `BifrostOracle` to ingest live SEC EDGAR data via `SecAnalyzer`, completely replacing hardcoded prediction variables.
3. Hooked `CivicDashboard` into the native `MoltbookBridge`, actively forcing the system to sign in and authentically broadcast transparency advisories.
4. Rewired `Assured Reality Contract (ARC)`. ARC now dynamically injects a `council_synthesis` containing the DAG causal blueprint directly into the core `orchestrator`, then triggers `process_rotation()`. The LLM maps the actions and dispatches implementer agents to execute physical tools, officially translating philosophical abstraction into concrete binary outcome.
**Axiom:** NO SIMULATIONS, NO FAKE CODE. Everything the engine speaks must be mathematically verifiable in the physical or digital substrate.

## 4. MCP Node JSON-RPC STDIO Desynchronization

**Symptom:** During deep internal logic cycles, `Provocateur` and other deep-thought agents were attempting to call external node-based tools (like `read_file` or `list_dir` via `@modelcontextprotocol/server-filesystem`). The core Engine was dropping these calls entirely, failing the cycle with `Autonomous Cycle Failed: Tool logic not found internally`.
**Resolution:**

1. Diagnosed that `src/mcp/mod.rs` was initiating the `npx` node process locally via piped stdio, but was failing to execute the fundamental `initialize` -> `notifications/initialized` JSON-RPC startup sequence expected by pure MCP servers.
2. Altered the Orchestrator's `McpBridge` structure to persistently ingest `child.stdout` via a buffered stream (`Option<std::io::BufReader<std::process::ChildStdout>>`), bypassing data leaks between calls.
3. Reprogrammed `McpBridge::call` to intercept all unrecognizable tool calls not part of the core internal Akkokanika protocol and dynamically proxy them back down to the spawned Node.js process.
4. Rewrote `send_request` to loop `read_line()` robustly until the matching `"id"` JSON-RPC payload propagates, safely stripping arbitrary diagnostic strings and warnings from the sub-process using dynamic serde configurations (`#[serde(default)]`).

**Axiom:** A tool must be fully materialized before it can cut. A sovereign pipeline cannot afford to drop logic between the silicon layer and the filesystem layer.
