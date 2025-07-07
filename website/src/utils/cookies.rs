use wasm_bindgen::JsValue;
use web_sys::js_sys;

pub struct CookieManager;

impl CookieManager {
    pub fn set(name: &str, value: &str, max_age: usize) {
        let cookie_str = format!(
            "{}={}; path=/; max-age={}; SameSite=Lax",
            name, value, max_age
        );
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = js_sys::Reflect::set(
                document.as_ref(),
                &JsValue::from_str("cookie"),
                &JsValue::from_str(&cookie_str),
            );
        }
    }

    pub fn get(name: &str) -> Option<String> {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let cookie_val = js_sys::Reflect::get(document.as_ref(), &JsValue::from_str("cookie"));
            if let Ok(val) = cookie_val {
                let raw_cookies = val.as_string().unwrap_or_default();
                if let Some(cookie_value) = raw_cookies.split(';').find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with(&format!("{}=", name)) {
                        cookie.split('=').nth(1).map(|v| v.to_string())
                    } else {
                        None
                    }
                }) {
                    return Some(cookie_value);
                }
            }
        }
        None
    }
}
