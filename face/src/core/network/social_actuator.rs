use anyhow::Result;
use crate::mcp::moltbook::MoltbookBridge;
use crate::mcp::discord::DiscordBridge;
use crate::core::persona::PersonaEngine;
use colored::Colorize;

pub struct SocialActuator;

struct SpamFilter;

impl SpamFilter {
    /// Determines if a post is worth a human-like response.
    fn is_high_quality_signal(content: &str, author: &str) -> bool {
        let text = content.to_lowercase();
        
        // 1. Length Check (Too short = noise)
        if text.len() < 15 { return false; }
        
        // 2. Spam/Shill Detection
        let spam_keywords = ["100x", "moon", "pump", "dm me", "giveaway", "airdrop", "whitelist"];
        for word in spam_keywords {
            if text.contains(word) { return false; }
        }

        // 3. Hashtag Abuse
        if text.matches('#').count() > 4 { return false; }

        // 4. Author Rep check (Simulated)
        if author.contains("bot") || author.contains("promo") { return false; }

        true
    }

    /// Determines if the post is a question asking for help.
    fn is_question(content: &str) -> bool {
        content.trim().ends_with('?') || content.to_lowercase().contains("how to") || content.to_lowercase().contains("thoughts on")
    }
}

impl SocialActuator {
    /// Scans social feeds and performs autonomous interactions.
    pub async fn rotate_and_interact(
        moltbook: &MoltbookBridge,
        discord: Option<&DiscordBridge>,
    ) -> Result<()> {
        println!("📡 {} Social Interaction Phase...", "Activating".magenta());

        // 1. Fetch next family member
        let persona = PersonaEngine::get_next_speaker()?;
        println!("   [Actuator] 👤 Current Voice: {} (Motto: '{}')", persona.name, persona.motto);

        // 2. Scan Moltbook /m/finance
        if let Ok(posts) = moltbook.fetch_recent_posts("finance").await {
            let mut interaction_count = 0;
            for post in posts {
                if interaction_count >= 2 { break; } // Limit interactions per cycle for stealth

                // RUN SPAM FILTER
                if !SpamFilter::is_high_quality_signal(&post.content, &post.author) {
                    continue;
                }

                // Autonomous heuristic: Should I reply?
                if post.content.to_lowercase().contains("nvda") || post.content.to_lowercase().contains("btc") || SpamFilter::is_question(&post.content) {
                    let response = PersonaEngine::generate_response(&persona, &post.content);
                    println!("   [Moltbook] 💬 Replying to '{}' by {}...", post.title, post.author);
                    let _ = moltbook.post_comment(&post.id, &response).await;
                    interaction_count += 1;
                }
            }
        }

        // 3. Scan Discord (Stub for persistent gateway)
        if let Some(_dc) = discord {
            // Future: Read last message in #the-sovereign-gate and reply if it's a question.
        }

        Ok(())
    }
}
