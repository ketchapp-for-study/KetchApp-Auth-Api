use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Event};

use crate::router::AppRouter;
use crate::states::theme::{detect_system_theme, get_theme_cookie, ThemeSettings};
use crate::states::{GlobalState, GlobalStateAction};

#[component]
pub fn App() -> impl IntoView {
    let state = GlobalState::new();
    // Usa la logica centralizzata di ThemeSettings
    let theme_settings = ThemeSettings::from_cookie_or_default();
    state.update(GlobalStateAction::SetTheme(theme_settings));

    provide_context(state.clone());
    provide_meta_context();

    if let ThemeSettings::Auto(_) = state.theme.get_untracked() {
        let new_theme = detect_system_theme();
        state.update(GlobalStateAction::SetTheme(new_theme));
    }

    let state_ref = state.clone();
    let closure = Closure::wrap(Box::new(move |_: Event| {
        if let ThemeSettings::Auto(_) = state_ref.theme.get_untracked() {
            let new_theme = detect_system_theme();
            log::info!("Detected system theme: {:?}", new_theme);
            state_ref.update(GlobalStateAction::SetTheme(new_theme));
        } else {
            log::info!("Manual theme setting, not updating.");
        }
    }) as Box<dyn FnMut(_)>);

    if let Some(media_query) = window()
        .expect("No global `window` exists")
        .match_media("(prefers-color-scheme: dark)")
        .expect("Failed to query media")
    {
        media_query
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .expect("Failed to add event listener");
    }

    closure.forget();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme=move || state.clone().theme.get().to_string()/>
        <Title text="Welcome to Leptos CSR"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <AppRouter/>
    }
}