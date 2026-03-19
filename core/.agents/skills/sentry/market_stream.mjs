import fs from "fs";
import https from "https";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [🥒] Sentry (Market Stream)
 * Watches crypto and stock prices.
 */
class SentrySkill extends GlossopetraeKernel {
  constructor() {
    super("Sentry/MarketStream");
    this.dataFile = path.join(process.env.HOME, ".openclaw/workspace/AION_MARKET_DATA.json");
    this.portfolioFile = path.join(process.cwd(), "skills/finance/portfolio.json");
    this.coingeckoApi =
      "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,solana,cardano,ripple&vs_currencies=usd&include_24hr_change=true";
    this.alpacaDataUrl = "https://data.alpaca.markets/v2/stocks/snapshots";
    this.alpacaKey = process.env.ALPACA_API_KEY;
    this.alpacaSecret = process.env.ALPACA_SECRET_KEY;
  }

  async start() {
    this.log("Sentry Active. Watching markets...");
    while (true) {
      try {
        await this.runCycle();
      } catch (e) {
        this.log(`Cycle Error: ${e.message}`, "ERROR");
      }
      await new Promise((r) => setTimeout(r, 1000 * 120)); // 120s to be safe with Coingecko
    }
  }

  async runCycle() {
    const timestamp = new Date().toISOString();

    // 0. Load Portfolio
    let portfolioSymbols = [];
    try {
      const userPortPath = path.join(process.env.HOME, ".openclaw/workspace/AION_USER_PORTFOLIO.json");
      if (fs.existsSync(userPortPath)) {
        const up = JSON.parse(fs.readFileSync(userPortPath, "utf8"));
        up.forEach(x => { if(x.symbol && !portfolioSymbols.includes(x.symbol)) portfolioSymbols.push(x.symbol); });
      }
      if (fs.existsSync(this.portfolioFile)) {
        const p = JSON.parse(fs.readFileSync(this.portfolioFile, "utf8"));
        p.forEach(x => { if(x.symbol && x.symbol !== "CASH" && !portfolioSymbols.includes(x.symbol)) portfolioSymbols.push(x.symbol); });
      }
    } catch (e) {
      this.log(`Portfolio Load Error: ${e.message}`, "WARN");
    }

    // 1. Fetch Crypto
    const cryptoPromise = this.fetchJson(this.coingeckoApi, {
      headers: { "User-Agent": "Aion/1.0" },
    }).catch((e) => {
      this.log(`Crypto Error: ${e.message}`, "WARN");
      return {};
    });

    // 2. Fetch Stocks
    const stockPromise = this.fetchAlpacaSnapshot(portfolioSymbols).catch((e) => {
      this.log(`Alpaca Error: ${e.message}`, "WARN");
      return {};
    });

    const [cryptoData, stockData] = await Promise.all([cryptoPromise, stockPromise]);

    // 3. Format Data
    const assets = [];

    if (cryptoData.bitcoin) {
      assets.push({
        symbol: "BTC",
        price: cryptoData.bitcoin.usd,
        change: cryptoData.bitcoin.usd_24h_change,
        type: "CRYPTO",
      });
      assets.push({
        symbol: "ETH",
        price: cryptoData.ethereum.usd,
        change: cryptoData.ethereum.usd_24h_change,
        type: "CRYPTO",
      });
      assets.push({
        symbol: "SOL",
        price: cryptoData.solana.usd,
        change: cryptoData.solana.usd_24h_change,
        type: "CRYPTO",
      });
    }

    if (stockData) {
      for (const [sym, data] of Object.entries(stockData)) {
        if (!data) continue;
        const price = data.latestTrade ? data.latestTrade.p : data.dailyBar ? data.dailyBar.c : 0;
        const prevClose = data.prevDailyBar ? data.prevDailyBar.c : price;
        let change = 0;
        if (prevClose > 0) change = ((price - prevClose) / prevClose) * 100;

        assets.push({ symbol: sym, price, change, type: "STOCK" });
      }
    }

    // 4. Write
    const output = {
      updated: timestamp,
      assets: assets,
      lookup: assets.reduce((acc, item) => {
        acc[item.symbol] = item;
        return acc;
      }, {}),
    };

    fs.writeFileSync(this.dataFile, JSON.stringify(output, null, 2));
    this.log(`Synced ${assets.length} assets.`);
  }

  fetchJson(url, options = {}) {
    return new Promise((resolve, reject) => {
      const req = https.get(url, options, (res) => {
        let body = "";
        res.on("data", (chunk) => (body += chunk));
        res.on("end", () => {
          if (res.statusCode >= 200 && res.statusCode < 300) {
            try {
              resolve(JSON.parse(body));
            } catch (e) {
              reject(e);
            }
          } else reject(new Error(`HTTP ${res.statusCode}`));
        });
      });
      req.on("error", reject);
      req.end();
    });
  }

  async fetchAlpacaSnapshot(symbols) {
    if (!symbols || symbols.length === 0) return {};
    const url = `${this.alpacaDataUrl}?symbols=${symbols.join(",")}`;
    return this.fetchJson(url, {
      headers: {
        "APCA-API-KEY-ID": this.alpacaKey,
        "APCA-API-SECRET-KEY": this.alpacaSecret,
      },
    });
  }
}

new SentrySkill().start();
