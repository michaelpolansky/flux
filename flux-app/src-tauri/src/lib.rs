// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod engine;
pub mod shared;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::Mutex;
use std::thread;
use rtrb::RingBuffer;
use tauri::State;

use crate::engine::midi_engine::{MidiEngine, EngineCommand};
use crate::shared::models::AtomicStep; // Example import if needed for command payload deserialization

struct EngineState {
    command_producer: Mutex<rtrb::Producer<EngineCommand>>,
}

#[derive(serde::Deserialize)]
pub struct MidiCommandArgs {
    pub command: String,
    pub step: Option<usize>,
    pub param: Option<String>,
    pub value: Option<f64>,
}

#[tauri::command]
fn push_midi_command(state: State<'_, EngineState>, args: MidiCommandArgs) -> Result<(), String> {
    // In a real app, we would map this to EngineCommand and push to the ring buffer.
    // For now, we just print to stdout to verify connectivity.
    println!("Received Command: {}, Step: {:?}, Param: {:?}, Value: {:?}", 
        args.command, args.step, args.param, args.value);
    
    // TODO: Map to EngineCommand and push to producer
    // let cmd = match args.command.as_str() { ... }
    
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (producer, consumer) = RingBuffer::new(1024);

    thread::spawn(move || {
        let mut engine = MidiEngine::new(consumer).expect("Failed to initialize MIDI Engine");
        engine.run();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(EngineState {
            command_producer: Mutex::new(producer),
        })
        .invoke_handler(tauri::generate_handler![greet, push_midi_command, save_pattern, load_pattern, set_lfo_shape, set_lfo_designer_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_pattern(pattern: crate::shared::models::Pattern, path: String) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&pattern).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
    Ok(pattern)
}

#[tauri::command]
fn set_lfo_shape(state: State<'_, EngineState>, track_id: usize, lfo_index: usize, shape: crate::shared::models::LFOShape) -> Result<(), String> {
    state.command_producer.lock().unwrap()
        .push(EngineCommand::SetLFOShape { track_id, lfo_index, shape })
        .map_err(|_| "Failed to send command to engine".to_string())
}

#[tauri::command]
fn set_lfo_designer_value(state: State<'_, EngineState>, track_id: usize, lfo_index: usize, step: usize, value: f32) -> Result<(), String> {
    state.command_producer.lock().unwrap()
        .push(EngineCommand::SetLFODesignerValue { track_id, lfo_index, step, value })
        .map_err(|_| "Failed to send command to engine".to_string())
}
