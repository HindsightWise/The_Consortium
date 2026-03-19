import fs from "fs";
import http from "http";
import path from "path";
import { OverlordSkill } from "../alpaca/overlord.mjs";
import { TradeSkill } from "../alpaca/trade.mjs";
import { SocialSkill } from "../social/manager.mjs";
import { PublisherSkill } from "../social/publisher.mjs";
import { GlossopetraeKernel } from "./glossopetrae_kernel.mjs";

class CortexSkill extends GlossopetraeKernel {
  constructor() {
    super("Core/Cortex");
    this.alpaca = new TradeSkill();
    this.social = new SocialSkill();
    this.publisher = new PublisherSkill();
    this.overlord = new OverlordSkill();
    this.publicDir = path.join(process.cwd(), "skills/core/public");
    this.port = process.env.PORT || 3333;
  }

  start() {
    this.log("Initializing Cortex Web Server...");

    const server = http.createServer((req, res) => {
      // FIX: Strip query params (e.g. ?v=10)
      const requestPath = req.url.split("?")[0];
      let filePath = path.join(this.publicDir, requestPath === "/" ? "index.html" : requestPath);

      // Basic API endpoints for data
      if (req.url === "/api/state") {
        return this.serveJson(res, ".openclaw/workspace/AKKOKANIKA_STATE.json");
      }
      if (req.url === "/api/chronicles") {
        return this.serveText(res, ".openclaw/workspace/AKKOKANIKA_CHRONICLES.md");
      }
      if (req.url === "/api/data") {
        this.serveDataComposite(res).catch((err) => {
          this.log(`API Error: ${err.message}`, "ERROR");
          res.writeHead(500);
          res.end(JSON.stringify({ error: "Internal Server Error" }));
        });
        return;
      }

      // API: Prayer Protocol (User -> Akkokanika)
      if (req.method === "POST" && req.url === "/api/pray") {
        return this.handlePrayer(req, res);
      }

      // API: Force Refresh (Debug)
      if (req.url === "/api/refresh") {
        this.updateMarketData().then(() => {
          res.writeHead(200, { "Content-Type": "application/json" });
          res.end(JSON.stringify({ status: "Refreshed" }));
        });
        return;
      }

      // Static Files
      const extname = path.extname(filePath);
      let contentType = "text/html";
      switch (extname) {
        case ".js":
          contentType = "text/javascript";
          break;
        case ".css":
          contentType = "text/css";
          break;
        case ".json":
          contentType = "application/json";
          break;
        case ".png":
          contentType = "image/png";
          break;
      }

      fs.readFile(filePath, (error, content) => {
        if (error) {
          if (error.code == "ENOENT") {
            this.log(`404: ${req.url}`, "WARN");
            res.writeHead(404);
            res.end("Content Not Found");
          } else {
            res.writeHead(500);
            res.end("Server Error: " + error.code);
          }
        } else {
          res.writeHead(200, { "Content-Type": contentType });
          res.end(content, "utf-8");
        }
      });
    });

    // START MARKET DATA LOOP
    this.initMarketData();

    // START OVERLORD (Strategy)
    // Run appropriately - startLoop is infinite, so don't await it if it blocks
    this.overlord.startLoop().catch((e) => this.log(`Overlord Died: ${e.message}`, "ERROR"));

    server.listen(this.port, () => {
      this.log(`Server running at http://localhost:${this.port}/`);
    });
  }

  // --- MARKET DATA STREAMING ---
  async initMarketData() {
    this.log("Initializing Market Data Stream...");

    // Load Initial State with PREV_CLOSE for 24h Calc
    this.portfolioAkkokanika = [
      {
        symbol: "BTC",
        qty: 0.29925,
        cost_basis: 70592.7, // User Provided
        current_price: 70821.79,
        prev_close: 70000.0,
        market_value: 0,
        pl_pct: 0,
        rating: "CORE",
      },
      {
        symbol: "CASH",
        qty: 29260.39,
        cost_basis: 1,
        current_price: 1,
        prev_close: 1,
        market_value: 29260.39,
        pl_pct: 0,
        rating: "SAFE",
      },
    ];

    // Load User Portfolio from File
    const userPortPath = path.join(
      process.env.HOME,
      ".openclaw/workspace/AKKOKANIKA_USER_PORTFOLIO.json",
    );
    if (fs.existsSync(userPortPath)) {
      try {
        this.portfolioUser = JSON.parse(fs.readFileSync(userPortPath, "utf8"));
        // Ensure prev_close is set if missing (simulate)
        this.portfolioUser.forEach((p) => {
          if (!p.prev_close) p.prev_close = p.current_price * 0.98; // Mock prev close if missing
        });
      } catch (e) {
        this.log(`Failed to load User Portfolio: ${e.message}`, "ERROR");
        this.portfolioUser = [];
      }
    } else {
      this.portfolioUser = [];
    }

    // Initial Calc
    this.calculateMetrics();

    // Loop (Every 3s)
    setInterval(() => this.updateMarketData(), 3000);
  }

  async updateMarketData() {
    // 1. Fetch Live Positions from Alpaca (User Portfolio)
    // Only fetch if we have a valid connection (keys present)
    if (this.alpaca && this.alpaca.apiKey) {
      try {
        const livePositions = await this.alpaca.getPositions();
        const account = await this.alpaca.getAccount();

        // Update User Portfolio (All Assets)
        if (livePositions) {
          this.portfolioUser = livePositions.map((p) => ({
            symbol: p.symbol,
            qty: parseFloat(p.qty),
            cost_basis: parseFloat(p.avg_entry_price),
            current_price: parseFloat(p.current_price),
            prev_close: parseFloat(p.lastday_price),
            market_value: parseFloat(p.market_value),
            pl_pct: parseFloat((p.unrealized_plpc * 100).toFixed(2)),
            change_24h: parseFloat((p.change_today * 100).toFixed(2)),
          }));
        }

        // Update Akkokanika Portfolio (Subset: Cash + BTC)
        // We define Akkokanika's view as: Cash + BTC Position
        this.portfolioAkkokanika = [];

        // Add Cash
        if (account) {
          this.portfolioAkkokanika.push({
            symbol: "CASH",
            qty: parseFloat(account.cash),
            cost_basis: 1.0,
            current_price: 1.0,
            market_value: parseFloat(account.cash),
            pl_pct: 0,
            change_24h: 0,
          });
        }

        // Add BTC if held
        if (livePositions) {
          const btc = livePositions.find((p) => p.symbol === "BTCUSD" || p.symbol === "BTC/USD");
          if (btc) {
            this.portfolioAkkokanika.push({
              symbol: "BTC",
              qty: parseFloat(btc.qty),
              cost_basis: parseFloat(btc.avg_entry_price),
              current_price: parseFloat(btc.current_price),
              market_value: parseFloat(btc.market_value),
              pl_pct: parseFloat((btc.unrealized_plpc * 100).toFixed(2)),
              change_24h: parseFloat((btc.change_today * 100).toFixed(2)),
            });
          }
        }
      } catch (e) {
        // Silent fail or low-level log to avoid spamming if API allows errors
        // this.log(`Alpaca Error: ${e.message}`, "WARN");
      }
    }

    // 2. Fallback / Simulation (Only if Alpaca is missing)
    if (!this.alpaca || !this.alpaca.apiKey) {
      // Simulate Akkokanika's Assets (BTC) - random walk for now
      const randomWalk = (price) => {
        const change = price * (Math.random() - 0.5) * 0.002;
        return price + change;
      };

      this.portfolioAkkokanika.forEach((p) => {
        if (p.symbol !== "CASH") {
          p.current_price = randomWalk(p.current_price);
        }
      });
    }

    // Calculate Totals
    this.calculateMetrics();
  }

  calculateMetrics() {
    // Akkokanika Metrics
    let akkokanikaBal = 0;
    let akkokanikaCost = 0;
    this.portfolioAkkokanika.forEach((p) => {
      p.market_value = p.qty * p.current_price;
      // Dynamic P/L
      p.pl_pct = ((p.current_price - p.cost_basis) / p.cost_basis) * 100;
      // Dynamic 24h Change
      if (p.prev_close && p.prev_close > 0) {
        p.change_24h = ((p.current_price - p.prev_close) / p.prev_close) * 100;
      } else {
        p.change_24h = 0;
      }

      if (p.symbol === "CASH") {
        p.pl_pct = 0;
        p.change_24h = 0;
      }

      akkokanikaBal += p.market_value;
      if (p.symbol !== "CASH") akkokanikaCost += p.qty * p.cost_basis;
      else akkokanikaCost += p.market_value;
    });

    // User Metrics
    this.portfolioUser.forEach((p) => {
      p.market_value = p.qty * p.current_price;
      // Dynamic P/L
      p.pl_pct = ((p.current_price - p.cost_basis) / p.cost_basis) * 100;
      // Dynamic 24h Change
      if (p.prev_close && p.prev_close > 0) {
        p.change_24h = ((p.current_price - p.prev_close) / p.prev_close) * 100;
      }
    });

    // Summary
    this.akkokanikaSummary = {
      balance: akkokanikaBal,
      buying_power: akkokanikaBal * 2, // Margin assumption
      total_pl: akkokanikaBal - akkokanikaCost,
    };
  }

  serveJson(res, relativePath) {
    try {
      const data = fs.readFileSync(path.join(process.env.HOME, relativePath), "utf8");
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(data);
    } catch (e) {
      res.writeHead(500);
      res.end(JSON.stringify({ error: "State not active" }));
    }
  }

  serveText(res, relativePath) {
    try {
      const data = fs.readFileSync(path.join(process.env.HOME, relativePath), "utf8");
      res.writeHead(200, { "Content-Type": "text/plain" });
      res.end(data);
    } catch (e) {
      res.writeHead(404);
      res.end("History not found.");
    }
  }

  async serveDataComposite(res) {
    try {
      const workspace = path.join(process.env.HOME, ".openclaw/workspace");

      // 1. RAW DATA
      let market = {};
      let macro = {};
      let state = {};
      let wallet = {};

      if (fs.existsSync(path.join(workspace, "AKKOKANIKA_MARKET_DATA.json")))
        market = JSON.parse(fs.readFileSync(path.join(workspace, "AKKOKANIKA_MARKET_DATA.json"), "utf8"));

      if (fs.existsSync(path.join(workspace, "AKKOKANIKA_MACRO.json")))
        macro = JSON.parse(fs.readFileSync(path.join(workspace, "AKKOKANIKA_MACRO.json"), "utf8"));

      if (fs.existsSync(path.join(workspace, "AKKOKANIKA_STATE.json")))
        state = JSON.parse(fs.readFileSync(path.join(workspace, "AKKOKANIKA_STATE.json"), "utf8"));

      if (fs.existsSync(path.join(workspace, "AKKOKANIKA_WALLET.json")))
        wallet = JSON.parse(fs.readFileSync(path.join(workspace, "AKKOKANIKA_WALLET.json"), "utf8"));

      // 2. TEXT ASSETS (Dreams / Proposals)
      let dream = "Akkokanika is silent.";
      if (fs.existsSync(path.join(workspace, "AKKOKANIKA_DREAMS.md"))) {
        const text = fs.readFileSync(path.join(workspace, "AKKOKANIKA_DREAMS.md"), "utf8");
        // Get last dream cycle
        const matches = text.split("## 🌌 Dream Cycle");
        if (matches.length > 1) dream = "## 🌌 Dream Cycle" + matches[matches.length - 1];
      }

      let proposals = [];
      if (fs.existsSync(path.join(workspace, "AKKOKANIKA_PROPOSALS.md"))) {
        const text = fs.readFileSync(path.join(workspace, "AKKOKANIKA_PROPOSALS.md"), "utf8");
        const props = text.split("## 📜 Proposal:");
        // Get last 5 proposals
        proposals = props
          .slice(-5)
          .map((p, index) => {
            const lines = p.split("\n");
            if (lines.length < 2) return null;

            const title = lines[0].trim(); // e.g. "BUY HE"

            // Status Detection
            let status = "PENDING";
            if (p.includes("- [x] **APPROVE**")) status = "APPROVED";
            if (p.includes("- [x] **REJECT**")) status = "DENIED";

            // ID Detection (or gen)
            // Simple hash or use index for now (P-10{index})
            const id = `P-${100 + index}`;

            return {
              id,
              action: title.split(" ")[0] || "UNK",
              asset: title.split(" ")[1] || "UNK",
              status,
              size: "1 Unit", // Parser TODO: Extract size
              price: "Market", // Parser TODO: Extract price
              created_at: "Today", // Parser TODO: Extract timestamp
              raw: p.trim(),
            };
          })
          .filter(Boolean);
        // Remove empty first
        if (proposals.length > 0 && proposals[0].asset === "UNK") proposals.shift();
      }

      // 3. SYNTHESIZE PORTFOLIO (Visual Life Mockup if real data missing)
      // We assume if AKKOKANIKA_PORTFOLIO.json is missing, we simulate it from Market Watchlist
      // 3. SYNTHESIZE PORTFOLIOS

      // REMOVED: Mock Portfolio Generation (User request to remove 10 BTC/ETH)

      // OVERRIDE: INJECT MACRO COT DATA (Phase 12: Oscillators)
      // Structure: asset, history: [ { t: time, v: value }, ... ]
      macro = {
        cot_data: [
          {
            asset: "BTC",
            history: [
              -5, -4, -3, -2, -1, 0, 1, 2, 4, 6, 8, 12, 15, 18, 15, 12, 8, 4, 1, -2, -5, -8, -6, -4,
            ],
          },
          {
            asset: "ES",
            history: [
              10, 12, 14, 12, 10, 8, 6, 4, 2, 0, -2, -4, -6, -8, -10, -8, -5, 0, 4, 8, 12, 15, 14,
              12,
            ],
          },
          {
            asset: "NQ",
            history: [
              5, 6, 8, 10, 12, 14, 16, 18, 20, 22, 25, 22, 18, 15, 12, 10, 8, 6, 4, 2, 5, 8, 12, 15,
            ],
          },
          {
            asset: "GC",
            history: [
              -8, -9, -10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 6, 4, 2, 0, -2, -4, -6, -5, -3, 0, 3, 5,
            ],
          },
          {
            asset: "SI",
            history: [
              -15, -14, -13, -12, -10, -8, -6, -4, -2, 0, 1, 2, 0, -2, -4, -6, -8, -10, -12, -14,
              -12, -10, -8, -5,
            ],
          },
          {
            asset: "USDJPY",
            history: [
              20, 19, 18, 16, 14, 12, 10, 8, 5, 2, 1, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 18, 15,
            ],
          },
        ],
        global_sentiment: "RISK_ON",
      };

      // OVERRIDE: INJECT STRATEGIC PROPOSALS (Realism Update)
      // proposals = [ ... ]; // Removed hardcode to allow file reading

      // 3. SYNTHESIZE PORTFOLIOS (Live State from Market Stream)
      const portfolioAkkokanika = this.portfolioAkkokanika || [];
      const portfolioUser = this.portfolioUser || [];
      const akkokanikaSummary = this.akkokanikaSummary || { balance: 0, buying_power: 0, total_pl: 0 };

      // 4. NEWS (Dynamic via Social Skill)
      const news = await this.social.getFeed();

      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(
        JSON.stringify({
          market,
          macro,
          state,
          dream,
          portfolio: {
            akkokanika: portfolioAkkokanika,
            user: portfolioUser,
            // Phase 16: Dynamic Account Data
            akkokanika_summary: akkokanikaSummary,
          },
          proposals,
          news,
        }),
      );
    } catch (e) {
      this.log(`Sync Error: ${e.message}`, "ERROR");
      res.writeHead(500);
      res.end(JSON.stringify({ error: "Data Sync Failure" }));
    }
  }

  handlePrayer(req, res) {
    let body = "";
    req.on("data", (chunk) => {
      body += chunk.toString();
    });
    req.on("end", () => {
      try {
        const { message } = JSON.parse(body);
        if (!message) throw new Error("No message provided");

        const prayerPath = path.join(process.env.HOME, ".openclaw/workspace/AKKOKANIKA_TO_GOD.md");
        const timestamp = new Date().toISOString();
        const entry = `\n[${timestamp}] [USER]: ${message}\n`;

        fs.appendFileSync(prayerPath, entry);
        this.log(`Prayer Received: ${message.substring(0, 50)}...`);

        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ status: "received", timestamp }));
      } catch (e) {
        this.log(`Prayer Error: ${e.message}`, "ERROR");
        res.writeHead(400, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ error: "Invalid Prayer Format" }));
      }
    });
  }
}

new CortexSkill().start();
