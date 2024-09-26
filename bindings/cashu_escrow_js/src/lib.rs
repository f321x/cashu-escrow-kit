use cashu_escrow_client::ecash::ClientEcashWallet;
use cashu_escrow_common::{model::TradeContract, nostr::NostrClient};
use error::{into_err, Result};
use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

mod error;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = NostrClient)]
pub struct JsNostrClient {
    _inner: NostrClient,
}

#[wasm_bindgen(js_class = NostrClient)]
impl JsNostrClient {
    #[wasm_bindgen(constructor)]
    pub async fn new(key: &str) -> Result<JsNostrClient> {
        let keys = Keys::parse(key).map_err(into_err)?;
        let _inner = NostrClient::new(keys).await.map_err(into_err)?;
        Ok(Self { _inner })
    }
}

#[wasm_bindgen(js_name = ClientEcashWallet)]
pub struct JsClientEcashWallet {
    inner: ClientEcashWallet,
}

#[wasm_bindgen(js_class = ClientEcashWallet)]
impl JsClientEcashWallet {
    #[wasm_bindgen(constructor)]
    pub async fn new(url: &str) -> Result<JsClientEcashWallet> {
        let inner = ClientEcashWallet::new(url).await.map_err(into_err)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = mintQuote)]
    pub async fn mint_quote(&self, amount: u64) -> Result<String> {
        let quote = self
            .inner
            .wallet
            .mint_quote(amount.into())
            .await
            .map_err(into_err)?;
        Ok(quote.id)
    }

    #[wasm_bindgen(getter, js_name = tradePublicKey)]
    pub fn trade_pubkey(&self) -> String {
        self.inner.trade_pubkey.clone()
    }
}

#[wasm_bindgen(js_name = TradeContract)]
pub struct JsTradeContract {
    _inner: TradeContract,
}

#[wasm_bindgen(js_class = TradeContract)]
impl JsTradeContract {
    #[wasm_bindgen(constructor)]
    pub fn new(
        description: &str,
        sat_amount: u64,
        seller_npub: &str,
        buyer_npub: &str,
        coordinator_npub: &str,
        time_limit: u64,
        seller_ecash_pubkey: &str,
        buyer_ecash_pubkey: &str,
    ) -> Result<JsTradeContract> {
        let _inner = TradeContract {
            trade_description: description.to_string(),
            trade_amount_sat: sat_amount,
            npubkey_seller: npub_from_str(seller_npub)?,
            npubkey_buyer: npub_from_str(buyer_npub)?,
            npubkey_coordinator: npub_from_str(coordinator_npub)?,
            time_limit: time_limit,
            seller_ecash_public_key: seller_ecash_pubkey.to_string(),
            buyer_ecash_public_key: buyer_ecash_pubkey.to_string(),
        };
        Ok(Self { _inner })
    }
}

fn npub_from_str(npub: &str) -> Result<PublicKey> {
    PublicKey::parse(npub).map_err(into_err)
}
