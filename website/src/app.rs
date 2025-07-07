use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Event};
use gloo_net::http::{Request};
use leptos::task::spawn_local;
use leptos_router::components::Router;
use leptos_router::hooks::{use_location, use_navigate};
use crate::router::AppRouter;
use crate::states::theme::{detect_system_theme, ThemeSettings};
use crate::states::{GlobalState, GlobalStateAction};
use crate::pages::auth::auth_view::AuthDialog;
use crate::utils::notification::NotificationView;
use crate::utils::show_modal::show_modal_by_id;
use log::info;
use crate::states::FORBIDDEN_ERROR;

#[component]
pub fn ForbiddenModal() -> impl IntoView {
    let error = FORBIDDEN_ERROR.clone();
    let modal_id = "forbidden-modal";

    create_effect(move |_| {
        let val = error.get();
        info!("[ForbiddenModal] error.get() = {:?}", val);
        if val.is_some() {
            info!("[ForbiddenModal] Chiamo show_modal_by_id");
            show_modal_by_id(modal_id);
        }
    });

    let navigate = use_navigate();
    view! {
        <Show when=move || error.get().is_some() fallback=|| ()>
            <dialog id=modal_id style="z-index:10000;" open>
                <h2 style="color:#c00;">Errore</h2>
                <p style="margin-bottom:2rem;">{move || {
                    let val = error.get();
                    info!("[ForbiddenModal] Render p: error.get() = {:?}", val);
                    val.unwrap_or("Non hai i permessi per questa azione.".to_string())
                }}</p>
                <button style="padding:0.5rem 1.5rem;font-size:1.1rem;" on:click=move |_| {
                    info!("[ForbiddenModal] Click Go Home");
                    error.set(None);
                    (navigate)("/", leptos_router::NavigateOptions::default());
                }>
                    Go Home
                </button>
            </dialog>
        </Show>
    }
}

#[component]
pub fn MainLayout() -> impl IntoView {
    let state = expect_context::<crate::states::GlobalState>();
    let is_auth = state.is_auth;
    let location = use_location();
    view! {
        <ForbiddenModal />
        <NotificationView />
        <Show when=move || {
            let path = location.pathname.get();
            !is_auth.get() && path != "/login" && path != "/register"
        } fallback=|| ()>
            <AuthDialog />
        </Show>
        <AppRouter/>
    }
}

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
            state_ref.update(GlobalStateAction::SetTheme(new_theme));
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

    // --- Autenticazione ---
    // Chiamata API /api/me subito al mount per verificare autenticazione
    let is_auth = state.is_auth;
    spawn_local(async move {
        match Request::get("http://localhost:8000/api/users/@me").credentials(web_sys::RequestCredentials::Include).send().await {
            Ok(resp) if resp.ok() => is_auth.set(true),
            _ => is_auth.set(false),
        }
    });

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme=move || state.clone().theme.get().to_string()/>
        <Title text="Welcome to Leptos CSR"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router>
            <MainLayout />
        </Router>
    }
}