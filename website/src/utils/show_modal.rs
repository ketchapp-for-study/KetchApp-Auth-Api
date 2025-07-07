use wasm_bindgen::JsCast;

pub fn show_modal_by_id(id: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(dialog) = document.get_element_by_id(id) {
                if let Some(dialog) = dialog.dyn_ref::<web_sys::HtmlDialogElement>() {
                    let _ = dialog.show_modal();
                }
            }
        }
    }
}

