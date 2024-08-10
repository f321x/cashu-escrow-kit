use nostr_sdk::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeContract {
    pub trade_description: String,
    pub trade_amount_sat: u64,
    pub npubkey_seller: PublicKey,
    pub npubkey_buyer: PublicKey,
    pub npubkey_coordinator: PublicKey,
    pub time_limit: u64,
    pub seller_ecash_public_key: String,
    pub buyer_ecash_public_key: String,
}

pub mod cli;
pub mod nostr;
