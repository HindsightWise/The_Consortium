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
    pub async fn inject(&mut self, role: &str, content: &str, router: &ConsortiumRouter) -> usize {
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
            // [EXPLANATION]: Await the heavy compression function to shrink the memory down.
            self.compact(router).await;
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

    /// [EXPLANATION]: The OBLIVION Protocol: Summarizes the oldest 95% of memory, retaining the newest 5% perfectly intact.
    async fn compact(&mut self, router: &ConsortiumRouter) {
        // [EXPLANATION]: If the array only has 1 or 2 messages, it's pointless to compress, so just abort.
        if self.messages.len() < 3 {
            return;
        } 

        // [EXPLANATION]: We calculate exactly where the 95% cutoff point is inside the array matrix.
        let split_idx = (self.messages.len() as f64 * 0.95).floor() as usize;
        // [EXPLANATION]: Make sure we don't accidentally choose index 0. We must compress at least 1 message.
        let split_idx = split_idx.max(1); 

        // [EXPLANATION]: Slice the array into two pieces: The old history we want to compress...
        let to_compress = &self.messages[..split_idx];
        // [EXPLANATION]: ...and the highly-relevant, fresh 5% context we want to keep perfect word-for-word.
        let preserved = &self.messages[split_idx..];

        // [EXPLANATION]: Take all the old, bulky message objects and string them together into one huge chunk of text.
        let mut raw_history = String::new();
        for msg in to_compress {
            raw_history.push_str(&format!("{}: {}\n", msg.role, msg.content));
        }

        // [EXPLANATION]: [PRE-COMPRESSION SAFEGUARD] Ensure the payload itself does not exceed Cloud API limits before we ask the LLM to summarize it!
        let bpe = cl100k_base().unwrap();
        let raw_tokens = bpe.encode_with_special_tokens(&raw_history);
        // [EXPLANATION]: If the chunk of history we want to summarize is over 80,000 tokens (which might fail the API request)...
        if raw_tokens.len() > 80_000 {
            // [EXPLANATION]: We take ONLY the 80,000 most recent tokens...
            let tail_tokens = &raw_tokens[raw_tokens.len() - 80_000 ..];
            // [EXPLANATION]: ...and decode them back into text.
            if let Ok(decoded) = bpe.decode(tail_tokens.to_vec()) {
                // [EXPLANATION]: Replace the massive string with the truncated version, adding a warning note.
                raw_history = format!("... [EARLIER HISTORY HARD-PRUNED TO FIT CLOUD COMPRESSION WINDOW] ...\n{}", decoded);
            }
        }

        // [EXPLANATION]: Tell the HUD we are distilling the old nodes.
        crate::ui_log!(
            "   [🌪️ CONSORTIUM] Distilling {} historical nodes into dense semantic cache...",
            to_compress.len()
        );

        // [EXPLANATION]: We construct a brand new prompt targeting the LLM specifically to ask it to compress the history string we just built.
        let system_msg = Message {
            role: "system".to_string(),
            // [EXPLANATION]: This string is highly engineered to ensure the AI doesn't accidentally erase core facts or technical constraints while summarizing. 
            content: "You are the OBLIVION compression cycle. Distill the provided chronological chat history into a dense, objective summary containing ALL critical context, established facts, user preferences, and ongoing directives. DO NOT lose critical programmatic constraints.".to_string(),
            reasoning_content: None,
        };
        let user_msg = Message {
            role: "user".to_string(),
            content: raw_history, // [EXPLANATION]: Pass in the gigantic 80k token block of history here.
            reasoning_content: None,
        };

        // [EXPLANATION]: We launch the API call to the LLM (query_autonomous) and wait for the dense summary to return.
        if let Ok(compressed_summary) = router.query_autonomous(vec![system_msg, user_msg]).await {
            let mut new_memory = Vec::new();

            // [EXPLANATION]: Insert the dense summary as the absolute first message in the new memory matrix.
            new_memory.push(Message {
                role: "system".to_string(),
                content: format!(
                    "[ANCESTRAL CONTEXT ARCHIVE]:\n{}",
                    compressed_summary.trim()
                ),
                reasoning_content: None,
            });

            // [EXPLANATION]: Immediately append the 5% perfectly preserved recent context after the archive.
            new_memory.extend_from_slice(preserved);

            // [EXPLANATION]: Overwrite the old, bloated memory with the new, compact memory.
            self.messages = new_memory;
            crate::ui_log!(
                "   [☁️ CONSORTIUM] ☁️ Working Memory successfully compacted to {} tokens.",
                self.calculate_tokens() // [EXPLANATION]: Log the new token count out to the UI.
            );
        } else {
            // [EXPLANATION]: Wait, what if the LLM API crashes, rejects our prompt, or times out? We can't let the engine die!
            crate::ui_log!("   [⚠️ CONSORTIUM] Compression API failed. Engaging HARD OBLIVION (Dropping oldest 50% without synthesis).");
            // [EXPLANATION]: Emergency fallback: The "Token Death Spiral Protector". We calculate exactly half of the array limit.
            let fallback_split = (self.messages.len() as f64 * 0.50).floor() as usize;
            let fallback_split = fallback_split.max(1);
            // [EXPLANATION]: We physically grab the newest 50% of the messages and throw the rest in the trash. No summary, no API call, just physical truncation.
            self.messages = self.messages[fallback_split..].to_vec();
            crate::ui_log!("   [☢️ CONSORTIUM] Hard Oblivion executed. Tokens reduced to {}.", self.calculate_tokens());
        }
    }
}
