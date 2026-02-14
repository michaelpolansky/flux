use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,        // 0-15
    pub triggered_tracks: [bool; 4],    // Which tracks fired this step
}

#[derive(Clone, Debug)]
pub struct GridUIState {
    pub hovered_step: Option<(usize, usize)>,  // (track, step)
    pub recent_triggers: Vec<TriggerEvent>,
}

#[derive(Clone, Debug)]
pub struct TriggerEvent {
    pub track: usize,
    pub step: usize,
    pub timestamp: f64,  // Milliseconds since animation start
}

impl Default for GridUIState {
    fn default() -> Self {
        Self {
            hovered_step: None,
            recent_triggers: Vec::with_capacity(64),
        }
    }
}

impl GridUIState {
    pub fn add_trigger(&mut self, track: usize, step: usize, timestamp: f64) {
        self.recent_triggers.push(TriggerEvent {
            track,
            step,
            timestamp,
        });
    }

    pub fn cleanup_old_triggers(&mut self, current_time: f64, max_age: f64) {
        self.recent_triggers.retain(|trigger| {
            current_time - trigger.timestamp < max_age
        });
    }
}
