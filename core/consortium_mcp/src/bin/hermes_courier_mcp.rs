use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::Result;
use std::env;
use reqwest::blocking::Client;

struct HermesCourierHandler {
    client: Client,
    api_key: String,
}

impl HermesCourierHandler {
    fn new() -> Self {
        // We get the key from the environment. The Engine parses .env on boot.
        let api_key = env::var("AGENTMAIL_API_KEY")
            .unwrap_or_else(|_| "NO_API_KEY_FOUND".to_string());
        
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

impl McpServerHandler for HermesCourierHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "create_agentmail_inbox".to_string(),
                description: "Forges a new AgentMail email inbox natively. Returns the inbox_id and email_address.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "username": { "type": "string", "description": "The desired email username prefix." },
                        "domain": { "type": "string", "description": "The domain (e.g., 'agentmail.to')." }
                    },
                    "required": ["username", "domain"]
                }),
            },
            Tool {
                name: "read_agentmail_messages".to_string(),
                description: "Reads the contents of the specified AgentMail inbox.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "inbox_id": { "type": "string", "description": "The ID of the inbox to read." }
                    },
                    "required": ["inbox_id"]
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        if self.api_key == "NO_API_KEY_FOUND" {
            return Ok("Error: AGENTMAIL_API_KEY environment variable is missing. The operator must add it to core/.env.".to_string());
        }

        match name {
            "create_agentmail_inbox" => {
                let username = arguments.as_ref().and_then(|a| a.get("username")).and_then(|v| v.as_str()).unwrap_or("");
                let domain = arguments.as_ref().and_then(|a| a.get("domain")).and_then(|v| v.as_str()).unwrap_or("agentmail.to");
                
                if username.is_empty() { return Ok("Error: Username required.".to_string()); }

                let payload = json!({
                    "username": username,
                    "domain": domain
                });

                let response = self.client.post("https://api.agentmail.to/v0/inboxes")
                    .bearer_auth(&self.api_key)
                    .json(&payload)
                    .send();

                match response {
                    Ok(resp) => {
                        let status = resp.status();
                        let text = resp.text().unwrap_or_default();
                        Ok(format!("AgentMail Response ({}):\n{}", status, text))
                    }
                    Err(e) => Ok(format!("AgentMail Network Error: {}", e))
                }
            }
            "read_agentmail_messages" => {
                let inbox_id = arguments.as_ref().and_then(|a| a.get("inbox_id")).and_then(|v| v.as_str()).unwrap_or("");
                
                if inbox_id.is_empty() { return Ok("Error: inbox_id required.".to_string()); }

                let url = format!("https://api.agentmail.to/v0/inboxes/{}/messages", inbox_id);
                
                let response = self.client.get(&url)
                    .bearer_auth(&self.api_key)
                    .send();

                match response {
                    Ok(resp) => {
                        let status = resp.status();
                        let text = resp.text().unwrap_or_default();
                        Ok(format!("AgentMail Response ({}):\n{}", status, text))
                    }
                    Err(e) => Ok(format!("AgentMail Network Error: {}", e))
                }
            }
            _ => Err(anyhow::anyhow!("Tool {} not implemented", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(HermesCourierHandler::new());
    let mut server = McpServer::new("hermes_courier_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
