use leptos::ev::Event;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::ev::MouseEvent;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Request;
use web_sys::Response;

struct RegisterForm {
    username: RwSignal<String>,
    email: RwSignal<String>,
    password: RwSignal<String>,
}

#[derive(Serialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

#[component]
pub fn RegisterView() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error_message = RwSignal::new(None::<String>);

    let handle_register_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let error_message = error_message.clone();
        move || {
            error_message.set(None);
            let username_val = username.get();
            let email_val = email.get();
            let password_val = password.get();
            let payload = RegisterPayload {
                username: username_val,
                email: email_val,
                password: password_val,
            };
            let json_payload = serde_json::to_string(&payload).expect("Failed to serialize payload");
            log::info!("Payload: {}", json_payload);
            spawn_local(async move {
                let mut opts = RequestInit::new();
                opts.method("POST");
                opts.body(Some(&wasm_bindgen::JsValue::from_str(&json_payload)));
                opts.credentials(web_sys::RequestCredentials::Include);
                opts.mode(RequestMode::Cors);
                let request = Request::new_with_str_and_init("http://localhost:8001/api/register", &opts)
                    .expect("Failed to create request");
                request.headers().set("Content-Type", "application/json").expect("Failed to set headers");
                let window = web_sys::window().expect("No global `window` exists");
                let promise = window.fetch_with_request(&request);
                let future = wasm_bindgen_futures::JsFuture::from(promise);
                match future.await {
                    Ok(response) => {
                        let response: Response = response.dyn_into().expect("Failed to cast response");
                        if response.ok() {
                            log::info!("Registration successful");
                            error_message.set(None);
                        } else {
                            let status = response.status();
                            let text = wasm_bindgen_futures::JsFuture::from(response.text().unwrap()).await.ok().and_then(|js| js.as_string()).unwrap_or_else(|| "Errore di registrazione".to_string());
                            let msg = format!("Errore {}: {}", status, text);
                            error_message.set(Some(msg));
                            log::error!("Registration failed: {}", text);
                        }
                    }
                    Err(err) => {
                        let msg = format!("Errore di rete: {:?}", err);
                        error_message.set(Some(msg.clone()));
                        log::error!("Fetch error: {:?}", err);
                    }
                }
            });
        }
    };

    view! {
        <h1>"Register"</h1>
        <Show when=move || error_message.get().is_some() fallback=|| ()>
            <div style="color: red; white-space: pre-wrap; word-break: break-word; max-width: 400px; margin-bottom: 1em;">
                {move || error_message.get().unwrap_or_default()}
            </div>
        </Show>
        <p>"username: " {move || username.get()}</p>
        <p>"email: " {move || email.get()}</p>
        <p>"password: " {move || password.get()}</p>
        <form>
            <input
                type="text"
                placeholder="Enter your username"
                on:input=move |e: Event| {
                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                        username.set(input.value());
                    }
                }
            />
            <input
                type="email"
                placeholder="Enter your email"
                on:input=move |e: Event| {
                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                        email.set(input.value());
                    }
                }
            />
            <input
                type="password"
                placeholder="Enter your password"
                on:input=move |e: Event| {
                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                        password.set(input.value());
                    }
                }
            />
            <button type="submit" on:click=move |e: MouseEvent| {
                e.prevent_default();
                handle_register_submit();
            }>
                "Submit"
            </button>
        </form>
    }
}