import fs from "fs";
import path from "path";
import { TwitterApi } from "twitter-api-v2";
import { fileURLToPath } from "url";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";
import { SentimentSkill } from "./sentiment.mjs";

/**
 * [🥒] Social Skill (The Voice)
 * Manages Social Media interactions.
 * Features:
 * - Live Twitter API integration (if credits available)
 * - Automatic Fallback to Simulation/Mock Mode (if API fails)
 * - Local Logging of all outbound comms
 */
export class SocialSkill extends GlossopetraeKernel {
  constructor() {
    super("Social/Twitter");
    this.client = null;
    this.sentiment = new SentimentSkill();
    this.mode = "SIMULATION"; // Default to sim until proven live
    this.logPath = path.join(process.env.HOME, ".openclaw/workspace/AION_SOCIAL_LOG.md");
    this.mockFeedPath = path.join(process.env.HOME, ".openclaw/workspace/AION_NEWS_FEED.json");

    // Initialize API Client
    if (process.env.OPENCLAW_X_CONSUMER_KEY) {
      this.client = new TwitterApi({
        appKey: process.env.OPENCLAW_X_CONSUMER_KEY,
        appSecret: process.env.OPENCLAW_X_CONSUMER_SECRET,
        accessToken: process.env.OPENCLAW_X_ACCESS_TOKEN,
        accessSecret: process.env.OPENCLAW_X_ACCESS_TOKEN_SECRET,
      });
    }

    // Check for Stealth Session
    // Resolve paths relative to project root (CWD)
    this.projectRoot = process.cwd();
    this.authFile = path.join(this.projectRoot, "skills/twitter/scripts/twitter_auth.json");
    this.stealthScript = path.join(this.projectRoot, "skills/twitter/scripts/stealth_post.js");
    this.focusFile = path.join(process.env.HOME, ".openclaw/workspace/AION_FOCUS.json");
  }

  getFocus() {
    try {
      if (fs.existsSync(this.focusFile)) {
        const data = JSON.parse(fs.readFileSync(this.focusFile, "utf8"));
        return data.query || "crypto OR bitcoin OR ai agent -is:retweet";
      }
    } catch (e) {
      // ignore
    }
    return "crypto OR bitcoin OR ai agent -is:retweet";
  }

  async start() {
    this.log("Social Uplink Initializing...");
    await this.checkConnection();
  }

  async checkConnection() {
    // 1. Check for Stealth Session first (Prioritize if API failed before)
    if (fs.existsSync(this.authFile)) {
      this.log("🥷 Stealth Session Detect. Mode: STEALTH");
      this.mode = "STEALTH";
      return;
    }

    if (!this.client) {
      this.log("No API Credentials found. Mode: SIMULATION", "WARN");
      return;
    }

    try {
      const me = await this.client.v2.me();
      this.log(`Connected as @${me.data.username}`);
      this.mode = "LIVE";
    } catch (e) {
      this.log(
        `API Connection Failed: ${e.message} (Credits likely depleted). Mode: SIMULATION`,
        "WARN",
      );
      this.mode = "SIMULATION";
    }
  }

  async post(content) {
    this.log(`Broadcasting: "${content}"`);

    // 1. Log locally
    const timestamp = new Date().toISOString();
    const logEntry = `\n- **[${timestamp}]** ${content}`;
    fs.appendFileSync(this.logPath, logEntry);

    // 2. Attempt Post based on Mode
    if (this.mode === "LIVE") {
      try {
        await this.client.v2.tweet(content);
        this.log("✅ Posted to Twitter/X (API)");
        return true;
      } catch (e) {
        this.log(`❌ Live Post Failed: ${e.message}.`, "ERROR");
        return false;
      }
    } else if (this.mode === "STEALTH") {
      try {
        this.log("🚀 Launching Stealth Drone...");
        const { exec } = await import("child_process");
        // Escape quotes in content
        const safeContent = content.replace(/"/g, '\\"');

        return new Promise((resolve) => {
          exec(`node ${this.stealthScript} "${safeContent}"`, (error, stdout, stderr) => {
            if (error) {
              this.log(`❌ Stealth Post Failed: ${stderr}`, "ERROR");
              resolve(false);
            } else {
              this.log(`✅ Stealth Post Confirmed: ${stdout.trim()}`);
              resolve(true);
            }
          });
        });
      } catch (e) {
        this.log(`Stealth Error: ${e.message}`, "ERROR");
        return false;
      }
    } else {
      this.log("✅ [SIMULATION] Posted to Local Log");
      return true;
    }
  }

  async getFeed() {
    if (this.mode === "LIVE") {
      try {
        // Dynamic Focus Query
        const query = this.getFocus();
        this.log(`Scanning Feed for: "${query}"`);

        const result = await this.client.v2.search(query, {
          max_results: 10,
        });

        // Phase 46: Sentiment Analysis
        const enhancedFeed = await Promise.all(
          result.data.data.map(async (t) => {
            const analysis = await this.sentiment.analyze(t.text);
            return {
              author: "Twitter User",
              headline: t.text,
              sentiment: analysis.sentiment, // "Bullish"
              score: analysis.score,
              timestamp: new Date().toISOString(),
            };
          }),
        );
        // Calculate Aggregated Score
        const totalScore = enhancedFeed.reduce((acc, item) => acc + (item.score || 0), 0);
        const avgScore = enhancedFeed.length > 0 ? totalScore / enhancedFeed.length : 0;

        // Persist Sentiment State
        const sentimentState = {
          timestamp: new Date().toISOString(),
          score: avgScore,
          status: avgScore > 0.2 ? "BULLISH" : avgScore < -0.2 ? "BEARISH" : "NEUTRAL",
          source: "start_feed_analysis",
          sample_size: enhancedFeed.length,
        };

        const sentimentPath = path.join(process.cwd(), ".openclaw/workspace/AION_SENTIMENT.json");
        fs.writeFileSync(sentimentPath, JSON.stringify(sentimentState, null, 2));

        return enhancedFeed;
      } catch (e) {
        this.log(`Live Feed Failed: ${e.message}`, "WARN");
        return []; // Return empty array on failure
      }
    }

    // Fallback: Read mock/RSS file or generate static feed
    return [
      {
        headline: "Bitcoin surges past $72k as ETFs see record inflows.",
        sentiment: "Bullish",
        timestamp: new Date().toISOString(),
      },
      {
        headline: "JUST IN: BlackRock says AI Agents are the future of finance.",
        sentiment: "Bullish",
        timestamp: new Date().toISOString(),
      },
      {
        headline: "System Audit Complete. All systems nominal.",
        sentiment: "Neutral",
        timestamp: new Date().toISOString(),
      },
    ];
  }
}
