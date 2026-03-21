pub mod economy;
pub mod escrow;
pub mod arbiter;
pub mod alpaca_trader;
pub mod hft_engine;
pub mod ingestion;
pub mod alpha_shard;
pub mod sec_analyzer;
pub mod blockchain_intel;
pub mod hermes;
pub mod omniscient;
pub mod defi_scanner;
pub mod sec_edgar;
pub mod cftc_cot;
pub mod polymarket;
pub mod ecocon;
pub mod catena_mdba_bridge;
pub mod opensecrets_api;

#[derive(Debug, PartialEq, Eq)]
pub enum TradeSignal {
    Buy,
    Sell,
    Hold,
}
