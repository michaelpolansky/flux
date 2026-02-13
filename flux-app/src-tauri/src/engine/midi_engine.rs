use thread_priority::*;

use crate::shared::models::{Pattern, AtomicStep, TrigType, TrigCondition, LogicOp};

pub enum EngineCommand {
    UpdatePattern(Pattern),
    SetLFOShape { track_id: usize, lfo_index: usize, shape: LFOShape },
    SetLFODesignerValue { track_id: usize, lfo_index: usize, step: usize, value: f32 },
}

pub struct MidiEngine {
    midi_out: MidiOutputConnection,
    command_consumer: Consumer<EngineCommand>,
    pattern: Option<Pattern>,
    ppqn: u32,
    bpm: f32,
}

impl MidiEngine {
    pub fn new(command_consumer: Consumer<EngineCommand>) -> Result<Self, Box<dyn std::error::Error>> {
        let midi_out = MidiOutput::new("Flux Sequencer")?;
        
        // Get output ports
        let out_ports = midi_out.ports();
        
        // Try to open a virtual port, fallback to first available port or just open a connection if virtual not supported (standard midir behavior is platform dependent)
        // For macOS/Linux virtual ports are supported.
        let conn = match midi_out.create_virtual("Flux Sequencer Out") {
            Ok(conn) => conn,
            Err(_) => {
                // Fallback for Windows or if virtual creation fails: pick the first available or create a regular connection
                if let Some(port) = out_ports.get(0) {
                     midi_out.connect(port, "Flux Sequencer Out")?
                } else {
                    return Err("No MIDI output ports available and could not create virtual port".into());
                }
            }
        };

        Ok(Self {
            midi_out: conn,
            command_consumer,
            pattern: None,
            ppqn: 24,
            bpm: 120.0,
        })
    }

    pub fn run(&mut self) {
        let mut next_tick_time = Instant::now();
        let mut tick_count: u64 = 0;
        
        // Calculate tick duration for 120 BPM @ 24 PPQN
        // 60 seconds / 120 beats = 0.5 seconds per beat
        // 0.5 / 24 = 0.020833 seconds per tick = 20.833 ms
        // Wait, standard MIDI clock is 24 PPQN.
        // 24 ticks per beat. 
        // Tick duration = 60 / (BPM * 24)
        
        // Set Thread Priority to High/Realtime
        // We try to set it to highest possible
        match set_current_thread_priority(ThreadPriority::Max) {
            Ok(_) => println!("MIDI Engine thread priority set to Max"),
            Err(e) => eprintln!("Failed to set MIDI Engine thread priority: {:?}", e),
        }

        loop {
            // 1. Process Commands
            while let Ok(cmd) = self.command_consumer.pop() {
                match cmd {
                    EngineCommand::UpdatePattern(p) => {
                        self.bpm = p.bpm;
                        self.pattern = Some(p);
                    },
                    EngineCommand::SetLFOShape { track_id, lfo_index, shape } => {
                        if let Some(p) = &mut self.pattern {
                            if let Some(track) = p.tracks.get_mut(track_id) {
                                if let Some(lfo) = track.lfos.get_mut(lfo_index) {
                                    lfo.shape = shape;
                                }
                            }
                        }
                    },
                    EngineCommand::SetLFODesignerValue { track_id, lfo_index, step, value } => {
                        if let Some(p) = &mut self.pattern {
                            if let Some(track) = p.tracks.get_mut(track_id) {
                                if let Some(lfo) = track.lfos.get_mut(lfo_index) {
                                    if let crate::shared::models::LFOShape::Designer(points) = &mut lfo.shape {
                                        if step < 16 {
                                            points[step] = value;
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            }

            // 2. Calculate next tick interval
            let tick_duration_secs = 60.0 / (self.bpm * self.ppqn as f32);
            let tick_duration = Duration::from_micros((tick_duration_secs * 1_000_000.0) as u64);
            
            next_tick_time += tick_duration;
            
            // 3. Wait for high precision clock
            let now = Instant::now();
            if now < next_tick_time {
                let wait_time = next_tick_time - now;
                
                // If wait time is > 1ms, use thread sleep to save CPU
                if wait_time > Duration::from_millis(1) {
                    thread::sleep(wait_time - Duration::from_millis(1));
                }
                
                // Spin loop for final precision
                while Instant::now() < next_tick_time {
                    spin_loop();
                }
            } else {
                // We are behind!
                // Implement catch-up logic or just reset next_tick_time if way behind
                if now - next_tick_time > tick_duration * 10 {
                   next_tick_time = now;
                }
            }

            // 4. Sequencer Logic
            if let Some(pattern) = &self.pattern {
                self.process_tick(tick_count, pattern);
            }
            
            tick_count += 1;
            
            // Verification Heartbeat with enhanced Drill check
            if tick_count % 1000 == 0 {
                let jitter = Instant::now().duration_since(next_tick_time);
                // We use micros for precision logging
                let drift_ms = jitter.as_micros() as f64 / 1000.0;
                
                println!("Heartbeat: Tick {}, Drift: {:.3} ms", tick_count, drift_ms);
                
                if drift_ms > 0.5 {
                    println!("WARNING: High Jitter Detected (>0.5ms)");
                }
            }
        }
    }

    fn process_tick(&mut self, tick_count: u64, pattern: &Pattern) {
        // Simple logic for now: Advance tracks
        // Assuming 16 step pattern for simplicity for now, but should use pattern length
        
        // 24 PPQN. 
        // 16th note = 6 ticks (24 / 4).

        // LFO Calculation
        // Calculate global phase (0.0 to 1.0) based on bar length (4 beats * 24 ticks = 96 ticks)
        // This is a simplification; should depend on Pattern Master Length
        let bar_ticks = 96.0;
        let global_phase = (tick_count as f32 % bar_ticks) / bar_ticks;

        for track in &pattern.tracks {
            // Process LFOs
            for lfo in &track.lfos {
                if lfo.amount != 0.0 {
                    let lfo_val = Self::calculate_lfo(lfo, global_phase);
                    // Map -1.0..1.0 to 0..127
                    // Center around 64? Or Additive? 
                    // Usually LFO is bipolar (-1 to 1). 
                    // CC is unipolar (0 to 127).
                    // We'll map [-1, 1] to [0, 127] for direct control, 
                    // OR we assume it modulates a parameter. 
                    // For this requirement: "LFO -> Filter Cutoff". 
                    // Let's sweep the whole range 0-127.
                    let cc_val = ((lfo_val + 1.0) / 2.0 * 127.0).clamp(0.0, 127.0) as u8;
                    
                    // Optimization: Only send if changed? 
                    // For now send every tick allows smooth 24 updates per beat (smooth-ish)
                    self.send_cc(track.id as u8, lfo.destination, cc_val);
                    
                    // Debug Log for Verification (Requested in Plan)
                    // if tick_count % 24 == 0 {
                    //      println!("Track {} LFO -> CC {}: {}", track.id, lfo.destination, cc_val);
                    // }
                }
            }
        }
        
        if tick_count % 6 == 0 {
            let step_index = (tick_count / 6) % 16;
            // println!("Step {}", step_index);
            
            for track in &pattern.tracks {
                // Check if track has a trig at this step
                // Currently Track has subtracks with steps. 
                // We need to map steps to the grid. 
                // Assuming steps are in order? Or Sparse?
                // The models say `steps: Vec<AtomicStep>`. This implies a list.
                // But usually step sequencer uses index-based access.
                // Let's assume `steps` is 16 elements long for now or check bounds.
                
                for subtrack in &track.subtracks {
                    if let Some(step) = subtrack.steps.get(step_index as usize) {
                         if step.trig_type == TrigType::Note {
                             self.send_note_on(track.id as u8, step.note, step.velocity);
                             
                             // Note Off scheduled? 
                             // For this MVP, we might skip note off or schedule it.
                             // MIDI usually needs Note Off. 
                             // We'll send a very short Note Off for now or implement a note stack later.
                             self.send_note_off(track.id as u8, step.note);
                         }
                    }
                }
            }
        }
    }
    
    fn calculate_lfo(lfo: &crate::shared::models::LFO, global_phase: f32) -> f32 {
        use crate::shared::models::LFOShape;
        // use std::f32::consts::PI; // Already imported or available? It was valid before.
        use std::f32::consts::PI;

        // Apply Speed and Phase Offset
        let mut phase = (global_phase * lfo.speed + lfo.phase) % 1.0;
        if phase < 0.0 { phase += 1.0; }

        let raw = match &lfo.shape {
            LFOShape::Sine => (phase * 2.0 * PI).sin(),
            LFOShape::Triangle => {
                // 0 -> 1 -> 0 -> -1 -> 0
                if phase < 0.25 {
                    phase * 4.0
                } else if phase < 0.75 {
                    1.0 - (phase - 0.25) * 4.0
                } else {
                    -1.0 + (phase - 0.75) * 4.0
                }
            },
            LFOShape::Square => if phase < 0.5 { 1.0 } else { -1.0 },
            LFOShape::Random => {
                // Return a hash based on phase step to adhere to deterministic "Random" if needed
                // Or just use rand. For seeded random we'd need a seed. 
                // Let's stick to pseudo-random based on tick for now or simple "Noise"
                // For simplicity, let's use Sine for now or implement a proper Rand later.
                (phase * 2.0 * PI).sin() // Placeholder
            },
            LFOShape::Designer(points) => {
                // Linear Interpolation between 16 points
                let len = 16;
                let idx_f = phase * len as f32;
                let idx = idx_f.floor() as usize;
                let next_idx = (idx + 1) % len;
                let frac = idx_f - idx as f32;
                
                let p1 = points[idx % len];
                let p2 = points[next_idx];
                
                // Lerp
                p1 + (p2 - p1) * frac
            }
        };

        raw * lfo.amount
    }

    fn send_note_on(&mut self, channel: u8, note: u8, velocity: u8) {
        // Channel 0-15
        let status = 0x90 | (channel & 0x0F);
        let _ = self.midi_out.send(&[status, note, velocity]);
    }
    
    fn send_note_off(&mut self, channel: u8, note: u8) {
        let status = 0x80 | (channel & 0x0F);
        let _ = self.midi_out.send(&[status, note, 0]);
    }

    fn send_cc(&mut self, channel: u8, cc: u8, val: u8) {
        let status = 0xB0 | (channel & 0x0F);
        let _ = self.midi_out.send(&[status, cc, val]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::models::{LFO, LFOShape};

    #[test]
    fn test_sine_lfo() {
        let lfo = LFO {
            shape: LFOShape::Sine,
            amount: 1.0,
            speed: 1.0,
            phase: 0.0,
            destination: 0,
        };
        
        // Phase 0.0 -> sin(0) = 0.0
        assert!((MidiEngine::calculate_lfo(&lfo, 0.0) - 0.0).abs() < 1e-6);
        // Phase 0.25 -> sin(PI/2) = 1.0
        assert!((MidiEngine::calculate_lfo(&lfo, 0.25) - 1.0).abs() < 1e-6);
        // Phase 0.75 -> sin(3PI/2) = -1.0
        assert!((MidiEngine::calculate_lfo(&lfo, 0.75) - -1.0).abs() < 1e-6);
    }

    #[test]
    fn test_designer_interpolation() {
        let mut points = [0.0; 16];
        points[0] = 0.0;
        points[1] = 1.0;
        
        let lfo = LFO {
            shape: LFOShape::Designer(points),
            amount: 1.0,
            speed: 1.0,
            phase: 0.0,
            destination: 0,
        };
        
        // At index 0.0 (Phase 0.0) -> 0.0
        assert!((MidiEngine::calculate_lfo(&lfo, 0.0) - 0.0).abs() < 1e-6);
        
        // At index 0.5 (Phase 0.5/16 = 0.03125) -> Interpolated 0.5
        let phase_mid = 0.5 / 16.0;
        assert!((MidiEngine::calculate_lfo(&lfo, phase_mid) - 0.5).abs() < 1e-6);
        
        // At index 1.0 (Phase 1/16 = 0.0625) -> 1.0
        let phase_one = 1.0 / 16.0;
        assert!((MidiEngine::calculate_lfo(&lfo, phase_one) - 1.0).abs() < 1e-6);
    }
}
