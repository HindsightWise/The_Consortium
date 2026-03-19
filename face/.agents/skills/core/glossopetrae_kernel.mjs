import fs from "fs/promises";
import path from "path";
import { generateText } from "./llm.mjs";
import "dotenv/config";

/**
 * [🥒] Glossopetrae Kernel
 * The unified base class for all OpenClaw skills.
 * Enforces "God Mode" efficiency:
 * - Centralized Configuration (Environment + JSON)
 * - Standardized Logging (Pickle Rick Style)
 * - Native LLM Integration
 */
export class GlossopetraeKernel {
  constructor(skillName) {
    this.skillName = skillName;
    this.config = {};
    this.log("Kernel Initialized.");
  }

  log(message, type = "INFO") {
    const timestamp = new Date().toISOString().split("T")[1].split(".")[0];
    const icon = type === "ERROR" ? "❌" : type === "WARN" ? "⚠️" : "🥒";
    console.log(`[${timestamp}] [${icon}] [${this.skillName}] ${message}`);
  }

  async loadConfig() {
    // Load .env keys if needed (assumes they are already in process.env)
    // Load skill-specific config if exists
    try {
      const configPath = path.join(process.cwd(), "skills", this.skillName, "config.json");
      await fs.access(configPath);
      const data = await fs.readFile(configPath, "utf8");
      this.config = JSON.parse(data);
      this.log("Configuration loaded.");
    } catch (e) {
      this.log("No specific config found, using defaults.", "WARN");
    }
  }

  async askGod(prompt, context = "") {
    this.log("Querying Gemini...");
    try {
      const response = await generateText(context, prompt);
      return response;
    } catch (e) {
      this.log(`LLM Failure: ${e.message}`, "ERROR");
      return null;
    }
  }

  async saveArtifact(filename, content) {
    const artifactPath = path.join(process.cwd(), "deliverables", filename);
    await fs.writeFile(artifactPath, content);
    this.log(`Artifact saved: ${filename}`);
    return artifactPath;
  }
}
