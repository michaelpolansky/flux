# FLUX Sequencer Architecture

## Table of Contents

1. [System Overview](#system-overview)
2. [Frontend Architecture](#frontend-architecture)
3. [Backend Architecture](#backend-architecture)
4. [Communication Layer](#communication-layer)
5. [Key Design Decisions](#key-design-decisions)

---

## System Overview

FLUX is a high-performance audio sequencer built with a split architecture: a reactive web-based frontend running in WebAssembly, and a real-time audio backend in native Rust. This separation enables a modern, responsive UI while maintaining deterministic audio processing with microsecond timing precision.

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                          FLUX APPLICATION                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                   FRONTEND (Browser / WebView)                │  │
│  │                                                                │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │              Leptos UI (Rust → WASM)                   │  │  │
│  │  │                                                          │  │  │
│  │  │  • App.rs - Root Component & State Management          │  │  │
│  │  │  • Components (Grid, Inspector, Toolbar, LFO Designer) │  │  │
│  │  │  • Reactive Signals (Current Step, Pattern, Playback)  │  │  │
│  │  │  • Event Handlers (Click, Keyboard, Parameter Changes) │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                              ↕                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │            Tauri IPC Layer (JavaScript Bridge)         │  │  │
│  │  │                                                          │  │  │
│  │  │  • safe_invoke() - Send commands to backend            │  │  │
│  │  │  • safe_listen_event() - Receive state updates         │  │  │
│  │  │  • Error handling & Tauri detection                    │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                 ↕                                    │
│                    Tauri IPC (JSON over IPC)                        │
│                                 ↕                                    │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                 BACKEND (Native Rust / Tauri)                 │  │
│  │                                                                │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │                 Tauri Runtime & Commands               │  │  │
│  │  │                                                          │  │  │
│  │  │  • set_playback_state() - Play/Stop transport          │  │  │
│  │  │  • toggle_step() - Enable/disable step triggers        │  │  │
│  │  │  • set_param_lock() - Set per-step parameter values    │  │  │
│  │  │  • save_pattern() / load_pattern() - File I/O          │  │  │
│  │  │  • set_lfo_shape() / set_lfo_designer_value()          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                              ↕                                 │  │
│  │                   Lock-Free Ring Buffer (rtrb)                 │  │
│  │                     [UI → Audio Commands]                      │  │
│  │                              ↕                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │                Audio Engine (FluxKernel)               │  │  │
│  │  │                                                          │  │  │
│  │  │  • Real-time audio callback (cpal stream)              │  │  │
│  │  │  • Sequencer clock (tempo sync, step advancement)      │  │  │
│  │  │  • Voice synthesis (sine wave, parameter resolution)   │  │  │
│  │  │  • Pattern execution (trigger detection, p-locks)      │  │  │
│  │  │  • Zero-allocation audio processing                    │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                              ↕                                 │  │
│  │              Lock-Free Triple Buffer (triple_buffer)           │  │
│  │                   [Audio → UI State Snapshots]                 │  │
│  │                              ↕                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │                   Sync Thread                          │  │  │
│  │  │                                                          │  │  │
│  │  │  • Polls state snapshots @ 60 FPS                       │  │  │
│  │  │  • Emits "playback-status" events to frontend          │  │  │
│  │  │  • Throttles updates (only emit when step changes)     │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                                                                │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │                MIDI Engine (Separate Thread)           │  │  │
│  │  │                                                          │  │  │
│  │  │  • High-precision MIDI clock (24 PPQN)                  │  │  │
│  │  │  • LFO calculation & CC output                          │  │  │
│  │  │  • Note On/Off message generation                       │  │  │
│  │  │  • Virtual MIDI port (midir)                            │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    AUDIO HARDWARE (cpal)                      │  │
│  │                                                                │  │
│  │  • Cross-platform audio output (CoreAudio/WASAPI/ALSA)       │  │
│  │  • Sample rate: 44.1kHz - 192kHz                              │  │
│  │  • Buffer size: Configurable (typically 256-512 samples)      │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Thread Model

FLUX runs on **4 primary threads**:

1. **Main Thread (Tauri)**: UI window management, IPC message routing, file I/O
2. **Audio Thread (cpal)**: Real-time audio processing, sequencer clock, synthesis
3. **Sync Thread**: Polls audio state snapshots and emits events to frontend
4. **MIDI Thread**: High-precision MIDI clock and message generation

---

## Frontend Architecture

### Technology Stack

- **Leptos 0.7**: Rust-based reactive web framework compiled to WebAssembly
- **WASM**: Runs in browser or Tauri's WebView with near-native performance
- **Tailwind CSS 4.x**: Utility-first styling with custom design tokens
- **wasm-bindgen**: JavaScript interop for Tauri IPC bindings

### Component Hierarchy

```
App (app.rs)
├── Header
│   ├── Title
│   └── Toolbar (Transport Controls)
│       ├── Play/Stop Button
│       ├── Save/Load Buttons
│       └── BPM Control
├── Sequencer Grid Section
│   └── Grid
│       └── GridStep (×16)
│           ├── StepBadge (Active Indicator)
│           └── PlayheadIndicator
├── Parameters Section
│   ├── Inspector (Track Parameters)
│   │   ├── FormControl (Sliders, Selects)
│   │   └── LFO Controls
│   │       └── LFODesigner (Waveform Canvas)
│   └── StepInspector (Per-Step P-Locks)
│       └── ParameterInput (×128 potential slots)
```

### State Management

FLUX uses Leptos's **reactive signals** for state synchronization:

```rust
// Global State (Provided via Context)
struct SequencerState {
    current_step: ReadSignal<usize>,        // Current playback position (0-15)
    selected_step: RwSignal<Option<(usize, usize)>>, // (track_id, step_idx)
}

// Playback State
struct PlaybackState {
    is_playing: bool,                       // Transport state
    current_position: usize,                // Redundant with current_step
    triggered_tracks: [bool; 4],            // Per-track trigger indicators
}

// Pattern State (Shared Data Model)
pattern_signal: ReadSignal<Pattern>         // Full sequencer pattern
set_pattern_signal: WriteSignal<Pattern>    // Pattern mutation
```

**Signal Flow**:

1. User clicks a step → `toggle_step()` Tauri command
2. Audio thread processes command → updates internal pattern
3. Sync thread emits `playback-status` event
4. Frontend event listener updates signals → UI reactively re-renders

### Tauri Detection & Error Handling

The frontend supports **dual-mode execution**:

- **Desktop Mode**: Full Tauri features (audio, file I/O, MIDI)
- **Browser Mode**: Preview UI without audio (shows warning banner)

```rust
// Tauri capability detection (runs once on startup)
pub struct TauriCapabilities {
    pub available: bool,      // Is Tauri API present?
    pub audio_enabled: bool,  // Can use audio features?
    pub events_enabled: bool, // Can listen to backend events?
}

// Safe wrappers (no-op in browser mode)
safe_invoke(cmd, args) -> Result<JsValue, TauriError>
safe_listen_event(event, callback) // Silent failure if unavailable
```

**Error Handling Strategy**:

- Desktop mode: Propagate errors to user (e.g., "Failed to save file")
- Browser mode: Silently disable features, show banner

### Event Listeners

Frontend listens to backend events via `safe_listen_event()`:

```rust
// Playback state updates (60 FPS max)
safe_listen_event("playback-status", move |event: AudioSnapshot| {
    set_current_step.set(event.current_step % 16);
    set_playback_state.update(|state| {
        state.is_playing = event.is_playing;
        state.triggered_tracks = event.triggered_tracks.unwrap_or([false; 4]);
    });
});
```

---

## Backend Architecture

### Technology Stack

- **Tauri 2.x**: Cross-platform desktop framework (WebView + Rust backend)
- **cpal**: Cross-platform audio I/O (CoreAudio, WASAPI, ALSA, JACK)
- **rtrb**: Lock-free ring buffer for UI→Audio commands (SPSC queue)
- **triple_buffer**: Lock-free state snapshots for Audio→UI (wait-free reads)
- **midir**: Cross-platform MIDI I/O library
- **thread-priority**: Set real-time thread scheduling

### Module Structure

```
src-tauri/src/
├── lib.rs              # Application entry point, Tauri setup
├── commands.rs         # Tauri command handlers (IPC layer)
├── engine/
│   ├── mod.rs          # Engine module exports
│   ├── kernel.rs       # FluxKernel (audio callback + sequencer)
│   ├── midi_engine.rs  # MIDI clock & LFO processing
│   └── domain.rs       # AudioSnapshot, parameter constants
└── shared/
    └── models.rs       # Data models (Pattern, Track, Step, LFO)
```

### FluxKernel (Audio Engine)

The `FluxKernel` is the heart of the audio system, running in the **cpal audio callback** (real-time thread):

```rust
pub struct FluxKernel {
    // Sequencer State
    pattern: Pattern,               // Active pattern (tracks, steps, machines)
    is_playing: bool,               // Transport state
    current_step: usize,            // Current step (0-15)
    step_phase: f32,                // Sub-step phase (sample counter)
    samples_per_step: f32,          // Clock resolution (derived from BPM)

    // Audio State
    playhead_sample: usize,         // Sample counter for voice envelopes
    current_frequency: f32,         // Active voice pitch (Hz)
    current_decay: f32,             // Envelope decay rate

    // Communication Channels
    command_consumer: Consumer<AudioCommand>,    // UI→Audio commands
    snapshot_producer: Input<AudioSnapshot>,     // Audio→UI state

    // Configuration
    sample_rate: f32,               // Audio device sample rate (44.1kHz etc.)
    tempo: f32,                     // BPM (beats per minute)
}
```

**Audio Processing Loop** (`process()` method):

```rust
fn process(&mut self, output_buffer: &mut [f32], channels: usize) {
    // 1. Process Commands (Lock-Free Pop)
    while let Ok(cmd) = self.command_consumer.pop() {
        match cmd {
            AudioCommand::Play => self.is_playing = true,
            AudioCommand::Stop => { /* reset state */ },
            AudioCommand::ToggleStep(track, step) => { /* modify pattern */ },
            AudioCommand::SetParamLock(track, step, param, val) => { /* apply p-lock */ },
        }
    }

    // 2. Audio Generation (Per-Sample Loop)
    for frame in output_buffer.chunks_mut(channels) {
        // Advance sequencer clock
        self.step_phase += 1.0;

        // Step boundary detection
        if self.step_phase >= self.samples_per_step {
            self.current_step = (self.current_step + 1) % 16;

            // Check for trigger
            if let Some(step) = pattern.get_step(self.current_step) {
                if step.trig_type != TrigType::None {
                    // Resolve pitch (P-Lock override or default note)
                    let note = step.p_locks[PARAM_PITCH].unwrap_or(step.note as f32);
                    self.current_frequency = midi_to_freq(note);

                    // Trigger envelope
                    self.playhead_sample = 0;
                }
            }
        }

        // Synthesize audio (simple sine wave)
        let t = self.playhead_sample as f32 / self.sample_rate;
        let sample = (t * self.current_frequency * 2.0 * PI).sin() * 0.1;

        // Write to output buffer (all channels)
        for out in frame.iter_mut() {
            *out = sample;
        }

        self.playhead_sample += 1;
    }

    // 3. Update State Snapshot (Lock-Free Write)
    self.snapshot_producer.write(AudioSnapshot {
        current_step: self.current_step,
        is_playing: self.is_playing,
    });
}
```

**Key Properties**:

- **Zero allocations**: All buffers pre-allocated during initialization
- **Deterministic timing**: No locks, no syscalls, no conditional branches in hot path
- **Sample-accurate clock**: Step transitions occur at exact sample boundaries

### MIDI Engine

The `MidiEngine` runs on a **separate high-priority thread** with a spin-loop scheduler:

```rust
pub struct MidiEngine {
    midi_out: MidiOutputConnection,         // Virtual MIDI port
    command_consumer: Consumer<EngineCommand>, // UI→MIDI commands
    pattern: Option<Pattern>,               // Current pattern
    ppqn: u32,                              // Pulses per quarter note (24)
    bpm: f32,                               // Tempo
}
```

**Clock Loop**:

```rust
fn run(&mut self) {
    let mut next_tick_time = Instant::now();
    let tick_duration = Duration::from_secs_f32(60.0 / (self.bpm * 24.0));

    loop {
        // 1. Process Commands (Pattern updates, LFO changes)
        while let Ok(cmd) = self.command_consumer.pop() { /* ... */ }

        // 2. High-Precision Sleep
        thread::sleep(wait_time - Duration::from_millis(1));
        while Instant::now() < next_tick_time {
            spin_loop(); // Active wait for final microsecond precision
        }

        // 3. Process MIDI Events
        if tick_count % 6 == 0 {  // 16th note (24 PPQN / 4)
            send_note_on(...);
            send_note_off(...);
        }

        // 4. LFO Output (Every Tick)
        for lfo in &pattern.lfos {
            let value = calculate_lfo(lfo, global_phase);
            send_cc(lfo.destination, value);
        }

        tick_count += 1;
        next_tick_time += tick_duration;
    }
}
```

**LFO Calculation**:

```rust
fn calculate_lfo(lfo: &LFO, global_phase: f32) -> f32 {
    let phase = (global_phase * lfo.speed + lfo.phase) % 1.0;

    let raw = match &lfo.shape {
        LFOShape::Sine => (phase * 2.0 * PI).sin(),
        LFOShape::Triangle => { /* ... */ },
        LFOShape::Square => if phase < 0.5 { 1.0 } else { -1.0 },
        LFOShape::Designer(points) => {
            // Linear interpolation between 16 user-drawn points
            let idx = (phase * 16.0) as usize;
            let frac = phase * 16.0 - idx as f32;
            points[idx] + (points[(idx + 1) % 16] - points[idx]) * frac
        },
    };

    raw * lfo.amount  // Scale by amount (-1.0 to 1.0)
}
```

### Sync Thread

The **sync thread** bridges the audio engine and frontend by polling state snapshots:

```rust
thread::spawn(move || {
    let mut last_step = 999;
    loop {
        // Read latest state (wait-free, always succeeds)
        let snapshot = snapshot_consumer.read();

        // Throttle updates (only emit on step changes)
        if snapshot.current_step != last_step {
            app_handle.emit("playback-status", snapshot);
            last_step = snapshot.current_step;
        }

        thread::sleep(Duration::from_millis(16)); // 60 FPS polling
    }
});
```

**Design Rationale**:

- Audio thread cannot emit Tauri events directly (requires mutex on AppHandle)
- Sync thread decouples audio processing from IPC overhead
- 60 FPS polling is imperceptible to users, allows audio thread to stay real-time

---

## Communication Layer

### UI → Backend (Commands)

The frontend sends commands via **Tauri's IPC** (JSON-RPC over WebView bridge):

```rust
// Frontend (Leptos WASM)
safe_invoke("toggle_step", serde_wasm_bindgen::to_value(&ToggleStepArgs {
    track_id: 0,
    step_idx: 4,
})?).await?;

// Backend (Tauri Command Handler)
#[tauri::command]
pub fn toggle_step(track_id: usize, step_idx: usize, state: State<AppState>)
    -> Result<(), String>
{
    let mut producer = state.command_producer.lock()?;
    producer.push(AudioCommand::ToggleStep(track_id, step_idx))?;
    Ok(())
}
```

**Command Types** (`AudioCommand` enum):

- `Play` / `Stop`: Transport control
- `SetGlobalVolume(f32)`: Master volume
- `ToggleStep(track, step)`: Enable/disable step trigger
- `SetParamLock(track, step, param, value)`: Set per-step parameter override

**Error Handling**:

- Tauri commands return `Result<T, String>`
- Frontend propagates errors to user (e.g., "Queue full" if ring buffer saturated)
- Desktop mode: Show error dialog
- Browser mode: Commands are no-ops (safe_invoke returns `TauriError::NotAvailable`)

### Backend → UI (Events)

The backend emits events via **Tauri's event system**:

```rust
// Backend (Sync Thread)
app_handle.emit("playback-status", AudioSnapshot {
    current_step: 7,
    is_playing: true,
})?;

// Frontend (Leptos Effect)
safe_listen_event("playback-status", move |event: AudioSnapshot| {
    set_current_step.set(event.current_step % 16);
});
```

**Event Types**:

- `playback-status`: Current step, play state, triggered tracks (60 FPS max)

### Lock-Free Queues

**Why Lock-Free?**

Audio threads must never block. Traditional mutexes can cause **priority inversion** (low-priority thread holds lock, high-priority audio thread waits).

**rtrb (Ring Buffer)**:

- **SPSC**: Single Producer (UI), Single Consumer (Audio)
- **Fixed capacity**: 1024 slots (pre-allocated at startup)
- **Wait-free writes**: `push()` never blocks (returns error if full)
- **Wait-free reads**: `pop()` never blocks (returns `None` if empty)

```rust
// Setup (in lib.rs)
let (audio_producer, audio_consumer) = RingBuffer::new(1024);

// UI Thread (Commands)
let mut producer = state.command_producer.lock().unwrap();
producer.push(AudioCommand::Play)?;

// Audio Thread (Kernel.process())
while let Ok(cmd) = self.command_consumer.pop() {
    match cmd { /* ... */ }
}
```

**triple_buffer (State Snapshot)**:

- **SPSC**: Single Writer (Audio), Single Reader (Sync Thread)
- **Triple buffering**: Audio writes to back buffer while sync reads from front buffer
- **Wait-free**: Writer never waits, reader always gets latest complete state
- **No tearing**: Reader never sees partially-written state

```rust
// Setup (in lib.rs)
let (snapshot_producer, mut snapshot_consumer) =
    TripleBuffer::new(&AudioSnapshot::default()).split();

// Audio Thread (Kernel.process())
self.snapshot_producer.write(AudioSnapshot {
    current_step: self.current_step,
    is_playing: self.is_playing,
});

// Sync Thread
let snapshot = snapshot_consumer.read();  // Always succeeds, no blocking
```

---

## Key Design Decisions

### 1. Split Frontend/Backend Architecture

**Decision**: Use Tauri (WebView + Rust backend) instead of a native GUI framework (e.g., iced, egui).

**Rationale**:

- **Web technologies**: Leverage CSS/Tailwind for rapid UI iteration
- **Reactive framework**: Leptos provides fine-grained reactivity similar to SolidJS/Svelte
- **Developer ergonomics**: Designers can work on UI without Rust knowledge
- **Cross-platform**: Single codebase for macOS, Windows, Linux

**Trade-offs**:

- IPC overhead: ~100μs latency for Tauri commands (acceptable for control changes)
- Larger binary: WebView adds ~50MB to app size
- No direct memory sharing: Must serialize data across IPC boundary

### 2. Lock-Free Audio Architecture

**Decision**: Use lock-free queues (rtrb + triple_buffer) instead of mutexes for audio communication.

**Rationale**:

- **Real-time safety**: Audio thread never blocks, even if UI thread is busy
- **Bounded latency**: Command processing has deterministic worst-case time
- **No priority inversion**: Audio thread cannot be blocked by lower-priority threads

**Trade-offs**:

- Fixed queue capacity: Commands can be dropped if UI floods the queue (1024 slots)
- Code complexity: Lock-free data structures are harder to reason about
- Debugging difficulty: No TSAN support for Rust atomics (manual verification required)

### 3. Separate MIDI Thread

**Decision**: Run MIDI clock on a dedicated thread instead of in the audio callback.

**Rationale**:

- **Timing precision**: MIDI clock requires 24 PPQN (every 20ms at 120 BPM), audio runs at 44.1kHz (samples every 22μs) — different time scales
- **Jitter tolerance**: MIDI has ~1ms latency tolerance, audio has ~10μs tolerance
- **LFO independence**: LFOs can output CC messages every tick without blocking audio synthesis

**Trade-offs**:

- Clock drift: MIDI and audio clocks are not sample-synchronized (potential for timing skew)
- Thread overhead: Extra thread consumes ~1MB RAM + CPU scheduling overhead
- Coordination complexity: Pattern updates must be synchronized across both threads

### 4. Shared Data Model (Rust Structs)

**Decision**: Define `Pattern`, `Track`, `Step` models in shared Rust module, serialize to JSON for IPC.

**Rationale**:

- **Type safety**: Same struct definitions in frontend and backend (compile-time validation)
- **Serialization**: Serde handles JSON conversion automatically
- **Zero-copy backend**: Audio thread accesses pattern data directly (no deserialization in hot path)

**Trade-offs**:

- Frontend/backend coupling: Changes to data model require recompiling WASM
- IPC overhead: Full pattern serialization can be 10KB+ (expensive for frequent updates)
- Versioning: No built-in migration strategy for saved pattern files

### 5. Parameter Lock Array (Fixed Size)

**Decision**: Use `[Option<f32>; 128]` for parameter locks instead of `HashMap<u8, f32>`.

**Rationale**:

- **Zero allocations**: Fixed-size array lives on stack (no heap allocations in audio thread)
- **Constant-time access**: `p_locks[PARAM_PITCH]` is O(1), HashMap is O(log n)
- **Cache efficiency**: Contiguous memory layout improves CPU cache hit rate

**Trade-offs**:

- Memory overhead: 512 bytes per step (16 steps × 512 bytes = 8KB per track), even if most slots unused
- MIDI-centric design: 128 slots match MIDI CC count (0-127), but limits extensibility
- Sparse data: Most steps have 0-2 active p-locks, wasting 99% of array space

### 6. Single-Track MVP

**Decision**: Current implementation hardcodes Track 0, Subtrack 0 (one synthesizer voice).

**Rationale**:

- **Phase 1-3 scope**: Prove audio engine architecture before adding multi-track complexity
- **Simplified UI**: Avoids track selection, mixer, routing UI
- **Faster iteration**: Add voice allocation system later without refactoring core engine

**Trade-offs**:

- Not production-ready: Elektron sequencers typically have 8-16 tracks
- Architectural assumptions: Voice allocation, polyphony, track selection not yet designed
- Data model mismatch: `Pattern` has `Vec<Track>`, but code only uses `tracks[0]`

### 7. Tauri Detection for Browser Compatibility

**Decision**: Support both Tauri (desktop) and browser (preview) modes via runtime detection.

**Rationale**:

- **Development workflow**: Preview UI changes in browser without compiling Tauri app
- **Graceful degradation**: Show warning banner instead of crashing in browser
- **Testing**: Unit test UI components in headless browser (Playwright, WebDriver)

**Trade-offs**:

- Dual code paths: All Tauri calls must use `safe_invoke()` wrapper (potential for bugs)
- Feature parity: Browser mode cannot test audio functionality (limited value for real testing)
- Maintenance burden: Must test both modes on every change

### 8. Sync Thread Polling (60 FPS)

**Decision**: Poll audio state snapshots at 60 FPS instead of 120 BPM (2 Hz) for event emission.

**Rationale**:

- **UI smoothness**: 60 FPS matches display refresh rate (smoother animations)
- **Step change detection**: Only emit events when step changes (throttling avoids spam)
- **Low overhead**: Reading triple buffer is wait-free (~10 CPU cycles)

**Trade-offs**:

- Wasted cycles: At 120 BPM (2 steps/sec), we poll 30× more often than necessary
- Event latency: Up to 16ms delay between step change and UI update (imperceptible)
- Battery impact: Constant polling prevents CPU from sleeping (mitigated by throttling)

### 9. Audio Engine in Main Thread (Not Separate Process)

**Decision**: Run audio callback in a thread within the Tauri process, not a separate IPC-connected process.

**Rationale**:

- **Simplicity**: No need for inter-process communication (sockets, shared memory)
- **Latency**: Function calls are faster than IPC (nanoseconds vs microseconds)
- **Shared memory**: Pattern data can be accessed directly by audio thread

**Trade-offs**:

- Process isolation: UI crash takes down audio engine (no fault tolerance)
- Priority scheduling: Cannot use real-time process priority on Linux (requires separate process)
- Resource contention: UI and audio compete for CPU cache (mitigated by thread affinity)

### 10. cpal for Audio I/O (Not PortAudio/JACK)

**Decision**: Use `cpal` as audio abstraction layer instead of `portaudio`, `jack`, or platform-specific APIs.

**Rationale**:

- **Pure Rust**: No C/C++ dependencies (easier cross-compilation, better error messages)
- **Cross-platform**: Supports CoreAudio (macOS), WASAPI (Windows), ALSA (Linux)
- **Active maintenance**: Part of RustAudio ecosystem, well-documented

**Trade-offs**:

- Feature gaps: No ASIO support on Windows (higher latency than pro audio apps)
- Stability: Younger than PortAudio (occasional platform-specific bugs)
- Ecosystem: Fewer examples/tutorials compared to established C libraries

---

## Future Architectural Considerations

### Potential Enhancements

1. **Voice Allocation System**: Pool of voices for polyphonic playback (track-per-voice architecture)
2. **State Diffing**: Only send changed parameters over IPC (reduce serialization overhead)
3. **WASM Audio Worklets**: Move audio processing to browser AudioWorklet for browser-based mode
4. **Sample-Accurate MIDI**: Synchronize MIDI and audio clocks via shared sample counter
5. **Plugin System**: VST/CLAP host for third-party synthesizers and effects
6. **Undo/Redo**: Persistent command log for pattern editing history
7. **Real-Time Process Priority**: Elevate audio thread priority via OS APIs (Linux RT kernel)

### Known Limitations

1. **No Multi-Track Support**: Only one track/voice implemented (MVP constraint)
2. **No Sample Playback**: Only sine wave synthesis (Machines phase not implemented)
3. **Fixed Pattern Length**: Hardcoded to 16 steps (no odd time signatures)
4. **No Undo/Redo**: Parameter changes are not reversible
5. **No Audio Effects**: No reverb, delay, filters (DSP pipeline not implemented)
6. **No Project Files**: Can only save/load individual patterns (no arrangements)

---

## Getting Started

For developers joining the project:

1. **Read this document** to understand the system architecture
2. **Review `README.md`** for setup instructions and project overview
3. **Explore `src/app.rs`** to see the UI component hierarchy
4. **Study `src-tauri/src/engine/kernel.rs`** to understand the audio engine
5. **Check `src/shared/models.rs`** for the data model definitions
6. **Run `npm run dev`** to start the development server
7. **Use `web_sys::console::log_1()` for frontend debugging**
8. **Use `println!()` for backend debugging** (logs appear in terminal)

For detailed development workflows, see `DEVELOPER_GUIDE.md` (coming soon).

---

**Last Updated**: 2026-02-13
**FLUX Version**: 0.1.0 (Phase 3 Complete)
