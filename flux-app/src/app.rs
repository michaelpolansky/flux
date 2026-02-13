use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos::ev;
use leptos::ev::KeyboardEvent;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

use crate::ui::components::grid::Grid;
use crate::ui::components::inspector::Inspector;
use crate::ui::components::toolbar::Toolbar;
use crate::ui::components::step_inspector::StepInspector;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct AudioSnapshot {
    current_step: usize,
    is_playing: bool,
}

// Create a context for the step
#[derive(Clone)]
pub struct SequencerState {
    pub current_step: ReadSignal<usize>,
    pub selected_step: RwSignal<Option<(usize, usize)>>, // (track_id, step_idx)
}

#[component]
pub fn App() -> impl IntoView {
    let (current_step, set_current_step) = signal(0);
    let selected_step = RwSignal::new(None);
    let (show_lfo, set_show_lfo) = signal(false); // LFO collapsed by default

    // Create Pattern signal
    let (pattern_signal, set_pattern_signal) = signal(crate::shared::models::Pattern::default());

    // Provide state to all children
    provide_context(SequencerState { current_step, selected_step });
    provide_context(pattern_signal);
    provide_context(set_pattern_signal);
    provide_context(show_lfo);
    provide_context(set_show_lfo);

    // ESC key handler to deselect step
    let handle_escape = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" {
            selected_step.set(None);
        }
    };

    // Attach to window
    window_event_listener(ev::keydown, handle_escape);

    // Setup Tauri Event Listener
    Effect::new(move |_| {
        spawn_local(async move {
            use crate::ui::tauri::listen_event;
            // "playback-status" matches the backend event name
            listen_event("playback-status", move |event: AudioSnapshot| {
                // Update the signal inside the callback
                set_current_step.set(event.current_step);
            }).await;
        });
    });

    view! {
        <main
            class="min-h-screen bg-zinc-950 text-zinc-50 p-6 font-sans selection:bg-red-900 selection:text-white"
            on:click=move |ev| {
                // Get the clicked element
                let target = ev.target();
                if let Some(element) = target.and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                    // Check if click is outside the grid
                    // If the clicked element or its ancestors don't have class "grid"
                    let mut current: Option<web_sys::Element> = Some(element.into());
                    let mut found_grid = false;

                    while let Some(el) = current {
                        let class_list = el.class_list();
                        if class_list.contains("grid") && class_list.contains("grid-cols-8") {
                            found_grid = true;
                            break;
                        }
                        current = el.parent_element();
                    }

                    if !found_grid {
                        selected_step.set(None);
                    }
                }
            }
        >
            <div class="max-w-7xl mx-auto space-y-5">
                <header class="flex items-center justify-between bg-zinc-900 border-b border-zinc-800 px-6 h-16">
                    <div class="flex flex-col">
                        <h1 class="text-xl font-bold tracking-tight text-zinc-50">FLUX</h1>
                        <p class="text-xs text-zinc-500 font-mono">"Audio Engine"</p>
                    </div>
                    <div class="flex items-center gap-4">
                        <Toolbar />
                    </div>
                </header>

                <section class="bg-zinc-900/50 rounded-lg p-4">
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Sequencer Grid"</h2>
                        <div class="text-xs font-mono text-zinc-600">"TRACK 1 - LEAD SYNTH"</div>
                    </div>
                    <Grid />
                </section>

                <section class="bg-zinc-900/50 rounded-lg p-4">
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-3">
                            <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Parameters"</h2>
                            <button
                                on:click=move |_| set_show_lfo.update(|v| *v = !*v)
                                class="text-xs bg-zinc-800 px-3 py-1 rounded hover:bg-zinc-700 cursor-pointer transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
                            >
                                {move || if show_lfo.get() { "LFO ▲" } else { "LFO ▼" }}
                            </button>
                        </div>
                         <div class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full"
                                class:bg-blue-500=move || selected_step.get().is_some()
                                class:bg-zinc-800=move || selected_step.get().is_none()
                            ></span>
                            <span class="text-xs font-mono text-zinc-500">
                                {move || if let Some(step) = selected_step.get() {
                                    format!("STEP {} LOCKED", step + 1)
                                } else {
                                    "TRACK DEFAULT".to_string()
                                }}
                            </span>
                        </div>
                    </div>
                    <Inspector />
                    <StepInspector />
                </section>
            </div>
        </main>
    }
}

