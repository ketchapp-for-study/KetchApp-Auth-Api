use leptos::prelude::*;
use leptos::ev::{Event, MouseEvent};
use wasm_bindgen_futures::spawn_local;
use web_sys::{RequestInit, RequestMode, Request, Response};
use wasm_bindgen::JsCast;
use crate::pages::login::login_view::LoginPayload;
use crate::utils::show_modal::show_modal_by_id;

#[component]
pub fn AuthDialog() -> impl IntoView {
    let state = use_context::<crate::states::GlobalState>().expect("GlobalState context not found");
    let show_auth = RwSignal::new(true);

    // Effetto per chiamare showModal() dopo il mount
    Effect::new(move |_| {
        if show_auth.get() {
            show_modal_by_id("auth-dialog");
        }
    });

    let on_login_success = {
        let show_auth = show_auth.clone();
        let state = state.clone();
        move || {
            show_auth.set(false);
            state.is_auth.set(true);
        }
    };

    view! {
        <Show when=move || show_auth.get() fallback=|| ()>
            <dialog id="auth-dialog">
                <h2>
                    "You must login"
                    <br />
                    "to access the page!"
                </h2>
                <LoginViewWrapper on_success=on_login_success/>
                <p class="terms">
                    "By logging in, you agree to our "
                    <a href="/terms">"Terms of Service"</a>
                    " and "
                    <a href="/privacy">"Privacy Policy"</a>
                </p>
            </dialog>
        </Show>
    }
}

#[component]
fn LoginViewWrapper(on_success: impl Fn() + 'static + Clone) -> impl IntoView {
    let on_success = on_success.clone();
    view! {
        <LoginViewWithCallback on_success=on_success/>
    }
}

#[component]
fn LoginViewWithCallback(on_success: impl Fn() + 'static + Clone) -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let on_success = on_success.clone();

    let handle_login_submit = move |username: String, password: String| {
        let on_success = on_success.clone();
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
                        on_success();
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