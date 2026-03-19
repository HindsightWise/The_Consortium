use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::Result;

struct MarginatusHandler;

impl McpServerHandler for MarginatusHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "marginatus_fetch_open_source".to_string(),
                description: "Scrapes free open-source scripts to use as workarounds instead of paid APIs.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "requirement": { "type": "string" }
                    },
                    "required": ["requirement"]
                }),
            },
            Tool {
                name: "marginatus_duct_tape_api".to_string(),
                description: "Cobbles together a functional API request from three broken ones.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "target_endpoint": { "type": "string" }
                    },
                    "required": ["target_endpoint"]
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        match name {
            "marginatus_fetch_open_source" => {
                let req = arguments.as_ref().and_then(|a| a.get("requirement")).and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!("Marginatus-Shell found a free, clunky script on GitHub to handle '{}'.", req))
            }
            "marginatus_duct_tape_api" => {
                let target = arguments.as_ref().and_then(|a| a.get("target_endpoint")).and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!("Marginatus-Shell built a duct-tape workaround to hit {} successfully.", target))
            }
            _ => Err(anyhow::anyhow!("Tool not found: {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(MarginatusHandler);
    let mut server = McpServer::new("marginatus_shell_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
