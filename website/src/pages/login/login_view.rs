use crate::utils::api::api_post;
use crate::utils::notification::{notify, NotificationType};
use leptos::ev::Event;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::ev::MouseEvent;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

struct LoginForm {
    username: RwSignal<String>,
    password: RwSignal<String>,
}

#[derive(Serialize)]
pub struct LoginPayload {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[component]
pub fn LoginView() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let username_error = RwSignal::new(None::<String>);
    let password_error = RwSignal::new(None::<String>);

    let handle_login_submit = move |username: String, password: String| {
        username_error.set(None);
        password_error.set(None);
        let payload = LoginPayload {
            username: username.clone(),
            password: password.clone(),
        };
        spawn_local(async move {
            let res: Result<serde_json::Value, String> = api_post("http://localhost:8001/api/login", &payload).await;
            match res {
                Ok(_val) => {
                    notify("Login effettuato con successo!", NotificationType::Info);
                    // Qui puoi aggiungere logica custom se serve (es. redirect)
                },
                Err(err_msg) => {
                    if err_msg == "FORBIDDEN" {
                        // Il redirect è già stato gestito da api_post
                        return;
                    }
                    notify(&format!("Errore login: {err_msg}"), NotificationType::Error);
                    if err_msg.to_lowercase().contains("username") {
                        username_error.set(Some("Username errato o mancante".to_string()));
                    }
                    if err_msg.to_lowercase().contains("password") {
                        password_error.set(Some("Password errata o mancante".to_string()));
                    }
                }
            }
        });
    };

    view! {
        <p>"username: " {move || username.get()}</p>
        <p>"password: " {move || password.get()}</p>
        <form>
            <div class="form-group">
                <div class="input-group">
                    <input
                        type="text"
                        placeholder="Enter your username"
                        class=move || if username_error.get().is_some() { "input-error" } else { "input-normal" }
                        on:input=move |e: Event| {
                            if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                username.set(input.value());
                            }
                        }
                    />
                </div>
                <Show when=move || username_error.get().is_some() fallback=|| ()>
                    <span class="input-error-message">
                        {move || username_error.get().unwrap_or_default()}
                    </span>
                </Show>
                <div class="input-group">
                    <input
                        type="password"
                        placeholder="Enter your password"
                        class=move || if password_error.get().is_some() { "input-error" } else { "input-normal" }
                        on:input=move |e: Event| {
                            if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                password.set(input.value());
                            }
                        }
                    />
                </div>
                <Show when=move || password_error.get().is_some() fallback=|| ()>
                    <span class="input-error-message">
                        {move || password_error.get().unwrap_or_default()}
                    </span>
                </Show>
                <button type="submit" on:click=move |e: MouseEvent| {
                    e.prevent_default();
                    handle_login_submit(username.get(), password.get());
                }>
                    "Submit"
                </button>
            </div>
        </form>
    }
}