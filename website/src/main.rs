mod app;
mod pages;
mod router;
mod services;
mod states;
mod utils;

use app::App;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}