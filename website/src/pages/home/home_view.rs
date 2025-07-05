use crate::pages::auth::auth_view::AuthDialog;
use crate::state::{GlobalState, GlobalStateAction, Theme};
use leptos::prelude::*;
use web_sys::{window, Event};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

fn detect_system_theme() -> Theme {
    let window = window().expect("No global `window` exists");
    let dark_mode_query = window
        .match_media("(prefers-color-scheme: dark)")
        .expect("Failed to query media")
        .map(|m| m.matches())
        .unwrap_or(false);

    if dark_mode_query {
        Theme::Dark
    } else {
        Theme::Light
    }
}

fn setup_system_theme_listener(state: GlobalState) {
    let window = window().expect("No global `window` exists");
    let closure = Closure::wrap(Box::new(move |_: Event| {
        let system_theme = detect_system_theme();
        state.update(GlobalStateAction::SetVisualTheme(system_theme)); // Aggiorna sempre il tema visivo
    }) as Box<dyn FnMut(_)>);

    if let Some(media_query) = window
        .match_media("(prefers-color-scheme: dark)")
        .expect("Failed to query media")
    {
        media_query
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .expect("Failed to add event listener");
    }

    closure.forget(); // Evita che il closure venga deallocato
}

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<GlobalState>().expect("GlobalState context not found");
    let state_for_view = state.clone();
    let (show_auth, set_show_auth) = signal(false);

    let state_for_toggle = state.clone();
    let toggle_theme = move |_| {
        state_for_toggle.update(GlobalStateAction::SetTheme(
            match state_for_toggle.theme.get() {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Auto,
                Theme::Auto => detect_system_theme(),
            },
        ));
    };

    setup_system_theme_listener(state);

    view! {
        <h1>
            "Welcome! Username: " {move || state_for_view.account.get().name} <br />
            "Current theme settings: " {move || state_for_view.theme.get().to_string()}
        "Current theme: " {move || state_for_view.theme.get().to_string()}
        </h1>
        
        <button on:click=toggle_theme>
            "Toggle Theme"
        </button>
        <button on:click=move |_| set_show_auth.set(!show_auth.get())>
            "Show Auth Dialog"
        </button>
        <Show
            when=move || show_auth.get()
            fallback=|| ()>

            <AuthDialog/>
        </Show>
    }
}