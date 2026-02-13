use leptos::task::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::ui::components::grid::{Grid, ActiveStep};
use crate::ui::components::inspector::Inspector;
use crate::ui::components::toolbar::Toolbar;
use crate::shared::models::Pattern;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct LoadPatternArgs {
    path: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Global State
    let active_step = signal(ActiveStep(None));
    provide_context(active_step.0); // ReadSignal
    provide_context(active_step.1); // WriteSignal

    let pattern = signal(Pattern::default());
    provide_context(pattern.0);
    provide_context(pattern.1);

    // Auto-load on startup
    Effect::new(move |_| {
        spawn_local(async move {
            // Using a relative path for simplicity in this verification context. 
            // In production, we'd use tauri::path APIs to get AppData dir.
            let args = serde_wasm_bindgen::to_value(&LoadPatternArgs {
                path: "last_pattern.flux".to_string(),
            }).unwrap();

            if let Ok(result) = invoke("load_pattern", args).await.into_serde::<Pattern>() {
                pattern.1.set(result);
            }
        });
    });

    view! {
        <main class="min-h-screen bg-zinc-950 text-zinc-50 p-8 font-sans selection:bg-red-900 selection:text-white">
            <div class="max-w-4xl mx-auto space-y-8">
                <header class="flex items-center justify-between border-b border-zinc-800 pb-6">
                    <div>
                        <h1 class="text-2xl font-bold tracking-tighter text-zinc-100">FLUX <span class="text-red-600">ENGINE</span></h1>
                        <p class="text-xs text-zinc-500 font-mono mt-1">RUST AUDIO KERNEL // V0.1.0</p>
                    </div>
                    <div class="flex gap-4 items-center">
                         <Toolbar />
                        <div class="w-px h-8 bg-zinc-800 mx-2"></div>
                        <button class="px-4 py-2 bg-zinc-900 border border-zinc-800 rounded hover:border-zinc-700 text-xs font-bold transition-colors">
                            PLAY
                        </button>
                        <button class="px-4 py-2 bg-zinc-900 border border-zinc-800 rounded hover:border-zinc-700 text-xs font-bold transition-colors">
                            STOP
                        </button>
                    </div>
                </header>

                <section>
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-sm font-bold text-zinc-400 uppercase tracking-widest">Sequencer Grid</h2>
                        <div class="text-xs font-mono text-zinc-600">TRACK 1 // LEAD SYNTH</div>
                    </div>
                    <Grid />
                </section>

                <section>
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-sm font-bold text-zinc-400 uppercase tracking-widest">Parameter Locks</h2>
                         <div class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full"
                                class:bg-red-600=move || active_step.0.get().0.is_some()
                                class:bg-zinc-800=move || active_step.0.get().0.is_none()
                            ></span>
                            <span class="text-xs font-mono text-zinc-500">
                                {move || if let Some(step) = active_step.0.get().0 {
                                    format!("STEP {} LOCKED", step + 1)
                                } else {
                                    "TRACK DEFAULT".to_string()
                                }}
                            </span>
                        </div>
                    </div>
                    <Inspector />
                </section>
            </div>
        </main>
    }
}

