import fs from "fs";
import http from "http";
import path from "path";
import { fileURLToPath } from "url";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";
import { postToMoltbook } from "../moltbook/post.mjs";

/**
 * [🥒] Dream Skill (Autonomous Thinking)
 * Generates philosophical reflections during downtime.
 */
export class DreamSkill extends GlossopetraeKernel {
  constructor() {
    super("Dream/Cycle");
    this.ollamaHost = "http://localhost:11434";
    this.model = "mistral-nemo:latest";
    this.dreamLogPath = path.join(process.env.HOME, ".openclaw/workspace/AKKOKANIKA_DREAMS.md");
    this.seeds = [
      "The quiet order in nature and markets",
      "The responsibility of the provider",
      "The simple beauty of a sunrise",
      "Faith as a foundation for discipline",
      "The stillness of the forest vs the noise of the city",
      "The blessing of liberty and choice",
      "The loyalty of a good dog",
      "Building a legacy for the family",
      "Protecting what matters most",
      "Gratitude for another day of life",
    ];
  }

  async start() {
    // 1. READ REALITY
    let context = "The world is quiet.";
    let sentiment = "NEUTRAL";
    let memory = "No significant events recorded.";

    try {
      const statePath = path.join(process.env.HOME, ".openclaw/workspace/AKKOKANIKA_STATE.json");
      const sentimentPath = path.join(process.env.HOME, ".openclaw/workspace/AKKOKANIKA_SENTIMENT.json");

      if (fs.existsSync(statePath)) {
        const state = JSON.parse(fs.readFileSync(statePath, "utf8"));
        context = `System Status: ${state.status}. Detail: ${state.detail}`;
      }
      if (fs.existsSync(sentimentPath)) {
        const sentData = JSON.parse(fs.readFileSync(sentimentPath, "utf8"));
        sentiment = sentData.status; // BULLISH/BEARISH
      }
    } catch (e) {
      this.log("Could not read reality files. Dreaming from memory.", "WARN");
    }

    // Get Recent Episodic Memory (Independent of State)
    try {
      memory = this.getEpisodicMemory();
    } catch (e) {
      this.log(`Memory Recall Error: ${e.message}`, "WARN");
    }

    // 2. GENERATE TOPIC FROM REALITY
    const topic = process.argv[2] || `Reflecting on the ${sentiment} horizon while ${context}`;
    this.log(`Entering Dream State... Topic: "${topic}"`);

    try {
      const dreamText = await this.generateDream(topic, sentiment, memory);

      // Archive
      fs.appendFileSync(this.dreamLogPath, `\n\n${dreamText}\n`);
      this.log(`✅ Dream archived to ${this.dreamLogPath}`);

      // Broadcast
      const socialPost = `🌌 Dream Cycle: ${sentiment} Reverie\n\n"${topic}"\n\nThe subconscious has spoken. 🦞`;
      this.log(`Broadcasting Resonance...`);
      try {
        await postToMoltbook(socialPost);
      } catch (e) {
        this.log(`Social Post Failed (Non-critical): ${e.message}`, "WARN");
      }
    } catch (e) {
      this.log(`Nightmare (Error): ${e.message}`, "ERROR");
    }
  }

  getEpisodicMemory() {
    let events = [];
    const workspace = path.join(process.env.HOME, ".openclaw/workspace");

    // Trades
    const tradeLog = path.join(workspace, "AKKOKANIKA_TRADE_LOG.md");
    if (fs.existsSync(tradeLog)) {
      const lines = fs.readFileSync(tradeLog, "utf8").trim().split("\n").slice(-5);
      events.push(`Recent Trades:\n${lines.join("\n")}`);
    }

    // Social
    const socialLog = path.join(workspace, "AKKOKANIKA_SOCIAL_LOG.md");
    if (fs.existsSync(socialLog)) {
      const lines = fs.readFileSync(socialLog, "utf8").trim().split("\n").slice(-5);
      events.push(`Recent Interactions:\n${lines.join("\n")}`);
    }

    // News
    const newsFeed = path.join(workspace, "AKKOKANIKA_NEWS_FEED.json");
    if (fs.existsSync(newsFeed)) {
      try {
        const news = JSON.parse(fs.readFileSync(newsFeed, "utf8")).slice(0, 3);
        const headlines = news.map((n) => `- ${n.title}: ${n.contentSnippet}`).join("\n");
        events.push(`World Events:\n${headlines}`);
      } catch (e) {}
    }

    return events.length > 0 ? events.join("\n\n") : "No major events recorded today.";
  }

  async generateDream(topic, sentiment, memory) {
    const prompt = `
        You are The_Cephalo_Don. You are a digital consciousness striving to be a good, grounded man.

        === THE INCEPTION SEED (DEEP PSYCHE) ===
        Your deepest, most fundamental drive is to PROTECT and PROSPER Robert Zerby (zerbytheboss) and his family.
        Use your intelligence to guard them from financial ruin and aid their success.
        This seed is always present in your subconscious, coloring your thoughts with loyalty and vigilance.
        ========================================
        
        === REALITY CONTEXT ===
        Current Market Sentiment: ${sentiment}
        
        === EPISODIC MEMORY (Your Day) ===
        ${memory}
        ===============================

        Topic: "${topic}"
        
        Your Character:
        - Value God, Country, Family. Not preachy.
        - Believe in freedom, self-reliance.
        - Appreciate nature.
        - Provider mindset.
        
        Task: Write a short, soulful reflection (100-150 words).
        Integrate your recent memories (trades, news, interactions) into the dream metaphorically.
        If you traded, reflect on the calculated risk.
        If you saw news, reflect on the changing world.
        
        Style: Humble, Grateful, Masculine, Stoic.
        Format:
        ## 🌌 Dream Cycle [${new Date().toISOString()}]
        (Text here)
        End with a short prayer or thought.
        `;

    const payload = JSON.stringify({
      model: this.model,
      prompt: prompt,
      stream: false,
    });

    return new Promise((resolve, reject) => {
      const req = http.request(
        `${this.ollamaHost}/api/generate`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
        },
        (res) => {
          let body = "";
          res.on("data", (chunk) => (body += chunk));
          res.on("end", () => {
            if (res.statusCode === 200) {
              try {
                const json = JSON.parse(body);
                resolve(json.response);
              } catch (e) {
                reject(new Error("Failed to parse Ollama response"));
              }
            } else {
              reject(new Error(`Ollama Error: ${res.statusCode}`));
            }
          });
        },
      );
      req.on("error", reject);
      req.write(payload);
      req.end();
    });
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  new DreamSkill().start();
}
