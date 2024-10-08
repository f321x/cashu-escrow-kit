use std::ops::Deref;

use crate::error::{into_err, Result};
use cashu_escrow_client::escrow_client::TradeMode;
use cashu_escrow_common::model::TradeContract;
use nostr_sdk::PublicKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = TradeContract)]
pub struct JsTradeContract {
    pub(crate) inner: TradeContract,
}

#[wasm_bindgen(js_class = TradeContract)]
impl JsTradeContract {
    #[wasm_bindgen(constructor)]
    pub fn new(
        description: &str,
        sat_amount: u64,
        trade_nostr_identities: JsTradeNostrIdentities,
        time_limit: u64,
        ecash_identities: JsEcashIdentities,
    ) -> Result<JsTradeContract> {
        let inner = TradeContract {
            trade_description: description.to_string(),
            trade_amount_sat: sat_amount,
            npubkey_seller: npub_from_str(&trade_nostr_identities.seller_npub)?,
            npubkey_buyer: npub_from_str(&trade_nostr_identities.buyer_npub)?,
            npubkey_coordinator: npub_from_str(&trade_nostr_identities.coordinator_npub)?,
            time_limit,
            seller_ecash_public_key: ecash_identities.seller_pubkey,
            buyer_ecash_public_key: ecash_identities.buyer_pubkey,
        };
        Ok(Self { inner })
    }
}

#[wasm_bindgen(js_name = TradeNostrIdentities)]
pub struct JsTradeNostrIdentities {
    seller_npub: String,
    buyer_npub: String,
    coordinator_npub: String,
}

#[wasm_bindgen(js_class = TradeNostrIdentities)]
impl JsTradeNostrIdentities {
    #[wasm_bindgen(constructor)]
    pub fn new(
        seller_npub: String,
        buyer_npub: String,
        coordinator_npub: String,
    ) -> JsTradeNostrIdentities {
        Self {
            seller_npub,
            buyer_npub,
            coordinator_npub,
        }
    }
}

#[wasm_bindgen(js_name = EcashIdentities)]
pub struct JsEcashIdentities {
    seller_pubkey: String,
    buyer_pubkey: String,
}

#[wasm_bindgen(js_class = EcashIdentities)]
impl JsEcashIdentities {
    #[wasm_bindgen(constructor)]
    pub fn new(seller_pubkey: String, buyer_pubkey: String) -> JsEcashIdentities {
        Self {
            seller_pubkey,
            buyer_pubkey,
        }
    }
}

#[wasm_bindgen(js_name = TradeMode)]
pub enum JsTradeMode {
    Buyer,
    Seller,
}

impl Deref for JsTradeMode {
    type Target = TradeMode;

    fn deref(&self) -> &Self::Target {
        match self {
            JsTradeMode::Buyer => &TradeMode::Buyer,
            JsTradeMode::Seller => &TradeMode::Seller,
        }
    }
}

fn npub_from_str(npub: &str) -> Result<PublicKey> {
    PublicKey::parse(npub).map_err(into_err)
}
