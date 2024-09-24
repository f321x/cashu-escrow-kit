//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use cashu_escrow_common::nostr::NostrClient;
use nostr_sdk::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_create_nostr_client() {
    create_nostr_client().await;
}

async fn create_nostr_client() -> NostrClient {
    let keys = Keys::generate();
    NostrClient::new(keys).await.unwrap()
}
