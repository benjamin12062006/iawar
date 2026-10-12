mod app;
pub(crate) mod canvas;
mod examples;

#[cfg(target_arch = "wasm32")]
use leptos::mount::mount_to;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsCast};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let app_root = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("app"))
        .ok_or_else(|| JsValue::from_str("missing #app mount point"))?
        .dyn_into::<web_sys::HtmlElement>()?;
    mount_to(app_root, || view! { <app::App /> }).forget();

    if let Some(window) = web_sys::window() {
        let cb = Closure::once(|| {
            canvas::run_canvas();
        });
        let _ = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 0);
        cb.forget();
    }
    Ok(())
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        canvas::run_canvas();
    }
}
