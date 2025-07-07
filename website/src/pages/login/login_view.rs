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

    let handle_login_submit = move |username: String, password: String| {
        let payload = LoginPayload {
            username,
            password,
        };

        let json_payload = serde_json::to_string(&payload).expect("Failed to serialize payload");
        log::info!("Payload: {}", json_payload);

        spawn_local(async move {
            let mut opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_body(&wasm_bindgen::JsValue::from_str(&json_payload));
            opts.set_credentials(web_sys::RequestCredentials::Include);
            opts.set_mode(RequestMode::Cors);

            let request = Request::new_with_str_and_init("http://localhost:8001/api/login", &opts)
                .expect("Failed to create request");
            request.headers().set("Content-Type", "application/json").expect("Failed to set headers");

            let window = web_sys::window().expect("No global `window` exists");
            let promise = window.fetch_with_request(&request);
            let future = wasm_bindgen_futures::JsFuture::from(promise);

            match future.await {
                Ok(response) => {
                    let response: Response = response.dyn_into().expect("Failed to cast response");
                    if response.ok() {
                        log::info!("Login successful");
                        // Qui puoi aggiungere logica custom se serve
                    } else {
                        log::error!("Login failed");
                    }
                }
                Err(err) => {
                    log::error!("Fetch error: {:?}", err);
                }
            }
        });
    };

    view! {
        <p>"username: " {move || username.get()}</p>
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
                handle_login_submit(username.get(), password.get());
            }>
                "Submit"
            </button>
        </form>
    }
}