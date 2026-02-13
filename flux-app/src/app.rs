use leptos::task::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub selected_step: RwSignal<Option<usize>>,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_step, set_current_step) = signal(0);
    let selected_step = RwSignal::new(None);

    // Create Pattern signal
    let (pattern_signal, set_pattern_signal) = signal(crate::shared::models::Pattern::default());

    // Provide state to all children
    provide_context(SequencerState { current_step, selected_step });
    provide_context(pattern_signal);
    provide_context(set_pattern_signal);

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
        <main class="min-h-screen bg-zinc-950 text-zinc-50 p-6 font-sans selection:bg-red-900 selection:text-white">
            <div class="max-w-7xl mx-auto space-y-6">
                <header class="flex items-center justify-between bg-zinc-900 border-b border-zinc-800 px-6 h-16">
                    <div class="flex flex-col">
                        <h1 class="text-xl font-bold tracking-tight text-zinc-50">FLUX</h1>
                        <p class="text-xs text-zinc-500 font-mono">"Audio Engine"</p>
                    </div>
                    <div class="flex items-center gap-4">
                        <Toolbar />
                    </div>
                </header>

                <section class="bg-zinc-900/50 rounded-lg p-6">
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Sequencer Grid"</h2>
                        <div class="text-xs font-mono text-zinc-600">"TRACK 1 - LEAD SYNTH"</div>
                    </div>
                    <Grid />
                </section>

                <section class="bg-zinc-900/50 rounded-lg p-6">
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Parameters"</h2>
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

