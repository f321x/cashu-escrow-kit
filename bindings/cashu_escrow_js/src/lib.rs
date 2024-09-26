use cashu_escrow_client::ecash::ClientEcashWallet;
use cashu_escrow_common::nostr::NostrClient;
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
}
