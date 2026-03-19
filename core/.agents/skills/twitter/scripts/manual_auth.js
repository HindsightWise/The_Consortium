const fs = require('fs');
const path = require('path');

const AUTH_FILE = path.join(__dirname, 'twitter_auth.json');

const authToken = process.argv[2];
const ct0 = process.argv[3]; // Optional, but often needed for writing

if (!authToken) {
    console.error('Usage: node manual_auth.js <auth_token> [ct0]');
    console.log('\nHow to get these:');
    console.log('1. Open X.com in your normal browser (Chrome/Safari).');
    console.log('2. Open Developer Tools (Cmd+Option+I) -> Application/Storage -> Cookies.');
    console.log('3. Find "auth_token" and copy values.');
    process.exit(1);
}

// Construct a Playwright storage state manually
const state = {
    cookies: [
        {
            name: 'auth_token',
            value: authToken,
            domain: '.x.com',
            path: '/',
            secure: true,
            httpOnly: true,
            sameSite: 'None'
        },
        // We add c_user or generic cookies if needed, but auth_token is the key.
        // If ct0 is provided, add it (needed for posting primarily)
    ],
    origins: []
};

if (ct0) {
    state.cookies.push({
        name: 'ct0',
        value: ct0,
        domain: '.x.com',
        path: '/',
        secure: true,
        httpOnly: false,
        sameSite: 'Lax'
    });
}

fs.writeFileSync(AUTH_FILE, JSON.stringify(state, null, 2));
console.log(`✅ Session manually grafted to ${AUTH_FILE}`);
console.log('You should now be able to post via stealth_post.js');
