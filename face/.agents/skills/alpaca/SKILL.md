---
name: alpaca
description: "Market Operations via Alpaca Paper Trading API."
---

# Alpaca Skill

Allows The_Cephalo_Don to execute trades and manage the portfolio on Alpaca (Paper Trading).

## Capabilities

### 1. Account Status

View buying power, portfolio value, and cash.

*   **Command:** `node skills/alpaca/trade.js status`

### 2. Execute Order

Buy or Sell assets.

*   **Buy:** `node skills/alpaca/trade.js buy <SYMBOL> <QTY>`
*   **Sell:** `node skills/alpaca/trade.js sell <SYMBOL> <QTY>`

**Example:**
```bash
node skills/alpaca/trade.js buy BTCUSD 0.1
node skills/alpaca/trade.js buy SPY 1
```

## Configuration

Requires `ALPACA_API_KEY`, `ALPACA_API_SECRET`, and `ALPACA_API_ENDPOINT` in `.env`.
Default Endpoint: `https://paper-api.alpaca.markets`.
