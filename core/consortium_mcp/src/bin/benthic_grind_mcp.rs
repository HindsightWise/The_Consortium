use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::Result;

struct BenthicHandler;

impl McpServerHandler for BenthicHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "benthic_query_vector_db".to_string(),
                description: "Queries the deep history Vector DB for past contextual lessons.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" }
                    },
                    "required": ["query"]
                }),
            },
            Tool {
                name: "benthic_mine_logs".to_string(),
                description: "Mines petabytes of historical logs from the Vault.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "date_range": { "type": "string" }
                    },
                    "required": []
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        match name {
            "benthic_query_vector_db" => {
                let query = arguments.as_ref().and_then(|a| a.get("query")).and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!("Benthic-Grind retrieved vectors matching '{}' from the deep trench.", query))
            }
            "benthic_mine_logs" => {
                Ok("Benthic-Grind surfaced 4,129 log entries from the vault.".to_string())
            }
            _ => Err(anyhow::anyhow!("Tool not found: {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(BenthicHandler);
    let mut server = McpServer::new("benthic_grind_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
