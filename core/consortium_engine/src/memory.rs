// =========================================================================
// [EXPLANATION]: This file manages the "Working Memory". It is the cognitive buffer 
// of the AI. As you chat, and as tools output text, it goes into an array. If that array 
// gets too big, the AI will crash because it exceeds the API limit. This file constantly 
// measures the memory and actively compresses it to ensure eternal uptime.
// =========================================================================

use consortium_core::llm::{ConsortiumRouter, Message}; // [EXPLANATION]: Imports the LLM connection tool (ConsortiumRouter) and the foundational 'Message' structure (role, content).
use serde::{Deserialize, Serialize}; // [EXPLANATION]: Allows us to easily serialize (save) and deserialize (load) the memory array to a JSON file on your hard drive.
use std::fs; // [EXPLANATION]: Rust's standard module for interacting with the File System.
use std::path::Path; // [EXPLANATION]: Utility for checking if the physical memory file already exists.
use tiktoken_rs::cl100k_base; // [EXPLANATION]: The highly precise tokenizer library. It mathematically calculates exactly how many 'tokens' a string consumes.
use tokio::task;
use anyhow::{Result, anyhow};

// [EXPLANATION]: The path to the physical JSON file where the short-term memory is persisted between reboots.
const MEMORY_FILE: &str = "./motor_cortex/working_memory.json";
// [EXPLANATION]: The upper safety limit. We never want to allow anywhere close to 131,072 (the true API limit), so we bound it at 64k.
const CONTEXT_LIMIT: usize = 64_000;
// [EXPLANATION]: The eviction trigger. When the `messages` array hits 80% of 64k (around 51,200 tokens), the Oblivion protocol fires.
const EVICTION_THRESHOLD: usize = (CONTEXT_LIMIT as f64 * 0.80) as usize;

#[derive(Debug, Serialize, Deserialize)]
// [EXPLANATION]: The core data structure embodying the AI's recent memory. It is simply a list of sequential Message objects.
pub struct WorkingMemory {
    pub messages: Vec<Message>,
}

impl WorkingMemory {
    // [EXPLANATION]: The constructor function. Called exactly once when the Engine boots.
    pub fn new() -> Self {
        // [EXPLANATION]: It first checks if a previous session left a memory file on disk.
        if Path::new(MEMORY_FILE).exists() {
            // [EXPLANATION]: If yes, read it into memory.
            if let Ok(content) = fs::read_to_string(MEMORY_FILE) {
                // [EXPLANATION]: Decode the JSON text directly into Rust `Message` structs.
                if let Ok(mem) = serde_json::from_str::<Self>(&content) {
                    return mem; // [EXPLANATION]: We successfully loaded the previous state!
                }
            }
        }
        // [EXPLANATION]: If there was no file (or it was corrupted), we start fresh with a completely empty memory vector.
        Self {
            messages: Vec::new(),
        }
    }

    // [EXPLANATION]: Saves the current state of memory instantly to the hard drive.
    pub fn save(&self) {
        // [EXPLANATION]: Converts the `messages` array into beautifully formatted `_pretty` JSON.
        if let Ok(json) = serde_json::to_string_pretty(self) {
            // [EXPLANATION]: Physically writes it to disk, overwriting the old file.
            let _ = fs::write(MEMORY_FILE, json);
        }
    }

    /// [EXPLANATION]: Calculate the total exact token count of the current memory array using Cloud-API-spec tokenizer (cl100k_base).
    pub fn calculate_tokens(&self) -> usize {
        let bpe = cl100k_base().unwrap(); // [EXPLANATION]: Load the tokenizer schema
        let mut total = 0; // [EXPLANATION]: Start a running sum at 0
        for msg in &self.messages {
            // [EXPLANATION]: For every single message in memory, convert the text into tokens and count the length.
            total += bpe.encode_with_special_tokens(&msg.content).len();
            // [EXPLANATION]: If the LLM outputted internal monologue (reasoning_content), we have to count that too!
            if let Some(reasoning) = &msg.reasoning_content {
                total += bpe.encode_with_special_tokens(reasoning).len();
            }
        }
        total // [EXPLANATION]: Return the exact sum.
    }

    /// [EXPLANATION]: This is the function the Engine calls to add new text (from you, from a tool, or from the LLM) into the buffer.
    pub async fn inject(&mut self, role: &str, content: &str, _router: &ConsortiumRouter) -> usize {
        // [EXPLANATION]: Push the new message into the end of the array.
        self.messages.push(Message {
            role: role.to_string(), // [EXPLANATION]: "user", "system", or "assistant"
            content: content.to_string(),
            reasoning_content: None, // [EXPLANATION]: Set to none since this is just an injection of text.
        });

        // [EXPLANATION]: Measure exactly how full the system is right now.
        let mut current_tokens = self.calculate_tokens();

        // [EXPLANATION]: THE SAFETY CHECK. Have we exceeded the 80% eviction threshold?
        if current_tokens > EVICTION_THRESHOLD {
            // [EXPLANATION]: Log the warning to the UI, let the user know we are hitting limits.
            crate::ui_log!("   [🧠 CONSORTIUM] ⚠️ Working Memory saturated ({} tokens). Triggering 80% OBLIVION Protocol.", current_tokens);
            // [EXPLANATION]: Await the high-performance non-blocking PRUNE function to shrink the memory down.
            let _ = oblivion_prune(&mut self.messages).await;
            // [EXPLANATION]: Re-calculate the new, much smaller size after compression.
            current_tokens = self.calculate_tokens();

            // [EXPLANATION]: The FATAL OVERFLOW safeguard. If after Oblivion compression we are STILL 
            // over the absolute physical limit, we must purge everything to stop the engine from soft-locking.
            if current_tokens > CONTEXT_LIMIT {
                crate::ui_log!("   [☢️ CONSORTIUM] FATAL MEMORY OVERFLOW. Purging matrix entirely to save engine.");
                self.messages.clear();
                current_tokens = 0;
            }
        }

        // [EXPLANATION]: Save the newly updated (and possibly compressed) memory back to the physical file.
        self.save();
        current_tokens // [EXPLANATION]: Return the current size so the UI knows what to display on the HUD gauge.
    }
}

// =========================================================================
// OBLIVION PROTOCOL: HIGH-PERFORMANCE SLIDING-WINDOW PRUNING
// =========================================================================

#[derive(Debug)]
pub struct PruneResult {
    pub messages_removed: usize,
    pub tokens_reduced: usize,
}

pub async fn oblivion_prune(messages: &mut Vec<Message>) -> Result<PruneResult> {
    let min_messages_to_keep = 25; // System prompt + enough recent context
    let max_safe_tokens = EVICTION_THRESHOLD;

    if messages.is_empty() || messages.len() <= min_messages_to_keep {
        return Ok(PruneResult { messages_removed: 0, tokens_reduced: 0 });
    }

    // Fast path: check current token count on background thread
    let initial_tokens: usize = task::spawn_blocking({
        let msgs = messages.clone();
        move || calculate_token_count(&msgs)
    }).await?;

    if initial_tokens <= max_safe_tokens {
        return Ok(PruneResult { messages_removed: 0, tokens_reduced: 0 });
    }

    let mut removed = 0usize;

    // Prune loop: remove from the oldest end (after index 0)
    while messages.len() > min_messages_to_keep {
        let current_tokens: usize = task::spawn_blocking({
            let msgs = messages.clone();
            move || calculate_token_count(&msgs)
        }).await?;

        if current_tokens <= max_safe_tokens {
            break;
        }

        // Remove oldest pair if possible (User + Assistant)
        if messages.len() >= 3 {
            // Check if index 1 is user and index 2 is assistant (common pattern)
            let idx1 = &messages[1];
            let idx2 = &messages[2];
            if idx1.role == "user" && idx2.role == "assistant" {
                messages.remove(1);
                messages.remove(1); // now index 1 is what was 2
                removed += 2;
                continue;
            }
        }

        // Fallback: remove single oldest message (after index 0)
        if messages.len() > 1 {
            messages.remove(1);
            removed += 1;
        } else {
            crate::ui_log!("   [⚠️ CONSORTIUM] Oblivion reached safety floor unexpectedly");
            break;
        }
    }

    // Atomic save to disk using temporary file pattern
    let _ = atomic_save_working_memory(messages).await;

    let final_tokens = calculate_token_count(messages); // sync final count is fine
    let reduced = initial_tokens.saturating_sub(final_tokens);

    crate::ui_log!(
        "   [🧠 OBLIVION] Protocol executed: Pruned {} historically rigid nodes. Matrix stabilized at {} tokens.",
        removed, final_tokens
    );

    Ok(PruneResult {
        messages_removed: removed,
        tokens_reduced: reduced,
    })
}

fn calculate_token_count(messages: &[Message]) -> usize {
    let tokenizer = cl100k_base().expect("cl100k_base tokenizer failed to load");
    let mut total = 0;

    for msg in messages {
        total += tokenizer.encode_ordinary(&msg.content).len();
        if let Some(ref reasoning) = msg.reasoning_content {
            total += tokenizer.encode_ordinary(reasoning).len();
        }
    }

    total
}

/// Reusable atomic writer (mirror of ozymandias checkpoint style)
async fn atomic_save_working_memory(messages: &[Message]) -> Result<()> {
    let path = std::path::Path::new(MEMORY_FILE);
    let dir = path.parent().ok_or_else(|| anyhow!("No parent dir"))?;
    if !dir.exists() {
        tokio::fs::create_dir_all(dir).await?;
    }

    // Wrap the array in the WorkingMemory struct to preserve JSON schema!
    let mem = WorkingMemory { messages: messages.to_vec() };
    let serialized = serde_json::to_string_pretty(&mem)?;

    let tmp_path = path.with_extension("tmp.json");
    tokio::fs::write(&tmp_path, serialized).await?;
    if let Ok(file) = tokio::fs::File::open(&tmp_path).await {
        let _ = file.sync_all().await;
    }
    std::fs::rename(&tmp_path, path)?;

    Ok(())
}
