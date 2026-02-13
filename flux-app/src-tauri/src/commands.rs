use tauri::State;
use crate::AppState;
use crate::engine::kernel::AudioCommand;

#[tauri::command]
pub fn set_playback_state(playing: bool, state: State<'_, AppState>) -> Result<(), String> {
    let mut producer = state.command_producer.lock().map_err(|_| "Failed to lock mutex")?;
    
    let command = if playing {
        AudioCommand::Play
    } else {
        AudioCommand::Stop
    };

    producer.push(command).map_err(|_| "Command queue full")?;
    Ok(())
}

#[tauri::command]
pub fn toggle_step(track_id: usize, step_idx: usize, state: State<'_, AppState>) -> Result<(), String> {
    let mut producer = state.command_producer.lock().map_err(|_| "Failed to lock mutex")?;
    producer.push(AudioCommand::ToggleStep(track_id, step_idx)).map_err(|_| "Queue full")?;
    Ok(())
}

#[tauri::command]
pub fn set_param_lock(
    track_id: usize, 
    step_idx: usize, 
    param_id: usize, 
    value: Option<f32>, 
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut producer = state.command_producer.lock().map_err(|_| "Lock fail")?;
    producer.push(AudioCommand::SetParamLock(track_id, step_idx, param_id, value))
        .map_err(|_| "Queue full")?;
    Ok(())
}
