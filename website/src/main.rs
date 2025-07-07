mod app;
mod pages;
mod router;
mod services;
mod states;
mod utils;
mod middleware;

use app::App;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Info); // Cambiato il livello di log a Info
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}