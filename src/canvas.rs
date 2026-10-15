#[derive(Clone, Debug, PartialEq)]
pub enum SceneStatus {
    Loading(&'static str),
    Ready,
    Failed(String),
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_canvas() {
    crate::examples::run_active_example();
}

#[cfg(target_arch = "wasm32")]
pub fn run_canvas(status: leptos::prelude::RwSignal<SceneStatus>) {
    crate::examples::run_active_example(status);
}

/// Yield a browser task between loading stages, not just a Promise microtask.
/// The synchronous work inside a stage still executes on the main thread.
#[cfg(target_arch = "wasm32")]
pub async fn loading_step(
    status: leptos::prelude::RwSignal<SceneStatus>,
    message: &'static str,
) -> Result<(), String> {
    use leptos::prelude::Set;
    use wasm_bindgen::JsCast;

    status.set(SceneStatus::Loading(message));
    let window = web_sys::window().ok_or("No se encontró la ventana del navegador")?;
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        if let Err(error) = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(resolve.unchecked_ref(), 16)
        {
            let _ = reject.call1(&wasm_bindgen::JsValue::NULL, &error);
        }
    });
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map(|_| ())
        .map_err(|error| format!("No se pudo continuar la carga: {error:?}"))
}

pub fn set_playing(playing: bool) {
    crate::examples::set_playing(playing);
}

pub fn request_zoom(steps: i32) {
    crate::examples::request_zoom(steps);
}

#[cfg(target_arch = "wasm32")]
pub fn main_canvas() -> web_sys::HtmlCanvasElement {
    use wasm_bindgen::JsCast;

    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("main-canvas"))
        .expect("missing #main-canvas in index.html")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("#main-canvas is not an HtmlCanvasElement")
}

#[cfg(target_arch = "wasm32")]
pub fn focus_main_canvas() {
    let _ = main_canvas().focus();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn focus_main_canvas() {}
