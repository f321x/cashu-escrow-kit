use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeContract {
    pub trade_beginning_ts: u64,
    pub trade_description: String,
    pub trade_mint_url: String,
    pub trade_amount_sat: u64,
    pub npub_seller: String,
    pub npub_buyer: String,
    pub time_limit: u64,
    pub seller_ecash_public_key: String,
    pub buyer_ecash_public_key: String,
}

pub mod cli;
pub mod nostr;