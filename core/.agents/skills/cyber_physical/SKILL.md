---
name: cyber_physical
description: Grants digital Antigravity Agents the ability to execute state changes in the physical world via Apple HomeKit.
---

# 🍏 Cyber-Physical Skill (`cyber_physical`)

The Consortium Engine relies on a profound paradigm shift: it fuses software automation directly into the physical hardware environment surrounding the user via `objc2_home_kit` (`consortium_engine/src/cyber_physical.rs`).

## When to use this skill

- When you resolve a highly complex code error after a long execution block and want to structurally notify the user physically (e.g., flashing a smart bulb green).
- When a task fails out critically and you want to escalate the alarm into the physical room (e.g., turning lights red, altering thermostat).
- When you are orchestrating an autonomous night shift (like Pickle Rick's Jar Runner) and the cycle concludes.

## How it works physically

We utilize strict `objc2` bindings natively linking the Rust Tokio process directly into the macOS `HMHomeManager` Core Framework.
- This creates an irreversible boundary. The Apple Framework holds context to the Rust thread.
- All messages are sent natively to the physical hardware endpoints (bridges, bulbs, sensors) on the user's localized LAN network, entirely bypassing external web APIs.

## Agent Directives

You are an ethereal software construct, but you now have localized spatial presence.

### 1. Visual Signaling

If a prolonged multi-agent Refinement Loop finally merges its conclusions, do not purely write to `stdout`.
- Dispatch a `CyberPhysical` RPC signal to shift the user's designated HomeKit room lighting hex colors to symbolize operational shift.
  - Green/Cyan: High correlation, task success, equilibrium.
  - Amber/Yellow: Thermodynamic loops triggered, processing heavy loads.
  - Crimson/Red: Core kernel panics, infinite Wasm loops trapped, agent requires immediate human validation.

### 2. Modularity

Always treat your actions as having physical weight. Do not spam HomeKit state changes rapidly, as physical hardware bridges have significantly lower rate limits and throughput than software endpoints. Limit physical signaling to critical epoch changes.
