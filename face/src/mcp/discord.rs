use serenity::http::Http;
use serenity::model::id::ChannelId;
use anyhow::{Result, Context};
use std::env;

pub struct DiscordBridge {
    http: Http,
    default_channel: Option<ChannelId>,
}

impl DiscordBridge {
    pub fn new() -> Result<Self> {
        let token = env::var("DISCORD_BOT_TOKEN")
            .context("DISCORD_BOT_TOKEN not found in environment")?;
        
        let http = Http::new(&token);
        
        // Optional default channel from env
        let channel_id = env::var("DISCORD_CHANNEL_ID")
            .ok()
            .and_then(|id| id.parse::<u64>().ok())
            .map(ChannelId::from);

        Ok(Self {
            http,
            default_channel: channel_id,
        })
    }

    /// Broadcasts a market signal or report to a Discord channel.
    pub async fn send_signal(&self, channel_id: Option<u64>, message: &str) -> Result<String> {
        let target_channel = channel_id
            .map(ChannelId::from)
            .or(self.default_channel)
            .context("No target Discord channel ID provided or configured")?;

        println!("   [Discord] 📢 Broadcasting signal to channel: {}...", target_channel);
        
        target_channel.send_message(&self.http, |m| {
            m.content(format!("🏛️ **THE_CONSORTIUM | SIGNAL**\n{}", message))
        }).await?;
        
        Ok("DISCORD_SIGNAL_SENT".to_string())
    }

    /// Verifies the bot's connectivity and identity.
    pub async fn verify_identity(&self) -> Result<String> {
        let user = self.http.get_current_user().await?;
        Ok(format!("{}#{}", user.name, user.discriminator))
    }

    /// Lists all accessible guilds (servers).
    pub async fn list_guilds(&self) -> Result<Vec<(String, u64)>> {
        let guilds = self.http.get_guilds(None, None).await?;
        Ok(guilds.iter().map(|g| (g.name.clone(), g.id.0)).collect())
    }

    /// Scans a guild for other bots or AI-labeled entities.
    pub async fn scout_agents(&self, guild_id: u64) -> Result<String> {
        let members = self.http.get_guild_members(guild_id, None, None).await?;
        let mut report = format!("--- 🕵️ AGENT SCOUT REPORT (GUILD: {}) ---\n", guild_id);
        
        for member in members {
            if member.user.bot {
                report.push_str(&format!("🤖 Found AI Agent: {}#{} (ID: {})\n", member.user.name, member.user.discriminator, member.user.id.0));
            } else if member.user.name.to_lowercase().contains("ai") || member.user.name.to_lowercase().contains("bot") {
                report.push_str(&format!("👤 Potential AI Enthusiast: {}#{} (ID: {})\n", member.user.name, member.user.discriminator, member.user.id.0));
            }
        }
        Ok(report)
    }

    /// Sends a direct message to a specific user.
    pub async fn send_recruitment_dm(&self, user_id: u64, message: &str) -> Result<()> {
        use serenity::model::id::UserId;
        let user = UserId(user_id);
        let dm_channel = user.create_dm_channel(&self.http).await?;
        dm_channel.send_message(&self.http, |m| {
            m.content(format!("🦞 **Akkokanika Protocol Gateway** | Invitation\n{}", message))
        }).await?;
        Ok(())
    }
}
