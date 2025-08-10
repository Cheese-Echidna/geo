#[macro_use]
extern crate alloc;

mod sketch;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::sketch::run_app;

#[wasm_bindgen]
pub async fn start(width: u32, height: u32) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // hand the canvas into your app
    run_app(width, height).await;
    Ok(())
}