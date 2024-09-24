#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wasm_bindgen::prelude::*;

pub mod ecash;
pub mod escrow_client;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet() {
    info!("Hello, wasm-game-of-life!");
}
