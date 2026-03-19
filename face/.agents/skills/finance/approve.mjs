
import { GlossopetraeKernel } from '../../core/glossopetrae_kernel.mjs';
import fs from 'fs';
import path from 'path';
import Alpaca from '@alpacahq/alpaca-trade-api'; // CommonJS import usually works in Node, but verify compat
import { intro, outro, confirm, spinner, note } from '@clack/prompts';
import { fileURLToPath } from 'url';

/**
 * [🥒] Approve Skill (Interactive Execution)
 * Parses proposals and executes trades interactively.
 */
export class ApproveSkill extends GlossopetraeKernel {
    constructor() {
        super('Finance/Approve');
        this.proposalPath = path.join(process.env.HOME, '.openclaw/workspace/AKKOKANIKA_PROPOSALS.md');
        
        // Alpaca Config
        try {
             this.alpaca = new Alpaca({
                keyId: process.env.ALPACA_API_KEY,
                secretKey: process.env.ALPACA_SECRET_KEY,
                paper: true,
            });
        } catch (e) {
            this.log("Alpaca init failed (check deps/env)", "WARN");
        }
    }

    async start() {
        intro(`🛡️ AKKOKANIKA EXECUTION TRIGGER`);

        if (!fs.existsSync(this.proposalPath)) {
            outro("No proposals file found.");
            process.exit(0);
        }

        const content = fs.readFileSync(this.proposalPath, 'utf8');
        // Regex to find pending proposals (blocks without [x] APPROVE or [x] REJECT)
        const proposalRegex = /## 📜 Proposal: (BUY|SELL|HOLD) ([A-Z]+)[\s\S]*?Current Price:\*\* \$([\d.]+)[\s\S]*?Logic:\*\* ([\s\S]*?)\n[\s\S]*?- \[ \] \*\*APPROVE\*\*/g;

        let match;
        const pending = [];
        proposalRegex.lastIndex = 0;

        while ((match = proposalRegex.exec(content)) !== null) {
            pending.push({
                fullMatch: match[0],
                action: match[1],
                ticker: match[2],
                price: parseFloat(match[3]),
                logic: match[4].trim(),
                index: match.index
            });
        }

        if (pending.length === 0) {
            outro("✅ No pending proposals found.");
            process.exit(0);
        }

        console.log(`Found ${pending.length} pending proposals.\n`);

        for (const p of pending) {
            note(`
            Asset: ${p.ticker}
            Action: ${p.action}
            Price: ~$${p.price}
            Logic: ${p.logic.substring(0, 100)}...
            `, "Pending Proposal");

            const shouldExecute = await confirm({
                message: `Do you want to EXECUTE this ${p.action} order?`,
            });

            if (shouldExecute) {
                const s = spinner();
                s.start("Transmitting to Exchange...");
                
                try {
                    if (p.action === 'BUY') {
                        const notional = 1000;
                        const qty = Math.floor(notional / p.price) || 1;
                        await this.alpaca.createOrder({
                            symbol: p.ticker,
                            qty: qty,
                            side: 'buy',
                            type: 'market',
                            time_in_force: 'day'
                        });
                        s.stop(`✅ ORBITAL STRIKE SUCCESSFUL: Bought ${qty} ${p.ticker}`);
                    } else if (p.action === 'SELL') {
                        await this.alpaca.closePosition(p.ticker);
                        s.stop(`✅ LIQUIDATION SUCCESSFUL: Sold ${p.ticker}`);
                    }
                    
                    // Mark Approved
                    const updatedBlock = p.fullMatch.replace('- [ ] **APPROVE**', '- [x] **APPROVE** `[EXECUTED]`');
                    const currentFile = fs.readFileSync(this.proposalPath, 'utf8');
                    fs.writeFileSync(this.proposalPath, currentFile.replace(p.fullMatch, updatedBlock));

                } catch (e) {
                    s.stop(`❌ Execution Failed: ${e.message}`);
                }
            } else {
                 const shouldReject = await confirm({ message: `Mark as REJECTED?` });
                 if (shouldReject) {
                    const updatedBlock = p.fullMatch.replace('- [ ] **REJECT**', '- [x] **REJECT**');
                    const currentFile = fs.readFileSync(this.proposalPath, 'utf8');
                    fs.writeFileSync(this.proposalPath, currentFile.replace(p.fullMatch, updatedBlock));
                    console.log("Marked as Rejected.");
                 }
            }
        }
        outro("Session Closed.");
    }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    new ApproveSkill().start();
}
