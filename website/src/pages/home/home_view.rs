use crate::pages::auth::auth_view::AuthDialog;
use crate::state::{GlobalState, GlobalStateAction, Theme};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<GlobalState>().expect("GlobalState context not found");
    let state_for_view = state.clone();
    let (show_auth, set_show_auth) = signal(false);

    let toggle_theme = move |_| {
        state.update(GlobalStateAction::SetTheme(
            if state.theme.get() == Theme::Light {
                Theme::Dark
            } else {
                Theme::Light
            },
        ));
    };

    view! {
        <h1>
            "Welcome! Username: " {move || state_for_view.account.get().name} <br />
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