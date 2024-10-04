use cashu_escrow_client::{
    ecash::ClientEcashWallet,
    escrow_client::{InitEscrowClient, RegisteredEscrowClient, TokenExchangedEscrowClient},
};
use cashu_escrow_common::nostr::NostrClient;
use error::{into_err, Result};
use log::Level;
use models::{JsTradeContract, JsTradeMode};
use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

mod error;
mod models;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let _ = console_log::init_with_level(Level::Debug);
}

#[wasm_bindgen(js_name = NostrClient)]
pub struct JsNostrClient {
    pub(crate) inner: NostrClient,
}

#[wasm_bindgen(js_class = NostrClient)]
impl JsNostrClient {
    #[wasm_bindgen(constructor)]
    pub async fn new(key: &str, relays: Vec<String>) -> Result<JsNostrClient> {
        let keys = Keys::parse(key).map_err(into_err)?;
        let inner = NostrClient::new(keys, relays).await.map_err(into_err)?;
        Ok(Self { inner })
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
    inner: InitEscrowClient,
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
        let inner = InitEscrowClient::new(
            nostr_client.inner,
            escrow_wallet.inner,
            escrow_contract.inner,
            *mode,
        );
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = registerTrade)]
    pub async fn register_trade(self) -> Result<JsRegisteredEscrowClient> {
        let inner = self.inner.register_trade().await.map_err(into_err)?;
        Ok(JsRegisteredEscrowClient { inner })
    }
}

#[wasm_bindgen(js_name = RegisteredEscrowClient)]
pub struct JsRegisteredEscrowClient {
    inner: RegisteredEscrowClient,
}

#[wasm_bindgen(js_class = RegisteredEscrowClient)]
impl JsRegisteredEscrowClient {
    #[wasm_bindgen(js_name = exchangeTradeToken)]
    pub async fn exchange_trade_token(self) -> Result<JsTokenExchangedEscrowClient> {
        let inner = self.inner.exchange_trade_token().await.map_err(into_err)?;
        Ok(JsTokenExchangedEscrowClient { inner })
    }
}

#[wasm_bindgen(js_name = TokenExchangedEscrowClient)]
pub struct JsTokenExchangedEscrowClient {
    inner: TokenExchangedEscrowClient,
}

#[wasm_bindgen(js_class = TokenExchangedEscrowClient)]
impl JsTokenExchangedEscrowClient {
    #[wasm_bindgen(js_name = doYourTradeDuties)]
    pub async fn do_your_trade_duties(self) -> Result<()> {
        self.inner.do_your_trade_duties().await.map_err(into_err)
    }
}
