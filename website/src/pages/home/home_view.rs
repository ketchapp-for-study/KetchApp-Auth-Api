use crate::pages::auth::auth_view::AuthDialog;
use crate::states::{GlobalState, GlobalStateAction, Theme, ThemeSettings};
use leptos::prelude::*;


#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<GlobalState>().expect("GlobalState context not found");
    let state_for_view = state.clone();
    let (show_auth, set_show_auth) = signal(false);

    let state_for_toggle = state.clone();
    let toggle_theme = move |_| {
        state_for_toggle.cycle_theme();
        log::info!("Theme toggled to: {:?}", state_for_toggle.theme.get_untracked());
    };

    let current_theme = move || state_for_view.theme.get().to_string();

    view! {
        <h1>
            "Welcome! Username: " {move || state_for_view.account.get().name} <br />
            "Current theme: " {current_theme}
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