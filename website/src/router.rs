use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::pages::home::home_view::Home;
use crate::pages::not_found::not_found_view::NotFound;
use crate::pages::profile::profile_view::Profile;
use crate::pages::register::register_view::RegisterView;
use crate::pages::login::login_view::LoginView;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/profile") view=Profile />
                <Route path=path!("/register") view=RegisterView />
                <Route path=path!("/login") view=LoginView />
            </Routes>
        </Router>
    }
}