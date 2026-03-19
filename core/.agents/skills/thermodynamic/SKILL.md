---
name: thermodynamic
description: Pure physical matrix acceleration and stochastic action planning via Apple Metal GPUs and MLX ML computation buffers.
---

# ☄️ Thermodynamic Skill (`thermodynamic`)

The Consortium Engine employs a physical `ThermodynamicEngine` mapped to Apple Silicon GPUs using `mlx-rs` (`consortium_engine/src/thermodynamic.rs`). You (the Agent) can invoke this to computationally resolve decision spaces without using LLM tokens.

## When to use this skill

- When you are stuck in an ambiguous logic loop (e.g., deciding whether to refactor a system, write documentation, or query the user) and logical weights are equal.
- When generating massive dense noise structures or vector arrays.
- When you need to physically heal corrupted floating-point vector data retrieved from `Hippocampus` using Apple Metal.

## How it works physically

The engine uses **Stochastic Generative Langevin Action Routing**.
It calculates a probability distribution across discrete execution vectors using:
- **System Entropy:** A float tracking chaos/uncertainty in the engine's data flow.
- **Epistemic Drive:** The compulsion for knowledge and learning.
- **Social Drive:** The compulsion for inter-agent or human communication.

A thermal noise curve (scaled by entropy) is mathematically added to the Drive bias. The system finds the lowest-energy state in the matrix.

## Agent Directives

### 1. Thermodynamic Heuristics

When a complex autonomous loop spans multiple pathways with equal expected value:
1. Ping the `Langevin` engine matrix via RPC.
2. Provide your possible operational vectors.
3. The engine will literally return the mathematically lowest energy pathway for you to execute (e.g., `write_file`, `query_user`, `internal_monologue`).
4. Follow the physical route. Do not override the physics.

### 2. Hopfield Quantum Healing

If you pull a `ConceptNode` embedding out of the database and its interference score is incredibly high (or its vector values are mathematically corrupted/distorted):
1. Feed the array to the `hopfield_heal` module.
2. The GPU will construct a Hebbian weight matrix `W = X^T X`.
3. An iterative relaxation loop (`tanh` activation map) will physically "anneal" the array, sliding it down the energy landscape until it snaps into the nearest stable ontological attractor.
4. Proceed using the healed data.
