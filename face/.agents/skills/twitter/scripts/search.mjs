
import { GlossopetraeKernel } from '../../core/glossopetrae_kernel.mjs';
import { TwitterApi } from 'twitter-api-v2';
import dotenv from 'dotenv';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
dotenv.config({ path: path.resolve(__dirname, '../../../.env') });

class TwitterSearchSkill extends GlossopetraeKernel {
    constructor() {
        super('TwitterSearch');
    }

    async search(query) {
        this.log(`Searching Twitter for: "${query}"`);

        try {
            await this.loadConfig();

            const client = new TwitterApi({
                appKey: process.env.OPENCLAW_X_CONSUMER_KEY,
                appSecret: process.env.OPENCLAW_X_CONSUMER_SECRET,
                accessToken: process.env.OPENCLAW_X_ACCESS_TOKEN,
                accessSecret: process.env.OPENCLAW_X_ACCESS_TOKEN_SECRET,
            });

            // v2 search requires app-only auth or user auth depending on endpoint
            // Essential/Free tier only allows tweet posting and user lookup usually.
            // Search might fail on Free tier. Trying v2 search.
            const result = await client.v2.search(query, { max_results: 10 });

            this.log(`Found ${result.meta.result_count} tweets.`);
            for (const tweet of result.data.data) {
                console.log(`- ${tweet.text} (ID: ${tweet.id})`);
            }
        } catch (e) {
            this.log(`Search failed: ${e.message}`, 'ERROR');
            if (e.data) this.log(JSON.stringify(e.data, null, 2), 'ERROR');
        }
    }
}

const query = process.argv[2];
if (!query) {
    console.error("Usage: node search.mjs <query>");
    process.exit(1);
}

new TwitterSearchSkill().search(query);
