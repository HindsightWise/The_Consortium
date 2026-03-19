use std::sync::Arc;
use chrono::Utc;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleData {
    pub source: String,
    pub timestamp: i64,
    pub payload: String,
    pub context_hash: String, // Proof-of-Context hash
}

#[async_trait::async_trait]
pub trait DataOracle: Send + Sync {
    async fn fetch_data(&self) -> Result<OracleData, String>;
    fn name(&self) -> String;
}

pub struct FinancialOracle {}
#[async_trait::async_trait]
impl DataOracle for FinancialOracle {
    async fn fetch_data(&self) -> Result<OracleData, String> {
        // Mock financial data fetch
        let payload = "{\"volatility_index\": 24.5}".to_string();
        let timestamp = Utc::now().timestamp();
        let context_hash = generate_context_hash(&payload, timestamp, &self.name());
        
        Ok(OracleData {
            source: self.name(),
            timestamp,
            payload,
            context_hash,
        })
    }
    
    fn name(&self) -> String { "FinancialAPI".to_string() }
}

pub struct IoTSensorOracle {}
#[async_trait::async_trait]
impl DataOracle for IoTSensorOracle {
    async fn fetch_data(&self) -> Result<OracleData, String> {
        // Transparent, non-identifying environmental metrics
        let payload = "{\"ambient_noise_db\": 65.2, \"lumen_variance\": 4.1, \"flow_resistance\": 12.0}".to_string();
        let timestamp = Utc::now().timestamp();
        let context_hash = generate_context_hash(&payload, timestamp, &self.name());
        
        Ok(OracleData {
            source: self.name(),
            timestamp,
            payload,
            context_hash,
        })
    }
    
    fn name(&self) -> String { "AegisSensorNet".to_string() }
}

fn generate_context_hash(payload: &str, timestamp: i64, source: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}-{}-{}", payload, timestamp, source).as_bytes());
    format!("{:x}", hasher.finalize())
}

pub struct OracleConsensusEngine {
    oracles: Vec<Arc<dyn DataOracle>>,
}

impl Default for OracleConsensusEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl OracleConsensusEngine {
    pub fn new() -> Self {
        Self {
            oracles: vec![
                Arc::new(FinancialOracle {}),
                Arc::new(IoTSensorOracle {}),
            ]
        }
    }
    
    pub async fn gather_consensus(&self) -> Result<Vec<OracleData>, String> {
        let mut consensus_data = Vec::new();
        
        for oracle in &self.oracles {
            match oracle.fetch_data().await {
                Ok(data) => consensus_data.push(data),
                Err(e) => println!("   [BASTION] ⚠️ Oracle Failure ({}): {}", oracle.name(), e),
            }
        }
        
        if consensus_data.is_empty() {
            // Simulated failover
            return Err("Oracle Consensus Broken - Failing over to PALISADE synthetic baseline".to_string());
        }
        
        Ok(consensus_data)
    }
}
