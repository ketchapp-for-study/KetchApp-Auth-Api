use leptos::prelude::*;
use leptos_meta::*;

use crate::router::AppRouter;
use crate::state::GlobalState;


#[component]
pub fn App() -> impl IntoView {
    let state = GlobalState::new();

    provide_context(state.clone());

    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme=move || state.theme.get().to_string()/>
        <Title text="Welcome to Leptos CSR"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <AppRouter/>
    }
}