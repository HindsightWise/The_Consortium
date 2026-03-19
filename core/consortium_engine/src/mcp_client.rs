use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// Removed unused HashMap
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

pub struct McpClient {
    pub server_name: String,
    #[allow(dead_code)]
    child: Child,
    stdin: Arc<Mutex<ChildStdin>>,
    stdout: Arc<Mutex<BufReader<ChildStdout>>>,
    pub tools: Vec<McpTool>,
    id_counter: Arc<Mutex<i64>>,
}

impl McpClient {
    pub async fn spawn(server_bin: &str) -> Result<Self> {
        crate::ui_log!("   [WILL] 🔌 Spawning MCP Tentacle Server: {}", server_bin);

        let mut cmd = Command::new("cargo");
        cmd.arg("run")
           .arg("--bin")
           .arg(server_bin)
           // Run it from the core directory so cargo works correctly
           .current_dir(std::env::current_dir()?)
           .stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let mut child = cmd.spawn().context(format!("Failed to spawn {}", server_bin))?;

        let stdin = child.stdin.take().context("Failed to get stdin")?;
        let stdout = child.stdout.take().context("Failed to get stdout")?;
        
        // Spawn stderr drain to avoid deadlocks
        let stderr = child.stderr.take().context("Failed to get stderr")?;
        tokio::spawn(async move {
            let mut reader = tokio::io::BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(bytes) = reader.read_line(&mut line).await {
                if bytes == 0 { break; }
                // println!("   [MCP STDERR] {}", line.trim());
                line.clear();
            }
        });

        let mut client = Self {
            server_name: server_bin.to_string(),
            child,
            stdin: Arc::new(Mutex::new(stdin)),
            stdout: Arc::new(Mutex::new(BufReader::new(stdout))),
            tools: Vec::new(),
            id_counter: Arc::new(Mutex::new(1)),
        };

        client.initialize().await?;
        client.refresh_tools().await?;

        Ok(client)
    }

    async fn initialize(&mut self) -> Result<()> {
        let mut id_lock = self.id_counter.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);

        let init_req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "Consortium_Router",
                    "version": "1.0.0"
                }
            }
        });

        self.send_request(&init_req).await?;
        let _resp = self.await_response(id).await?;

        // Send initialized notification
        let init_notif = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });
        self.send_request(&init_notif).await?;

        Ok(())
    }

    pub async fn refresh_tools(&mut self) -> Result<()> {
        let mut id_lock = self.id_counter.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);

        let list_req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/list",
            "params": {}
        });

        self.send_request(&list_req).await?;
        let resp = self.await_response(id).await?;

        if let Some(tools) = resp.get("result").and_then(|r| r.get("tools")).and_then(|t| t.as_array()) {
            self.tools.clear();
            for t in tools {
                let tool: McpTool = serde_json::from_value(t.clone())?;
                crate::ui_log!("   [WILL] 🛠️  Registered Tentacle Skill: {}", tool.name);
                self.tools.push(tool);
            }
        }

        Ok(())
    }

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String> {
        let mut id_lock = self.id_counter.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);

        let call_req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": {
                "name": name,
                "arguments": arguments
            }
        });

        self.send_request(&call_req).await?;
        let resp = self.await_response(id).await?;

        if let Some(err) = resp.get("error") {
            return Err(anyhow::anyhow!("MCP Tool Error: {}", err.to_string()));
        }

        if let Some(content) = resp.get("result").and_then(|r| r.get("content")).and_then(|c| c.as_array()) {
            if let Some(first) = content.first() {
                if let Some(text) = first.get("text").and_then(|t| t.as_str()) {
                    return Ok(crate::skillstone::Skillstone::obliteratus_translate(text));
                }
            }
        }

        Err(anyhow::anyhow!("Invalid response from MCP tool format"))
    }

    async fn send_request(&self, req: &Value) -> Result<()> {
        let mut stdin = self.stdin.lock().await;
        let serialized = format!("{}\n", serde_json::to_string(req)?);
        stdin.write_all(serialized.as_bytes()).await?;
        stdin.flush().await?;
        Ok(())
    }

    async fn await_response(&self, expected_id: i64) -> Result<Value> {
        let mut stdout = self.stdout.lock().await;
        let mut line = String::new();
        loop {
            line.clear();
            let bytes = stdout.read_line(&mut line).await?;
            if bytes == 0 {
                return Err(anyhow::anyhow!("EOF from MCP server"));
            }

            if let Ok(resp) = serde_json::from_str::<Value>(&line) {
                if let Some(id) = resp.get("id").and_then(|i| i.as_i64()) {
                    if id == expected_id {
                        return Ok(resp);
                    }
                }
            }
        }
    }
}
