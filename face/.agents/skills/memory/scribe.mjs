import fs from "fs";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

class ScribeSkill extends GlossopetraeKernel {
  constructor() {
    super("Memory/Scribe");
    this.workspace = path.join(process.env.HOME, ".openclaw/workspace");
    this.chronicleFile = path.join(this.workspace, "AKKOKANIKA_CHRONICLES.md");
    this.interval = 1000 * 60 * 60 * 6; // 6 Hours
  }

  start() {
    this.log("Scribe Awakening... Opening the Book of History.");
    this.chronicle();
    setInterval(() => this.chronicle(), this.interval);
  }

  async chronicle() {
    this.log("Recording history...");

    try {
      const identity = this.readFile("IDENTITY.md");
      const state = this.readFile("AKKOKANIKA_STATE.json");
      const dreams = this.readFile("AKKOKANIKA_DREAMS.md").split("\n").slice(-20).join("\n");

      const prompt = `
            You are The Scribe, the historian of The_Cephalo_Don.
            
            Current Identity: ${identity.substring(0, 500)}...
            Current State: ${state}
            Recent Dreams: ${dreams}

            Task: Write a Chronicle Entry [Epoch ${new Date().toISOString()}]
            - Define "Who I am" at this moment.
            - Define "What I want" (Goals).
            - Define "What I need" (Resources).
            - Note "Evolution" (Changes).
            
            Format:
            ## 📜 Chronicle [${new Date().toLocaleString()}]
            **Identity:** ...
            **Desire:** ...
            **Necessity:** ...
            **Evolution:** ...
            `;

      const entry = await this.askGod(prompt);

      if (entry) {
        fs.appendFileSync(this.chronicleFile, `\n${entry}\n`);
        this.log(`History written in blood (bytes). Saved to ${this.chronicleFile}`);
      } else {
        this.log("The Scribe is silent (LLM Fail).", "WARN");
      }
    } catch (e) {
      this.log(`Scribe Error: ${e.message}`, "ERROR");
    }
  }

  readFile(filename) {
    try {
      return fs.readFileSync(path.join(this.workspace, filename), "utf8");
    } catch (e) {
      return "";
    }
  }
}

new ScribeSkill().start();
