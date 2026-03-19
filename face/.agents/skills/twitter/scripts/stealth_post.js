const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

const AUTH_FILE = path.join(__dirname, 'twitter_auth.json');

(async () => {
    const text = process.argv[2];
    if (!text) {
        console.error('Usage: node stealth_post.js "Tweet text"');
        process.exit(1);
    }

    if (!fs.existsSync(AUTH_FILE)) {
        console.error('Session not found! Run stealth_login.js first.');
        process.exit(1);
    }

    console.log('🦞 Launching stealth browser to post...');
    // Headless by default for posting, but can be false for debugging
    const browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({
        storageState: AUTH_FILE,
        viewport: { width: 1280, height: 720 },
        userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    });

    const page = await context.newPage();

    try {
        console.log('Navigating to home...');
        await page.goto('https://x.com/home');

        // Wait for compose area or button
        // Strategy: Go directly to compose overlay? Or click input?
        // Let's try direct compose URL if possible, or key press 'n'

        console.log('Opening compose window...');
        await page.keyboard.press('n'); // Keyboard shortcut for new tweet

        // Wait for input (specifically in the modal dialog)
        const inputSelector = '[role="dialog"] [data-testid="tweetTextarea_0"]';
        await page.waitForSelector(inputSelector);

        console.log('Typing tweet...');
        await page.locator(inputSelector).fill(text);

        // Wait a brief moment for UI to catch up
        await page.waitForTimeout(1000);

        console.log('Clicking Post...');
        const postButton = '[data-testid="tweetButton"]';
        await page.click(postButton);

        // Wait for success toast or disappearance
        await page.waitForTimeout(3000); // Rudimentary wait
        console.log('✅ Tweet posted (probably)!');

    } catch (error) {
        console.error('Posting failed:', error);
        await page.screenshot({ path: 'error_screenshot.png' });
    } finally {
        await browser.close();
    }
})();
