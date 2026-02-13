# FLUX Sequencer

An Elektron-inspired hardware sequencer built with Rust, Tauri, and Leptos.

## Overview

FLUX is a high-performance software sequencer and audio engine that unifies the workflow of classic Elektron hardware (Octatrack, Analog Four, Digitakt) into a single, modern application. Built with real-time audio processing in Rust and a reactive web-based UI.

## Features

- **16-step sequencer grid** with 2×8 layout for optimal visibility
- **Parameter locking (P-Lock)** - Elektron-style per-step parameter automation
- **Real-time audio engine** with lock-free UI↔Audio communication
- **LFO designer** with custom waveform drawing
- **Professional UI** - Ableton-inspired dark theme with amber accents
- **Keyboard accessible** - Full keyboard navigation with visible focus indicators

## Tech Stack

- **Backend**: Rust + Tauri 2.x
  - `cpal` for cross-platform audio I/O
  - `rtrb` for lock-free ring buffers (UI→Audio commands)
  - `triple_buffer` for lock-free state snapshots (Audio→UI)
- **Frontend**: Leptos 0.7 (Rust WASM framework)
  - Reactive signals for real-time UI updates
  - Tailwind CSS 4.x for styling
  - Trunk for WASM bundling

## Development Setup

### Prerequisites

- Rust (via rustup): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 18+ and npm
- Xcode Command Line Tools (macOS)

### Installation

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Install dependencies
npm install

# Run development server
npm run dev
```

The app will open automatically at `http://localhost:1420`

### Build for Production

```bash
npm run tauri build
```

## Project Structure

```
flux-app/
├── src/                    # Leptos frontend (WASM)
│   ├── app.rs             # Root component, layout
│   ├── ui/components/     # UI components
│   │   ├── grid.rs        # 16-step sequencer grid
│   │   ├── inspector.rs   # Parameter controls
│   │   ├── toolbar.rs     # Transport controls
│   │   └── lfo_designer.rs # LFO waveform editor
│   ├── services/          # Frontend services
│   └── shared/models.rs   # Shared data structures
├── src-tauri/             # Rust backend
│   └── src/
│       ├── engine/        # Audio engine
│       │   ├── kernel.rs  # Audio callback (cpal)
│       │   ├── midi_engine.rs # MIDI processing
│       │   └── domain.rs  # Data structures
│       ├── commands.rs    # Tauri commands
│       └── lib.rs         # App initialization
└── docs/plans/            # Design documents
```

## UI Design

The interface follows an Ableton-inspired design philosophy:

- **Dark theme** (zinc-950 background) for reduced eye strain
- **Amber accents** (amber-500) for active musical elements
- **Blue indicators** (blue-500) for UI selection states
- **Clear hierarchy** - Grid dominates the view as primary focus
- **Consistent spacing** - 6/4/2 unit scale for visual rhythm
- **Professional polish** - Smooth transitions, tactile button feedback

See `docs/plans/2026-02-13-ui-redesign-design.md` for complete design system.

## Architecture Highlights

### Lock-Free Audio↔UI Communication

```
[UI Thread (Leptos)]  →  [Command Queue (rtrb)]  →  [Audio Thread (cpal)]
        ↑                                                     ↓
        ←─────────  [State Snapshot (triple_buffer)]  ←──────┘
```

- **Commands** (UI→Audio): Parameter changes, step toggles
- **Snapshots** (Audio→UI): Current step, playback state
- **Zero allocations** in audio thread for jitter-free timing

### Data Model

All sequencer events are represented as `AtomicStep`:
- Trig type (Note, Lock, Trigless, OneShot)
- MIDI note + velocity
- Micro-timing offset (1/384th precision)
- Parameter locks (128 slots per step)
- Probability + logic conditions

## Current Status

### Completed Phases

- ✅ **Phase 1**: The Pulse - Audio engine heartbeat
- ✅ **Phase 2**: The Grid - 16-step sequencer with real-time sync
- ✅ **Phase 3**: The P-Lock - Per-step parameter automation
- ✅ **UI Redesign**: Professional Ableton-inspired interface

### In Progress

- 🚧 **Phase 4**: The Machines - Sample playback, synthesis engines
- 🚧 **Phase 5**: Modulation - LFO routing and modulation matrix

## Keyboard Shortcuts

- `Space` / `Enter` - Activate buttons, toggle steps
- `Tab` - Navigate between controls
- `Arrow keys` - Adjust sliders
- `Right-click` - Select step for parameter locking
- `Esc` - Close step inspector

## Contributing

See `FLUX_ARCHITECTURE_V1.md` for detailed architecture documentation.

## License

[Your License Here]

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
