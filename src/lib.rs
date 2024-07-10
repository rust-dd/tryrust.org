#![feature(never_type)]

pub mod app;
pub mod exercises;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod instruction;
pub mod server;
pub mod terminal;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
