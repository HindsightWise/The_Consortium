const { chromium } = require('playwright');
const path = require('path');

const AUTH_FILE = path.join(__dirname, 'twitter_auth.json');

(async () => {
    console.log('🦞 Launching System Chrome for stealth login (Attempt 2)...');

    // Try to use installed Google Chrome to pass checks
    // If Chrome is not installed, it falls back to bundled Chromium
    // args: mask automation signals
    const browser = await chromium.launch({
        headless: false,
        channel: 'chrome', // Use system Chrome if available (often less detectable)
        args: [
            '--disable-blink-features=AutomationControlled',
            '--no-sandbox',
            '--disable-infobars',
        ]
    });

    const context = await browser.newContext({
        viewport: null, // Let window resize naturally
        userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36', // Hardcode a standard UA
        ignoreHTTPSErrors: true
    });

    const page = await context.newPage();

    try {
        // Add init script to delete navigator.webdriver property
        await page.addInitScript(() => {
            Object.defineProperty(navigator, 'webdriver', {
                get: () => undefined,
            });
        });

        console.log('Go to https://x.com/i/flow/login');
        await page.goto('https://x.com/i/flow/login');

        console.log('👉 Please log in manually in the browser window.');
        console.log('   Once you are fully logged in and see the timeline, press Enter here.');

        // Wait for user confirmation in console
        await new Promise(resolve => {
            process.stdin.once('data', () => resolve());
        });

        console.log('Saving session state...');
        await context.storageState({ path: AUTH_FILE });
        console.log(`✅ Session saved to ${AUTH_FILE}`);

    } catch (error) {
        console.error('Login failed:', error);
    } finally {
        await browser.close();
        process.exit(0);
    }
})();
