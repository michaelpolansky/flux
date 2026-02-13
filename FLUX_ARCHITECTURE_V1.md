# FLUX_ARCHITECTURE_V1.md

## 1. Executive Summary
**Flux** is a high-performance software sequencer and audio engine designed to emulate the "Elektron Workflow" using Rust. It unifies the disparate triggering logics of the Octatrack (Samples/Slices), Analog Four (Synth/CV), and Tonverk (Bus/Subtracks) into a single, atomic event system.

## 2. Logic Reconciliation & Analysis
To create a unified engine, we must reconcile the following divergent hardware behaviors identified in the manuals:

| Feature | Octatrack MKII | Analog Four MKII | Digitakt II | Tonverk (Superset) | **Flux Solution** |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Trig Types** | Sample, Note, Trigless Trig, Trigless Lock, One Shot | Note, Lock, Note Slide, Param Slide | Note, Lock | Note, Lock, Retrig | **`AtomicTrig` Enum** covering all states. |
| **Track Logic** | 8 Stereo Audio / 8 MIDI | 4 Synth, 1 FX, 1 CV | 16 Audio/MIDI | 16 Audio/MIDI + 4 Bus + Subtracks | **`TrackType` Enum** with Subtrack vectors. |
| **Microtiming** | 1/384 step | Microtiming + Swing | Microtiming | Microtiming + Retrigs | **`u32` offset** relative to step grid. |
| **Modulation** | 3 LFOs, X-Fader Scenes | 2 LFOs, 2 Envelopes | 3 LFOs | 4 LFOs (Voice + FX) | **Mod Matrix** with dynamic slot allocation. |
| **Chaining** | Parts, Arrangements | Chain Mode, Song Mode | Song Mode | Song Mode, Clip Launching | **`Arranger` struct** handling Pattern ptrs. |

### The "Subtrack" Paradigm (Tonverk Influence)
The Tonverk manual introduces **Subtracks** (polyphonic layering within a track). Flux will adopt this. A "Track" is a container; a "Voice" is the execution unit.
*   **Legacy Mode:** 1 Subtrack per Track (Digitakt behavior).
*   **Flux Mode:** Up to 8 Subtracks per Track (Tonverk behavior).

---

## 3. Rust Data Model (`src/engine`)

This data model allows for zero-allocation performance during playback.

### 3.1. The Atomic Step
All sequencer events are flattened into `AtomicStep`.

```rust
// src/engine/domain/sequencer.rs

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrigType {
    None,
    Note,           // Standard Note On
    Lock,           // Parameter change only (Trigless Lock)
    SynthTrigger,   // Trigs Envelope/LFO but no Note (Trigless Trig)
    OneShot,        // Plays once (Yellow trig)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrigCondition {
    pub prob: u8,          // 0-100% Probability
    pub logic: LogicOp,    // A:B, Fill, NEI, PRE, etc.
}

// Optimization: Fixed size array for P-Locks to avoid allocation in audio thread
// Index corresponds to Parameter ID (e.g., 0 = Pitch, 1 = Filter Cutoff)
pub type ParameterLocks = [Option<f32>; 128]; 

#[derive(Clone, Debug)]
pub struct AtomicStep {
    pub trig_type: TrigType,
    pub note: u8,               // MIDI Note (0-127)
    pub velocity: u8,
    pub length: f32,            // Step duration
    pub micro_timing: i8,       // -23 to +23 (1/384th steps)
    pub condition: TrigCondition,
    pub sound_lock: Option<u16>,// Sound Pool ID (Digitakt style)
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
```

### 3.2. Track & Pattern Structure

```rust
// src/engine/domain/structure.rs

pub enum MachineType {
    OneShot,    // Digitakt II
    Werp,       // Digitakt II
    Slice,      // Octatrack
    FmTone,     // Digitone
    Subtractive,// Analog Four
    TonverkBus, // Tonverk
    MidiCC,     // External
}

pub struct Subtrack {
    pub voice_id: usize, // Internal Audio Engine Voice ID
    pub steps: Vec<AtomicStep>, // 16-64 steps
}

pub struct Track {
    pub id: usize,
    pub machine: MachineType,
    pub subtracks: Vec<Subtrack>, // Vector to support Tonverk layering
    pub length: u32,
    pub scale: f32, // 1x, 2x, 1/2x, etc.
}

pub struct Pattern {
    pub tracks: [Track; 16], // 16 Tracks per pattern (Tonverk standard)
    pub bpm: f32,
    pub master_length: u32,
}
```

---

## 4. State Synchronization (The "Flux" Strategy)

To achieve 1ms jitter-free timing while using a web-based UI (Tauri/Leptos), we cannot share memory directly between the UI thread and the Audio thread. We will use a **Triple-Buffer / Command Queue** architecture.

### 4.1. Architecture Diagram
```mermaid
[ UI Thread (Leptos) ]  ---> [ Command Queue (RingBuffer) ] ---> [ Audio Thread (Rust) ]
       ^                                                                |
       |                                                                |
       +---------------- [ State Snapshot (TripleBuffer) ] <------------+
```

### 4.2. Communication Primitives

1.  **Command Queue (UI -> Engine):**
    *   Uses `rtrb` (Wait-free RingBuffer).
    *   Sends extremely lightweight enums.
    *   *Example:* `Command::SetStep(track_id, step_idx, AtomicStep)` or `Command::SetParam(param_id, value)`.

2.  **State Snapshot (Engine -> UI):**
    *   Uses `triple_buffer` (Non-blocking read/write).
    *   The Audio thread pushes a "View" struct (playhead position, current active trigs, peak meters) every audio callback block (approx 2-5ms).
    *   Leptos uses `request_animation_frame` to poll this buffer and update Signals.

3.  **Pattern Persistence (Load/Save):**
    *   **Serialization:** `Pattern` struct derives `serde::Serialize` / `serde::Deserialize`.
    *   **Storage:** `save_pattern(pattern, path)` command writes JSON to disk.
    *   **Restoration:** `load_pattern(path)` reads JSON, deserializes, and updates the Frontend Signal `Pattern`. Use `last_pattern.flux` for auto-load.
    *   **Synchronization:** The Frontend is the "Editor State". When hitting "Play", we will eventually need to push the *entire* Pattern to the Engine (Atomic Swap) or stream changes via Command Queue. For now, the Frontend mimics the Engine state.

### 4.3. The Audio Kernel (cpal)
The Audio Kernel is the "Source of Truth." It advances the sequencer logic.

```rust
// src/engine/kernel.rs
pub struct FluxKernel {
    pub pattern: Pattern,
    pub playhead: f64, // Precise sample-accurate position
    command_consumer: Consumer<Command>,
    snapshot_producer: Producer<Snapshot>,
}

impl FluxKernel {
    pub fn process(&mut self, buffer: &mut [f32]) {
        // 1. Apply UI Commands (Lock-free)
        while let Ok(cmd) = self.command_consumer.pop() {
            self.apply_command(cmd);
        }

        // 2. Advance Sequencer Logic
        // Calculate triggers based on sample_rate and playhead
        // Process Logic (Conditions, Probability)
        
        // 3. Generate Audio (Voice allocation)
        
        // 4. Push Snapshot for UI (Playhead position, active LED indices)
        self.update_snapshot();
    }
}
```

---

## 5. File Roadmap

### Backend (`src-tauri/src/`)

| File | Purpose |
| :--- | :--- |
| `main.rs` | Tauri entry point, initializes audio thread. |
| `engine/mod.rs` | Module definitions. |
| `engine/kernel.rs` | The `cpal` audio callback loop. Real-time safe code ONLY. |
| `engine/sequencer.rs` | Logic for playhead advancement, micro-timing calculation, and Swing. |
| `engine/voice.rs` | DSP logic. Envelopes, LFOs, and sample playback engines. |
| `engine/domain.rs` | `AtomicStep`, `Pattern`, `Track` struct definitions. |
| `engine/sync.rs` | Wrapper for `rtrb` and `triple_buffer` logic. |
| `commands.rs` | Tauri commands callable from Frontend (e.g., `load_sample`, `save_project`). |

### Frontend (`src/`) - Leptos

| File | Purpose |
| :--- | :--- |
| `main.rs` | Mounts the Leptos app. |
| `app.rs` | Root component, Global Context providers (AudioState). |
| `components/sequencer_grid.rs` | The 16-step grid. Renders `AtomicStep` states via Signals. (Implemented as `src/ui/components/grid.rs`) |
| `components/param_page.rs` | The rotary encoder view (FLTR, AMP, LFO). Handles P-Lock logic. (Implemented as `src/ui/components/inspector.rs`) |
| `components/header.rs` | BPM, Transport controls, Global Volume. |
| `stores/project_store.rs` | Local state mirroring the Engine state for immediate UI feedback. |
| `services/audio_bridge.ts` | (WASM Bindings) Polling loop. (Implemented as `src/ui/tauri.rs` for Commands). |

## 6. Implementation Stages

### Phase 1: The Pulse
*   Initialize `cpal` audio stream.
*   Implement `rtrb` command queue.
*   Create a basic metronome in Rust.
*   Leptos UI visualizes the beat (LED flash) via TripleBuffer.

### Phase 2: The Grid
*   Implement `AtomicStep` and `Pattern` structs.
*   Build `SequencerGrid` UI in Leptos.
*   Implement `AddTrig` / `RemoveTrig` commands.
*   Engine logic to trigger a simple Sine wave on Step events.

### Phase 3: The P-Lock
*   Implement `ParameterLocks` array in Rust.
*   UI logic: Holding a trig in `SequencerGrid` updates `ParamPage` context to "Lock Mode".
*   Engine logic: Modulating parameters on a per-step basis during audio generation.

### Phase 4: The Machines
*   Implement the `MachineType` enum logic.
*   Basic Sampler (Digitakt style).
*   Basic Synth (Analog Four style).
*   Tonverk Subtrack routing.

*   Tonverk Subtrack routing.

### Phase 5: Modulation Architecture
*   **LFO Engine**:
    *   **Phase-Sync**: LFOs are calculated based on global transport time (PPQN) to ensure deterministic playback.
    *   **Designer LFO**: A vector of 16 values interpolated linearly (Lerp) to create custom waveforms.
    *   **Routing**: LFOs can target MIDI CCs or internal parameters.
    *   **Data Model**: `LFO` struct with `LFOShape` enum stored in `Track`.

---

**Approved by:** Lead Engineer
**Date:** 2024-05-20
```