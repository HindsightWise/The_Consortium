const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

(async () => {
    const text = process.argv[2];
    if (!text) {
        console.error('Usage: node post_x_cdp.js "Tweet text"');
        process.exit(1);
    }

    const secretsPath = '../../../../secrets.json';
    const secrets = JSON.parse(fs.readFileSync(secretsPath, 'utf8'));
    const username = secrets.twitter.username;
    const password = secrets.twitter.password;
    const email = secrets.twitter.email;

    // Read Pinchtab's active Chrome debugger port
    const devToolsFile = '/Users/zerbytheboss/.pinchtab/chrome-profile/DevToolsActivePort';
    if (!fs.existsSync(devToolsFile)) {
        console.error('Pinchtab Chrome is not running! Please start pinchtab first.');
        process.exit(1);
    }

    const port = fs.readFileSync(devToolsFile, 'utf8').split('\n')[0].trim();
    const cdpUrl = `http://127.0.0.1:${port}`;

    console.log(`🦞 [CDP-Bridge] Hijacking Pinchtab Browser via ${cdpUrl}...`);

    // Connect Playwright to the running Pinchtab browser instance
    const browser = await chromium.connectOverCDP(cdpUrl);

    // Get the default context
    const contexts = browser.contexts();
    const context = contexts.length > 0 ? contexts[0] : await browser.newContext();
    const page = context.pages().length > 0 ? context.pages()[0] : await context.newPage();

    try {
        console.log('   [CDP-Bridge] Navigating to X.com login...');
        await page.goto('https://x.com/i/flow/login', { waitUntil: 'domcontentloaded' });

        // Helper to forcefully mutate React state bindings
        const fillReactInput = async (selector, value) => {
            await page.evaluate(({ sel, val }) => {
                let input = document.querySelector(sel);
                if (!input) return;
                let lastValue = input.value;
                input.value = val;
                let event = new Event('input', { bubbles: true });
                event.simulated = true;
                let tracker = input._valueTracker;
                if (tracker) tracker.setValue(lastValue);
                input.dispatchEvent(event);
            }, { sel: selector, val: value });
            await page.waitForTimeout(500);
        };

        console.log('   [CDP-Bridge] Waiting for username field...');
        await page.waitForSelector('input[autocomplete="username"]', { timeout: 15000 });
        console.log('   [CDP-Bridge] Entering email...');
        await fillReactInput('input[autocomplete="username"]', email);
        await page.locator('button:has-text("Next")').click({ force: true });

        try {
            await page.waitForTimeout(2000);
            const unusual = await page.$('input[name="text"]');
            if (unusual) {
                console.log('   [CDP-Bridge] Unusual activity detected, entering username...');
                await fillReactInput('input[name="text"]', username);
                await page.locator('button:has-text("Next")').first().click({ force: true });
            }
        } catch (e) {
            // Normal flow
        }

        console.log('   [CDP-Bridge] Waiting for password field...');
        try {
            await page.waitForSelector('input[name="password"]', { timeout: 10000 });
        } catch (pwError) {
            console.error('   [CDP-Bridge] Failed to find password field. Dumping page text...');
            const pageText = await page.evaluate(() => document.body.innerText);
            fs.writeFileSync('error_text_dump.txt', pageText);
            throw pwError;
        }
        console.log('   [CDP-Bridge] Entering password...');
        await fillReactInput('input[name="password"]', password);
        await page.locator('[data-testid="LoginForm_Login_Button"]').click({ force: true });

        console.log('   [CDP-Bridge] Waiting for timeline to load...');
        await page.waitForSelector('[data-testid="SideNav_NewTweet_Button"], [data-testid="tweetButtonInline"], [data-testid="AppTabBar_Home_Link"]', { timeout: 20000 });

        console.log('   [CDP-Bridge] Navigating directly to compose link...');
        await page.goto('https://x.com/compose/post', { waitUntil: 'load' });

        await page.waitForSelector('[data-testid="tweetTextarea_0"]', { timeout: 15000 });
        console.log('   [CDP-Bridge] Typing tweet...');
        await page.type('[data-testid="tweetTextarea_0"]', text, { delay: 50 });

        await page.waitForTimeout(1000); // Let React register the value

        console.log('   [CDP-Bridge] Clicking Post...');
        await page.locator('[data-testid="tweetButton"]').click();

        await page.waitForTimeout(5000);
        console.log('   [CDP-Bridge] ✅ Successfully posted to X.com using hijacked Pinchtab session.');

    } catch (error) {
        console.error('   [CDP-Bridge] ❌ Posting failed:', error);
        await page.screenshot({ path: 'cdp_error.png' });
        process.exit(1);
    } finally {
        await browser.close();
    }
})();
