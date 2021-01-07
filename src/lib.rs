#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use yew::prelude::App;

mod app;
mod components;
mod types;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
