use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;

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
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
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
                default_path: Some("pattern.flux".to_string()),
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
        <div class="flex items-center gap-2">
            <button
                on:click=save_project
                class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950"
            >
                SAVE
            </button>
            <button
                on:click=load_project
                class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950"
            >
                LOAD
            </button>

            <div class="w-px h-6 bg-zinc-700 mx-2"></div>

            <div class="text-sm font-mono text-zinc-400 px-3">
                "120 BPM"
            </div>

            <div class="w-px h-6 bg-zinc-700"></div>

            <button
                on:click=move |_| {
                    leptos::task::spawn_local(async {
                        crate::services::audio::set_playback_state(true).await;
                    });
                }
                class="h-10 px-4 bg-green-600 hover:bg-green-500 rounded-md text-sm font-medium text-white transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950"
            >
                ▶
            </button>
            <button
                on:click=move |_| {
                    leptos::task::spawn_local(async {
                        crate::services::audio::set_playback_state(false).await;
                    });
                }
                class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950"
            >
                ■
            </button>
        </div>
    }
}
