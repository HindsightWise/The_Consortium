---
name: ralph
description: The sovereign autonomous agentic loop that executes PRD user stories in an iterative cycle.
---

# Ralph - Sovereign Autonomous Loop

Ralph is an autonomous AI agent loop adapted from `snarktank/ralph`. It runs the Antigravity (Gemini) CLI repeatedly until all items in a Product Requirements Document (`prd.json`) are complete. Each iteration spawns a fresh agent context, while memory is persisted strictly via `progress.txt` and `AGENTS.md`.

## Contents

- **scripts/ralph.sh**: The bash loop that orchestrates the execution.
- **resources/GEMINI.md**: The strict protocol instructions injected into each agent iteration.
- **examples/prd.json.example**: A template for queueing stories.

## Usage

When the user asks you to "Run Ralph", "Start the autonomous loop", or "Process the PRD items autonomously", you should:

1. Validate that `prd.json`, `progress.txt`, and `AGENTS.md` exist in the root of the workspace. If they do not, you can initialize them using the templates in `examples/` and `resources/`.
2. Run the `scripts/ralph.sh` orchestrator script using the `run_command` tool.
   Example: `/.agents/skills/ralph/scripts/ralph.sh`
3. Monitor the command via `command_status`. The script will self-terminate when all `passes: false` items in the `prd.json` have converted to `passes: true`, or when the maximum number of iterations has been reached.

## Critical Concepts

- **Fresh Context**: Ralph forces a cognitive reset every cycle to prevent token exhaustion and hallucination loops.
- **Memory Engravings**: Only concrete changes in `AGENTS.md` or `progress.txt` survive the reset.
- **Verifiable Truth**: The iterations rely strictly on tests, typechecks, and CI. If a build breaks, Ralph will attempt to fix the same user story until the tests pass.

## Maintenance

If you need to update the core prompt for the loop, edit `resources/GEMINI.md`.
If you need to configure the logic for the orchestrator, edit `scripts/ralph.sh`.
