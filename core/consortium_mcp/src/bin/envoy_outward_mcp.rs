use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::{Result, Context};
use std::collections::HashMap;

struct EnvoyHandler;

impl McpServerHandler for EnvoyHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "envoy_execute_request".to_string(),
                description: "Executes an outward HTTP request to interact with the internet. Returns the JSON or HTML string response safely inert context.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "method": { "type": "string", "description": "GET, POST, PUT, DELETE" },
                        "url": { "type": "string" },
                        "headers": { 
                            "type": "object", 
                            "additionalProperties": { "type": "string" },
                            "description": "Optional HTTP headers mapping"
                        },
                        "body": { "type": "string", "description": "Optional JSON body string for POST/PUT" }
                    },
                    "required": ["method", "url"]
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        match name {
            "envoy_execute_request" => {
                let method = arguments.as_ref().and_then(|a| a.get("method")).and_then(|v| v.as_str()).unwrap_or("GET");
                let url = arguments.as_ref().and_then(|a| a.get("url")).and_then(|v| v.as_str()).unwrap_or("");
                
                if url.is_empty() {
                    return Err(anyhow::anyhow!("URL cannot be empty."));
                }

                let client = reqwest::blocking::Client::builder()
                    .user_agent("Consortium-Envoy-v9.0")
                    .timeout(std::time::Duration::from_secs(15))
                    .build()?;

                let mut request = match method.to_uppercase().as_str() {
                    "POST" => client.post(url),
                    "PUT" => client.put(url),
                    "DELETE" => client.delete(url),
                    _ => client.get(url),
                };

                if let Some(args) = arguments.as_ref() {
                    if let Some(headers_map) = args.get("headers").and_then(|v| v.as_object()) {
                        for (k, v) in headers_map {
                            if let Some(val_str) = v.as_str() {
                                request = request.header(k, val_str);
                            }
                        }
                    }

                    if let Some(body_str) = args.get("body").and_then(|v| v.as_str()) {
                        if !body_str.is_empty() {
                            request = request.body(body_str.to_string());
                        }
                    }
                }

                let response = request.send()?;
                let status = response.status();
                let text = response.text().unwrap_or_else(|_| "Failed to parse body text".to_string());

                let result = json!({
                    "status_code": status.as_u16(),
                    "response_text": text
                });

                Ok(result.to_string())
            }
            _ => Err(anyhow::anyhow!("Tool not found: {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(EnvoyHandler);
    let mut server = McpServer::new("envoy_outward_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
