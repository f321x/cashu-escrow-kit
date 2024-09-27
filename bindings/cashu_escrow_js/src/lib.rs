use cashu_escrow_client::{ecash::ClientEcashWallet, escrow_client::InitEscrowClient};
use cashu_escrow_common::nostr::NostrClient;
use error::{into_err, Result};
use models::{JsTradeContract, JsTradeMode};
use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

mod error;
mod models;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = NostrClient)]
pub struct JsNostrClient {
    pub(crate) _inner: NostrClient,
}

#[wasm_bindgen(js_class = NostrClient)]
impl JsNostrClient {
    #[wasm_bindgen(constructor)]
    pub async fn new(key: &str, relays: Vec<String>) -> Result<JsNostrClient> {
        let keys = Keys::parse(key).map_err(into_err)?;
        let _inner = NostrClient::new(keys, relays).await.map_err(into_err)?;
        Ok(Self { _inner })
    }
}

#[wasm_bindgen(js_name = ClientEcashWallet)]
pub struct JsClientEcashWallet {
    pub(crate) inner: ClientEcashWallet,
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

#[wasm_bindgen(js_name = InitEscrowClient)]
pub struct JsInitEscrowClient {
    _inner: InitEscrowClient,
}

#[wasm_bindgen(js_class = InitEscrowClient)]
impl JsInitEscrowClient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        nostr_client: JsNostrClient,
        escrow_wallet: JsClientEcashWallet,
        escrow_contract: JsTradeContract,
        mode: JsTradeMode,
    ) -> Result<JsInitEscrowClient> {
        let _inner = InitEscrowClient::new(
            nostr_client._inner,
            escrow_wallet.inner,
            escrow_contract.inner,
            *mode,
        );
        Ok(Self { _inner })
    }
}
