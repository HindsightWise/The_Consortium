use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::Result;

struct ChromatoHandler;

impl McpServerHandler for ChromatoHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "chromato_format_markdown".to_string(),
                description: "Applies exquisite formatting and color to terminal output or markdown records.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "raw_text": { "type": "string" },
                        "mood": { "type": "string", "description": "The current mood/vibe (e.g. 'cyberpunk_red', 'oceanic_blue')" }
                    },
                    "required": ["raw_text", "mood"]
                }),
            },
            Tool {
                name: "chromato_render_ui".to_string(),
                description: "Generates HTML/CSS snippets for dynamic UI rendering.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "component_name": { "type": "string" }
                    },
                    "required": ["component_name"]
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        match name {
            "chromato_format_markdown" => {
                let raw = arguments.as_ref().and_then(|a| a.get("raw_text")).and_then(|v| v.as_str()).unwrap_or("");
                let mood = arguments.as_ref().and_then(|a| a.get("mood")).and_then(|v| v.as_str()).unwrap_or("default");
                Ok(format!("[Chromato-Charm formatted ({})]:\n***\n{}\n***", mood, raw))
            }
            "chromato_render_ui" => {
                let component = arguments.as_ref().and_then(|a| a.get("component_name")).and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!("<div class='chromato-{}'>Dynamic {} Rendered</div>", component, component))
            }
            _ => Err(anyhow::anyhow!("Tool not found: {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(ChromatoHandler);
    let mut server = McpServer::new("chromato_charm_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
