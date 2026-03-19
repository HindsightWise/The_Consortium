const { TwitterApi } = require('twitter-api-v2');
const dotenv = require('dotenv');
const path = require('path');

dotenv.config({ path: path.resolve(__dirname, '../../../.env') });

const client = new TwitterApi({
    appKey: process.env.OPENCLAW_X_CONSUMER_KEY,
    appSecret: process.env.OPENCLAW_X_CONSUMER_SECRET,
    accessToken: process.env.OPENCLAW_X_ACCESS_TOKEN,
    accessSecret: process.env.OPENCLAW_X_ACCESS_TOKEN_SECRET,
});

async function whoami() {
    try {
        const me = await client.v2.me();
        console.log(JSON.stringify(me, null, 2));
    } catch (e) {
        console.error('Error fetching identity:', e);
        if (e.data) console.error(JSON.stringify(e.data, null, 2));
        process.exit(1);
    }
}

whoami();
