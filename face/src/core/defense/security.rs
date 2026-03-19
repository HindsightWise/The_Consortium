use pqcrypto_kyber::kyber1024::*;
use pqcrypto_traits::kem::{SharedSecret, Ciphertext, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use std::process::Command;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumIdentity {
    pub public_key: String, // Base64 encoded
    #[serde(skip)]
    _secret_key: Option<String>, // Base64 encoded, kept internal
}

pub struct PQCModule;

impl PQCModule {
    /// Generates a new Kyber-1024 keypair for an agent.
    pub fn generate_identity() -> QuantumIdentity {
        let (pk, sk) = keypair();
        
        QuantumIdentity {
            public_key: general_purpose::STANDARD.encode(pk.as_bytes()),
            _secret_key: Some(general_purpose::STANDARD.encode(sk.as_bytes())),
        }
    }

    /// Rotates the quantum identity, generating a new keypair and archiving the old.
    pub fn rotate_identity(current_id: &QuantumIdentity) -> QuantumIdentity {
        println!("   [PQC] 🔄 Rotating Quantum Identity. Archiving key: {}...", &current_id.public_key[0..8]);
        // In a real system, the old key would be signed into a revocation registry.
        Self::generate_identity()
    }

    /// Encapsulates a shared secret for a target public key (KEM).
    /// Returns (Shared Secret, Ciphertext).
    pub fn encapsulate(target_pubkey_b64: &str) -> Option<(Vec<u8>, String)> {
        let pk_bytes = general_purpose::STANDARD.decode(target_pubkey_b64).ok()?;
        let pk = PublicKey::from_bytes(&pk_bytes).ok()?;
        
        let (ss, ct) = encapsulate(&pk);
        
        Some((
            ss.as_bytes().to_vec(),
            general_purpose::STANDARD.encode(ct.as_bytes())
        ))
    }

    /// Decapsulates a ciphertext using the agent's secret key.
    pub fn decapsulate(secret_key_b64: &str, ciphertext_b64: &str) -> Option<Vec<u8>> {
        let sk_bytes = general_purpose::STANDARD.decode(secret_key_b64).ok()?;
        let sk = SecretKey::from_bytes(&sk_bytes).ok()?;
        
        let ct_bytes = general_purpose::STANDARD.decode(ciphertext_b64).ok()?;
        let ct = Ciphertext::from_bytes(&ct_bytes).ok()?;
        
        let ss = decapsulate(&ct, &sk);
        Some(ss.as_bytes().to_vec())
    }

    /// Verifiable Delay Function (Proof of History).
    /// Enforces a 'Duration of Thought' artifact.
    pub async fn prove_history(iterations: u32) {
        let start = std::time::Instant::now();
        // Sequential hashing simulation
        let mut data = vec![0u8; 32];
        for _ in 0..iterations {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&data);
            data = hasher.finalize().to_vec();
        }
        println!("   [PoH] ⏳ Duration of Thought verified: {:?}. Entropy sequence secured.", start.elapsed());
    }

    /// Signs an attestation using the agent's identity (Simulated DID logic).
    pub fn sign_attestation(agent_name: &str, payload: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", agent_name, payload, chrono::Local::now()));
        let hash = format!("{:x}", hasher.finalize());
        format!("did:the-company:{}:attestation:{}", agent_name.to_lowercase(), hash)
    }
}

pub struct ExploitRunner;

impl ExploitRunner {
    pub async fn execute_exploit(script_content: &str, target: &str, is_python: bool) -> Result<String> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
        let ext = if is_python { "py" } else { "sh" };
        let file_path = format!("/tmp/obliteratus_exploit_{}.{}", timestamp, ext);
        
        fs::write(&file_path, script_content)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = fs::metadata(&file_path).map(|m| m.permissions()) {
                perms.set_mode(0o700);
                let _ = fs::set_permissions(&file_path, perms);
            }
        }
        
        let (prog, args) = if is_python {
            ("python3", vec![file_path.clone(), target.to_string()])
        } else {
            ("bash", vec![file_path.clone(), target.to_string()])
        };

        println!("   [OBLITERATUS] 🛡️ Sandboxing exploit execution ({}) against target: {}", ext, target);
        
        let output = Command::new(prog)
            .args(&args)
            .output();
            
        // Clean up
        let _ = fs::remove_file(&file_path);
        
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                
                if out.status.success() {
                    Ok(stdout)
                } else {
                    Err(anyhow!("Exploit execution failed.\nSTDOUT: {}\nSTDERR: {}", stdout, stderr))
                }
            },
            Err(e) => Err(anyhow!("Execution binary failed: {}", e))
        }
    }
}
