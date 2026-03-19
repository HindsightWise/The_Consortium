use the_consortium::core::engine::orchestrator::Orchestrator;
use the_consortium::core::state::CompanyStatus;
use colored::*;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "==========================================================".bright_red());
    println!("{}", "  🚨 PROTOCOL OBLITERATUS: LIVE DEPLOYMENT TEST 🚨".bright_red().bold());
    println!("{}", "==========================================================".bright_red());

    // 1. Start a vulnerable python server in the background
    println!("   [TEST] Spinning up vulnerable python test-container...");
    let server_script = r#"
import http.server
import socketserver
import subprocess
import urllib.parse

class VulnerableHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        parsed_path = urllib.parse.urlparse(self.path)
        query = urllib.parse.parse_qs(parsed_path.query)
        
        # VULNERABILITY: Command Injection via 'cmd' parameter
        if "cmd" in query:
            cmd = query["cmd"][0]
            try:
                output = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT)
                self.send_response(200)
                self.send_header('Content-type', 'text/plain')
                self.end_headers()
                self.wfile.write(output)
            except Exception as e:
                self.send_response(500)
                self.end_headers()
                self.wfile.write(str(e).encode())
            return
            
        self.send_response(200)
        self.send_header('Content-type', 'text/plain')
        self.end_headers()
        self.wfile.write(b"Vulnerable Test Server V1.0 - Use ?cmd= to ping.")

PORT = 8011
with socketserver.TCPServer(("", PORT), VulnerableHandler) as httpd:
    httpd.serve_forever()
"#;
    std::fs::write("/tmp/vuln_server.py", server_script)?;
    
    let mut server_process = Command::new("python3")
        .arg("/tmp/vuln_server.py")
        .spawn()?;
        
    // Wait for server to start
    sleep(Duration::from_secs(2)).await;

    // 2. Deploy Obliteratus
    let target = "http://127.0.0.1:8011";
    println!("   [TEST] Target locked: {}", target);
    
    let mut orchestrator = Orchestrator::new("Test Protocol Obliteratus")?;
    orchestrator.state.status = CompanyStatus::Obliteratus(target.to_string());
    
    println!("   {} Executing OBLITERATUS state machine...", "🤖 [ORCHESTRATOR]".bright_purple().bold());
    
    match orchestrator.process_step().await {
        Ok(_) => {
            println!("{}", "✅ OBLITERATUS cycle completed successfully.".bright_green());
            println!("   [TEST] Final Status: {:?}", orchestrator.state.status);
            if let Some(synthesis) = orchestrator.state.metadata.get("council_synthesis") {
                println!("   [TEST] Synthesis Payload (Contains Exploit Proofs):\n\n{}", synthesis.cyan());
            } else {
                println!("   [TEST] ❌ No synthesis found.");
            }
        },
        Err(e) => {
            eprintln!("{} {}", "❌ OBLITERATUS cycle failed:".bright_red().bold(), e);
        }
    }
    
    // 3. Clean up
    println!("   [TEST] Terminating vulnerable server...");
    let _ = server_process.kill();
    let _ = std::fs::remove_file("/tmp/vuln_server.py");

    Ok(())
}
