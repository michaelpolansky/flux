use crate::shared::models::{AtomicStep, MachineType, Pattern, Subtrack, Track, TrigType};
use crate::engine::domain::AudioSnapshot;
use rtrb::Consumer;
use triple_buffer::Input;
use std::f32::consts::PI;

// Helper to convert MIDI note to Hz
fn midi_to_freq(note: f32) -> f32 {
    440.0 * 2.0_f32.powf((note - 69.0) / 12.0)
}

pub enum AudioCommand {
    Play,
    Stop,
    SetGlobalVolume(f32),
    ToggleStep(usize, usize),
    SetParamLock(usize, usize, usize, Option<f32>), // Track, Step, Param, Value
}

pub struct FluxKernel {
    pub pattern: Pattern,
    pub is_playing: bool,
    pub playhead_sample: usize,
    pub sample_rate: f32,
    pub command_consumer: Consumer<AudioCommand>,
    pub snapshot_producer: Input<AudioSnapshot>,
    
    // Sequencer Clock State
    pub tempo: f32,
    pub samples_per_step: f32,
    pub step_phase: f32,
    pub current_step: usize,

    // Voice State
    pub current_frequency: f32,
    pub current_decay: f32,
}

impl FluxKernel {
    pub fn new(sample_rate: f32, command_consumer: Consumer<AudioCommand>, snapshot_producer: Input<AudioSnapshot>) -> Self {
        let tempo = 120.0;
        let samples_per_step = sample_rate * 60.0 / (tempo * 4.0);

        // Create a default pattern with 1 track, 1 subtrack, 16 steps
        let mut steps = Vec::new();
        for i in 0..16 {
            let mut step = AtomicStep::default();
            // Four-on-the-floor: Kick on 0, 4, 8, 12
            if i % 4 == 0 {
                step.trig_type = TrigType::Note;
                step.note = 60; // Middle C
            }



            steps.push(step);
        }

        let subtrack = Subtrack {
            voice_id: 0,
            steps,
        };

        let track = Track {
            id: 0,
            machine: MachineType::OneShot, // Or whatever default
            subtracks: vec![subtrack],
            length: 16,
            scale: 1.0,
            lfos: Vec::new(),
        };

        let mut pattern = Pattern::default();
        pattern.tracks.push(track);
        pattern.bpm = tempo;

        Self {
            pattern,
            is_playing: false,
            playhead_sample: 0,
            sample_rate,
            command_consumer,
            snapshot_producer,
            tempo,
            samples_per_step,
            step_phase: samples_per_step, // Start ready to trigger
            current_step: 15, // Start at end so next step is 0
            current_frequency: 440.0,
            current_decay: 0.5,
        }
    }

    pub fn process(&mut self, output_buffer: &mut [f32], channels: usize) {
        // 1. Process Commands
        while let Ok(cmd) = self.command_consumer.pop() {
            match cmd {
                AudioCommand::Play => self.is_playing = true,
                AudioCommand::Stop => {
                    self.is_playing = false;
                    self.playhead_sample = 0;
                    self.current_step = 15;
                    self.step_phase = self.samples_per_step;
                }
                AudioCommand::SetGlobalVolume(_) => {} // TODO
                AudioCommand::ToggleStep(track_id, step_idx) => {
                    if let Some(track) = self.pattern.tracks.get_mut(track_id) {
                        if let Some(subtrack) = track.subtracks.get_mut(0) {
                            if let Some(step) = subtrack.steps.get_mut(step_idx) {
                                step.trig_type = match step.trig_type {
                                    TrigType::None => TrigType::Note,
                                    _ => TrigType::None,
                                };
                            }
                        }
                    }
                }
                AudioCommand::SetParamLock(track_id, step_idx, param_id, val) => {
                    if let Some(track) = self.pattern.tracks.get_mut(track_id) {
                        if let Some(subtrack) = track.subtracks.get_mut(0) {
                            if let Some(step) = subtrack.steps.get_mut(step_idx) {
                                // Safety check for param array bounds if needed, though fixed [128] is safe
                                if param_id < 128 {
                                    step.p_locks[param_id] = val;
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Audio Generation
        for frame in output_buffer.chunks_mut(channels) {
            let mut sample = 0.0;
            
            if self.is_playing {
                self.step_phase += 1.0;
                
                // Check if we crossed a step boundary
                if self.step_phase >= self.samples_per_step {
                    self.step_phase -= self.samples_per_step;
                    self.current_step = (self.current_step + 1) % 16;
                    
                    // CHECK FOR TRIGGER
                    // Safety check: Ensure track and subtrack exist
                    if let Some(track) = self.pattern.tracks.get(0) {
                        if let Some(subtrack) = track.subtracks.get(0) {
                            if let Some(step) = subtrack.steps.get(self.current_step) {
                                if step.trig_type != TrigType::None {
                                    // Trigger the sound!
                                    
                                    // 1. Resolve Pitch
                                    // Check for P-Lock first, then fallback to Step Note
                                    let note_val = step.p_locks[crate::engine::domain::PARAM_PITCH]
                                        .unwrap_or(step.note as f32);
                                        
                                    self.current_frequency = midi_to_freq(note_val);

                                    // 2. Trigger Envelope (Reset Phase)
                                    self.playhead_sample = 0;
                                    println!("Step: {} [TRIG] Freq: {:.2}", self.current_step, self.current_frequency);
                                } else {
                                    // println!("Step: {}", self.current_step);
                                }
                            }
                        }
                    }
                }

                self.playhead_sample += 1;
                // Test Tone: Sine Wave with current_frequency
                let t = self.playhead_sample as f32 / self.sample_rate;
                sample = (t * self.current_frequency * 2.0 * PI).sin() * 0.1;
            }

            // Write to all channels
            for out in frame.iter_mut() {
                *out = sample;
            }
        }

        // 3. Update Snapshot
        self.snapshot_producer.write(AudioSnapshot {
            current_step: self.current_step,
            is_playing: self.is_playing,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rtrb::RingBuffer;
    use crate::shared::models::{AtomicStep, TrigType};
    use crate::engine::domain::{PARAM_PITCH, AudioSnapshot};

    // Helper to setup a kernel for testing
    fn setup_kernel() -> (FluxKernel, rtrb::Producer<AudioCommand>) {
        let (producer, consumer) = RingBuffer::new(1024);
        let (snapshot_prod, _) = triple_buffer::TripleBuffer::new(&AudioSnapshot::default()).split();
        let sample_rate = 44100.0;
        let kernel = FluxKernel::new(sample_rate, consumer, snapshot_prod);
        (kernel, producer)
    }

    #[test]
    fn test_initialization() {
        let (kernel, _) = setup_kernel();
        assert_eq!(kernel.sample_rate, 44100.0);
        assert_eq!(kernel.is_playing, false);
        // Default starts at 15 so next is 0
        assert_eq!(kernel.current_step, 15);
    }

    #[test]
    fn test_transport_command() {
        let (mut kernel, mut producer) = setup_kernel();
        
        // 1. Send Play Command
        producer.push(AudioCommand::Play).unwrap();
        
        // 2. Process a tiny buffer (1 frame) to consume the command
        let mut buffer = [0.0; 2]; // 1 sample, stereo
        kernel.process(&mut buffer, 2);
        
        // 3. Verify State
        assert_eq!(kernel.is_playing, true);
        
        // 4. Send Stop Command
        producer.push(AudioCommand::Stop).unwrap();
        kernel.process(&mut buffer, 2);
        assert_eq!(kernel.is_playing, false);
    }

    #[test]
    fn test_clock_advancement() {
        let (mut kernel, mut producer) = setup_kernel();
        
        // Set Tempo to 120 BPM
        // Samples per step = (44100 * 60) / (120 * 4) = 5512.5 samples
        kernel.tempo = 120.0;
        kernel.samples_per_step = (kernel.sample_rate * 60.0) / (kernel.tempo * 4.0);
        
        // Start Playing
        producer.push(AudioCommand::Play).unwrap();
        
        // Process exactly enough samples to reach the next step (Step 0)
        // We start at Step 15 with phase maxed out.
        // First sample -> Step 0.
        // Process another full step -> Step 1.
        // We need > 5512.5 samples. Use 6000.
        let samples_to_process = 6000;
        let mut buffer = vec![0.0; samples_to_process * 2]; 
        
        kernel.process(&mut buffer, 2);
        
        // Should have advanced from Step 0 to Step 1 within the buffer
        assert_eq!(kernel.current_step, 1);
    }

    #[test]
    fn test_p_lock_application() {
        let (mut kernel, mut producer) = setup_kernel();
        producer.push(AudioCommand::Play).unwrap();

        // 1. Setup a Pattern: Step 1 has a P-Lock on Pitch
        // Note: Step 0 is default (Empty), Step 1 is the target.
        let mut step = AtomicStep::default();
        step.trig_type = TrigType::Note;
        step.note = 60; // Default Middle C
        step.p_locks[PARAM_PITCH] = Some(72.0); // Lock to High C
        
        // Inject into Pattern (Track 0, Subtrack 0, Step 1)
        if let Some(track) = kernel.pattern.tracks.get_mut(0) {
             if let Some(subtrack) = track.subtracks.get_mut(0) {
                 if subtrack.steps.len() > 1 {
                     subtrack.steps[1] = step;
                 }
             }
        }
        
        // 2. Advance Clock to reach Step 1
        // We need to cross Step 0 and reach Step 1.
        // 120 BPM = ~5512.5 samples per step. 
        // We process 6000 samples to reliably hit Step 1.
        let mut buffer = vec![0.0; 6000 * 2];
        kernel.process(&mut buffer, 2);
        
        assert_eq!(kernel.current_step, 1);
        
        // 3. Verify Frequency
        // 72.0 MIDI = 523.25 Hz
        let expected_freq = 440.0 * 2.0_f32.powf((72.0 - 69.0) / 12.0);
        
        // Use epsilon for float comparison
        assert!((kernel.current_frequency - expected_freq).abs() < 0.1, 
            "Expected freq {}, got {}", expected_freq, kernel.current_frequency);
    }
}
