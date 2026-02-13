use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct AudioSnapshot {
    pub current_step: usize,
    pub is_playing: bool,
}

// Parameter Indices
pub const PARAM_PITCH: usize = 0; // MIDI Note Number (0.0 - 127.0)
pub const PARAM_DECAY: usize = 1; // 0.0 to 1.0
