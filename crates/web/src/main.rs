//! Wasm frontend for glypher. Everything runs in the browser, no uploads.
#![warn(clippy::pedantic)]

mod app;
mod export;
mod source;
mod state;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
