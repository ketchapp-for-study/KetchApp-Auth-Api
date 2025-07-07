use leptos::*;
use gloo_net::http::Request;
use crate::utils::notification::{notify, NotificationType};
use crate::states::{GlobalState, set_forbidden_error};
use leptos::context::use_context;
use leptos::prelude::Set;

pub async fn api_get<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Result<T, String> {
    let resp = Request::get(url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {e:?}"))?;
    handle_api_response(resp).await
}

pub async fn api_post<T: for<'de> serde::Deserialize<'de>, B: serde::Serialize>(url: &str, body: &B) -> Result<T, String> {
    let resp = Request::post(url)
        .credentials(web_sys::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(body).unwrap())
        .map_err(|e| format!("Body error: {e:?}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e:?}"))?;
    handle_api_response(resp).await
}

async fn handle_api_response<T: for<'de> serde::Deserialize<'de>>(resp: gloo_net::http::Response) -> Result<T, String> {
    if resp.status() == 403 {
        set_forbidden_error("Non hai accesso a questa pagina.".to_string());
        return Err("FORBIDDEN".to_string());
    }
    if !resp.ok() {
        let msg = resp.text().await.unwrap_or_else(|_| "Errore generico".to_string());
        notify(&msg, NotificationType::Error);
        return Err(msg);
    }
    resp.json::<T>().await.map_err(|e| format!("Errore parsing risposta: {e:?}"))
}
