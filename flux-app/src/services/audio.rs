use crate::ui::tauri::{safe_invoke, TauriError};

#[derive(serde::Serialize)]
struct PlaybackStateArgs {
    playing: bool,
}

pub async fn set_playback_state(playing: bool) {
    let args = match serde_wasm_bindgen::to_value(&PlaybackStateArgs { playing }) {
        Ok(v) => v,
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to serialize playback args: {:?}", e).into());
            return;
        }
    };

    match safe_invoke("set_playback_state", args).await {
        Ok(_) => { /* success */ },
        Err(TauriError::NotAvailable) => {
            web_sys::console::log_1(&"Tauri not available - playback command disabled".into());
        },
        Err(TauriError::InvokeFailed(msg)) => {
            web_sys::console::error_1(&format!("Playback command failed: {}", msg).into());
        }
    }
}
