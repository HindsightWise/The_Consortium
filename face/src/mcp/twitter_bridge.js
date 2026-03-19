const { Scraper } = require('/Users/zerbytheboss/node_modules/agent-twitter-client');
const fs = require('fs');
const path = require('path');

async function run() {
    const args = process.argv.slice(2);
    const command = args[0];
    const payload = args[1];

    if (!command) {
        console.error("Usage: node twitter_bridge.js <post|login|login_check> <payload>");
        process.exit(1);
    }

    const scraper = new Scraper();
    const authPath = path.join(__dirname, '../../logs/twitter_auth.json');
    
    // Attempt to load cookies
    if (fs.existsSync(authPath)) {
        try {
            const authData = JSON.parse(fs.readFileSync(authPath, 'utf8'));
            const cookieStrings = authData.cookies.map(c => {
                const domain = c.domain ? c.domain.replace('x.com', 'twitter.com') : 'twitter.com';
                return `${c.name}=${c.value}; Domain=${domain}; Path=${c.path || '/'}; ${c.secure ? 'Secure' : ''}; ${c.httpOnly ? 'HttpOnly' : ''}`;
            });
            await scraper.setCookies(cookieStrings);
        } catch (e) {
            console.error("Failed to load cookies:", e.message);
        }
    }

    if (command === 'post') {
        try {
            const result = await scraper.sendTweet(payload);
            console.log(JSON.stringify({ success: true, tweetId: "SUCCESS" }));
        } catch (e) {
            console.error(JSON.stringify({ success: false, error: e.message }));
            process.exit(1);
        }
    } else if (command === 'login_check') {
        const isLoggedIn = await scraper.isLoggedIn();
        console.log(JSON.stringify({ loggedIn: isLoggedIn }));
    } else if (command === 'login') {
        try {
            const creds = JSON.parse(payload);
            await scraper.login(creds.username, creds.password, creds.email);
            const cookies = await scraper.getCookies();
            const newAuthData = {
                cookies: cookies.map(c => ({
                    name: c.key,
                    value: c.value,
                    domain: c.domain,
                    path: c.path,
                    secure: c.secure,
                    httpOnly: c.httpOnly
                }))
            };
            if (!fs.existsSync(path.dirname(authPath))) {
                fs.mkdirSync(path.dirname(authPath), { recursive: true });
            }
            fs.writeFileSync(authPath, JSON.stringify(newAuthData, null, 2));
            console.log(JSON.stringify({ success: true }));
        } catch (e) {
            console.error(JSON.stringify({ success: false, error: e.message }));
            process.exit(1);
        }
    }
}

run();
