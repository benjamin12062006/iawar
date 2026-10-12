#[cfg(target_arch = "wasm32")]
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use three_d::*;

#[cfg(target_arch = "wasm32")]
static PLAYING: AtomicBool = AtomicBool::new(false);
#[cfg(target_arch = "wasm32")]
static ZOOM_STEPS: AtomicI32 = AtomicI32::new(0);

#[cfg(target_arch = "wasm32")]
pub fn set_playing(playing: bool) {
    PLAYING.store(playing, Ordering::Relaxed);
}

#[cfg(target_arch = "wasm32")]
pub fn request_zoom(steps: i32) {
    ZOOM_STEPS.fetch_add(steps, Ordering::Relaxed);
}

pub fn run_scene() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        #[cfg(target_arch = "wasm32")]
        canvas: Some(crate::canvas::main_canvas()),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let orbit_target = vec3(0.0, 0.0, -0.5);

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2));
    let mut cylinder = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    cylinder
        .set_transformation(Mat4::from_translation(vec3(1.3, 0.0, 0.0)) * Mat4::from_scale(0.2));
    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 100,
                },
                ..Default::default()
            },
        ),
    );
    cube.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 1.3)) * Mat4::from_scale(0.2));
    let axes = Axes::new(&context, 0.1, 2.0);
    let bounding_box_sphere = Gm::new(
        BoundingBox::new(&context, sphere.aabb()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    let bounding_box_cube = Gm::new(
        BoundingBox::new(&context, cube.aabb()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    let bounding_box_cylinder = Gm::new(
        BoundingBox::new(&context, cylinder.aabb()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, 0.5, 0.5));

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);

        #[cfg(target_arch = "wasm32")]
        let can_control = PLAYING.load(Ordering::Relaxed);
        #[cfg(not(target_arch = "wasm32"))]
        let can_control = true;

        if can_control {
            #[cfg(target_arch = "wasm32")]
            {
                let steps = ZOOM_STEPS.swap(0, Ordering::Relaxed);
                if steps != 0 {
                    let distance = orbit_target.distance(camera.position());
                    let zoom_amount = distance * steps as f32 * 0.12;
                    camera.zoom_towards(orbit_target, zoom_amount, 1.0, 100.0);
                }
            }

            for event in frame_input.events.iter_mut() {
                match event {
                    Event::MouseMotion {
                        delta,
                        button,
                        handled,
                        ..
                    } if !*handled && Some(MouseButton::Left) == *button => {
                        let speed = 0.01;
                        camera.rotate_around_with_fixed_up(
                            orbit_target,
                            speed * delta.0,
                            speed * delta.1,
                        );
                        *handled = true;
                    }
                    Event::MouseWheel { delta, handled, .. } if !*handled => {
                        let distance = orbit_target.distance(camera.position());
                        let zoom_amount = distance * (1.0 - (-delta.1 * 0.01).exp());
                        camera.zoom_towards(orbit_target, zoom_amount, 1.0, 100.0);
                        *handled = true;
                    }
                    Event::PinchGesture { delta, handled, .. } if !*handled => {
                        let speed = orbit_target.distance(camera.position()) + 0.1;
                        camera.zoom_towards(orbit_target, speed * *delta, 1.0, 100.0);
                        *handled = true;
                    }
                    _ => {}
                }
            }
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                sphere
                    .into_iter()
                    .chain(&cylinder)
                    .chain(&cube)
                    .chain(&axes)
                    .chain(&bounding_box_sphere)
                    .chain(&bounding_box_cube)
                    .chain(&bounding_box_cylinder),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}
