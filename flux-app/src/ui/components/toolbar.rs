use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::ui::components::grid::ActiveStep;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// WASM bindings for Tauri Dialog Plugin
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "plugin", "dialog"])]
    async fn save(options: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "plugin", "dialog"])]
    async fn open(options: JsValue) -> JsValue;
}

#[derive(serde::Serialize)]
struct SavePatternArgs {
    pattern: serde_json::Value, // We'll need to fetch the actual pattern from state
    path: String,
}

#[derive(serde::Serialize)]
struct LoadPatternArgs {
    path: String,
}

#[derive(serde::Serialize)]
struct DialogFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(serde::Serialize)]
struct SaveDialogOptions {
    filters: Vec<DialogFilter>,
    defaultPath: Option<String>,
}

#[derive(serde::Serialize)]
struct OpenDialogOptions {
    filters: Vec<DialogFilter>,
    multiple: bool,
    directory: bool,
}

#[component]
pub fn Toolbar() -> impl IntoView {
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");

    let save_project = move |_| {
        leptos::task::spawn_local(async move {
            let options = SaveDialogOptions {
                filters: vec![DialogFilter {
                    name: "Flux Pattern".to_string(),
                    extensions: vec!["flux".to_string()],
                }],
                defaultPath: Some("pattern.flux".to_string()),
            };
            
            let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
            
            if let Ok(path) = save(options_js).await.into_serde::<Option<String>>() {
                if let Some(path) = path {
                    // Send CURRENT pattern state
                    let current_pattern = pattern_signal.get_untracked();
                    // We use serde_json::to_value because SavePatternArgs expects serde_json::Value or serialized struct
                    // But wait, SavePatternArgs.pattern is serde_json::Value? 
                    // Let's just serialize it directly if possible.
                    // Actually, let's redefine args to take the struct directly:
                    
                    #[derive(serde::Serialize)]
                    struct Args {
                        pattern: crate::shared::models::Pattern,
                        path: String,
                    }

                    let args = serde_wasm_bindgen::to_value(&Args {
                        pattern: current_pattern,
                        path: path.clone(), // Clone path to potential use later if needed, though not here
                    }).unwrap();
                    
                    invoke("save_pattern", args).await;

                    // Also save to last_pattern.flux for auto-load
                    if !path.ends_with("last_pattern.flux") {
                         let auto_args = serde_wasm_bindgen::to_value(&Args {
                            pattern: pattern_signal.get_untracked(),
                            path: "last_pattern.flux".to_string(),
                        }).unwrap();
                        invoke("save_pattern", auto_args).await;
                    }
                }
            }
        });
    };

    let load_project = move |_| {
        leptos::task::spawn_local(async move {
             let options = OpenDialogOptions {
                filters: vec![DialogFilter {
                    name: "Flux Pattern".to_string(),
                    extensions: vec!["flux".to_string()],
                }],
                multiple: false,
                directory: false,
            };
            
            let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
            
             if let Ok(path) = open(options_js).await.into_serde::<Option<String>>() {
                if let Some(path) = path {
                     let args = serde_wasm_bindgen::to_value(&LoadPatternArgs {
                        path: path.clone(),
                    }).unwrap();
                    
                    if let Ok(loaded_pattern) = invoke("load_pattern", args).await.into_serde::<crate::shared::models::Pattern>() {
                        set_pattern_signal.set(loaded_pattern);
                    }
                }
            }
        });
    };

    view! {
        <div class="flex gap-2">
            <button
                on:click=save_project
                class="px-3 py-1 bg-zinc-900 border border-zinc-700 rounded text-xs font-bold hover:bg-zinc-800 text-zinc-300"
            >
                SAVE
            </button>
             <button
                on:click=load_project
                class="px-3 py-1 bg-zinc-900 border border-zinc-700 rounded text-xs font-bold hover:bg-zinc-800 text-zinc-300"
            >
                LOAD
            </button>
        </div>
    }
}
