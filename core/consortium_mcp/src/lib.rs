use std::io::{self, BufRead, Write};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

pub trait McpServerHandler {
    fn tools(&self) -> Vec<Tool>;
    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String>;
}

pub struct McpServer {
    name: String,
    version: String,
    handler: Box<dyn McpServerHandler + Send + Sync>,
}

impl McpServer {
    pub fn new(name: &str, version: &str, handler: Box<dyn McpServerHandler + Send + Sync>) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            handler,
        }
    }

    pub fn run_stdio(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut line = String::new();

        loop {
            line.clear();
            if handle.read_line(&mut line)? == 0 {
                break; // EOF
            }

            let req_str = line.trim();
            if req_str.is_empty() {
                continue;
            }

            if let Ok(req) = serde_json::from_str::<Value>(req_str) {
                if let Some(method) = req.get("method").and_then(|m| m.as_str()) {
                    let id = req.get("id");
                    match method {
                        "initialize" => {
                            let resp = json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "protocolVersion": "2024-11-05",
                                    "capabilities": {
                                        "tools": {}
                                    },
                                    "serverInfo": {
                                        "name": self.name,
                                        "version": self.version
                                    }
                                }
                            });
                            Self::respond(&resp)?;
                        }
                        "notifications/initialized" => {
                            // Acknowledge initialization without response
                        }
                        "tools/list" => {
                            let tools = self.handler.tools();
                            let resp = json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "tools": tools
                                }
                            });
                            Self::respond(&resp)?;
                        }
                        "tools/call" => {
                            let params = req.get("params");
                            let name = params.and_then(|p| p.get("name")).and_then(|n| n.as_str()).unwrap_or("");
                            let arguments = params.and_then(|p| p.get("arguments")).cloned();

                            match self.handler.call_tool(name, arguments) {
                                Ok(content) => {
                                    let resp = json!({
                                        "jsonrpc": "2.0",
                                        "id": id,
                                        "result": {
                                            "content": [{
                                                "type": "text",
                                                "text": content
                                            }]
                                        }
                                    });
                                    Self::respond(&resp)?;
                                }
                                Err(e) => {
                                    let resp = json!({
                                        "jsonrpc": "2.0",
                                        "id": id,
                                        "error": {
                                            "code": -32603,
                                            "message": e.to_string()
                                        }
                                    });
                                    Self::respond(&resp)?;
                                }
                            }
                        }
                        _ => {
                            // Unsupported method
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn respond(value: &Value) -> Result<()> {
        let mut stdout = io::stdout().lock();
        writeln!(stdout, "{}", serde_json::to_string(value)?)?;
        stdout.flush()?;
        Ok(())
    }
}
