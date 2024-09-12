use cdk::nuts::PublicKey as CDKPubkey;
use nostr_sdk::{PublicKey as NostrPubkey, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TradeContract {
    pub trade_description: String,
    pub trade_amount_sat: u64,
    pub npubkey_seller: NostrPubkey,
    pub npubkey_buyer: NostrPubkey,
    pub npubkey_coordinator: NostrPubkey,
    pub time_limit: u64,
    pub seller_ecash_public_key: String,
    pub buyer_ecash_public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EscrowRegistration {
    pub escrow_id_hex: String,
    #[serde(with = "crate::cdk_pubkey_serde")]
    pub coordinator_escrow_pubkey: CDKPubkey,
    pub escrow_start_time: Timestamp,
}

impl EscrowRegistration {
    pub fn new(
        trade_id_hex: String,
        coordinator_escrow_pubkey: CDKPubkey,
        escrow_start_time: Timestamp,
    ) -> Self {
        Self {
            escrow_id_hex: trade_id_hex,
            coordinator_escrow_pubkey,
            escrow_start_time,
        }
    }
}
