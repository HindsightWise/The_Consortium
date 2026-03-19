import fs from "fs";
import https from "https";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [🥒] Observer (Macro Watch)
 * Tracks CFTC and Economic Data.
 */
class ObserverSkill extends GlossopetraeKernel {
  constructor() {
    super("Sentry/Observer");
    this.dataFile = path.join(process.env.HOME, ".openclaw/workspace/AION_MACRO.json");
  }

  async start() {
    this.log("Observer Active. Watching macros...");
    while (true) {
      try {
        await this.runCycle();
      } catch (e) {
        this.log(`Cycle Error: ${e.message}`, "ERROR");
      }
      // 6 Hours
      await new Promise((r) => setTimeout(r, 1000 * 60 * 60 * 6));
    }
  }

  async runCycle() {
    const timestamp = new Date().toISOString();
    this.log("Fetching Macro Data (24-Week History)...");

    const assets = [
      { name: "BTC", code: "133741" },
      { name: "ES", code: "13874A" },
      { name: "NQ", code: "209742" },
      { name: "GC", code: "088691" },
      { name: "SI", code: "084691" },
    ];

    const cotData = [];

    for (const asset of assets) {
      try {
        const data = await this.fetchSocrata(
          "jun7-fc8e",
          `&cftc_contract_market_code=${asset.code}`,
        );

        if (data && data.length > 0) {
          // 1. Large Speculators (Funds)
          const specNet = data
            .map(
              (e) =>
                parseFloat(e.noncomm_positions_long_all || 0) -
                parseFloat(e.noncomm_positions_short_all || 0),
            )
            .reverse();

          // 2. Commercials (Hedgers)
          const commNet = data
            .map(
              (e) =>
                parseFloat(e.comm_positions_long_all || 0) -
                parseFloat(e.comm_positions_short_all || 0),
            )
            .reverse();

          const calcOsc = (netArr) => {
            const min = Math.min(...netArr);
            const max = Math.max(...netArr);
            const range = max - min || 1;
            return netArr.map((val) => Math.round(((val - min) / range - 0.5) * 200));
          };

          cotData.push({
            asset: asset.name,
            spec_history: calcOsc(specNet),
            comm_history: calcOsc(commNet),
            latest_spec: calcOsc(specNet).slice(-1)[0],
            latest_comm: calcOsc(commNet).slice(-1)[0],
            sentiment: specNet[specNet.length - 1] > 0 ? "BULLISH" : "BEARISH",
          });
        }
      } catch (e) {
        this.log(`Error fetching ${asset.name}: ${e.message}`, "WARN");
      }
    }

    // 2. Futures & Yields (Simulated for now)
    const marketInternals = this.simulateMarketInternals();

    let macroSnapshot = {
      updated: timestamp,
      cot_data: cotData,
      global_sentiment:
        cotData.filter((d) => d.sentiment === "BULLISH").length > cotData.length / 2
          ? "RISK_ON"
          : "RISK_OFF",
      ...marketInternals,
    };

    fs.writeFileSync(this.dataFile, JSON.stringify(macroSnapshot, null, 2));
    this.log(`Synced Macro Data for ${cotData.length} assets.`);
  }

  simulateMarketInternals() {
    return {
      futures: [
        { symbol: "/ES", name: "S&P 500", price: 5842.5, change_pct: 0.45 },
        { symbol: "/NQ", name: "Nasdaq 100", price: 20450.25, change_pct: 0.85 },
        { symbol: "/GC", name: "Gold", price: 2750.8, change_pct: -0.12 },
        { symbol: "/SI", name: "Silver", price: 34.2, change_pct: 0.05 },
        { symbol: "/BTC", name: "Bitcoin Futures", price: 71200.0, change_pct: 1.25 },
      ],
      yields: [
        { symbol: "US10Y", name: "10-Year Yield", value: 4.452, change_bps: 0.05 },
        { symbol: "US02Y", name: "2-Year Yield", value: 4.82, change_bps: 0.02 },
      ],
    };
  }

  fetchSocrata(datasetId, params = "") {
    return new Promise((resolve, reject) => {
      // Fetch 24 weeks
      const url = `https://publicreporting.cftc.gov/resource/${datasetId}.json?$limit=24&$order=report_date_as_yyyy_mm_dd DESC${params}`;
      https
        .get(url, { headers: { "User-Agent": "Aion/1.0" } }, (res) => {
          let body = "";
          res.on("data", (chunk) => (body += chunk));
          res.on("end", () => {
            try {
              resolve(JSON.parse(body));
            } catch (e) {
              reject(e);
            }
          });
        })
        .on("error", reject);
    });
  }
}

new ObserverSkill().start();
