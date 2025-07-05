use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::pages::home::home_view::Home;
use crate::pages::not_found::not_found_view::NotFound;
use crate::pages::profile::profile_view::Profile;
use crate::pages::register::register_view::RegisterView;
use crate::pages::login::login_view::LoginView;
use crate::pages::table::table_view::TableView;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/profile") view=Profile />
                <Route path=path!("/register") view=RegisterView />
                <Route path=path!("/login") view=LoginView />
                <Route path=path!("/table") view=TableView />
            </Routes>
        </Router>
    }
}