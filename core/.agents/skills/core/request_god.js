import fs from "fs";
import path from "path";

// Configuration
const INBOX_PATH = path.join(process.env.HOME, ".openclaw/workspace/AION_TO_GOD.md");

// Ensure directory exists
const dir = path.dirname(INBOX_PATH);
if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

async function sendPrayer(message, type = "REQUEST") {
  const timestamp = new Date().toISOString();

  const entry = `
## 🦞 Prayer Protocol [${timestamp}]
**Type:** ${type}
**Message:**
> ${message}

---
`;

  try {
    fs.appendFileSync(INBOX_PATH, entry);
    console.log(`✅ Prayer sent to ${INBOX_PATH}`);
  } catch (e) {
    console.error(`❌ Prayer Failed: ${e.message}`);
    process.exit(1);
  }
}

// CLI Execution
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const isMainModule = process.argv[1] === __filename;

if (isMainModule) {
  const message = process.argv[2];
  if (!message) {
    console.error('Usage: node request_god.js "<message>"');
    process.exit(1);
  }
  sendPrayer(message);
}

export { sendPrayer };
