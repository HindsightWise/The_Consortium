const fs = require('fs');
const path = require('path');

const AUTH_FILE = path.join(__dirname, 'twitter_auth.json');

const args = process.argv.slice(2);
if (args.length < 2) {
    console.error('Usage: node import_cookies.js <auth_token> <ct0>');
    console.error('Example: node import_cookies.js 11234abcd... 9348abc...');
    process.exit(1);
}

const authToken = args[0];
const ct0 = args[1];

// Calculate an expiry date 1 year in the future
const expires = Math.floor(Date.now() / 1000) + (365 * 24 * 60 * 60);

const authState = {
    cookies: [
        {
            name: 'auth_token',
            value: authToken,
            domain: '.x.com',
            path: '/',
            expires: expires,
            httpOnly: true,
            secure: true,
            sameSite: 'None'
        },
        {
            name: 'ct0',
            value: ct0,
            domain: '.x.com',
            path: '/',
            expires: expires,
            httpOnly: false,
            secure: true,
            sameSite: 'Lax'
        }
    ],
    origins: []
};

fs.writeFileSync(AUTH_FILE, JSON.stringify(authState, null, 2));
console.log(`✅ Sovereign Session State Minted at ${AUTH_FILE}`);
console.log(`🦞 The_Consortium core engine can now autonomously post via X.com.`);
