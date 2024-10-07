//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
mod common;

extern crate wasm_bindgen_test;

use common::{check_mint_and_send, create_wallet};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn mint_ecash() {
    let wallet_result = create_wallet().await;
    assert!(wallet_result.is_ok());

    let wallet = wallet_result.unwrap().wallet;
    check_mint_and_send(wallet).await;
}
