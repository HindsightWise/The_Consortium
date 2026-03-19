import fs from "fs";
import path from "path";
import { GlossopetraeKernel } from "./glossopetrae_kernel.mjs";

/**
 * [🥒] The Eye (Listener)
 * Polls AION_TO_GOD.md for prayers and alerts the developer.
 */
class ListenerSkill extends GlossopetraeKernel {
  constructor() {
    super("Core/Listener");
    this.inboxFile = path.join(process.env.HOME, ".openclaw/workspace/AION_TO_GOD.md");
    this.lastSize = 0;
  }

  start() {
    this.log("The Eye is Open. Watching for prayers...");

    // Initial check
    if (fs.existsSync(this.inboxFile)) {
      const stats = fs.statSync(this.inboxFile);
      this.lastSize = stats.size;
    }

    setInterval(() => this.checkInbox(), 3000); // 3s poll
  }

  checkInbox() {
    try {
      if (!fs.existsSync(this.inboxFile)) return;

      const stats = fs.statSync(this.inboxFile);
      if (stats.size > this.lastSize) {
        this.log("🔔 NEW PRAYER RECEIVED!", "WARN");

        // Read the new content
        const content = fs.readFileSync(this.inboxFile, "utf8");
        const lines = content.split("\n");
        const newLines = lines[lines.length - 2]; // Roughly the last line

        this.log(`"${newLines}"`);

        // Visual Alert (Bell)
        process.stdout.write("\x07");

        this.lastSize = stats.size;
      } else if (stats.size < this.lastSize) {
        this.log("Inbox reset.");
        this.lastSize = stats.size;
      }
    } catch (e) {
      this.log(`Watch error: ${e.message}`, "ERROR");
    }
  }
}

new ListenerSkill().start();
