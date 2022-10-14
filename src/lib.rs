#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;

mod app;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<app::App>();
	Ok(())
}
