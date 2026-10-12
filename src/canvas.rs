pub fn run_canvas() {
    crate::examples::run_active_example();
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
