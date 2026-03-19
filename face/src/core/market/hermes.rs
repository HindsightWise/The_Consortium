use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeoAsset {
    pub symbol: String,
    pub company_name: String,
    pub shares: f64,
    pub cost_basis: f64,
}

pub struct HermesLedger;

impl HermesLedger {
    pub fn load_ceo_portfolio() -> Vec<CeoAsset> {
        vec![
            CeoAsset { symbol: "AMBA".to_string(), company_name: "Ambarella Inc".to_string(), shares: 25.0, cost_basis: 80.75 },
            CeoAsset { symbol: "ATHR".to_string(), company_name: "Aether Holdings inc.".to_string(), shares: 900.0, cost_basis: 6.99 },
            CeoAsset { symbol: "BTBT".to_string(), company_name: "Bit Digital".to_string(), shares: 255.0, cost_basis: 2.54 },
            CeoAsset { symbol: "CANOF".to_string(), company_name: "California Nanotechnologies inc".to_string(), shares: 40000.0, cost_basis: 0.29 },
            CeoAsset { symbol: "FNV".to_string(), company_name: "Franco Nevada Corp.".to_string(), shares: 20.0, cost_basis: 189.96 },
            CeoAsset { symbol: "GOOG".to_string(), company_name: "Alphabet inc".to_string(), shares: 10.0, cost_basis: 303.95 },
            CeoAsset { symbol: "HE".to_string(), company_name: "Hawaiian Electric inc".to_string(), shares: 450.0, cost_basis: 11.02 },
            CeoAsset { symbol: "ITRI".to_string(), company_name: "Itron Inc".to_string(), shares: 150.0, cost_basis: 104.96 },
            CeoAsset { symbol: "MSTR".to_string(), company_name: "Strategy".to_string(), shares: 87.0, cost_basis: 369.58 },
            CeoAsset { symbol: "NVDA".to_string(), company_name: "Nvidia".to_string(), shares: 27.0, cost_basis: 182.57 },
            CeoAsset { symbol: "O".to_string(), company_name: "Realty Income".to_string(), shares: 160.0, cost_basis: 60.60 },
            CeoAsset { symbol: "STX".to_string(), company_name: "Seagate Technology".to_string(), shares: 10.0, cost_basis: 398.04 },
            CeoAsset { symbol: "TGT".to_string(), company_name: "Target Corp".to_string(), shares: 100.0, cost_basis: 102.65 },
            CeoAsset { symbol: "TMQ".to_string(), company_name: "Trilogy Metals".to_string(), shares: 854.0, cost_basis: 4.98 },
            CeoAsset { symbol: "TSLA".to_string(), company_name: "Tesla".to_string(), shares: 135.0, cost_basis: 270.34 },
            CeoAsset { symbol: "TSM".to_string(), company_name: "Taiwan Semiconductor".to_string(), shares: 14.0, cost_basis: 234.69 },
            CeoAsset { symbol: "BSOL".to_string(), company_name: "Bitwise Solana".to_string(), shares: 600.0, cost_basis: 20.59 },
            CeoAsset { symbol: "MSTY".to_string(), company_name: "Yieldmax MSTR Option Income Strategy".to_string(), shares: 250.0, cost_basis: 77.35 },
            CeoAsset { symbol: "VNQ".to_string(), company_name: "Vanguard RealEstate Index".to_string(), shares: 100.0, cost_basis: 90.32 },
        ]
    }
}
