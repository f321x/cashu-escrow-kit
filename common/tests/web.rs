//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

mod common;

use common::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn wasm_receive_1_message() -> anyhow::Result<()> {
    test_receive_1_message().await
}

#[wasm_bindgen_test]
async fn wasm_receive_2_messages_from_cache() -> anyhow::Result<()> {
    test_receive_2_messages_from_cache().await
}
