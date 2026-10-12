#[path = "../examples/shapes/src/main.rs"]
mod shapes;
#[path = "../examples/triangle/src/main.rs"]
mod triangle;
#[path = "../examples/lighting/src/main.rs"]
mod lighting;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ActiveExample {
    Shapes,
    Triangle,
    Lighting,
}

pub const ACTIVE_EXAMPLE: ActiveExample = ActiveExample::Lighting;

pub fn run_active_example() {
    match ACTIVE_EXAMPLE {
        ActiveExample::Shapes => shapes::run_scene(),
        ActiveExample::Triangle => triangle::main(),
        ActiveExample::Lighting => {
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async {
                lighting::run().await;
            });
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_playing(playing: bool) {
    match ACTIVE_EXAMPLE {
        ActiveExample::Shapes => shapes::set_playing(playing),
        ActiveExample::Triangle => {},
        ActiveExample::Lighting => {}
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_playing(_playing: bool) {}

#[cfg(target_arch = "wasm32")]
pub fn request_zoom(steps: i32) {
    match ACTIVE_EXAMPLE {
        ActiveExample::Shapes => shapes::request_zoom(steps),
        ActiveExample::Triangle => {}
        ActiveExample::Lighting => {}
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn request_zoom(_steps: i32) {}
