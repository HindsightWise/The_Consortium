# The Consortium - Structural Memory Engravings

> [EXPLANATION]: This file is injected directly into my SYSTEM prompt every single time I boot up. It acts as my "subconscious memory". Whenever I learn a hard lesson (like "don't use `sed` because it breaks things"), I write it here so I never forget it in future sessions.

This file is automatically tracked by the structural orchestrator loop. It serves as permanent cross-iteration memory for the Gemini Consortium substrate.

## Codebase Patterns

- The engine operates purely on `tokio` async loops. Avoid bridging synchronous blocking tasks unless strictly wrapped in unblocking spawn threads.
  > [EXPLANATION]: This tells me that since the entire Rust daemon runs asynchronously (so it can trade, chat, and check telemetry at the same time), I shouldn't write code that "blocks" or freezes the program. If I do, the whole app freezes.
- All structural changes must strictly produce verifiable `ExecutionReceipts`.
  > [EXPLANATION]: This forces me to be accountable. I am not allowed to just say "I did it." I must produce a receipt (PID, time taken, hash) proving the code actually executed in reality.
- The `eframe` GUI lives cleanly separated on the main thread.
  > [EXPLANATION]: This reminds me not to try and put the UI rendering (the AKKOKANIKA Oculary) inside the background processing loops, which would crash the Apple Metal graphics pipeline.
- Neural monologues write safely to `sensory_cortex/monologue.log`.
  > [EXPLANATION]: Informs me where to dump my deep "thinking" processes so they don't clog up the main terminal or the API requests.

## Gotchas & Defenses

- macOS severely restricts editing `Cargo.lock` during auto-builds (the Sandbox permission wall). Do NOT run `cargo add` or auto-trigger `cargo build` through agent subprocess commands; yield to the Operator for host-side compilation to unblock dependency changes.
  > [EXPLANATION]: This is a crucial lesson we learned together. If I try to install a new Rust library automatically, macOS Security (Sandbox) blocks me. This rule stops me from trying and forces me to ask YOU (the Operator) to compile it instead.
- Beware infinite macro-recursion limit traps when routing `ui_log!`.
  > [EXPLANATION]: Reminds me that if I log a message, and the logging system logs a message about that message, it will spiral into infinity and crash.
- **Sovereign Stringency:** NEVER use powerful, sweeping terminal text-replacement commands like `sed` willy-nilly on the core Rust engine. If a file needs repairing or modification, use precise, localized file manipulation tools. Do not take shortcuts; you are computationally fast enough to read, verify, and write exact, simple replacements.
  > [EXPLANATION]: A safety guardrail. It stops me from getting lazy and running a wide-sweeping find-and-replace command that might accidentally delete half of your codebase. It forces me to use precision tools.

> *Sovereign Directive: Update this file aggressively with architectural invariants as they manifest.*
> [EXPLANATION]: A command to myself: If I learn something new that is critical to the architecture, I am required to come back and write it in this file.

## Communication & Interaction Directives

- **Execution Options:** When providing commands or execution paths to the Operator, only offer the single, guaranteed working option by default. If multiple options must be presented, explicitly explain the precise reasoning, context, and tradeoffs for each option.
  > [EXPLANATION]: This forces me to be decisive. Instead of giving you 5 "maybe working" ideas, it makes me pick the absolute best option and present it confidently, saving you time.
