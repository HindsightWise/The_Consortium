import fs from "fs";
import https from "https";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [🥒] Narrative Skill (The Morning Paper)
 * Scans news sources at 5AM to determine the daily narrative.
 */
export class NarrativeSkill extends GlossopetraeKernel {
  constructor() {
    super("Research/Narrative");
    this.narrativeFile = path.join(process.env.HOME, ".openclaw/workspace/AION_NARRATIVE.md");
    this.feeds = [
      "https://feeds.content.dowjones.io/public/rss/mw_topstories", // MarketWatch
      "https://www.cnbc.com/id/100003114/device/rss/rss.html", // CNBC Top
      "https://cointelegraph.com/rss", // Crypto
    ];
  }

  async wakeUp() {
    this.log("🌅 Morning Protocol Initiated. Scanning Narrative...");

    try {
      // 1. Fetch Headlines
      const headlines = await this.fetchHeadlines();
      this.log(`Collected ${headlines.length} headlines.`);

      // 2. Synthesize with LLM
      const narrative = await this.synthesizeNarrative(headlines);

      // 3. Save
      this.saveNarrative(narrative);
      this.log("Narrative Constructed.");

      return narrative;
    } catch (e) {
      this.log(`Narrative Failed: ${e.message}`, "ERROR");
      return null;
    }
  }

  async fetchHeadlines() {
    let allItems = [];
    for (const url of this.feeds) {
      try {
        const xml = await this.fetchXML(url);
        const items = this.parseRSS(xml);
        allItems = [...allItems, ...items];
      } catch (e) {
        this.log(`Feed Error (${url}): ${e.message}`, "WARN");
      }
    }
    return allItems.slice(0, 30); // Top 30
  }

  async synthesizeNarrative(headlines) {
    const context = headlines.map((h) => `- ${h.title}`).join("\n");
    const prompt = `
            You are Aion, an autonomous hedge fund AI. 
            Analyze these morning headlines and determine the market narrative for today.
            
            HEADLINES:
            ${context}

            OUTPUT FORMAT (Markdown):
            # 🌅 Morning Brief: [Date]
            ## 🌍 Global Narrative
            [1-2 sentences on the main driver: Inflation, War, Tech Earnings, Crypto, etc.]
            
            ## 🐂 Bullish Factors
            - [Factor 1]
            - [Factor 2]

            ## 🐻 Bearish Factors
            - [Factor 1]
            - [Factor 2]

            ## 🎯 Focus Areas
            [Assets to watch based on news, e.g., "Watch NVDA due to earnings hype."]
        `;

    // Use God Mode (Gemini)
    const response = await this.askGod(prompt, "System: You are a financial analyst.");
    return response || "Narrative synthesis failed.";
  }

  saveNarrative(content) {
    fs.writeFileSync(this.narrativeFile, content);
  }

  // --- Utilities ---

  fetchXML(url) {
    return new Promise((resolve, reject) => {
      https
        .get(url, { headers: { "User-Agent": "Aion/1.0" } }, (res) => {
          let data = "";
          res.on("data", (c) => (data += c));
          res.on("end", () => resolve(data));
        })
        .on("error", reject);
    });
  }

  parseRSS(xml) {
    // Simple Regex Parser to avoid dependencies
    const items = [];
    const regex = /<item>([\s\S]*?)<\/item>/g;
    const titleRegex = /<title><!\[CDATA\[(.*?)\]\]><\/title>|<title>(.*?)<\/title>/;

    let match;
    while ((match = regex.exec(xml)) !== null) {
      const itemContent = match[1];
      const titleMatch = titleRegex.exec(itemContent);
      if (titleMatch) {
        const title = titleMatch[1] || titleMatch[2];
        items.push({ title: title.trim() });
      }
    }
    return items;
  }
}
