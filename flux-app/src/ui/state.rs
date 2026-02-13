use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,        // 0-15
    pub triggered_tracks: [bool; 4],    // Which tracks fired this step
}
