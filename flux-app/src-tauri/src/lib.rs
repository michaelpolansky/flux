// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod engine;
pub mod shared;
pub mod commands;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Mutex;
use std::thread;
use rtrb::RingBuffer;
use tauri::{Emitter, State};
use triple_buffer::TripleBuffer;
use crate::engine::domain::AudioSnapshot;
use std::time::Duration;

use crate::engine::midi_engine::{MidiEngine, EngineCommand};
use crate::engine::kernel::{AudioCommand, FluxKernel};

pub struct AppState {
    command_producer: Mutex<rtrb::Producer<AudioCommand>>,
}

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
fn push_midi_command(_state: State<'_, EngineState>, args: MidiCommandArgs) -> Result<(), String> {
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
    // 1. Setup Audio via CPAL
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().expect("No default config");
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // 2. Create Command Queue (RingBuffer) for Audio
    let (audio_producer, audio_consumer) = RingBuffer::new(1024);

    // 3. Create State Snapshot (TripleBuffer)
    let (snapshot_producer, mut snapshot_consumer) = TripleBuffer::new(&AudioSnapshot::default()).split();

    // 4. Initialize Kernel
    // Move the consumer into the audio thread (Kernel)
    let mut kernel = FluxKernel::new(sample_rate, audio_consumer, snapshot_producer);

    // 4. Build Audio Stream
    // We run the stream in a separate thread managed by CPAL
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // CALL THE KERNEL PROCESS LOOP HERE
            kernel.process(data, channels);
        },
        |err| eprintln!("Stream error: {}", err),
        None // Timeout
    ).expect("Failed to build audio stream");

    // 6. Start Stream
    stream.play().expect("Failed to play stream");

    // LEAK THE STREAM so it stays alive for the duration of the app
    // (In a production app, we would store this in the AppHandle)
    Box::leak(Box::new(stream));

    // Existing MIDI Engine setup
    let (midi_producer, midi_consumer) = RingBuffer::new(1024);

    thread::spawn(move || {
        let mut engine = MidiEngine::new(midi_consumer).expect("Failed to initialize MIDI Engine");
        engine.run();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            
            // Spawn Sync Thread
            thread::spawn(move || {
                let mut last_step = 999;
                loop {
                    // Read latest state
                    let snapshot = snapshot_consumer.read();
                    
                    // Only emit if step changed
                    if snapshot.current_step != last_step {
                         // Emit to Frontend
                         let _ = app_handle.emit("playback-status", snapshot);
                         last_step = snapshot.current_step;
                    }
                    
                    thread::sleep(Duration::from_millis(16)); // ~60 FPS polling
                }
            });
            Ok(())
        })
        .manage(AppState {
            command_producer: Mutex::new(audio_producer),
        })
        .manage(EngineState {
            command_producer: Mutex::new(midi_producer),
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            push_midi_command, 
            save_pattern, 
            load_pattern, 
            set_lfo_shape, 
            set_lfo_designer_value, 
            commands::set_playback_state, 
            commands::toggle_step,
            commands::set_param_lock
        ])
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
fn load_pattern(path: String) -> Result<crate::shared::models::Pattern, String> {
    let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let pattern = serde_json::from_str(&json).map_err(|e| e.to_string())?;
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
