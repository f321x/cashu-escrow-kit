use nostr_sdk::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeContract {
    pub trade_description: String,
    pub trade_amount_sat: u64,
    pub npub_seller: String,
    pub npub_buyer: String,
    pub npub_coordinator: PublicKey,
    pub time_limit: u64,
    pub seller_ecash_public_key: String,
    pub buyer_ecash_public_key: String,
}

pub mod cli;
pub mod nostr;
