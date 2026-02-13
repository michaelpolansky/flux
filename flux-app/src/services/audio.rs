use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(serde::Serialize)]
struct PlaybackStateArgs {
    playing: bool,
}

pub async fn set_playback_state(playing: bool) {
    let args = serde_wasm_bindgen::to_value(&PlaybackStateArgs { playing }).unwrap();
    invoke("set_playback_state", args).await;
}
