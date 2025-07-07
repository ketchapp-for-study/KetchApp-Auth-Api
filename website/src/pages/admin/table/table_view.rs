use crate::utils::api::api_get;
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_struct_table::*;
use serde::Deserialize;

use crate::utils::notification::{notify, NotificationType, NotificationView};

#[derive(TableRow, Clone, Deserialize, Debug)]
#[table(impl_vec_data_provider)]
pub struct UsersTable {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[component]
pub fn TableView() -> impl IntoView {
    let users = RwSignal::new(Vec::<UsersTable>::new());
    let forbidden = crate::states::FORBIDDEN_ERROR.clone();

    // Blocca la tabella se forbidden è attivo
    view! {
        <Show when=move || forbidden.get().is_none() fallback=move || view! { () }>
            <button style="position: fixed; top: 5rem; right: 2rem; z-index: 10000;" on:click=move |_| notify("Notifica di esempio!", NotificationType::Error)>
                Mostra Notifica
            </button>
            <table>
                <Show when=move || !users.get().is_empty() fallback=move || view! { <tr><td>Loading...</td></tr> }>
                    <TableContent rows=users.get() scroll_container="html" />
                </Show>
            </table>
        </Show>
    }
}