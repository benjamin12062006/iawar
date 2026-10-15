use crate::canvas;
use leptos::{ev, leptos_dom::helpers::window_event_listener, prelude::*};

#[component]
pub fn App() -> impl IntoView {
    let (playing, set_playing_state) = signal(false);
    let (name, set_name) = signal(String::from("IAWAR"));
    let scene_status = RwSignal::new(canvas::SceneStatus::Loading("Preparando mundo…"));

    #[cfg(target_arch = "wasm32")]
    canvas::run_canvas(scene_status);

    let _esc_listener = window_event_listener(ev::keydown, {
        let set_playing_state = set_playing_state;
        move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                set_playing_state.set(false);
                canvas::set_playing(false);
            }
        }
    });

    let start = move |_| {
        if scene_status.get() != canvas::SceneStatus::Ready {
            return;
        }
        set_playing_state.set(true);
        canvas::set_playing(true);
        canvas::focus_main_canvas();
    };

    let exit = move |_| {
        set_playing_state.set(false);
        canvas::set_playing(false);
    };

    let zoom_in = move |_| {
        canvas::request_zoom(1);
    };

    let zoom_out = move |_| {
        canvas::request_zoom(-1);
    };

    view! {
        <div
            class="absolute inset-0 z-30"
            style:pointer-events=move || if playing.get() { "none" } else { "auto" }
        >
            <div
                class="absolute inset-0 z-20 flex items-center justify-center bg-slate-950/55 backdrop-blur-[1px]"
                style:display=move || if playing.get() { "none" } else { "flex" }
            >
                <section class="w-[92vw] max-w-2xl rounded-xl border border-white/20 bg-slate-900/75 text-slate-100 shadow-2xl p-6 md:p-8">
                    <p class="text-xs uppercase tracking-[0.22em] text-cyan-300/90">Panel Reactivo</p>
                    <h1 class="mt-2 text-3xl md:text-4xl font-semibold leading-tight">"Panel de Inicio"</h1>
                    <p class="mt-3 text-sm md:text-base text-slate-200/90">
                        "Canvas 3D separado del panel UI. Escribe y mira el cambio reactivo:"
                    </p>

                    <label class="mt-5 block text-sm text-slate-300">"Nombre:"</label>
                    <input
                        class="mt-2 w-full rounded-lg border border-white/20 bg-slate-950/60 px-3 py-2 text-slate-100 outline-none focus:border-cyan-400"
                        prop:value=move || name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                    />

                    <p class="mt-4 text-base text-slate-200">
                        "Vista previa: "
                        <span class="font-semibold text-cyan-300">{move || name.get()}</span>
                    </p>

                    <button
                        class="mt-7 w-full rounded-lg bg-cyan-500 px-4 py-3 text-sm md:text-base font-semibold text-slate-950 hover:bg-cyan-400 active:bg-cyan-300 transition-colors"
                        type="button"
                        on:click=start
                        disabled=move || scene_status.get() != canvas::SceneStatus::Ready
                    >
                        {move || match scene_status.get() {
                            canvas::SceneStatus::Ready => "Click para iniciar",
                            canvas::SceneStatus::Loading(_) => "Preparando escena 3D…",
                            canvas::SceneStatus::Failed(_) => "Escena no disponible",
                        }}
                    </button>
                    <p class="mt-3 text-sm text-slate-200" role="status" aria-live="polite">
                        {move || match scene_status.get() {
                            canvas::SceneStatus::Loading(message) => {
                                format!("{message} Puedes seguir usando este panel.")
                            }
                            canvas::SceneStatus::Ready => "Mundo listo.".to_string(),
                            canvas::SceneStatus::Failed(error) => {
                                format!("No se pudo preparar el mundo: {error}")
                            }
                        }}
                    </p>
                </section>
            </div>
        </div>

        <div class="absolute inset-0 z-30 pointer-events-none">
            <button
                class="pointer-events-auto absolute top-4 right-4 rounded-lg bg-slate-900/75 border border-white/20 px-4 py-2 text-sm font-medium text-slate-100 hover:bg-slate-800 transition-colors"
                type="button"
                on:click=exit
                style:display=move || if playing.get() { "block" } else { "none" }
                style:top="max(1rem, calc(env(safe-area-inset-top) + 0.5rem))"
                style:right="max(1rem, env(safe-area-inset-right))"
            >
                "Salir al panel"
            </button>

            <div
                class="pointer-events-auto absolute right-4 bottom-20 flex flex-col gap-3"
                style:display=move || if playing.get() { "flex" } else { "none" }
                style:bottom="max(5rem, calc(env(safe-area-inset-bottom) + 4.5rem))"
                style:right="max(1rem, env(safe-area-inset-right))"
            >
                <button
                    class="grid h-14 w-14 place-items-center rounded-full bg-slate-900/80 border border-white/25 text-3xl font-semibold leading-none text-slate-100 shadow-xl active:bg-cyan-500 active:text-slate-950"
                    type="button"
                    style="width:64px; height:64px; min-width:64px; min-height:64px; border-radius:9999px; font-size:34px; line-height:1; touch-action:manipulation;"
                    aria-label="Acercar zoom"
                    title="Acercar zoom"
                    on:click=zoom_in
                >
                    "+"
                </button>

                <button
                    class="grid h-14 w-14 place-items-center rounded-full bg-slate-900/80 border border-white/25 text-3xl font-semibold leading-none text-slate-100 shadow-xl active:bg-cyan-500 active:text-slate-950"
                    type="button"
                    style="width:64px; height:64px; min-width:64px; min-height:64px; border-radius:9999px; font-size:34px; line-height:1; touch-action:manipulation;"
                    aria-label="Alejar zoom"
                    title="Alejar zoom"
                    on:click=zoom_out
                >
                    "-"
                </button>
            </div>

            <p
                class="absolute bottom-4 left-1/2 -translate-x-1/2 rounded-md bg-slate-900/70 border border-white/15 px-3 py-1.5 text-xs text-slate-100"
                style:display=move || if playing.get() { "block" } else { "none" }
            >
                "ESC pars volvefegwrhdter al panel"
            </p>
        </div>
    }
}
