use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TrigType {
    None,
    Note,           // Standard Note On
    Lock,           // Parameter change only (Trigless Lock)
    SynthTrigger,   // Trigs Envelope/LFO but no Note (Trigless Trig)
    OneShot,        // Plays once (Yellow trig)
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LogicOp {
    // Basic logic operators can be added here
    Match,
    Not,
    Pre,
    Nei,
    Fill,
    // Add more as needed
}

impl Default for LogicOp {
    fn default() -> Self {
        Self::Match
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrigCondition {
    pub prob: u8,          // 0-100% Probability
    pub logic: LogicOp,    // A:B, Fill, NEI, PRE, etc.
}

impl Default for TrigCondition {
    fn default() -> Self {
        Self {
            prob: 100,
            logic: LogicOp::default(),
        }
    }
}

// Optimization: Fixed size array for P-Locks to avoid allocation in audio thread
// Index corresponds to Parameter ID (e.g., 0 = Pitch, 1 = Filter Cutoff)
pub type ParameterLocks = [Option<f32>; 128];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AtomicStep {
    pub trig_type: TrigType,
    pub note: u8,               // MIDI Note (0-127)
    pub velocity: u8,
    pub length: f32,            // Step duration
    pub micro_timing: i8,       // -23 to +23 (1/384th steps)
    pub condition: TrigCondition,
    pub sound_lock: Option<u16>,// Sound Pool ID (Digitakt style)
    #[serde(with = "serde_big_array::BigArray")]
    pub p_locks: ParameterLocks,// Parameter Modulations
    pub is_slide: bool,         // Analog Four Parameter Slide
    pub retrig_rate: u8,        // 0 = Off
}

impl Default for AtomicStep {
    fn default() -> Self {
        // Return a clean, empty step
        Self {
            trig_type: TrigType::None,
            note: 60,
            velocity: 100,
            length: 1.0,
            micro_timing: 0,
            condition: TrigCondition::default(),
            sound_lock: None,
            p_locks: [None; 128], // Compiler optimizes this
            is_slide: false,
            retrig_rate: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MachineType {
    OneShot,    // Digitakt II
    Werp,       // Digitakt II
    Slice,      // Octatrack
    FmTone,     // Digitone
    Subtractive,// Analog Four
    TonverkBus, // Tonverk
    MidiCC,     // External
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subtrack {
    pub voice_id: usize, // Internal Audio Engine Voice ID
    pub steps: Vec<AtomicStep>, // 16-64 steps
}

impl Default for Subtrack {
    fn default() -> Self {
        Self {
            voice_id: 0,
            steps: vec![AtomicStep::default(); 16],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: usize,
    pub machine: MachineType,
    pub subtracks: Vec<Subtrack>, // Vector to support Tonverk layering
    pub length: u32,
    pub scale: f32, // 1x, 2x, 1/2x, etc.
    #[serde(with = "serde_big_array::BigArray")]
    pub default_params: [f32; 128], // Track-level default parameters
    pub lfos: Vec<LFO>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LFOShape {
    Sine,
    Triangle,
    Square,
    Random,
    Designer(Vec<f32>), // 16 values
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LFO {
    pub shape: LFOShape,
    pub destination: u8, // MIDI CC Number (0-127) or specific internal param ID
    pub amount: f32,     // -1.0 to 1.0
    pub speed: f32,      // Cycles per bar, e.g., 1.0 = 1 cycle per bar
    pub phase: f32,      // Start phase offset (0.0-1.0)
}

impl Default for LFO {
    fn default() -> Self {
        Self {
            shape: LFOShape::Triangle,
            destination: 74, // Filter Cutoff
            amount: 0.0,
            speed: 1.0,
            phase: 0.0,
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            id: 0,
            machine: MachineType::OneShot,
            subtracks: vec![Subtrack::default()],
            length: 16,
            scale: 1.0,
            default_params: [0.5; 128], // Default to mid-range
            lfos: vec![LFO::default()],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub tracks: Vec<Track>, // Changed from [Track; 16] to Vec for easier serialization, usually fixed size in logic
    pub bpm: f32,
    pub master_length: u32,
}

impl Default for Pattern {
    fn default() -> Self {
        let mut tracks = Vec::with_capacity(4);
        for i in 0..4 {
            let mut t = Track::default();
            t.id = i;
            tracks.push(t);
        }
        Self {
            tracks,
            bpm: 120.0,
            master_length: 16,
        }
    }
}
