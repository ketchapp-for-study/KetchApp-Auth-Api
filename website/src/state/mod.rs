use leptos::prelude::{RwSignal, Set};
pub(crate) use crate::state::theme::Theme;
use crate::state::user::UserProfile;

mod user;
mod theme;

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub theme: RwSignal<Theme>,
    pub account: RwSignal<UserProfile>,
    pub is_auth: RwSignal<bool>,
    pub is_loaded: RwSignal<bool>,
}

pub enum GlobalStateAction {
    SetTheme(Theme),
    SetIsAuth(bool),
    SetIsLoaded(bool),
}
impl GlobalState {
    pub fn new() -> Self {
        Self {
            theme: RwSignal::new(Theme::Light),
            account: RwSignal::new(UserProfile {
                id: 0,
                name: "Alessandro".to_string(),
            }),
            is_auth: RwSignal::new(false),
            is_loaded: RwSignal::new(false),
        }
    }

    pub fn update(&self, action: GlobalStateAction) {
        match action {
            GlobalStateAction::SetTheme(theme) => {
                self.theme.set(theme);
            }
            GlobalStateAction::SetIsAuth(is_auth) => {
                self.is_auth.set(is_auth);
            }
            GlobalStateAction::SetIsLoaded(is_loaded) => {
                self.is_loaded.set(is_loaded);
            }
        }
    }
}