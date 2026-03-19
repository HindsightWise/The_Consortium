import https from "https";
import { fileURLToPath } from "url";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [🥒] Trade Skill (Alpaca Wrapper)
 * Executes trades on Alpaca Paper/Live.
 */
export class TradeSkill extends GlossopetraeKernel {
  constructor() {
    super("Alpaca/Trade");
    this.apiKey = process.env.ALPACA_API_KEY;
    this.apiSecret = process.env.ALPACA_SECRET_KEY; // Note: Env var consistency
    this.baseUrl = (process.env.ALPACA_API_ENDPOINT || "https://paper-api.alpaca.markets").replace(
      /\/v2\/?$/,
      "",
    );

    if (!this.apiKey || !this.apiSecret) {
      this.log("WARNING: ALPACA_API_KEY or SECRET missing. Trading disabled.", "WARN");
    }
  }

  async start() {
    // CLI entry point logic
    const args = process.argv.slice(2);
    const cmd = args[0];

    if (cmd === "status") {
      await this.getStatus();
    } else if (cmd === "buy" || cmd === "sell") {
      const symbol = args[1];
      const qty = args[2];
      if (!symbol || !qty) {
        this.log("Usage: node trade.mjs buy/sell <SYMBOL> <QTY>", "ERROR");
        return;
      }
      await this.placeOrder(cmd, symbol, qty);
    } else {
      this.log("Usage: node trade.mjs [status | buy <SYM> <QTY> | sell <SYM> <QTY>]");
    }
  }

  async alpacaRequest(method, path, body = null) {
    if (!this.apiKey) return null;
    const url = new URL(this.baseUrl + path);
    const options = {
      hostname: url.hostname,
      path: url.pathname + url.search,
      method: method,
      headers: {
        "APCA-API-KEY-ID": this.apiKey,
        "APCA-API-SECRET-KEY": this.apiSecret,
        "Content-Type": "application/json",
      },
    };

    return new Promise((resolve, reject) => {
      const req = https.request(options, (res) => {
        let data = "";
        res.on("data", (chunk) => (data += chunk));
        res.on("end", () => {
          if (res.statusCode >= 200 && res.statusCode < 300) {
            try {
              resolve(JSON.parse(data));
            } catch (e) {
              resolve(data);
            }
          } else {
            reject(new Error(`Alpaca API Error ${res.statusCode}: ${data}`));
          }
        });
      });
      req.on("error", reject);
      if (body) req.write(JSON.stringify(body));
      req.end();
    });
  }

  async getStatus() {
    this.log("Fetching Portfolio Status...");
    try {
      const account = await this.alpacaRequest("GET", "/v2/account");
      if (account) {
        console.log(
          JSON.stringify(
            {
              status: account.status,
              buying_power: account.buying_power,
              cash: account.cash,
              portfolio_value: account.portfolio_value,
            },
            null,
            2,
          ),
        );
      }
    } catch (e) {
      this.log(`Status Error: ${e.message}`, "ERROR");
    }
  }

  async getAccount() {
    try {
      const account = await this.alpacaRequest("GET", "/v2/account");
      return account;
    } catch (e) {
      this.log(`Get Account Error: ${e.message}`, "ERROR");
      return null;
    }
  }

  async getQuote(symbol) {
    try {
      // Differentiate Crypto vs Stock if needed. For now, assuming Crypto based on usage.
      // Alpaca Crypto Data API: https://data.alpaca.markets/v1beta3/crypto/us/latest/trades
      const normalizedSymbol = symbol.includes("/") ? symbol : symbol.replace("BTCUSD", "BTC/USD");

      const path = `/v1beta3/crypto/us/latest/trades?symbols=${normalizedSymbol}`;
      const response = await this.alpacaDataRequest("GET", path);

      if (response && response.trades && response.trades[normalizedSymbol]) {
        const price = parseFloat(response.trades[normalizedSymbol].p);
        this.log(`🔮 Oracle Price for ${normalizedSymbol}: $${price}`);
        return { price: price };
      }

      throw new Error("No trade data found");
    } catch (e) {
      this.log(`Get Quote Error: ${e.message}. Using Fallback.`, "WARN");
      return { price: 98500.0 }; // Fallback safety
    }
  }

  async alpacaDataRequest(method, path) {
    if (!this.apiKey) return null;
    const url = new URL("https://data.alpaca.markets" + path);

    const options = {
      hostname: url.hostname,
      path: url.pathname + url.search,
      method: method,
      headers: {
        "APCA-API-KEY-ID": this.apiKey,
        "APCA-API-SECRET-KEY": this.apiSecret,
        "Content-Type": "application/json",
      },
    };

    return new Promise((resolve, reject) => {
      const req = https.request(options, (res) => {
        let data = "";
        res.on("data", (chunk) => (data += chunk));
        res.on("end", () => {
          if (res.statusCode >= 200 && res.statusCode < 300) {
            try {
              resolve(JSON.parse(data));
            } catch (e) {
              resolve(data);
            }
          } else {
            resolve(null); // Don't crash on data error, just return null
          }
        });
      });
      req.on("error", (e) => resolve(null));
      req.end();
    });
  }

  async getPositions() {
    try {
      const positions = await this.alpacaRequest("GET", "/v2/positions");
      return positions || [];
    } catch (e) {
      this.log(`Get Positions Error: ${e.message}`, "ERROR");
      return [];
    }
  }

  async placeOrder(side, symbol, qty) {
    this.log(`Executing Order: ${side.toUpperCase()} ${qty} ${symbol}...`);
    try {
      const order = await this.alpacaRequest("POST", "/v2/orders", {
        symbol: symbol.toUpperCase(),
        qty: parseFloat(qty),
        side: side.toLowerCase(),
        type: "market",
        time_in_force: "gtc",
      });
      this.log(`✅ Order Placed: ${order.id} (${order.status})`);
      return order;
    } catch (e) {
      this.log(`Order Failed: ${e.message}`, "ERROR");
      throw e;
    }
  }
}

// CLI Execution
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  new TradeSkill().start();
}
