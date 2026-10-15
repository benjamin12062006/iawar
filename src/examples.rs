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

pub fn run_active_example(
    #[cfg(target_arch = "wasm32")] status: leptos::prelude::RwSignal<crate::canvas::SceneStatus>,
) {
    match ACTIVE_EXAMPLE {
        ActiveExample::Shapes => {
            #[cfg(target_arch = "wasm32")]
            {
                use leptos::prelude::Set;
                status.set(crate::canvas::SceneStatus::Ready);
            }
            shapes::run_scene();
        }
        ActiveExample::Triangle => {
            #[cfg(target_arch = "wasm32")]
            {
                use leptos::prelude::Set;
                status.set(crate::canvas::SceneStatus::Ready);
            }
            triangle::main();
        }
        ActiveExample::Lighting => {
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                use leptos::prelude::Set;
                if let Err(error) = lighting::run(status).await {
                    status.set(crate::canvas::SceneStatus::Failed(error));
                }
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
