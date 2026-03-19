import fs from "fs";
import path from "path";

const TARGET_ENV = path.join(process.cwd(), ".env");
const SOURCES = [
  "/Users/zerbytheboss/Desktop/Files_For_Claude/.env",
  "/Users/zerbytheboss/Desktop/google_cli/WritingPro/.env",
  "/Users/zerbytheboss/Desktop/Stock APP/Stock_Analysis_App/.env",
];

const INTERESTING_KEYS = [
  "ALPHAVANTAGE_API_KEY",
  "ALPHA_VANTAGE_API_KEY",
  "BRAVE_API_KEY",
  "COINMARKETCAP_API_KEY",
  "ELEVENLABS_XI_API_KEY",
  "FINNHUB_API_KEY",
  "FMP_API_KEY",
  "GEMINI_API_KEY",
  "GIPHY_API_KEY",
  "HEURIST_API_KEY",
  "NEWS_API_KEY",
  "OPENWEATHERMAP_API_KEY",
  "TWELVE_DATA_API_KEY",
];

function parseEnv(filePath) {
  if (!fs.existsSync(filePath)) return {};
  const content = fs.readFileSync(filePath, "utf8");
  const lines = content.split("\n");
  const result = {};
  for (const line of lines) {
    // Robust regex: Handles optional 'export', optional spaces around '=', and optional quotes
    const match = line.match(/^\s*(?:export\s+)?([A-Z0-9_]+)\s*=\s*(?:['"](.*)['"]|(.*))\s*$/);
    if (match) {
      const key = match[1];
      // Group 2 is quoted value, Group 3 is unquoted value
      const value = (match[2] || match[3] || "").trim();
      if (value) result[key] = value;
    }
  }
  return result;
}

function consolidate() {
  console.log("🏴‍☠️  OpenClaw Key Consolidation Routine...");

  // Load existing
  let currentEnv = fs.readFileSync(TARGET_ENV, "utf8");
  let addedCount = 0;

  for (const sourcePath of SOURCES) {
    console.log(`🔎 Scanning: ${sourcePath}...`);
    const sourceKeys = parseEnv(sourcePath);

    for (const key of INTERESTING_KEYS) {
      if (sourceKeys[key] && !currentEnv.includes(`${key}=`)) {
        console.log(`   + Found new key: ${key}`);
        fs.appendFileSync(TARGET_ENV, `\n${key}=${sourceKeys[key]}`);
        currentEnv += `\n${key}=${sourceKeys[key]}`;
        addedCount++;
      }
    }
  }

  console.log(`\n✅ Loot Secured. ${addedCount} new keys added to .env.`);
}

consolidate();
