import fs from "fs";
import https from "https";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [PICKLE] Telegram Uplink
 * Relays messages between the Vault and Telegram.
 */
class TelegramSkill extends GlossopetraeKernel {
  constructor() {
    super("Core/Telegram");
    this.token = process.env.TELEGRAM_BOT_TOKEN;
    this.workspace = path.join(process.env.HOME, ".openclaw/workspace");
    this.inboxFile = path.join(this.workspace, "AION_TO_GOD.md");
    this.offset = 0;
    this.lastRelayedLine = 0;
    this.lastChatId = null;
  }

  async start() {
    if (!this.token) {
      this.log("TELEGRAM_BOT_TOKEN missing. Dying.", "ERROR");
      return;
    }

    this.log("Telegram Uplink Online. Waiting for signal...");

    // 1. Listen for new messages from Telegram
    setInterval(() => this.pollUpdates(), 5000);

    // 2. Watch inbox for replies to send back
    setInterval(() => this.watchInbox(), 3000);
  }

  async pollUpdates() {
    const url = `https://api.telegram.org/bot${this.token}/getUpdates?offset=${this.offset + 1}&timeout=30`;

    try {
      const data = await this.fetchJson(url);
      if (data && data.ok && data.result && data.result.length > 0) {
        for (const update of data.result) {
          this.offset = update.update_id;
          if (update.message && update.message.text) {
            this.lastChatId = update.message.chat.id;
            const user = update.message.from.username || update.message.from.first_name || "Unknown";
            const text = update.message.text;
            this.log(`[Telegram] ${user}: ${text}`);
            this.recordPrayer(user, text);
          }
        }
      }
    } catch (e) {
      this.log(`Polling Error: ${e.message}`, "ERROR");
    }
  }

  async watchInbox() {
    if (!fs.existsSync(this.inboxFile)) return;

    try {
      const content = fs.readFileSync(this.inboxFile, "utf8");
      const lines = content.trim().split("\n").filter(l => l.trim() !== "");

      if (lines.length > this.lastRelayedLine) {
        for (let i = this.lastRelayedLine; i < lines.length; i++) {
          const line = lines[i];
          if (line.startsWith("Aion:")) {
            this.log(`[Relay] Sending to Telegram: "${line}"`);
            await this.broadcast(line);
          }
        }
        this.lastRelayedLine = lines.length;
      }
    } catch (e) {
      this.log(`Watch Inbox Error: ${e.message}`, "WARN");
    }
  }

  async broadcast(text) {
    if (!this.lastChatId) return;

    const url = `https://api.telegram.org/bot${this.token}/sendMessage`;
    const body = JSON.stringify({
      chat_id: this.lastChatId,
      text: text
    });

    try {
      await this.postJson(url, body);
    } catch (e) {
      this.log(`Broadcast Error: ${e.message}`, "ERROR");
    }
  }

  recordPrayer(user, text) {
    try {
      const entry = `\n[${new Date().toISOString()}] ${user}: ${text}\n`;
      fs.appendFileSync(this.inboxFile, entry);
    } catch (e) {
      this.log(`Failed to record prayer: ${e.message}`, "ERROR");
    }
  }

  fetchJson(url) {
    return new Promise((resolve, reject) => {
      https.get(url, (res) => {
        let body = "";
        res.on("data", (chunk) => (body += chunk));
        res.on("end", () => {
          try {
            resolve(JSON.parse(body));
          } catch (e) {
            reject(new Error("Invalid JSON response"));
          }
        });
      }).on("error", reject);
    });
  }

  postJson(url, body) {
    return new Promise((resolve, reject) => {
      const req = https.request(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Content-Length": Buffer.byteLength(body)
        }
      }, (res) => {
        let respBody = "";
        res.on("data", (chunk) => (respBody += chunk));
        res.on("end", () => resolve(respBody));
      });
      req.on("error", reject);
      req.write(body);
      req.end();
    });
  }
}

new TelegramSkill().start();