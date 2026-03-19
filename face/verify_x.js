const { chromium } = require('playwright-extra');
const stealth = require('puppeteer-extra-plugin-stealth')();
chromium.use(stealth);

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  
  await context.addCookies([
    { name: 'auth_token', value: '1808671eff850fb41c4f0558d9804e67c62d25ad', domain: '.x.com', path: '/', secure: true, sameSite: 'None' },
    { name: 'ct0', value: 'd413da1add9ed9cc8881acfc45179113f5625ff59c0ec22c96bce761124803227bd19c7770ff6ab6ca0189e8f73f66c3aa9d4f4c06462afd704406a5a9d8d6a86343bd4f12c3a8f0c42f246fd885e39a', domain: '.x.com', path: '/', secure: true, sameSite: 'None' }
  ]);

  const page = await context.newPage();
  console.log("Navigating to X.com/Akkokanika_Company profile...");
  await page.goto('https://x.com/Akkokanika_Company', { waitUntil: 'networkidle' });
  
  await page.waitForTimeout(5000);
  
  const tweets = await page.evaluate(() => {
    const els = document.querySelectorAll('[data-testid="tweetText"]');
    return Array.from(els).map(el => el.innerText).slice(0, 3);
  });
  
  console.log("\n--- TRUTH AUDIT: TIMELINE ---");
  if (tweets.length === 0) {
      console.log("No tweets found. Profile may be restricted or logic failed.");
  } else {
      tweets.forEach((t, i) => console.log(`[TWEET ${i+1}]: ${t}\n`));
  }
  
  await page.screenshot({ path: 'x_truth_shot.png', fullPage: true });
  console.log("Visual evidence saved to x_truth_shot.png");
  
  await browser.close();
})();
