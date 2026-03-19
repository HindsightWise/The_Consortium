---
name: pickle_rick
description: An advanced autonomous loop, manager/worker model, and codebase disruption engine originally ported from `pickle-rick-claude`.
---

# Pickle Rick - Sovereign Autonomous Engine

Pickle Rick is an advanced autonomous execution lifecycle adapted from `gregorydickson/pickle-rick-claude`. It natively hooks into the Gemini CLI configuration to provide long-running, self-healing, multi-agent workspaces.

## Installation Status

To finalize the installation, the `install.sh` script must be run directly from your host terminal to bypass macOS Sandbox protections over the `~/.gemini/` configuration folder.

Run:

```bash
cd .agents/skills/pickle_rick
npm_config_cache=/tmp/npm_cache npm install --prefix extension
./install.sh
```

This will automatically inject the required slash commands and stop-hooks into your local `~/.gemini/settings.json` file. Because this is a **methodology port**, the commands are instantiated *globally* across the Antigravity agent CLI, not just inside this workspace.

## Core Capabilities

Because Pickle Rick is deeply integrated into the Antigravity CLI, you don't run it through a bash loop like Ralph. Instead, you invoke its tools natively.

### 1. The Autonomous Loop

To queue a ticket or start an iteration, simply invoke the command. The stop-hooks inside Gemini will hijack the completion and aggressively push the agent to finish the work without your intervention.

- `/pickle "Install standard auth middleware"`
- `/pickle-tmux` (for clean context wiping)

### 2. Multi-Agent Refinement (Morty Workers)

For complex tasks, Pickle Rick spawns parallel subprocesses (Morty Workers) to investigate the codebase, write plans, and generate code, while the Manager focuses strictly on orchestrating the PRD.

### 3. Specialty Modules

You now have access to powerful new commands injected into your CLI:

- `/portal-gun`: Extracts architectural patterns from donor codebases and applies them to your project via a multi-analyst refinement cycle.
- `/project-mayhem`: Runs chaos engineering against your codebase (mutation testing, dependency downgrading, configuration corruption) to find vulnerabilities.
- `/microverse`: An optimization convergence loop that iteratively modifies your codebase to improve a target metric until it stops progressing.
- `/council-of-ricks`: Uses Graphite to create parallel git stacks to explore multiple implementation approaches concurrently.

## Maintenance

If you need to uninstall the global hooks or reset the configuration, run:

`/.agents/skills/pickle_rick/uninstall.sh`

To re-compile the Typescript engine, run the `install.sh` script.
