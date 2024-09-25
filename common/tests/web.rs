//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use cashu_escrow_common::nostr::NostrClient;
use nostr_sdk::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_create_nostr_client() {
    let keys = Keys::generate();
    let nostr_client = NostrClient::new(keys).await;
    assert!(nostr_client.is_ok());
}
