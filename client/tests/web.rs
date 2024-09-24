//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate cashu_escrow_client;
use cashu_escrow_client::greet;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_greet() {
    greet();
}
