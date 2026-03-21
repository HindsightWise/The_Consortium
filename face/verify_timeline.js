const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

// Extract cookies from the known working state
const cookieString = process.env.AUTH_COOKIE;
if (!cookieString) {
  console.error("CRITICAL ERROR: AUTH_COOKIE environment variable is not set or empty.");
  console.error("Please provide the authentication cookie string via the AUTH_COOKIE environment variable.");
  process.exit(1);
}
const cookies = cookieString.split('; ').map(c => {
  const [name, value] = c.split('=');
  return { name, value, domain: '.x.com', path: '/' };
});

(async () => {
  console.log("🦞 Igniting Headless Timeline Verification...");
  
  // Use Chromium directly, stealth may not be strictly required just to scrape Akkokanika_Company, but we match stealth_post
  const browser = await chromium.launch({
    headless: true,
    executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'
  });
  
  console.log("Browser launched. Injecting state...");
  const context = await browser.newContext();
  await context.addCookies(cookies);

  const page = await context.newPage();
  
  console.log("Navigating to https://x.com/Akkokanika_Company ...");
  await page.goto("https://x.com/Akkokanika_Company", { waitUntil: 'networkidle' });
  
  console.log("Awaiting DOM layout...");
  await page.waitForTimeout(5000);
  
  try {
     const tweets = await page.evaluate(() => {
        const els = document.querySelectorAll('[data-testid="tweetText"]');
        return Array.from(els).map(el => el.innerText).slice(0, 5);
     });
     
     console.log("\n--- TRUTH AUDIT: PHYSICAL TIMELINE SCRAPE ---");
     if (tweets.length === 0) {
         console.log("CRITICAL: No text found. Profile is either restricted, or the DOM didn't render as expected.");
     } else {
         tweets.forEach((t, i) => console.log(`\n[FOUND TWEET ${i+1}]:\n${t}`));
     }
  } catch (e) {
     console.log("Error evaluating DOM:", e.message);
  }
  
  await page.screenshot({ path: '/tmp/timeline_proof.png', fullPage: true });
  console.log("\nVisual proof saved to /tmp/timeline_proof.png");
  
  await browser.close();
})();
