use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::ui::tauri::{safe_invoke, safe_dialog_save, safe_dialog_open, TauriError};

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

            match safe_dialog_save(options_js).await {
                Ok(Some(path)) => {
                    // Capture pattern state once to ensure consistency across both saves
                    let current_pattern = pattern_signal.get_untracked();

                    #[derive(serde::Serialize)]
                    struct Args {
                        pattern: crate::shared::models::Pattern,
                        path: String,
                    }

                    let args = serde_wasm_bindgen::to_value(&Args {
                        pattern: current_pattern.clone(),
                        path,
                    }).unwrap();

                    // Note: Errors are logged to console only, no user-facing notifications
                    match safe_invoke("save_pattern", args).await {
                        Ok(_) => {},
                        Err(TauriError::NotAvailable) => {
                            web_sys::console::log_1(&"Tauri not available - save command disabled".into());
                        },
                        Err(TauriError::InvokeFailed(msg)) => {
                            web_sys::console::error_1(&format!("Save command failed: {}", msg).into());
                        }
                    }

                    // Also save to last_pattern.flux for auto-load (using same pattern state)
                    if !path.ends_with("last_pattern.flux") {
                         let auto_args = serde_wasm_bindgen::to_value(&Args {
                            pattern: current_pattern.clone(),
                            path: "last_pattern.flux".to_string(),
                        }).unwrap();

                        match safe_invoke("save_pattern", auto_args).await {
                            Ok(_) => {},
                            Err(TauriError::NotAvailable) => {
                                web_sys::console::log_1(&"Tauri not available - auto-save command disabled".into());
                            },
                            Err(TauriError::InvokeFailed(msg)) => {
                                web_sys::console::error_1(&format!("Auto-save command failed: {}", msg).into());
                            }
                        }
                    }
                },
                Ok(None) => {
                    // User cancelled the dialog
                },
                Err(TauriError::NotAvailable) => {
                    web_sys::console::log_1(&"Tauri not available - save dialog disabled".into());
                },
                Err(TauriError::InvokeFailed(msg)) => {
                    web_sys::console::error_1(&format!("Save dialog failed: {}", msg).into());
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

            match safe_dialog_open(options_js).await {
                Ok(Some(path)) => {
                     let args = serde_wasm_bindgen::to_value(&LoadPatternArgs {
                        path,
                    }).unwrap();

                    // Note: Errors are logged to console only, no user-facing notifications
                    match safe_invoke("load_pattern", args).await {
                        Ok(result) => {
                            match result.into_serde::<crate::shared::models::Pattern>() {
                                Ok(loaded_pattern) => {
                                    set_pattern_signal.set(loaded_pattern);
                                },
                                Err(e) => {
                                    web_sys::console::error_1(&format!("Failed to deserialize pattern: {:?}", e).into());
                                }
                            }
                        },
                        Err(TauriError::NotAvailable) => {
                            web_sys::console::log_1(&"Tauri not available - load command disabled".into());
                        },
                        Err(TauriError::InvokeFailed(msg)) => {
                            web_sys::console::error_1(&format!("Load command failed: {}", msg).into());
                        }
                    }
                },
                Ok(None) => {
                    // User cancelled the dialog
                },
                Err(TauriError::NotAvailable) => {
                    web_sys::console::log_1(&"Tauri not available - open dialog disabled".into());
                },
                Err(TauriError::InvokeFailed(msg)) => {
                    web_sys::console::error_1(&format!("Open dialog failed: {}", msg).into());
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
                "▶"
            </button>
            <button
                on:click=move |_| {
                    leptos::task::spawn_local(async {
                        crate::services::audio::set_playback_state(false).await;
                    });
                }
                class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950"
            >
                "■"
            </button>
        </div>
    }
}
