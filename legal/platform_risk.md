# Platform Risk Radar (18-Month De-Risk)

## Current External API Dependencies

1. **Alpaca API (High Risk):** Live market data and execution. If paper-trading or WS streams change rules, system halts.
   - *Abstraction Epic:* Stand up generic exchange interfacing logic (traits/interfaces) instead of directly calling `alpaca-sdk`. Assign due date: 18 months from today.
2. **OpenAI / DeepSeek / Gemini (High Risk):** Cognitive engines.
   - *Abstraction Epic:* `RouterCore` already exists. Must expand MLX 4-bit local fallbacks to 100% feature parity so the API layer can be severed if rates jump.
3. **Telegram (Medium Risk):** Primary Human I/O interface.
   - *Abstraction Epic:* Build an internal, Sovereign WebSockets command-center frontend.
4. **Discord (Medium Risk):** Autonomous marketing drone output.
   - *Abstraction Epic:* Standardize webhook outgoing shapes so X/Reddit or generic targets can be hot-swapped.
5. **Stripe (High Risk):** Capital ingress.
   - *Abstraction Epic:* Ensure Consortium Protocol Gateway (CPG) Crypto fallback operates universally without KYC/AML freezes locking the treasury.

**SOP 2.1 STATUS:** Tracked. Risk mitigation epics filed.
