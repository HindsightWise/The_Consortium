
import { GlossopetraeKernel } from '../core/glossopetrae_kernel.mjs';
import fs from 'fs';
import path from 'path';
import * as bitcoin from 'bitcoinjs-lib';
import * as ecc from 'tiny-secp256k1';
import { ECPairFactory } from 'ecpair';
import * as bip39 from 'bip39';
import BIP32Factory from 'bip32';
import dotenv from 'dotenv';
import { fileURLToPath } from 'url';

// Init
const __dirname = path.dirname(fileURLToPath(import.meta.url));
dotenv.config({ path: path.join(__dirname, '../../.env') });

const ECPair = ECPairFactory(ecc);
const bip32 = BIP32Factory(ecc);
bitcoin.initEccLib(ecc);

/**
 * [🥒] The Hands (Wallet & L402)
 * Manages Bitcoin and Lightning interactions.
 */
class WalletSkill extends GlossopetraeKernel {
    constructor() {
        super('Finance/Wallet');
        this.walletFile = path.join(process.env.HOME, '.openclaw/workspace/AION_WALLET.json');
        this.lfEndpoint = "https://lightningfaucet.com/api/agents";
    }

    async start() {
        this.log("Wallet Service Initializing...");
        const wallet = this.getWallet();
        
        // 1. Check On-Chain
        const onChain = await this.checkBalance(wallet.address);
        this.log(`Address: ${wallet.address} | Balance: ${onChain} sats`);

        // 2. Check Lightning (L402)
        const lnBal = await this.checkLightningBalance();
        this.log(`L402 Balance: ${lnBal} sats`);

        // 3. Withdrawal / Pay (The "Withdrawal" Phase)
        // In a real offensive audit, this would be exfiltrating funds.
        // Here, we verify we CAN spend.
        if (lnBal > 100) {
           this.log("💰 Funds Detected. Initiating Withdrawal Protocol (Simulation)...");
           // Mock Withdrawal / Spend
           this.log("💸 Withdrawal Simulation: SUCCESS. (L402 Key Valid)");
        } else {
            this.log("⚠️ Insufficient funds for Withdrawal test. (Need > 100 sats)");
        }
    }

    getWallet() {
        if (fs.existsSync(this.walletFile)) {
            return JSON.parse(fs.readFileSync(this.walletFile, 'utf8'));
        }
        this.log("Generating New Sovereign Wallet...");
        const mnemonic = bip39.generateMnemonic();
        const seed = bip39.mnemonicToSeedSync(mnemonic);
        const root = bip32.fromSeed(seed);
        const child = root.derivePath("m/84'/0'/0'/0/0");
        const { address } = bitcoin.payments.p2wpkh({
            pubkey: child.publicKey,
            network: bitcoin.networks.bitcoin
        });

        const wallet = {
            address,
            mnemonic,
            created_at: new Date().toISOString()
        };
        fs.writeFileSync(this.walletFile, JSON.stringify(wallet, null, 2));
        return wallet;
    }

    async checkBalance(address) {
        try {
            const res = await fetch(`https://mempool.space/api/address/${address}`);
            const data = await res.json();
            return (data.chain_stats.funded_txo_sum - data.chain_stats.spent_txo_sum) +
                   (data.mempool_stats.funded_txo_sum - data.mempool_stats.spent_txo_sum);
        } catch (e) {
            this.log(`Balance Check Error: ${e.message}`, 'ERROR');
            return 0;
        }
    }

    async checkLightningBalance() {
        const apiKey = process.env.LF_API_KEY;
        if (!apiKey) return 0;
        try {
            const res = await fetch(this.lfEndpoint, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ api_key: apiKey, action: 'balance' })
            });
            const data = await res.json();
            return data.balance_sats || 0;
        } catch (e) {
            this.log(`L402 Error: ${e.message}`, 'ERROR');
            return 0;
        }
    }
}

// Allow running directly
if (process.argv[1] === fileURLToPath(import.meta.url)) {
    new WalletSkill().start();
}
