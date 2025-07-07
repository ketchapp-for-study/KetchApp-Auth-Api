use leptos::prelude::*;
use crate::pages::login::login_view::LoginView;
use crate::utils::show_modal::show_modal_by_id;

#[component]
pub fn AuthDialog() -> impl IntoView {
    // Effetto per chiamare showModal() dopo il mount
    create_effect(move |_| {
        show_modal_by_id("auth-dialog");
    });

    view! {
        <dialog id="auth-dialog">
            <h2>
                "You must login"
                <br />
                "to access the ${path}!"
            </h2>
            <LoginView/>
            <p class="terms">
                "By logging in, you agree to our "
                <a href="/terms">"Terms of Service"</a>
                " and "
                <a href="/privacy">"Privacy Policy"</a>
            </p>
        </dialog>
    }
}