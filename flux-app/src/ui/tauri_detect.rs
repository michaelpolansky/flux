use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"], thread_local_v2)]
    static __TAURI__: JsValue;

    #[wasm_bindgen(js_namespace = ["window"], thread_local_v2)]
    static __TAURI_SAFE__: JsValue;
}

#[derive(Clone, Copy, Debug)]
pub struct TauriCapabilities {
    pub available: bool,
    pub audio_enabled: bool,
    pub events_enabled: bool,
}

impl Default for TauriCapabilities {
    fn default() -> Self {
        Self {
            available: false,
            audio_enabled: false,
            events_enabled: false,
        }
    }
}

/// Detect if Tauri APIs are available
pub fn detect_tauri() -> TauriCapabilities {
    // Check if window.__TAURI__ exists (native Tauri object)
    // We use __TAURI__ for detection because it's always available in Tauri mode
    // but we call through __TAURI_SAFE__ wrappers for actual API calls
    let tauri_exists = __TAURI__.with(|t| !t.is_undefined() && !t.is_null());

    if tauri_exists {
        TauriCapabilities {
            available: true,
            audio_enabled: true,
            events_enabled: true,
        }
    } else {
        TauriCapabilities::default()
    }
}
