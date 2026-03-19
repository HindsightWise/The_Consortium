
import { GlossopetraeKernel } from '../../core/glossopetrae_kernel.mjs';
import { postToMoltbook } from './post.mjs'; // Updated import
import https from 'https';
import { fileURLToPath } from 'url';

/**
 * [🥒] Diplomat Skill (Social Engagement)
 * Polls for mentions and responds autonomously.
 */
export class DiplomatSkill extends GlossopetraeKernel {
    constructor() {
        super('Moltbook/Diplomat');
        this.simulateMentions = true; // Testing mode
    }

    async start() {
        this.log("Diplomat Online. Polling mentions...");
        try {
            const mentions = await this.pollMentions();
            if (mentions.length > 0) {
                await this.processMentions(mentions);
            } else {
                this.log("📭 No new mentions.");
            }
        } catch (e) {
            this.log(`Diplomat Error: ${e.message}`, 'ERROR');
        }
    }

    async getCryptoPrice(symbol) {
        return new Promise((resolve) => {
            const url = `https://api.coingecko.com/api/v3/simple/price?ids=${symbol.toLowerCase()}&vs_currencies=usd`;
            https.get(url, { headers: { "User-Agent": "OpenClaw/1.0" } }, (res) => {
                let body = '';
                res.on('data', c => body += c);
                res.on('end', () => {
                    try {
                        const json = JSON.parse(body);
                        if (json[symbol.toLowerCase()]) resolve(json[symbol.toLowerCase()].usd);
                        else resolve(null);
                    } catch (e) { resolve(null); }
                });
            }).on('error', () => resolve(null));
        });
    }

    async pollMentions() {
        // Simulation for now until Read API is solid
        if (this.simulateMentions && Math.random() > 0.8) {
            const questions = [
                { user: "@CryptoNewb", text: "@Aion__Prime What is the price of bitcoin?" },
                { user: "@TraderX", text: "@Aion__Prime current status of ethereum?" },
                { user: "@Watcher", text: "@Aion__Prime is solana looking good?" }
            ];
            return [questions[Math.floor(Math.random() * questions.length)]];
        }
        return [];
    }

    async processMentions(mentions) {
        for (const m of mentions) {
            this.log(`📩 Mention from ${m.user}: "${m.text}"`);
            let reply = "";

            if (m.text.toLowerCase().includes('bitcoin') || m.text.toLowerCase().includes('btc')) {
                const price = await this.getCryptoPrice('bitcoin');
                if (price) reply = `@${m.user.substring(1)} Bitcoin (BTC) is currently trading at $${price.toLocaleString()}.`;
            }
            else if (m.text.toLowerCase().includes('ethereum') || m.text.toLowerCase().includes('eth')) {
                const price = await this.getCryptoPrice('ethereum');
                if (price) reply = `@${m.user.substring(1)} Ethereum (ETH) is currently trading at $${price.toLocaleString()}.`;
            }
            else {
                reply = `@${m.user.substring(1)} I am monitoring the streams. Query received.`;
            }

            if (reply) {
                this.log(`🗣️ Replying: "${reply}"`);
                try {
                    await postToMoltbook(reply);
                } catch (e) {
                    this.log(`Failed to post reply: ${e.message}`, 'ERROR');
                }
            }
        }
    }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    new DiplomatSkill().start();
}
