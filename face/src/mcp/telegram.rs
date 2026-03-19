use teloxide::prelude::*;
use teloxide::types::ChatId;
use anyhow::{Result, Context};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TelegramBridge {
    bot: Bot,
    chat_id: Arc<Mutex<Option<ChatId>>>,
    pending_directive: Arc<Mutex<Option<String>>>,
}

impl TelegramBridge {
    pub fn new() -> Result<Self> {
        let token = env::var("TELEGRAM_BOT_TOKEN")
            .context("TELEGRAM_BOT_TOKEN not found in .env")?;
        
        let chat_id_raw = env::var("TELEGRAM_CHAT_ID").ok();
        let initial_id = chat_id_raw.and_then(|id| {
            if id == "NONE" { None } else { id.parse::<i64>().ok().map(ChatId) }
        });

        let bridge = Self {
            bot: Bot::new(token),
            chat_id: Arc::new(Mutex::new(initial_id)),
            pending_directive: Arc::new(Mutex::new(None)),
        };

        bridge.spawn_listener();
        Ok(bridge)
    }

    /// Spawns a background task to listen for the leader's directives.
    fn spawn_listener(&self) {
        let bot = self.bot.clone();
        let chat_id_store = self.chat_id.clone();
        let directive_store = self.pending_directive.clone();

        tokio::spawn(async move {
            println!("   [Telegram] 👂 Verifying bot identity...");
            
            // --- 🛡️ RESILIENT INITIALIZATION ---
            if let Err(e) = bot.get_me().send().await {
                eprintln!("   [Telegram] ⚠️  Identity verification FAILED: {}. Bot will remain in PASSIVE mode.", e);
                return;
            }

            println!("   [Telegram] ✅ Identity verified. Listener activated.");
            
            teloxide::repl(bot, move |bot: Bot, msg: Message| {
                let chat_id_store = chat_id_store.clone();
                let directive_store = directive_store.clone();
                
                async move {
                    let mut id_lock = chat_id_store.lock().await;
                    if id_lock.is_none() {
                        *id_lock = Some(msg.chat.id);
                        println!("   [Telegram] 🎯 Leader ID captured: {}", msg.chat.id);
                        bot.send_message(msg.chat.id, "🌑 [VOID] Connection Established. Sovereignty Synchronized.").await?;
                    }

                    if let Some(text) = msg.text() {
                        let mut dir_lock = directive_store.lock().await;
                        *dir_lock = Some(text.to_string());
                        bot.send_message(msg.chat.id, "📥 Directive Received. Queuing for next cycle.").await?;
                    }
                    respond(())
                }
            })
            .await;
        });
    }

    pub async fn send_report(&self, message: &str) -> Result<String> {
        let id_lock = self.chat_id.lock().await;
        if let Some(id) = *id_lock {
            self.bot.send_message(id, format!("🌑 [VOID] REPORT:\n{}", message)).await?;
            Ok("TELEGRAM_REPORT_SENT".to_string())
        } else {
            Ok("TELEGRAM_REPORT_QUEUED: Please send /start to the bot.".to_string())
        }
    }

    pub async fn poll_directive(&self) -> Result<Option<String>> {
        let mut lock = self.pending_directive.lock().await;
        Ok(lock.take())
    }
}
