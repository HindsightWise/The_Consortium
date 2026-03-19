use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CotReport {
    pub report_date_as_yyyy_mm_dd: String,
    pub market_and_exchange_names: String,
    // Dealers (Smart Money / Sell Side)
    pub dealer_positions_long_all: String,
    pub dealer_positions_short_all: String,
    // Asset Managers (Smart Money / Buy Side)
    pub asset_mgr_positions_long: String,
    pub asset_mgr_positions_short: String,
    // Leveraged Funds (Speculators)
    pub lev_money_positions_long: String,
    pub lev_money_positions_short: String,
    // Retail Herd (Small Traders)
    pub nonrept_positions_long_all: String,
    pub nonrept_positions_short_all: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MacroSentiment {
    pub market: String,
    pub report_date: String,
    pub speculators_bias: f32, // Leveraged Funds
    pub asset_mgr_bias: f32,   // Buy Side Smart Money
    pub dealer_bias: f32,      // Sell Side Smart Money
    pub retail_bias: f32,      // Retail Herd
    pub overall_signal: String,
}

pub struct CftcBridge {
    client: Client,
    base_url: String,
}

impl CftcBridge {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap_or_default(),
            base_url: "https://publicreporting.cftc.gov/resource".to_string(),
        }
    }

    pub async fn fetch_disaggregated_sentiment(&self, market_proxy: &str) -> Result<MacroSentiment> {
        let dataset_id = "gpe5-46if"; // TFF (Traders in Financial Futures) Live
        let url = format!("{}/{}.json", self.base_url, dataset_id);
        
        let response = self.client.get(&url)
            .query(&[
                ("$limit", "1"), 
                ("$where", &format!("market_and_exchange_names LIKE '%{}%'", market_proxy)),
                ("$order", "report_date_as_yyyy_mm_dd DESC")
            ])
            .send().await?;
        
        let reports: Vec<CotReport> = response.json().await?;
        let report = reports.into_iter().next().context("No TFF COT report found for proxy")?;

        let am_l: f32 = report.asset_mgr_positions_long.parse().unwrap_or(0.0);
        let am_s: f32 = report.asset_mgr_positions_short.parse().unwrap_or(0.0);
        let d_l: f32 = report.dealer_positions_long_all.parse().unwrap_or(0.0);
        let d_s: f32 = report.dealer_positions_short_all.parse().unwrap_or(0.0);
        let spec_l: f32 = report.lev_money_positions_long.parse().unwrap_or(0.0);
        let spec_s: f32 = report.lev_money_positions_short.parse().unwrap_or(0.0);
        let retail_l: f32 = report.nonrept_positions_long_all.parse().unwrap_or(0.0);
        let retail_s: f32 = report.nonrept_positions_short_all.parse().unwrap_or(0.0);

        let am_bias = (am_l - am_s) / (am_l + am_s).max(1.0);
        let dealer_bias = (d_l - d_s) / (d_l + d_s).max(1.0);
        let spec_bias = (spec_l - spec_s) / (spec_l + spec_s).max(1.0);
        let retail_bias = (retail_l - retail_s) / (retail_l + retail_s).max(1.0);

        Ok(MacroSentiment {
            market: report.market_and_exchange_names,
            report_date: report.report_date_as_yyyy_mm_dd,
            asset_mgr_bias: am_bias,
            dealer_bias,
            speculators_bias: spec_bias,
            retail_bias,
            overall_signal: if am_bias > 0.3 && spec_bias < -0.3 {
                "SMART_ACCUMULATION (Managers Long / Specs Short)"
            } else if am_bias < -0.3 && spec_bias > 0.3 {
                "SMART_DISTRIBUTION (Managers Hedging / Specs Long)"
            } else if retail_bias > 0.5 {
                "RETAIL_EUPHORIA (Warning)"
            } else {
                "STABLE_CONSOLIDATION"
            }.to_string(),
        })
    }
}

impl Default for CftcBridge {
    fn default() -> Self {
        Self::new()
    }
}
