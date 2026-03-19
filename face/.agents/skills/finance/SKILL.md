---
name: finance
description: "Market Intelligence tools (CFTC COT Data, Price Feeds)."
---

# Finance Skill

Tools for analyzing market structure and institutional positioning.

## Capabilities

### 1. COT Data (Commitments of Traders)

Fetches the latest institutional positioning from the CFTC TFF Report.
Useful for seeing what "Smart Money" (Asset Managers) and "Hedge Funds" (Leveraged Money) are doing.

*   **Command:** `node skills/finance/fetch_cot.js <TICKER>`
*   **Supported Tickers:** BTC, ETH, GOLD, ES (S&P 500)

**Example:**
```bash
node skills/finance/fetch_cot.js BTC
```
*Returns JSON with net positioning and week-over-week changes.*
