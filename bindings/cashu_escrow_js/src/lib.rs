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
        let relays = vec![String::from("ws://localhost:4736")];
        let _inner = NostrClient::new(keys, relays).await.map_err(into_err)?;
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
        trade_nostr_identities: JsTradeNostrIdentities,
        time_limit: u64,
        ecash_identities: JsEcashIdentities,
    ) -> Result<JsTradeContract> {
        let _inner = TradeContract {
            trade_description: description.to_string(),
            trade_amount_sat: sat_amount,
            npubkey_seller: npub_from_str(&trade_nostr_identities.seller_npub)?,
            npubkey_buyer: npub_from_str(&trade_nostr_identities.buyer_npub)?,
            npubkey_coordinator: npub_from_str(&trade_nostr_identities.coordinator_npub)?,
            time_limit,
            seller_ecash_public_key: ecash_identities.seller_pubkey,
            buyer_ecash_public_key: ecash_identities.buyer_pubkey,
        };
        Ok(Self { _inner })
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

fn npub_from_str(npub: &str) -> Result<PublicKey> {
    PublicKey::parse(npub).map_err(into_err)
}
