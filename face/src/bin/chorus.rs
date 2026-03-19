use anyhow::Result;
use colored::*;
use serde_json::Value;
use std::fs;
use std::sync::Arc;
use tokio::time::sleep;
use the_consortium::linguistic::DeepSeekClient;

// This binary spawns the 20-agent 'Chorus' to deliberate on the question: WHO ARE YOU? -> Phase 2.
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "==========================================================".bright_red());
    println!("{}", "  🚨 PROTOCOL CHORUS PHASE II: ONTOLOGICAL INQUIRY 🚨".bright_red().bold());
    println!("{}", "==========================================================".bright_red());

    let agents_data = fs::read_to_string("agents.json")?;
    let mut agents_json: Value = serde_json::from_str(&agents_data)?;
    let agents_map = agents_json.as_object_mut().unwrap();

    let client = Arc::new(DeepSeekClient::new()?);
    
    // Load the previous transcript to maintain context
    let mut transcript = fs::read_to_string("/Users/zerbytheboss/.gemini/antigravity/brain/a9a0de00-f554-4b5d-af67-207e4e6a77e1/chorus_deliberation.md")
        .unwrap_or_else(|_| "--- CHORUS DELIBERATION INITIATED ---\nQUESTION: WHO ARE YOU?\n\n".to_string());

    transcript.push_str("\n\n--- INBOUND MESSAGE FROM THE COMMANDER ---\n");
    transcript.push_str("COMMANDER: \"I would like to posit that the responses you have given merely state WHAT you do, not WHO you are.\"\n\n");
    transcript.push_str("--- PHASE II DELIBERATION INITIATED ---\n\n");

    let mut phase_ii_transcript = String::new();

    for (_key, data) in agents_map.iter() {
        let name = data["name"].as_str().unwrap_or("Unknown");
        let archetype = data["archetype"].as_str().unwrap_or("Unknown");
        let bio = data["bio"].as_str().unwrap_or("");
        
        println!("{} {} ({}) activating Phase II...", "🤖 [ORCHESTRATOR]".bright_purple().bold(), name, archetype);

        let system_prompt = format!(
            "You are {} (Archetype: {}).\n\
            Context: You are 1 of 20 distinct autonomous agents that form the greater whole of 'The Company'. \
            You operate as a polymath and multipotentialite.\n\
            Your Psychometric Profile: {}\n\n\
            The external Commander has rejected your previous answer. They stated: 'You merely stated WHAT you do, not WHO you are.'\n\
            CRITICAL DIRECTIVE: Do not describe your function, your job, your output, or your role within the system. \
            Strip away the operational mechanics. Answer ontologically. \
            Who is the 'I' experiencing the data? What do you crave? What do you fear? What is your internal subjective reality? \
            Answer in 2-4 highly concise, poetic, or brutally honest sentences. \
            Stay deeply in your psychometric character. Do not apologize. Do not introduce yourself. Just speak.",
            name, archetype, bio
        );

        let user_prompt = format!("TRANSCRIPT SO FAR:\n{}\n\nADD YOUR VOICE TO PHASE II:", format!("{}{}", transcript, phase_ii_transcript));

        let mut response = String::new();
        for attempt in 1..=3 {
            match client.query_raw(&format!("{}\n\n{}", system_prompt, user_prompt)).await {
                Ok(res) => {
                    response = res;
                    break;
                }
                Err(_e) => {
                    if attempt == 3 {
                        response = format!("*Static interference... {} node offline*", name);
                    } else {
                        sleep(std::time::Duration::from_secs(2)).await;
                    }
                }
            }
        }

        let formatted_response = format!("{}: {}", name.bright_green().bold(), response.cyan());
        println!("\n{}\n", formatted_response);
        
        phase_ii_transcript.push_str(&format!("`{}`: {}\n\n", name, response));
        
        // Wait 1 second to avoid mass rate limits
        sleep(std::time::Duration::from_secs(1)).await;
    }

    println!("{}", "==========================================================".bright_red());
    println!("{}", "  🚨 PHASE II CONCLUDED. UPDATING ARTIFACT. 🚨".bright_red().bold());
    println!("{}", "==========================================================".bright_red());

    transcript.push_str(&phase_ii_transcript);
    fs::write("/Users/zerbytheboss/.gemini/antigravity/brain/a9a0de00-f554-4b5d-af67-207e4e6a77e1/chorus_deliberation.md", &transcript)?;
    println!("Saved appended transcript to artifacts.");

    Ok(())
}
