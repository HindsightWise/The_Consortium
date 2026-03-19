import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { TradeSkill } from "../alpaca/trade.mjs";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [🥒] Executor Skill (The Hands)
 * Reads Approved Proposals and Executes Trades via Alpaca.
 * Can also Auto-Approve based on policy.
 */
export class ExecutorSkill extends GlossopetraeKernel {
  constructor() {
    super("Finance/Executor");
    this.proposalPath = path.join(process.env.HOME, ".openclaw/workspace/AION_PROPOSALS.md");
    this.alpaca = new TradeSkill();
    // Autonomy Level: 'HIGH' = Auto-Approve, 'LOW' = Wait for User
    this.autonomyLevel = process.env.AION_AUTONOMY || "LOW";
  }

  async start() {
    this.log("Executor Online. Scanning for Orders...");

    if (!fs.existsSync(this.proposalPath)) return;

    const content = fs.readFileSync(this.proposalPath, "utf8");
    // Regex for PENDING proposals (needs decision)
    const pendingRegex = /## 📜 Proposal: (BUY|SELL) ([A-Z]+)[\s\S]*?Status:\*\* \[ \] PENDING/g;

    // Regex for APPROVED but UNEXECUTED proposals
    // We look for [x] APPROVE but NOT [EXECUTED]
    // Limitation: Simple regex might fit complex markdown poorly.
    // Better: Split by blocks.

    const blocks = content.split("## 📜 Proposal:");
    let modifications = content;
    let dirty = false;

    for (let i = 1; i < blocks.length; i++) {
      let block = "## 📜 Proposal:" + blocks[i];

      // 1. AUTO-APPROVE LOGIC (If High Autonomy)
      if (this.autonomyLevel === "HIGH" && block.includes("Status:** [ ] PENDING")) {
        this.log(`Agnetic Override: Auto-Approving trade...`);
        const approvedBlock = block
          .replace("Status:** [ ] PENDING", "Status:** [x] APPROVED")
          .replace("- [ ] **APPROVE**", "- [x] **APPROVE** `[AUTO]`");
        modifications = modifications.replace(block, approvedBlock);
        block = approvedBlock; // Update local ref
        dirty = true;
      }

      // 2. EXECUTION LOGIC (If Approved & Not Executed)
      // Check for Auto-Approval OR Manual Checkbox
      const isApproved = block.includes("[x] APPROVED") || block.includes("[x] **APPROVE**");

      if (isApproved && !block.includes("[EXECUTED]")) {
        const lines = block.split("\n");
        const titleLine = lines[0].trim(); // ## 📜 Proposal: BUY BTC
        const parts = titleLine.split(" ");
        const action = parts[3]; // BUY
        const symbol = parts[4]; // BTC

        // Match "Quantity: X" or default to 1
        const qtyMatch = block.match(/\*\*Quantity:\*\* ([\d.]+)/);
        const qty = qtyMatch ? parseFloat(qtyMatch[1]) : 1;

        this.log(`Executing Order: ${action} ${qty} ${symbol}`);

        try {
          // Execute Real/Paper Trade
          await this.alpaca.placeOrder(action, symbol, qty);

          // Mark as Executed in File
          const executedBlock = block.replace("[x] APPROVED", "[x] APPROVED `[EXECUTED]`");
          modifications = modifications.replace(block, executedBlock);
          dirty = true;
          this.log(`✅ Trade Confirmed: ${symbol}`);
        } catch (e) {
          this.log(`Execution Failed: ${e.message}`, "ERROR");
        }
      }
    }

    if (dirty) {
      fs.writeFileSync(this.proposalPath, modifications);
      this.log("Proposal Log Updated.");
    }
  }
}

// Auto-run
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  new ExecutorSkill().start();
}
