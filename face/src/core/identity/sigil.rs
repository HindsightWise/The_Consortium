use serde::{Deserialize, Serialize};
use crate::core::alpha_shard::AlphaShard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HermeticSeal {
    pub sigil_ascii: String,
    pub ladder_status: Vec<GateState>,
    pub planetary_invocation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateState {
    pub sphere: String,
    pub symbol: String,
    pub passed: bool,
}

pub struct SigilGenerator;

impl SigilGenerator {
    pub fn generate_seal(shard: &AlphaShard) -> HermeticSeal {
        let gates = vec![
            // 1. Moon (Liquidity)
            GateState { 
                sphere: "Moon".to_string(), 
                symbol: "☽".to_string(), 
                passed: shard.financials.price > 0.0 
            },
            
            // 2. Logic (Mercury)
            GateState { 
                sphere: "Mercury".to_string(), 
                symbol: "☿".to_string(), 
                passed: shard.integrity_score > 50.0 
            },

            // 3. Venus (Desirability)
            GateState { 
                sphere: "Venus".to_string(), 
                symbol: "♀".to_string(), 
                passed: shard.financials.revenue_growth > 0.0 
            },

            // 4. Sun (Truth)
            GateState { 
                sphere: "Sun".to_string(), 
                symbol: "☉".to_string(), 
                passed: shard.physical_proof.confidence > 0.8 
            },

            // 5. Mars (Force)
            GateState { 
                sphere: "Mars".to_string(), 
                symbol: "♂".to_string(), 
                passed: shard.financials.revenue_growth > 1.0 
            },

            // 6. Jupiter (Expansion)
            GateState { 
                sphere: "Jupiter".to_string(), 
                symbol: "♃".to_string(), 
                passed: true 
            },

            // 7. Saturn (Constraint)
            GateState { 
                sphere: "Saturn".to_string(), 
                symbol: "♄".to_string(), 
                passed: shard.reality_gap < 10.0 
            },
        ];

        let sigil_ascii = Self::draw_rose_sigil(&shard.signature);
        let invocation = format!("By the {} gates, we bind the truth of {} to the substrate.", 
            gates.iter().filter(|g| g.passed).count(),
            shard.target
        );

        HermeticSeal {
            sigil_ascii,
            ladder_status: gates,
            planetary_invocation: invocation,
        }
    }

    fn draw_rose_sigil(hash: &str) -> String {
        let mut grid = vec![vec![' '; 11]; 5];
        let bytes = hex::decode(&hash[0..10]).unwrap_or_default();
        
        for (i, byte) in bytes.iter().enumerate() {
            let x = (byte % 11) as usize;
            let y = i % 5;
            grid[y][x] = if byte % 2 == 0 { '*' } else { '+' };
        }

        let mut sigil = String::from("\n  -- SIGIL --\n");
        for row in grid {
            sigil.push_str("  ");
            for &c in row.iter() {
                sigil.push(c);
            }
            sigil.push('\n');
        }
        sigil.push_str("  -----------\n");
        sigil
    }
}
