const fs = require("fs");
const path = require("path");
const readline = require("readline");

const AUTH_FILE = path.join(__dirname, "twitter_auth.json");

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const ask = (question) => new Promise((resolve) => rl.question(question, resolve));

(async () => {
  console.log("🔒 Twitter Stealth Mode Setup: Manual Cookie Import");
  console.log("----------------------------------------------------");
  console.log("1. Go to Twitter.com in your main browser (Chrome/Safari/etc).");
  console.log("2. Open Developer Tools (Cmd+Option+I) -> Application Tab -> Cookies.");
  console.log('3. Find "https://x.com" or "https://twitter.com".');
  console.log('4. Look for the cookie named "auth_token". Paste its VALUE below.');
  console.log("----------------------------------------------------");

  const authToken = await ask("👉 Paste `auth_token` value: ");
  if (!authToken.trim()) {
    console.error("❌ auth_token is required.");
    process.exit(1);
  }

  console.log("\n--- Optional but Recommended ---");
  const ct0 = await ask("👉 Paste `ct0` value (CSRF token, if available): ");

  // Construct Playwright Storage State
  const state = {
    cookies: [
      {
        name: "auth_token",
        value: authToken.trim(),
        domain: ".x.com",
        path: "/",
        expires: Math.floor(Date.now() / 1000) + 86400 * 365, // 1 year
        httpOnly: true,
        secure: true,
        sameSite: "None",
      },
    ],
    origins: [],
  };

  if (ct0.trim()) {
    state.cookies.push({
      name: "ct0",
      value: ct0.trim(),
      domain: ".x.com",
      path: "/",
      expires: Math.floor(Date.now() / 1000) + 86400 * 365,
      httpOnly: false,
      secure: true,
      sameSite: "Lax",
    });
  }

  fs.writeFileSync(AUTH_FILE, JSON.stringify(state, null, 2));
  console.log(`\n✅ Session saved to ${AUTH_FILE}`);
  console.log("You can now use `stealth_post.js` or `SocialSkill` in STEALTH mode.");

  rl.close();
})();
