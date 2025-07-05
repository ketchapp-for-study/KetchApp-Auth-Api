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

fn handle_register_submit(username: String, email: String, password: String) {
    let payload = RegisterPayload {
        username,
        email,
        password,
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
                } else {
                    log::error!("Registration failed");
                }
            }
            Err(err) => {
                log::error!("Fetch error: {:?}", err);
            }
        }
    });
}

#[component]
pub fn RegisterView() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    view! {
            <h1>"Register"</h1>
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
                handle_register_submit(username.get(), email.get(), password.get());
            }>
                "Submit"
            </button>
        </form>
    }
}