use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use leptos::prelude::*;
use crate::ui::tauri_detect::TauriCapabilities;

#[derive(Debug, Clone)]
pub enum TauriError {
    NotAvailable,
    InvokeFailed(String),
}

/// Check if Tauri is available (cached from detection)
fn is_tauri_available() -> bool {
    use_context::<TauriCapabilities>()
        .map(|caps| caps.available)
        .unwrap_or(false)
}

/// Safe invoke - returns error if Tauri unavailable
pub async fn safe_invoke(cmd: &str, args: JsValue) -> Result<JsValue, TauriError> {
    if !is_tauri_available() {
        return Err(TauriError::NotAvailable);
    }

    invoke_with_error(cmd, args)
        .await
        .map_err(|e| TauriError::InvokeFailed(format!("{:?}", e)))
}

/// Safe event listener - no-op if Tauri unavailable
pub async fn safe_listen_event<T>(event_name: &str, callback: impl Fn(T) + 'static)
where T: for<'a> Deserialize<'a> + 'static
{
    if !is_tauri_available() {
        // Log once for debugging
        web_sys::console::log_1(
            &format!("Tauri not available - event listener '{}' disabled", event_name).into()
        );
        return;
    }

    // Call existing listen_event implementation
    listen_event(event_name, callback).await
}

/// Safe dialog save - returns error if Tauri unavailable
pub async fn safe_dialog_save(options: JsValue) -> Result<Option<String>, TauriError> {
    if !is_tauri_available() {
        return Err(TauriError::NotAvailable);
    }

    dialog_save_with_error(options)
        .await
        .map_err(|e| TauriError::InvokeFailed(format!("{:?}", e)))
        .and_then(|js_val| {
            serde_wasm_bindgen::from_value::<Option<String>>(js_val)
                .map_err(|e| TauriError::InvokeFailed(format!("Failed to deserialize path: {:?}", e)))
        })
}

/// Safe dialog open - returns error if Tauri unavailable
pub async fn safe_dialog_open(options: JsValue) -> Result<Option<String>, TauriError> {
    if !is_tauri_available() {
        return Err(TauriError::NotAvailable);
    }

    dialog_open_with_error(options)
        .await
        .map_err(|e| TauriError::InvokeFailed(format!("{:?}", e)))
        .and_then(|js_val| {
            serde_wasm_bindgen::from_value::<Option<String>>(js_val)
                .map_err(|e| TauriError::InvokeFailed(format!("Failed to deserialize path: {:?}", e)))
        })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_SAFE__"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

// Alias for compatibility
async fn invoke_with_error(cmd: &str, args: JsValue) -> Result<JsValue, JsValue> {
    invoke(cmd, args).await
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_SAFE__"], js_name = dialogSave, catch)]
    async fn dialog_save_with_error(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI_SAFE__"], js_name = dialogOpen, catch)]
    async fn dialog_open_with_error(options: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
pub struct MidiCommandArgs {
    pub command: String,
    pub step: Option<usize>,
    pub param: Option<String>,
    pub value: Option<f64>,
}

pub async fn push_midi_command(command: &str, step: Option<usize>, param: Option<String>, value: Option<f64>) {
    if !is_tauri_available() {
        return; // Silent - feature disabled in browser mode
    }

    let args = serde_wasm_bindgen::to_value(&MidiCommandArgs {
        command: command.to_string(),
        step,
        param,
        value,
    }).unwrap();

    if let Err(e) = invoke_with_error("push_midi_command", args).await {
        web_sys::console::error_1(&format!("push_midi_command failed: {:?}", e).into());
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetLFODesignerValueArgs {
    pub track_id: usize,
    pub lfo_index: usize,
    pub step: usize,
    pub value: f32,
}

pub async fn set_lfo_designer_value(track_id: usize, lfo_index: usize, step: usize, value: f32) {
    if !is_tauri_available() {
        return; // Silent - feature disabled in browser mode
    }

    let args = serde_wasm_bindgen::to_value(&SetLFODesignerValueArgs {
        track_id,
        lfo_index,
        step,
        value,
    }).unwrap();

    if let Err(e) = invoke_with_error("set_lfo_designer_value", args).await {
        web_sys::console::error_1(&format!("set_lfo_designer_value failed: {:?}", e).into());
    }
}

#[derive(Serialize, Deserialize)]
pub struct ToggleStepArgs {
    pub track_id: usize,
    pub step_idx: usize,
}

pub async fn toggle_step(track_id: usize, step_idx: usize) {
    if !is_tauri_available() {
        return; // Silent - feature disabled in browser mode
    }

    let args = serde_wasm_bindgen::to_value(&ToggleStepArgs {
        track_id,
        step_idx,
    }).unwrap();

    if let Err(e) = invoke_with_error("toggle_step", args).await {
        web_sys::console::error_1(&format!("toggle_step failed: {:?}", e).into());
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TauriEvent<T> {
    #[allow(dead_code)]
    pub event: String,
    pub payload: T,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_SAFE__"], catch)]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Result<JsValue, JsValue>;
}

pub async fn listen_event<T>(event_name: &str, callback: impl Fn(T) + 'static)
where T: for<'a> Deserialize<'a> + 'static
{
    let handler = Closure::wrap(Box::new(move |val: JsValue| {
        if let Ok(event_struct) = serde_wasm_bindgen::from_value::<TauriEvent<T>>(val) {
            callback(event_struct.payload);
        }
    }) as Box<dyn FnMut(JsValue)>);

    // We intentionally leak the closure to keep it alive for the lifetime of the app
    if let Ok(_) = listen(event_name, &handler).await {
        handler.forget();
    }
}
