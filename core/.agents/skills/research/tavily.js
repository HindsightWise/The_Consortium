import 'dotenv/config';
import https from 'https';

const API_KEY = process.env.TAVILY_API_KEY;

if (!API_KEY) {
    console.error("❌ Error: TAVILY_API_KEY not found in environment.");
    process.exit(1);
}

async function searchTavily(query) {
    const body = JSON.stringify({
        api_key: API_KEY,
        query: query,
        search_depth: "advanced",
        include_answer: true,
        max_results: 5
    });

    const options = {
        hostname: 'api.tavily.com',
        path: '/search',
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Content-Length': body.length
        }
    };

    return new Promise((resolve, reject) => {
        const req = https.request(options, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => {
                if (res.statusCode >= 200 && res.statusCode < 300) {
                    try {
                        const json = JSON.parse(data);
                        resolve(json);
                    } catch (e) {
                        reject(new Error("Failed to parse Tavily API response"));
                    }
                } else {
                    reject(new Error(`Tavily API Error: ${res.statusCode} ${data}`));
                }
            });
        });

        req.on('error', (e) => reject(e));
        req.write(body);
        req.end();
    });
}

// CLI
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
if (process.argv[1] === __filename) {
    const query = process.argv[2];
    if (!query) {
        console.log("Usage: node tavily.js \"<query>\"");
        process.exit(1);
    }
    console.log(`🔎 Searching Tavily for: "${query}"...`);
    searchTavily(query).then(res => {
        console.log("\n--- Answer ---");
        console.log(res.answer || "No direct answer generated.");
        console.log("\n--- Results ---");
        res.results.forEach(r => console.log(`- [${r.title}](${r.url})`));
    }).catch(console.error);
}

export { searchTavily };
