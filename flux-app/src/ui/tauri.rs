use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct MidiCommandArgs {
    pub command: String,
    pub step: Option<usize>,
    pub param: Option<String>,
    pub value: Option<f64>,
}

pub async fn push_midi_command(command: &str, step: Option<usize>, param: Option<String>, value: Option<f64>) {
    let args = serde_wasm_bindgen::to_value(&MidiCommandArgs {
        command: command.to_string(),
        step,
        param,
        value,
    }).unwrap();
    
    // Fire and forget for now, or log error
    let _ = invoke("push_midi_command", args).await;
}

#[derive(Serialize, Deserialize)]
pub struct SetLFOShapeArgs {
    pub track_id: usize,
    pub lfo_index: usize,
    pub shape: crate::shared::models::LFOShape,
}

pub async fn set_lfo_shape(track_id: usize, lfo_index: usize, shape: crate::shared::models::LFOShape) {
    let args = serde_wasm_bindgen::to_value(&SetLFOShapeArgs {
        track_id,
        lfo_index,
        shape,
    }).unwrap();
    let _ = invoke("set_lfo_shape", args).await;
}

#[derive(Serialize, Deserialize)]
pub struct SetLFODesignerValueArgs {
    pub track_id: usize,
    pub lfo_index: usize,
    pub step: usize,
    pub value: f32,
}

pub async fn set_lfo_designer_value(track_id: usize, lfo_index: usize, step: usize, value: f32) {
    let args = serde_wasm_bindgen::to_value(&SetLFODesignerValueArgs {
        track_id,
        lfo_index,
        step,
        value,
    }).unwrap();
    let _ = invoke("set_lfo_designer_value", args).await;
}
