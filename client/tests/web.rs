//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use cashu_escrow_client::ecash::ClientEcashWallet;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn create_wallet() {
    let wallet = ClientEcashWallet::new("https://nomatter.what").await;
    assert!(wallet.is_ok());
}
