use leptos::task::spawn_local;
use leptos::prelude::*;
use crate::app::SequencerState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn StepInspector() -> impl IntoView {
    let state = use_context::<SequencerState>().expect("State missing");
    let selected = state.selected_step;

    // We need to read the current value from the pattern to initialize the slider correctly
    // But for this first pass, we'll default to 60 (Middle C) or just update on change.
    // Ideally, we'd read from the pattern signal like in the main Inspector.
    
    // For now, let's just make it write-only or assume a default for the prototype.
    // In a real implementation, we'd fetch the current P-Lock value.

    let on_pitch_change = move |ev| {
        let val = event_target_value(&ev).parse::<f32>().unwrap_or(60.0);
        if let Some((track_id, step_idx)) = selected.get() {
            spawn_local(async move {
                // Construct args object manually or use serde_wasm_bindgen
                let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                    "trackId": track_id,
                    "stepIdx": step_idx,
                    "paramId": 0, // PARAM_PITCH (0 is hardcoded for now)
                    "value": val
                })).unwrap();

                let _ = invoke("set_param_lock", args).await;
            });
        }
    };

    view! {
        <div class="p-4 border-t border-zinc-800 bg-zinc-900/50 mt-4 rounded-xl">
            {move || match selected.get() {
                Some((track_id, step_idx)) => view! {
                    <div class="flex flex-col gap-2 animate-in fade-in slide-in-from-top-2 duration-200">
                        <div class="flex items-center justify-between">
                            <span class="text-zinc-100 font-bold text-sm">"EDITING TRACK " {track_id + 1} ", STEP " {step_idx + 1}</span>
                            <button
                                class="text-xs text-zinc-500 hover:text-red-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
                                on:click=move |_| selected.set(None)
                            >
                                "CLOSE"
                            </button>
                        </div>
                        
                        <div class="flex flex-col gap-1">
                            <label class="text-xs font-bold text-blue-400 uppercase tracking-widest">"PITCH LOCK"</label>
                            <input type="range" min="0" max="127" step="1"
                                class="w-full h-2 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
                                on:input=on_pitch_change
                                // We should ideally bind 'value' here to the current P-Lock state
                            />
                             <div class="flex justify-between text-xs text-zinc-500 font-mono">
                                <span>"0 (C-1)"</span>
                                <span>"127 (G9)"</span>
                            </div>
                        </div>
                    </div>
                }.into_any(),
                None => view! { 
                    <div class="text-zinc-500 text-xs text-center py-4 italic">
                        "Right-click a step to edit parameters"
                    </div> 
                }.into_any()
            }}
        </div>
    }
}
