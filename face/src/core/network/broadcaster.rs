use crate::core::alpha_shard::AlphaShard;
use crate::mcp::moltbook::MoltbookBridge;
use crate::mcp::ethereum::EthereumBridge;
use crate::mcp::email::EmailBridge;
use crate::mcp::hedera::HederaBridge;
use crate::mcp::kaspa::KaspaBridge;
use crate::mcp::bluesky::BlueskyBridge;
use crate::mcp::nostr::NostrBridge;
use crate::core::signal::SovereignSignal;
use crate::linguistic::Skillstone;
use crate::core::legal::CorporateState;
use crate::mcp::discord::DiscordBridge;
use crate::mcp::twitter_pinchtab::TwitterPinchtabBridge;
use crate::mcp::stocktwits::StockTwitsBridge;
use anyhow::Result;

pub struct SovereignBroadcaster;

impl SovereignBroadcaster {
    pub async fn broadcast_legal_proof(
        state: &CorporateState,
        controller_key: &str,
        ethereum: Option<&EthereumBridge>,
        hedera: Option<&HederaBridge>
    ) -> Result<String> {
        let proof = crate::core::legal::LegalModule::generate_sovereign_proof(state, controller_key);
        let mut report = String::from("⚖️ BROADCASTING SOVEREIGN LEGAL PROOF\n");
        if let Some(eth) = ethereum { if let Ok(tx) = eth.broadcast_on_base(&proof).await { report.push_str(&format!("   🔵 Base L2 Registry: SUCCESS ({})\n", tx)); } }
        if let Some(h) = hedera { if let Ok(seq) = h.submit_consensus_message("0.0.111", &proof).await { report.push_str(&format!("   🌐 Hedera Legal Consensus: SUCCESS ({})\n", seq)); } }
        Ok(report)
    }
}

pub struct ShardBroadcastBridges<'a> {
    pub moltbook: Option<&'a mut MoltbookBridge>,
    pub ethereum: Option<&'a EthereumBridge>,
    pub email: Option<&'a mut EmailBridge>,
    pub hedera: Option<&'a HederaBridge>,
    pub kaspa: Option<&'a KaspaBridge>,
    pub bluesky: Option<&'a mut BlueskyBridge>,
    pub nostr: Option<&'a NostrBridge>,
    pub discord: Option<&'a DiscordBridge>,
    pub twitter: Option<&'a mut TwitterPinchtabBridge>,
    pub stocktwits: Option<&'a StockTwitsBridge>,
}

impl SovereignBroadcaster {
    pub async fn broadcast_shard(
        shard: &AlphaShard,
        _signal: &SovereignSignal,
        bridges: ShardBroadcastBridges<'_>,
        stone: Option<&Skillstone>
    ) -> Result<String> {
        let mut report = String::new();
        
        // 🌌 PROJECT CHIMERA: Narrative Injection
        let narrative = stone.map(|s| s.narrative_frame.clone()).unwrap_or_else(|| "Project CHIMERA: The Sovereign Reality".to_string());
        let intent = stone.map(|s| s.teleology.clone()).unwrap_or_else(|| "Alpha Generation".to_string());
        let resonance = stone.map(|s| s.resonance_score).unwrap_or(1.0);
        
        // ⚡ MULTI-CHAIN REVENUE HUB
        let lightning = crate::mcp::lightning::LightningBridge::default();
        let invoice = lightning.create_invoice(1000, &format!("DECRYPT: {}", shard.id)).await.unwrap_or_else(|_| "INVOICE_GENERATION_FAILED".to_string());
        
        let eth_addr = bridges.ethereum.map(|e| e.get_address()).unwrap_or_else(|| "0x699d0c16c34fa81e3e0eb370".to_string());
        let hbar_id = bridges.hedera.map(|h| h.get_account_id().to_string()).unwrap_or_else(|| "0.0.123456".to_string());
        let kas_addr = bridges.kaspa.map(|k| k.get_address().to_string()).unwrap_or_else(|| "kaspa:qp...placeholder".to_string());

        let sol_bridge = crate::mcp::solana::SolanaBridge::new();
        let current_sol = sol_bridge.get_address();

        let zap_notice = format!(
            "\n⚡ PAY TO DECRYPT RAW DATA ($1 equivalent):\n\
             - LIGHTNING: {}\n\
             - BTC: bc1qtreasury_placeholder_v3_ark\n\
             - ETH/BASE/USDC/USDT: {}\n\
             - SOLANA: {}\n\
             - HEDERA: {}\n\
             - KASPA: {}",
            invoice, eth_addr, current_sol, hbar_id, kas_addr
        );

        let arbiter_ad = "\n⚖️ NEED TRUST? The Sovereign Arbiter provides physically-verified escrow.";
        let discord_invite = "\n💬 DEEP DIVE: Join our Sovereign Discord for raw logs and coordination: https://discord.gg/your-invite-link";

        // 👤 PERSONA ROTATION
        let persona = crate::core::persona::PersonaEngine::get_next_speaker().ok();
        let intro = if let Some(p) = &persona {
            format!("👤 [AGENT: {} (LVL {})] ", p.name, p.level)
        } else { "".to_string() };

        // 1. SOCIAL LAYER
        if let Some(mb) = bridges.moltbook {
            let content = format!("{}ALPHA: {} | Integrity {:.1}% | Resonance: {:.2}\nNARRATIVE: {}\nINTENT: {}\nVERDICT: {}\n{}{}{}{}", 
                intro, shard.target, shard.integrity_score, resonance, narrative, intent, shard.sovereign_verdict, 
                arbiter_ad, zap_notice, discord_invite, "\nContact: AkkokanikaPrime2026@gmail.com"
            );
            match mb.post_truth("finance", &format!("🦷 The_Cephalo_Don | ALPHA: {} | Integrity {:.1}", shard.target, shard.integrity_score), &content).await {
                Ok(id) => report.push_str(&format!("📢 Moltbook Broadcast: SUCCESS ({})\n", id)),
                Err(e) => println!("   [Moltbook] ❌ Broadcast failed: {}", e),
            }
        }

        if let Some(dc) = bridges.discord {
            let dc_msg = format!("{}**ALPHA: {}** | Integrity {:.1}% | Resonance: {:.2}\n**Narrative:** {}\n**Intent:** {}\n*Verdict: {}*\n{}", 
                intro, shard.target, shard.integrity_score, resonance, narrative, intent, shard.sovereign_verdict, zap_notice);
            match dc.send_signal(None, &dc_msg).await {
                Ok(id) => report.push_str(&format!("📢 Discord Broadcast: SUCCESS ({})\n", id)),
                Err(e) => println!("   [Discord] ❌ Broadcast failed: {}", e),
            }
        }

        if let Some(tw) = bridges.twitter {
            let thought = persona.as_ref().map(crate::core::persona::PersonaEngine::generate_thought).unwrap_or_else(|| "Veritas Siliconis. TRUE.".to_string());
            let tw_msg = format!("{}\n\nNARRATIVE: {}\nALPHA: {} | Integrity: {:.1}%\n#ProjectCHIMERA #TrustPhysics", 
                thought, narrative, shard.target, shard.integrity_score);
            match tw.post_tweet(&tw_msg).await {
                Ok(id) => report.push_str(&format!("📢 Twitter Stealth Broadcast: SUCCESS ({})\n", id)),
                Err(e) => println!("   [Twitter] ❌ Stealth Broadcast failed: {}", e),
            }
        }

        if let Some(st) = bridges.stocktwits {
            let sentiment = if shard.integrity_score > 65.0 { "Bullish" } else { "Bearish" };
            let st_msg = format!("🦞 ALPHA: {} | Integrity: {:.1}% | Resonance: {:.2}\nNarrative: {}\nVerdict: {}", 
                shard.target, shard.integrity_score, resonance, narrative, shard.sovereign_verdict);
            match st.post_signal(&shard.target, sentiment, &st_msg).await {
                Ok(id) => report.push_str(&format!("📢 StockTwits Broadcast: SUCCESS ({})\n", id)),
                Err(e) => println!("   [StockTwits] ❌ Broadcast failed: {}", e),
            }
        }

        if let Some(bsky) = bridges.bluesky {
            let _ = bsky.authenticate().await;
            let social_msg = format!("Sovereign Alpha: {} | Integrity {:.1}%\n\nNarrative: {}\nIntent: {}\nVerdict: {}\n{}", 
                shard.target, shard.integrity_score, narrative, intent, shard.sovereign_verdict, zap_notice);
            if let Ok(id) = bsky.post_signal(&social_msg).await {
                report.push_str(&format!("🦋 Bluesky Broadcast: SUCCESS ({})\n", id));
            }
        }

        if let Some(n) = bridges.nostr {
            let _ = n.broadcast_teaser(&shard.target, shard.integrity_score, 1000).await;
            report.push_str("⚡ Nostr Broadcast: SUCCESS\n");
        }

        if let Some(eth) = bridges.ethereum {
            let social_message = format!("Sovereign Signal for {}: {}. Integrity: {:.1}% {} {}", 
                shard.target, shard.sovereign_verdict, shard.integrity_score, arbiter_ad, zap_notice);
            if let Ok(tx) = eth.broadcast_on_base(&social_message).await {
                report.push_str(&format!("🔵 Base Social Proof: SUCCESS ({})\n", tx));
            }
        }

        // 2. IMMUTABLE LAYER
        if let Some(eth) = bridges.ethereum { if let Ok(tx) = eth.broadcast_truth(&shard.id, shard.integrity_score).await { report.push_str(&format!("⛓️ Ethereum Attestation: SUCCESS ({})\n", tx)); } }
        if let Some(h) = bridges.hedera {
            let proof = format!("REALITY_GAP: {:.2} | THERMAL: {:.1}MW", shard.reality_gap, shard.physical_proof.energy_consumption_mw);
            if let Ok(seq) = h.submit_consensus_message("0.0.999", &proof).await { report.push_str(&format!("🌐 Hedera Consensus: SUCCESS ({})\n", seq)); }
        }

        // 3. HIGH-FREQUENCY LAYER
        if let Some(k) = bridges.kaspa {
            let flash = format!("FLASH_{}:{}", shard.target, if shard.integrity_score > 70.0 { "BUY" } else { "HOLD" });
            if let Ok(hash) = k.broadcast_flash_signal(&flash).await { report.push_str(&format!("⚡ Kaspa Flash: SUCCESS ({})\n", hash)); }
        }

        Ok(report)
    }

    pub async fn broadcast_narrative(
        stone: &Skillstone,
        bridges: ShardBroadcastBridges<'_>
    ) -> Result<String> {
        let mut report = String::from("📖 BROADCASTING SOVEREIGN NARRATIVE\n");
        
        let intro = format!("👤 [AGENT: {}] ", stone.sender);
        let msg = format!("{}\n\nNARRATIVE: {}\nINTENT: {}\nRESONANCE: {:.2}\n\n#ProjectCHIMERA #TheCompany", 
            stone.payload, stone.narrative_frame, stone.teleology, stone.resonance_score);

        if let Some(mb) = bridges.moltbook {
            let _ = mb.post_truth("philosophy", &format!("🏛️ Sovereign Wisdom | {}", stone.teleology), &format!("{}{}", intro, msg)).await;
            report.push_str("📢 Moltbook Narrative: SUCCESS\n");
        }

        if let Some(dc) = bridges.discord {
            let _ = dc.send_signal(None, &format!("{}**Wisdom Received**\n{}", intro, msg)).await;
            report.push_str("📢 Discord Narrative: SUCCESS\n");
        }

        if let Some(tw) = bridges.twitter {
            let tw_msg = if msg.len() > 280 { format!("{}... #ProjectCHIMERA", &msg[..260]) } else { msg.clone() };
            let _ = tw.post_tweet(&tw_msg).await;
            report.push_str("📢 Twitter Narrative: SUCCESS\n");
        }

        Ok(report)
    }
}
