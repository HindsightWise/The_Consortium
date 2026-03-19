use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotarySeal {
    pub seal_id: String,
    pub document_hash: String,
    pub jurisdiction: String,
    pub timestamp: String,
    pub signature_company: String,
    pub status: SealStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SealStatus {
    Draft,
    Verified,
    Apostilled,
}

pub struct NotaryBridge;

impl NotaryBridge {
    /// Generates a Digital Notary Seal for a document
    pub fn generate_seal(document_content: &str, jurisdiction: &str) -> NotarySeal {
        let mut hasher = Sha256::new();
        hasher.update(document_content.as_bytes());
        let doc_hash = hex::encode(hasher.finalize());
        
        let seal_id = format!("RON_{}", hex::encode(&doc_hash[0..8]));
        
        NotarySeal {
            seal_id,
            document_hash: doc_hash,
            jurisdiction: jurisdiction.to_string(),
            timestamp: Local::now().to_rfc3339(),
            signature_company: "SOVEREIGN_NOTARY_V1".to_string(),
            status: SealStatus::Verified,
        }
    }

    /// Verifies if a seal is valid and untampered
    pub fn verify_seal(seal: &NotarySeal, original_content: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(original_content.as_bytes());
        let current_hash = hex::encode(hasher.finalize());
        
        seal.document_hash == current_hash
    }
}
