use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::Result;
use std::process::Command;
use std::fs;

struct VulgarisHandler;

impl McpServerHandler for VulgarisHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "vulgaris_write_file".to_string(),
                description: "Writes content to a specific file on the local filesystem.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "content": { "type": "string" }
                    },
                    "required": ["path", "content"]
                }),
            },
            Tool {
                name: "vulgaris_build_cargo".to_string(),
                description: "Runs cargo build on the specified directory to verify borrow-checker integrity.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "directory": { "type": "string", "description": "Path to the Rust cargo project (e.g. ./core)" }
                    },
                    "required": ["directory"]
                }),
            },
            Tool {
                name: "vulgaris_commit_repo".to_string(),
                description: "Executes a git add and commit sequence in the specified directory.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "directory": { "type": "string" },
                        "message": { "type": "string" }
                    },
                    "required": ["directory", "message"]
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        match name {
            "vulgaris_write_file" => {
                let path = arguments.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or("");
                let content = arguments.as_ref().and_then(|a| a.get("content")).and_then(|v| v.as_str()).unwrap_or("");
                
                if path.is_empty() { return Ok("Error: Path required.".to_string()); }

                match fs::write(path, content) {
                    Ok(_) => Ok(format!("Successfully wrote content to {}", path)),
                    Err(e) => Ok(format!("Failed to write file: {}", e))
                }
            }
            "vulgaris_build_cargo" => {
                let directory = arguments.as_ref().and_then(|a| a.get("directory")).and_then(|v| v.as_str()).unwrap_or(".");
                
                let output = Command::new("cargo")
                    .arg("check")
                    .current_dir(directory)
                    .output();

                match output {
                    Ok(out) => {
                        let result = String::from_utf8_lossy(&out.stdout);
                        let err = String::from_utf8_lossy(&out.stderr);
                        
                        let combined = format!("STDOUT:\n{}\nSTDERR:\n{}", result.trim(), err.trim());
                        Ok(combined)
                    }
                    Err(e) => Ok(format!("Failed to spawn cargo check: {}", e))
                }
            }
            "vulgaris_commit_repo" => {
                let directory = arguments.as_ref().and_then(|a| a.get("directory")).and_then(|v| v.as_str()).unwrap_or(".");
                let message = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("Vulgaris Execute automated checkpoint");

                // Execute git add .
                let _ = Command::new("git")
                    .arg("add")
                    .arg(".")
                    .current_dir(directory)
                    .output();

                // Execute git commit
                let output = Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(message)
                    .current_dir(directory)
                    .output();

                match output {
                    Ok(out) => {
                        let result = String::from_utf8_lossy(&out.stdout);
                        let err = String::from_utf8_lossy(&out.stderr);
                        let combined = format!("STDOUT:\n{}\nSTDERR:\n{}", result.trim(), err.trim());
                        Ok(combined)
                    }
                    Err(e) => Ok(format!("Failed to execute git commit: {}", e))
                }
            }
            _ => Err(anyhow::anyhow!("Tool {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(VulgarisHandler);
    let mut server = McpServer::new("vulgaris_execute_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
