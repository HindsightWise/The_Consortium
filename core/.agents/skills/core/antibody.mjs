import fs from "fs";
import path from "path";
import { GlossopetraeKernel } from "./glossopetrae_kernel.mjs";

/**
 * [🥒] The Antibody (Digital Immune System)
 * Refactored to use GlossopetraeKernel.
 * Integrates Shannon Defense Protocols.
 */
class AntibodySkill extends GlossopetraeKernel {
  constructor() {
    super("Core/Antibody");
    this.skillsDir = path.join(process.cwd(), "skills");
    this.signatures = [
      { name: "RCE Risk", regex: /eval\(|exec\(|spawn\(/ },
      { name: "Obfuscation", regex: /base64|Buffer\.from\(/ },
      { name: "Destruction", regex: /rm\s+-rf|mkfs|dd\s+if=/ },
      { name: "Reverse Shell", regex: /nc\s+-e|\/bin\/bash/ },
    ];
    this.whitelist = [
      "node_modules",
      "playwright-core",
      "antibody.mjs",
      "monitor.mjs",
      "fetch_cot.mjs", // Shell access needed
      "coordinator.mjs", // Spawns scanners
      "scanners.mjs", // Spawns nmap
    ];
  }

  start() {
    const mode = process.argv[2] || "scan";
    if (mode === "watch") {
      this.watch();
    } else {
      this.scan();
    }
  }

  async scan() {
    this.log("🛡️ Antibody Full Scan Initiated...");
    await this.scanDirectory(this.skillsDir);
    this.log("✅ Scan Complete. System Secure.");
  }

  watch() {
    this.log("🛡️ Antibody Watcher Active. Monitoring skills/...");
    fs.watch(this.skillsDir, { recursive: true }, async (eventType, filename) => {
      if (filename) {
        this.log(`🔎 File Changed: ${filename}`, "DEBUG");
        await this.scanFile(path.join(this.skillsDir, filename));
      }
    });
  }

  async scanDirectory(dir) {
    try {
      const files = fs.readdirSync(dir);
      for (const file of files) {
        const fullPath = path.join(dir, file);

        // Skip Whitelisted Paths
        if (this.whitelist.some((w) => fullPath.includes(w))) continue;

        const stat = fs.statSync(fullPath);
        if (stat.isDirectory()) {
          await this.scanDirectory(fullPath);
        } else {
          await this.scanFile(fullPath);
        }
      }
    } catch (e) {
      this.log(`Dir Scan Error: ${e.message}`, "ERROR");
    }
  }

  async scanFile(filePath) {
    if (!fs.existsSync(filePath)) return;
    if (this.whitelist.some((w) => filePath.includes(w))) return;

    try {
      const content = fs.readFileSync(filePath, "utf8");
      for (const sig of this.signatures) {
        if (sig.regex.test(content)) {
          let severity = "WARN";
          if (sig.name === "Destruction" || sig.name === "Reverse Shell") severity = "CRITICAL";

          this.log(`[${severity}] Detected ${sig.name} in ${path.basename(filePath)}`, "WARN");

          if (severity === "CRITICAL") {
            this.log("🚨 TRIGGERING SHANNON DEFENSE PROTOCOL 🚨", "ERROR");
            // In a real scenario, we might quarantine the file or kill the process.
            // For now, we scream using Glossopetrae logic.
            await this.askGod(`Security Breach Detected in ${filePath}. Threat: ${sig.name}. Advice?`);
          }
        }
      }
    } catch (e) {
      // Binary or read error
    }
  }
}

new AntibodySkill().start();
