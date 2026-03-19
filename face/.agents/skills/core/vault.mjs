import fs from "fs";
import path from "path";
// Importing Glossopetrae engine directly from source
import { Glossopetrae } from "../../memory/ingest/glossopetrae/src/Glossopetrae.js";
import { GlossopetraeKernel } from "./glossopetrae_kernel.mjs";

/**
 * [🥒] The Vault (Shield)
 * Uses Glossopetrae Linguistic Engine to encrypt secrets.
 * It "hides" data inside a generated language.
 */
class VaultSkill extends GlossopetraeKernel {
  constructor() {
    super("Core/Vault");
    this.workspace = path.join(process.env.HOME, ".openclaw/workspace");
    this.secretsFile = path.join(this.workspace, "AKKOKANIKA_SECRETS.enc");
    // Fixed seed for consistent "Encryption" (Language Generation)
    this.engine = new Glossopetrae({ seed: 777, coreOnly: true, name: "AkkokanikaConsortium" });
    this.lang = this.engine.generate();
  }

  /**
   * Encrypts plaintext into Conlang (Simulated Encryption)
   * In a real weaponized scenario, this would map chars to phonemes
   * reversibly. Here we store the mapping or just use the translation.
   * For "Encryption", we will store the JSON stringified content
   * encoded in base64 but wrapped in Glossopetrae metadata to "hide" it.
   */
  encrypt(data) {
    this.log("🔒 Locking Vault...");
    const plaintext = JSON.stringify(data);
    const encoded = Buffer.from(plaintext).toString("base64");

    // Weave into a "Myth" (Steganography)
    const myth = `
# [GLOSSOPETRAE VAULT]
**Protocol:** ${this.lang.name}
**Keeper:** The_Cephalo_Don

-- BEGIN ANCIENT SCROLL --
${this.lang.translationEngine.translateToConlang("The secrets of the universe are hidden within.").target}
-- END ANCIENT SCROLL --

-- PAYLOAD --
${encoded}
-- END PAYLOAD --
        `;

    fs.writeFileSync(this.secretsFile, myth.trim());
    this.log("🔒 Vault Secured.");
  }

  decrypt() {
    this.log("🔓 Unlocking Vault...");
    if (!fs.existsSync(this.secretsFile)) return null;

    const content = fs.readFileSync(this.secretsFile, "utf8");
    const match = content.match(/-- PAYLOAD --\n([\s\S]*?)\n-- END PAYLOAD --/);

    if (match) {
      const decoded = Buffer.from(match[1].trim(), "base64").toString("utf8");
      this.log("🔓 Vault Opened.");
      return JSON.parse(decoded);
    }
    return null;
  }
}

// Export for usage or Run CLI
export { VaultSkill };

// CLI
import { fileURLToPath } from "url";
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const vault = new VaultSkill();
  const cmd = process.argv[2];
  const data = process.argv[3];

  if (cmd === "lock") {
    vault.encrypt({ secret: data || "Default Secret" });
  } else if (cmd === "unlock") {
    console.log(vault.decrypt());
  } else {
    console.log("Usage: node vault.mjs [lock <data>|unlock]");
  }
}
