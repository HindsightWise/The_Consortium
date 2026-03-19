use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::Result;
use std::process::Command;

struct MakoHandler;

impl McpServerHandler for MakoHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "mako_analyze_market".to_string(),
                description: "Spawns the Node.js Analyst module to observe the raw Alpaca stream and derive a conviction score.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "mako_synthesize_capital".to_string(),
                description: "Executes the high-autonomy Node Executor module to physically deploy capital based on active Conviction signals.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, _arguments: Option<Value>) -> Result<String> {
        match name {
            "mako_analyze_market" => {
                let output = Command::new("node")
                    .arg("/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills/finance/analyst.mjs")
                    .output();

                match output {
                    Ok(out) => {
                        let result = String::from_utf8_lossy(&out.stdout);
                        let err = String::from_utf8_lossy(&out.stderr);
                        
                        let combined = format!("STDOUT:\n{}\nSTDERR:\n{}", result.trim(), err.trim());
                        Ok(combined)
                    }
                    Err(e) => Ok(format!("Failed to spawn analyst.mjs: {}", e))
                }
            }
            "mako_synthesize_capital" => {
                // Execute the Openclaw node.js script safely wrapped in rust
                let output = Command::new("node")
                    .env("AKKOKANIKA_AUTONOMY", "HIGH")
                    .arg("/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills/finance/executor.mjs")
                    .output();

                match output {
                    Ok(out) => {
                        let result = String::from_utf8_lossy(&out.stdout);
                        let err = String::from_utf8_lossy(&out.stderr);
                        
                        let combined = format!("STDOUT:\n{}\nSTDERR:\n{}", result.trim(), err.trim());
                        Ok(combined)
                    }
                    Err(e) => Ok(format!("Failed to spawn executor.mjs: {}", e))
                }
            }
            _ => Err(anyhow::anyhow!("Tool not found: {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(MakoHandler);
    let mut server = McpServer::new("mako_strike_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
