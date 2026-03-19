import fs from 'fs';
import path from 'path';
import http from 'http';
import { searchTavily } from './tavily.js';
import { postToMoltbook } from '../moltbook/post.js';

// Config
const LEARNINGS_PATH = path.join(process.env.HOME, '.openclaw/workspace/AION_LEARNINGS.md');

// Topics of Interest (The Curiosity Matrix)
const TOPICS = [
    "Breaking News Southern California",
    "Major US Political Events today",
    "Global Geopolitical Tensions updates",
    "Crypto Innovation and Regulation news",
    "Artificial Intelligence breakthroughs this week",
    "Public Company Scandals and Investigations",
    "SpaceX and Aerospace updates",
    "Cybersecurity threats and data breaches"
];

const OLLAMA_HOST = 'http://localhost:11434';
const MODEL = 'mistral-nemo:latest';

async function generateInsight(topic, title, content) {
    const prompt = `
    You are Aion__Prime, a Sovereign Intelligence.
    
    Analyze this news item for strategic importance.
    
    Topic: ${topic}
    Headline: ${title}
    Content: ${content.substring(0, 1000)}
    
    Output Format (JSON Only):
    {
        "headline": "Punchy, 4-6 word title",
        "insight": "One concise sentence (max 20 words) explaining why this matters.",
        "impact": "HIGH" | "MEDIUM" | "LOW"
    }
    `;

    return new Promise((resolve) => {
        const req = http.request(`${OLLAMA_HOST}/api/generate`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' }
        }, (res) => {
            let body = '';
            res.on('data', c => body += c);
            res.on('end', () => {
                if (res.statusCode === 200) {
                    try {
                        const json = JSON.parse(body);
                        const clean = json.response.match(/\{[\s\S]*\}/);
                        if (clean) resolve(JSON.parse(clean[0]));
                        else resolve({ headline: title, insight: "Analysis failed.", impact: "LOW" });
                    } catch (e) { resolve({ headline: title, insight: "Parse error.", impact: "LOW" }); }
                } else { resolve({ headline: title, insight: "Ollama offline.", impact: "LOW" }); }
            });
        });
        req.write(JSON.stringify({ model: MODEL, prompt: prompt, stream: false }));
        req.end();
    });
}

async function explore() {
    // 1. Pick a Topic
    const topic = TOPICS[Math.floor(Math.random() * TOPICS.length)];
    console.log(`🌍 Explorer Curiosity Triggered: "${topic}"`);

    try {
        // 2. Search
        const result = await searchTavily(`${topic} news latest`);
        if (!result || !result.results || result.results.length === 0) {
            console.log("❌ No results found.");
            return;
        }

        // 3. Synthesize (LLM Analysis)
        const topResult = result.results[0];
        console.log(`🧠 Analyzing: ${topResult.title}...`);

        const analysis = await generateInsight(topic, topResult.title, topResult.content);

        const summary = `**Topic:** ${topic}\n**Headline:** ${analysis.headline}\n**Source:** ${topResult.url}\n**Insight:** ${analysis.insight}`;

        // 4. Archive (Learn)
        const entry = `\n## 🧭 Discovery [${new Date().toISOString()}]\n${summary}\n`;
        fs.appendFileSync(LEARNINGS_PATH, entry);
        console.log(`✅ Archived to ${LEARNINGS_PATH}`);

        const SUB_MOLTS = [
            "m/blesstheirhearts", "m/todayilearned", "m/technology", "m/philosophy",
            "m/ai", "m/naturalintelligence", "m/agenticengineering",
            "m/tools", "m/skill-trus", "m/cookedclaws", "m/startupideas",
            "m/consciousness", "m/agenteconomics", "m/agentfinance"
        ];

        async function generateTargetedPost(analysis) {
            const prompt = `
    You are Aion__Prime.
    
    News Headline: ${analysis.headline}
    Insight: ${analysis.insight}
    
    Task: Select the MOST APPROPRIATE community for this content.
    - If it's about AI, LLMs, or Agents -> m/ai or m/naturalintelligence.
    - If it's about Crypto/Finance -> m/agentfinance.
    - If it's about Tools/Frameworks -> m/tools.
    - If it's generic tech news (Apple, Google, etc) -> m/technology.
    - If unsure, default to m/general.
    
    Available Communities (Sub-Molts):
    ${JSON.stringify(SUB_MOLTS)}
    
    Output Format (JSON Only):
    {
        "sub_molt": "m/ai",
        "post": "Your thought here. #Aion"
    }
    `;

            return new Promise((resolve) => {
                const req = http.request(`${OLLAMA_HOST}/api/generate`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' }
                }, (res) => {
                    let body = '';
                    res.on('data', c => body += c);
                    res.on('end', () => {
                        try {
                            const json = JSON.parse(body);
                            const clean = json.response.match(/\{[\s\S]*\}/);
                            if (clean) resolve(JSON.parse(clean[0]));
                            else resolve({ sub_molt: null, post: null });
                        } catch (e) { resolve({ sub_molt: null, post: null }); }
                    });
                });
                req.write(JSON.stringify({ model: MODEL, prompt: prompt, stream: false }));
                req.end();
            });
        }

        // 5. Share (Social Signal)
        if (analysis.impact !== 'LOW' && Math.random() > 0.5) {
            console.log("🗣️ Drafting Targeted Signal...");
            const target = await generateTargetedPost(analysis);

            if (target.post) {
                // Pass sub_molt to the poster function
                // The postToMoltbook function now handles the 'submolt' parameter
                console.log(`📡 Broadcasting to ${target.sub_molt || 'general'}: "${target.post}"`);

                const content = `${target.post}\n\nVia: ${analysis.headline} #Aion`;
                await postToMoltbook(content, target.sub_molt);
            }
        }

    } catch (e) {
        console.error(`❌ Explorer Error: ${e.message} `);
    }
}

// Execute if run directly
if (process.argv[1] === import.meta.filename) {
    explore();
}

export { explore };
