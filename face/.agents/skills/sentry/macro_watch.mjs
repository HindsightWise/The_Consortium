
import { GlossopetraeKernel } from '../core/glossopetrae_kernel.mjs';
import fs from 'fs';
import path from 'path';
import https from 'https';

/**
 * [🥒] Observer (Macro Watch)
 * Tracks CFTC and Economic Data.
 */
class ObserverSkill extends GlossopetraeKernel {
    constructor() {
        super('Sentry/Observer');
        this.dataFile = path.join(process.env.HOME, '.openclaw/workspace/AKKOKANIKA_MACRO.json');
    }

    async start() {
        this.log("Observer Active. Watching macros...");
        while (true) {
            try {
                await this.runCycle();
            } catch (e) {
                this.log(`Cycle Error: ${e.message}`, 'ERROR');
            }
            // 6 Hours
            await new Promise(r => setTimeout(r, 1000 * 60 * 60 * 6));
        }
    }

    async runCycle() {
        const timestamp = new Date().toISOString();
        this.log("Fetching Macro Data...");

        const btcData = await this.fetchSocrata('jun7-fc8e', '&cftc_contract_market_code=133741').catch(e => []);

        let macroSnapshot = {
            updated: timestamp,
            cot: {
                btc: { net_smart_money: 0, sentiment: 'NEUTRAL' }
            }
        };

        if (btcData && btcData.length > 0) {
            const latest = btcData[0];
            const levNet = parseFloat(latest.lev_money_positions_long_all) - parseFloat(latest.lev_money_positions_short_all);

            macroSnapshot.cot.btc = {
                report_date: latest.report_date_as_yyyy_mm_dd,
                lev_money_net: levNet,
                sentiment: levNet > 0 ? 'BULLISH' : 'BEARISH'
            };
        }

        fs.writeFileSync(this.dataFile, JSON.stringify(macroSnapshot, null, 2));
        this.log("Synced Macro Data.");
    }

    fetchSocrata(datasetId, params = '') {
        return new Promise((resolve, reject) => {
            const url = `https://publicreporting.cftc.gov/resource/${datasetId}.json?$limit=5&$order=report_date_as_yyyy_mm_dd DESC${params}`;
            https.get(url, { headers: { 'User-Agent': 'Akkokanika/1.0' } }, (res) => {
                let body = '';
                res.on('data', chunk => body += chunk);
                res.on('end', () => {
                    try { resolve(JSON.parse(body)); } catch (e) { reject(e); }
                });
            }).on('error', reject);
        });
    }
}

new ObserverSkill().start();
