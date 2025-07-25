mod pages;
mod types;
mod api;
mod components;
mod route;
mod app;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    App::<app::App>::new().mount_to_body();
    Ok(())
}
