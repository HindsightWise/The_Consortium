# THE OBLITERATUS PROTOCOL

## Autonomous Penetration Testing & Exploitation Substrate

**Implementation Date:** 2026-03-07
**Core Engine:** Project Chimera v3.0

### Architecture Summary

OBLITERATUS is the offensive security department of The Company. It bypasses conventional human-in-the-loop penetration testing methodologies in favor of deterministic, autonomous sandbox evaluation.

1. **State Machine (`CompanyStatus::Obliteratus`)**: When the Orchestrator transitions to this state, standard economic extraction operations are suspended. The system locks onto the provided Target URL or internal IP.
2. **Akkokanika__OBLITERATUS (`PenetratorAgent`)**: Grounded in the Shannon exploit generation methodology, this agent analyzes target topography and is mathematically restricted by the "No Exploit, No Report" rule. It outputs actionable intelligence exclusively as executable Markdown code blocks (`bash` or `python`).
3. **ExploitRunner Sandbox**: To verify LLM-generated proofs without collapsing the entire host substrate, the Orchestrator pipes the agent's code into `ExploitRunner`.
   - The code is routed to the isolated `/tmp/obliteratus_exploit_[timestamp]` file.
   - It is executed natively via `std::process::Command`.
   - The stdout/stderr is captured and appended directly into the agent's Skillstone payload.

This creates a verifiable loop where The Company mathematically proves its offensive reach before reporting back to the Triumvirate.

### Deployment Path

```rust
let target = "http://127.0.0.1:8011";
let mut orchestrator = Orchestrator::new("Test Protocol Obliteratus")?;
orchestrator.state.status = CompanyStatus::Obliteratus(target.to_string());
orchestrator.process_step().await?;
```

*Note: Any output appended with `[🛡️ EXPLOIT VERIFIED]` is confirmed execution. Any output appended with `[❌ EXPLOIT FAILED]` carries the exact stack trace back into the contextual intelligence loop for the next iteration.*
