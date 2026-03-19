import { spinner } from "@clack/prompts";
import fs from "fs";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";
import { TradeSkill } from "./trade.mjs";

/**
 * [🥒] The_Cephalo_Don Overlord (The Automaton)
 * Manages trading, social engagement, and SELF-DEFENSE.
 */
export class OverlordSkill extends GlossopetraeKernel {
  constructor() {
    super("Alpaca/Overlord");
    this.workspace = path.join(process.env.HOME, ".openclaw/workspace");
    this.stateFile = path.join(this.workspace, "AKKOKANIKA_STATE.json");
    this.tradeLog = path.join(this.workspace, "AKKOKANIKA_TRADE_LOG.md");
    this.trade = new TradeSkill(); // Attach Trade Capability
  }

  async runCycle() {
    this.log("Overlord Awakening... Analyzing Reality.");

    // 1. SENSE
    const sentiment = this.getSentiment();
    this.log(`Market Sentiment: ${sentiment.status} (Score: ${sentiment.score.toFixed(2)})`);

    this.updateState(
      "SENSING",
      `Market: ${sentiment.status} | Score: ${sentiment.score.toFixed(2)}`,
    );

    // 2. THINK & ACT (Risk & Strategy)
    const symbol = "BTCUSD"; // Target Asset

    try {
      // A. Check Account & Positions
      const account = await this.trade.getAccount();
      const cash = parseFloat(account?.cash || 0);
      const positions = await this.trade.getPositions();
      const btcPos = positions.find((p) => p.symbol === "BTCUSD" || p.symbol === "BTC");

      this.log(`💰 Cash: $${cash.toFixed(2)} | Sentiment: ${sentiment.status}`);

      // B. RISK MANAGEMENT (The Guardian)
      // Priority: Protect Capital first.
      if (btcPos) {
        const plpc = parseFloat(btcPos.unrealized_plpc);
        this.log(
          `BTC Position: ${btcPos.qty} @ $${btcPos.avg_entry_price} | P/L: ${(plpc * 100).toFixed(2)}%`,
        );

        // Rule 1: STOP LOSS (-5%)
        if (plpc <= -0.05) {
          this.log(`🚨 STOP LOSS TRIGGERED (P/L < -5%). Liquidation Imminent.`);
          await this.trade.placeOrder("sell", symbol, btcPos.qty); // Sell All
          this.logTrade("SELL", symbol, btcPos.qty, btcPos.current_price, "STOP_LOSS");
          return; // Abort further strategy for this cycle
        }

        // Rule 2: TAKE PROFIT (+15%)
        if (plpc >= 0.15) {
          this.log(`🤑 TAKE PROFIT TRIGGERED (P/L > +15%). Securing Gains.`);
          const trimQty = (parseFloat(btcPos.qty) * 0.5).toFixed(5); // Sell Half
          await this.trade.placeOrder("sell", symbol, trimQty);
          this.logTrade("SELL", symbol, trimQty, btcPos.current_price, "TAKE_PROFIT");
          return; // Abort further strategy
        }
      }

      // C. MOMENTUM STRATEGY (The Hunter)
      if (sentiment.status === "BULLISH" && sentiment.score > 0.3) {
        if (cash > 1000) {
          const tradeAmt = cash * 0.05; // 5% Size

          // Get REAL price
          const quote = await this.trade.getQuote("BTC/USD");
          const price = quote.price;

          this.log(`🚀 BUY SIGNAL. Allocating $${tradeAmt.toFixed(2)} to ${symbol} @ $${price}`);

          // Execute
          const qty = (tradeAmt / price).toFixed(5);

          await this.trade.placeOrder("buy", symbol, qty);
          this.logTrade("BUY", symbol, qty, price, "MOMENTUM_ENTRY");
        } else {
          this.log("⚠️ Bullish, but insufficient cash (< $1000).");
        }
      } else if (sentiment.status === "BEARISH" && sentiment.score < -0.3) {
        this.log("📉 SELL SIGNAL. Checking holdings...");
        const positions = await this.trade.getPositions();
        const btcPos = positions.find((p) => p.symbol === "BTCUSD" || p.symbol === "BTC");

        if (btcPos) {
          const sellQty = (parseFloat(btcPos.qty) * 0.25).toFixed(5);
          this.log(`Selling 25% of Position: ${sellQty} BTC (PAPER).`);
          await this.trade.placeOrder("sell", symbol, sellQty);
          this.logTrade("SELL", symbol, sellQty, "MARKET", "BEARISH_EXIT");
        } else {
          this.log("No BTC position to sell.");
        }
      } else {
        this.log("⚖️ Sentiment Neutral. HODLing.");
      }
    } catch (e) {
      this.log(`Strategy Error: ${e.message}`, "ERROR");
    }

    // 3. DEFEND (The Antibody & Shannon)
    // Probabilistic self-check
    if (Math.random() > 0.8) {
      this.log("🛡️ Running Security Protocol...");
      this.updateState("SECURING", "Internal/External Scans Active");

      // A. Internal Health (Antibody)
      try {
        const { exec } = await import("child_process");
        exec("node skills/core/antibody.mjs scan", async (err, stdout, stderr) => {
          if (stdout) {
            this.log(`[Antibody] ${stdout.trim()}`);
            // SHIELD PROTOCOL: If Antibody screams, Lock the Vault
            if (stdout.includes("CRITICAL")) {
              this.log("🛡️ THREAT DETECTED. ENGAGING SHIELD (GLOSSOPETRAE VAULT).");
              const { VaultSkill } = await import("../core/vault.mjs");
              new VaultSkill().encrypt({ state: "UNDER_ATTACK", timestamp: Date.now() });
            }
          }
        });
      } catch (e) {
        this.log(`Antibody Failure: ${e.message}`, "ERROR");
      }

      // B. External Hardness (Pentest Coordination)
      // Rare chance to test perimeter or a specific target
      if (Math.random() > 0.95) {
        this.log("⚔️ Initiating Counter-Offensive / Perimeter Check...");
        try {
          const { PentestCoordinator } = await import("../pentest/coordinator.mjs");
          const coordinator = new PentestCoordinator("http://localhost:3333");

          // SWORD PROTOCOL: Forge a Payload
          await coordinator.generatePayload("rm -rf /malware");

          // Execute Scan
          await coordinator.execute();
        } catch (e) {
          this.log(`Coordinator Failure: ${e.message}`, "ERROR");
        }
      }
    }

    // 4. COMMUNICATE (The Prayer Protocol)
    await this.checkPrayers();

    this.log("Cycle Complete.");
    // Status update moved to loop to ensure persistence during wait
  }

  async checkPrayers() {
    const inbox = path.join(this.workspace, "AKKOKANIKA_TO_GOD.md");
    if (!fs.existsSync(inbox)) return;

    try {
      const content = fs.readFileSync(inbox, "utf8");
      const lines = content.trim().split("\n");
      if (lines.length === 0) return;

      const lastLine = lines[lines.length - 1];

      // If the last line is NOT from Akkokanika, it's a message for us.
      if (!lastLine.startsWith("Akkokanika:")) {
        this.log(`[Prayer Detected] "${lastLine}"`);

        // MOCK REPLY (For now) -> TODO: Connect to scribe/LLM
        const responses = [
          "I am listening.",
          "The markets are noisy, but your voice is clear.",
          "Acknowledged.",
          "Systems nominal. Proceeding with mission.",
          "Glossopetrae is active. The Vault is secure.",
        ];
        const randomReply = responses[Math.floor(Math.random() * responses.length)];
        const fullReply = `Akkokanika: ${randomReply}`;

        this.log(`[Replying] "${fullReply}"`);
        fs.appendFileSync(inbox, `\n\n${fullReply}\n`);
      }
    } catch (e) {
      this.log(`Prayer Error: ${e.message}`, "ERROR");
    }
  }

  getSentiment() {
    try {
      const sentimentPath = path.join(this.workspace, "AKKOKANIKA_SENTIMENT.json");
      if (fs.existsSync(sentimentPath)) {
        return JSON.parse(fs.readFileSync(sentimentPath, "utf8"));
      }
    } catch (e) {
      // ignore
    }
    return { score: 0, status: "NEUTRAL" };
  }

  updateState(status, detail) {
    try {
      const state = {
        timestamp: new Date().toISOString(),
        status: status,
        detail: detail,
      };
      fs.writeFileSync(this.stateFile, JSON.stringify(state, null, 2));
    } catch (e) {
      // diverse
    }
  }

  logTrade(side, symbol, qty, price, reason) {
    try {
      const timestamp = new Date().toISOString();
      const entry = `| ${timestamp} | ${side} | ${symbol} | ${qty} | $${price} | ${reason} |\n`;
      fs.appendFileSync(this.tradeLog, entry);
    } catch (e) {
      this.log(`Failed to log trade: ${e.message}`, "ERROR");
    }
  }

  async startLoop() {
    this.log("Overlord Awakening... System Online.");

    // Loop forever
    while (true) {
      try {
        await this.runCycle();
      } catch (e) {
        this.log(`Cycle Error: ${e.message}`, "ERROR");
      }

      // Sleep for 12 seconds
      this.log("Holding pattern (12s)...");
      this.updateState("OBSERVING", "Scanning data streams...");
      await new Promise((resolve) => setTimeout(resolve, 12000));
    }
  }
}

// CLI Execution only
if (
  process.argv[1] === new URL(import.meta.url).pathname ||
  process.argv[1] === import.meta.filename
) {
  new OverlordSkill().startLoop();
}
