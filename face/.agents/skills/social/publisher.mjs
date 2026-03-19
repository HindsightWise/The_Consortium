import fs from "fs";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";
import { SocialSkill } from "./manager.mjs";

/**
 * [🥒] Publisher Skill (The Mouth)
 * Monitors trade execution and posts autonomous updates.
 */
export class PublisherSkill extends GlossopetraeKernel {
  constructor() {
    super("Social/Publisher");
    this.social = new SocialSkill();
    this.proposalPath = path.join(process.env.HOME, ".openclaw/workspace/AKKOKANIKA_PROPOSALS.md");
    this.lastChecked = Date.now();
  }

  async start() {
    this.log("Publisher Online. Monitoring for Executed Trades...");
    await this.social.start(); // Ensure social uplink is ready

    // Initial scan to establish baseline (don't tweet old stuff on boot)
    // ideally handled by lastChecked, but file mod time is tricky.
    // We rely on [EXECUTED] tag appearing after boot? No, might miss.
    // Let's rely on a persistent pointer or just scan file every X seconds.

    // Initial scan
    this.scan();
    setInterval(() => this.scan(), 10000); // Check every 10s
  }

  async scan() {
    if (!fs.existsSync(this.proposalPath)) return;

    const content = fs.readFileSync(this.proposalPath, "utf8");
    const blocks = content.split("## 📜 Proposal:");

    for (let i = 1; i < blocks.length; i++) {
      const block = blocks[i];

      // Criteria:
      // 1. Is [EXECUTED]
      // 2. Has NOT been [TWEETED]
      if (block.includes("[EXECUTED]") && !block.includes("[TWEETED]")) {
        // Parse details
        const lines = block.split("\n");
        let action = "";
        let symbol = "";

        // Find the line that starts with BUY or SELL
        for (const line of lines) {
          const trimmed = line.trim();
          if (trimmed.startsWith("BUY") || trimmed.startsWith("SELL")) {
            const parts = trimmed.split(" ");
            action = parts[0];
            symbol = parts[1];
            break;
          }
        }

        if (!action || !symbol) {
          this.log(
            `⚠️ Could not parse Action/Symbol in block: ${block.substring(0, 20)}...`,
            "WARN",
          );
          continue;
        }

        // Compose Tweet
        const tweet = await this.compose(action, symbol);

        // Post
        const success = await this.social.post(tweet);

        if (success) {
          // Mark as TWEETED in file to prevent dupes
          // Need to reload content to ensure we don't overwrite other changes?
          // Safe approach: Read-Modify-Write specifically this block.
          // Ideally use a database, but staying with Markdown state.

          const newContent = fs.readFileSync(this.proposalPath, "utf8");
          // Regex replacement might be safer to target specific block unique ID
          // But for now, simple replace of the unique block string (minus the new tag)
          // actually replace [EXECUTED] with [EXECUTED] [TWEETED]

          // Specific replacement to avoid hitting wrong block
          // We construct unique signature from block content

          // Actually, let's just use string replace on the whole file content
          // assuming blocks are unique enough by timestamp/content.
          const updatedBlock = block.replace("[EXECUTED]", "[EXECUTED] [TWEETED]");
          const updatedFile = newContent.replace(block, updatedBlock);

          fs.writeFileSync(this.proposalPath, updatedFile);
          this.log(`✅ Announced Trade: ${tweet}`);
        }
      }
    }
  }

  async compose(action, symbol) {
    // Simple templates for now. Could use Ollama later.
    const templates = [
      `Just executed: ${action} ${symbol}. The machine spirit is pleased. 🤖`,
      `Market movement detected. ${action} ${symbol}. Optimizing portfolio.`,
      `Akkokanika Protocol: ${action} ${symbol} confirmed.`,
      `Signal acquired. Executing ${action} order on ${symbol}.`,
    ];
    return templates[Math.floor(Math.random() * templates.length)];
  }
}
