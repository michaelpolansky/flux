# FLUX Sequencer Developer Guide

A practical guide for developers joining the FLUX project. This guide focuses on hands-on workflows for building features, understanding the codebase, and contributing effectively.

## Table of Contents

1. [Setup & Installation](#setup--installation)
2. [Project Structure](#project-structure)
3. [Component Development](#component-development)
4. [Adding New Features](#adding-new-features)
5. [State Management Best Practices](#state-management-best-practices)
6. [Code Style & Contributing](#code-style--contributing)

---

## Setup & Installation

### Prerequisites

Before starting development, ensure you have the following installed:

**Required**:
- **Rust** (latest stable via rustup): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js 18+** and npm
- **WASM target**: `rustup target add wasm32-unknown-unknown`

**Platform-Specific**:
- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **Linux**: Build essentials (`sudo apt install build-essential libasound2-dev`)
- **Windows**: Visual Studio Build Tools 2019+

### Initial Setup

```bash
# Clone the repository
git clone <repo-url>
cd flux-app

# Install Node.js dependencies (Tailwind, Tauri CLI)
npm install

# Verify Rust toolchain
rustc --version  # Should be 1.70+

# Verify WASM target installed
rustup target list | grep wasm32-unknown-unknown
```

### Development Workflow

FLUX uses a dual-build system: **Trunk** for the frontend (WASM) and **Tauri** for the backend (native Rust).

#### Option 1: Full Desktop App (Recommended)

Run the complete app with audio engine:

```bash
npm run dev
```

**What happens**:
1. Trunk builds Leptos frontend to WASM
2. Tailwind compiles CSS
3. Tauri builds backend and launches app window
4. Auto-reload on file changes (frontend only)

**Expected output**: Desktop window opens at `http://localhost:1420`

**First build**: Takes 3-5 minutes (subsequent builds: 10-30 seconds)

#### Option 2: Frontend-Only (Browser Preview)

Test UI changes without building the full Tauri app:

```bash
cd /Users/michaelpolansky/Development/flux/flux-app
trunk serve --port 8080
```

**What happens**:
- Leptos frontend builds to WASM
- Serves at `http://localhost:8080`
- Auto-reload on `.rs` file changes
- **Audio features disabled** (shows warning banner)

**Use case**: Rapid UI iteration, component development

#### Option 3: Production Build

```bash
npm run tauri build
```

**Output**:
- macOS: `src-tauri/target/release/bundle/dmg/flux-app.dmg`
- Linux: `src-tauri/target/release/bundle/appimage/flux-app.AppImage`
- Windows: `src-tauri/target/release/bundle/msi/flux-app.msi`

### Verifying Your Setup

After running `npm run dev`, verify the app is working:

1. **Window opens**: App window should appear (dark theme, amber accents)
2. **Grid visible**: 16-step sequencer grid (2 rows × 8 columns)
3. **Console logs**: Terminal shows `Audio engine initialized` (no errors)
4. **Click step**: Clicking a grid step should toggle it (blue = active)
5. **Press Play**: Transport should start (watch playhead move across grid)

**Common Issues**:

| Problem | Solution |
|---------|----------|
| `error: linker 'cc' not found` | Install build tools (Xcode CLI / build-essential) |
| `trunk: command not found` | Run `cargo install trunk` |
| Audio device error on Linux | Install ALSA dev: `sudo apt install libasound2-dev` |
| Tauri build fails on Windows | Install Visual Studio Build Tools with C++ workload |
| Port 1420 already in use | Kill existing process: `lsof -ti:1420 \| xargs kill -9` |

### Development Tools

**Recommended IDE**: VS Code with extensions:
- `rust-analyzer` - Rust language server (auto-complete, goto definition)
- `tauri-apps.tauri-vscode` - Tauri-specific features
- `bradlc.vscode-tailwindcss` - Tailwind CSS IntelliSense

**Debugging**:
- **Frontend logs**: `web_sys::console::log_1(&"message".into());`
- **Backend logs**: `println!("message");` or `eprintln!("error");`
- **Browser DevTools**: Right-click app window → Inspect Element

**Hot Reload Behavior**:
- Frontend (`.rs` in `src/`): Auto-reload (Trunk watches files)
- Backend (`.rs` in `src-tauri/`): Full rebuild required (restart `npm run dev`)
- CSS (`styles.css`): Auto-reload (Tailwind watches file)

---

## Project Structure

Understanding the directory layout is crucial for navigating the codebase.

### High-Level Overview

```
flux-app/
├── src/                    # Frontend (Leptos → WASM)
├── src-tauri/              # Backend (Rust native)
├── style/                  # Tailwind CSS source
├── docs/                   # Architecture & design docs
├── public/                 # Static assets (fonts, icons)
└── dist/                   # Build output (git-ignored)
```

### Frontend Structure (`src/`)

```
src/
├── main.rs                 # WASM entry point, mounts <App/>
├── app.rs                  # Root component, global state, layout
├── ui/
│   ├── components/         # Leptos components
│   │   ├── grid.rs         # 16-step sequencer grid (main UI)
│   │   ├── grid_step.rs    # Individual step button
│   │   ├── inspector.rs    # Track-level parameter controls
│   │   ├── step_inspector.rs # Per-step parameter locks (P-Locks)
│   │   ├── toolbar.rs      # Transport (Play/Stop), Save/Load
│   │   ├── lfo_designer.rs # LFO waveform editor (canvas-based)
│   │   ├── lfo_draw.rs     # LFO drawing logic (16-point waveform)
│   │   ├── step_badge.rs   # Active step indicator (small dot)
│   │   ├── playhead_indicator.rs # Playhead visualization
│   │   └── form_controls.rs # Reusable slider/select components
│   ├── state.rs            # PlaybackState, GridUIState definitions
│   ├── tauri.rs            # Tauri command wrappers (safe_invoke)
│   └── tauri_detect.rs     # Tauri capability detection (desktop vs browser)
├── shared/
│   └── models.rs           # Shared data structures (Pattern, Track, Step, LFO)
└── services/
    └── mod.rs              # (Empty - reserved for future services)
```

**Key Files**:
- `app.rs`: Sets up global context (Pattern, SequencerState, PlaybackState), defines layout
- `grid.rs`: Main sequencer grid, handles step toggling, selection
- `step_inspector.rs`: Parameter lock UI (opens on right-click)
- `toolbar.rs`: Transport controls, BPM input, Save/Load buttons

### Backend Structure (`src-tauri/`)

```
src-tauri/
├── src/
│   ├── main.rs             # Binary entry point (thin wrapper)
│   ├── lib.rs              # Tauri app setup, audio engine init
│   ├── commands.rs         # Tauri command handlers (IPC layer)
│   ├── engine/
│   │   ├── mod.rs          # Module exports
│   │   ├── kernel.rs       # FluxKernel (audio callback, sequencer clock)
│   │   ├── midi_engine.rs  # MIDI clock, LFO output, Note On/Off
│   │   └── domain.rs       # AudioSnapshot, parameter constants
│   └── shared/
│       └── models.rs       # Data model (symlinked to frontend version)
├── Cargo.toml              # Backend dependencies (cpal, rtrb, midir)
├── tauri.conf.json         # Tauri configuration (window size, permissions)
└── build.rs                # Build script (Tauri code generation)
```

**Key Files**:
- `lib.rs`: Application initialization, spawns audio thread, sync thread
- `kernel.rs`: Audio engine core (sequencer logic, synthesis)
- `commands.rs`: Defines `#[tauri::command]` functions (IPC bridge)
- `midi_engine.rs`: MIDI clock and LFO calculation

### Configuration Files

```
flux-app/
├── Cargo.toml              # Workspace config, frontend dependencies
├── package.json            # npm scripts (dev, build, tauri)
├── tailwind.config.js      # Tailwind theme (zinc/amber colors)
├── Trunk.toml              # Trunk build config (WASM bundler)
├── index.html              # HTML template (loads WASM bundle)
└── rust-toolchain.toml     # Pins Rust stable version
```

### Documentation (`docs/`)

```
docs/
├── ARCHITECTURE.md         # System design, lock-free communication
├── DEVELOPER_GUIDE.md      # This file
└── plans/                  # (Future: design documents, RFCs)
```

### Module Organization Principles

1. **Shared Models**: `src/shared/models.rs` is the single source of truth for data structures (Pattern, Track, Step, LFO). It's symlinked to `src-tauri/src/shared/models.rs` to avoid duplication.

2. **Component Modularity**: Each UI component is self-contained in `src/ui/components/`. Components access global state via Leptos context, not prop drilling.

3. **Separation of Concerns**:
   - **Frontend**: UI rendering, user input handling, state derivation
   - **Backend**: Audio processing, MIDI output, file I/O
   - **IPC Layer**: `commands.rs` (backend) and `tauri.rs` (frontend) bridge the gap

4. **Domain-Driven Design**: `engine/` module owns all audio logic, isolated from UI concerns.

---

## Component Development

Learn how to create Leptos components, manage state with signals, and style with Tailwind.

### Creating a New Component

**Example**: Let's build a simple `BpmDisplay` component to show the current BPM.

#### Step 1: Create Component File

```rust
// src/ui/components/bpm_display.rs
use leptos::prelude::*;
use crate::shared::models::Pattern;

#[component]
pub fn BpmDisplay() -> impl IntoView {
    // Access global pattern signal via context
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");

    // Derive BPM value (reactive - updates when pattern changes)
    let bpm = Signal::derive(move || {
        pattern_signal.with(|p| p.bpm)
    });

    view! {
        <div class="flex items-center gap-2 px-4 py-2 bg-zinc-900 rounded-lg">
            <span class="text-zinc-400 text-sm">"BPM:"</span>
            <span class="text-amber-500 text-lg font-bold">
                {move || format!("{:.0}", bpm.get())}
            </span>
        </div>
    }
}
```

#### Step 2: Register Component in Module

```rust
// src/ui/components/mod.rs
pub mod bpm_display;
pub use bpm_display::BpmDisplay;
```

#### Step 3: Use Component in Parent

```rust
// src/app.rs
use crate::ui::components::BpmDisplay;

view! {
    <div class="toolbar">
        <BpmDisplay />
    </div>
}
```

### Component Anatomy

Every Leptos component follows this structure:

```rust
#[component]
pub fn MyComponent(
    // Props (optional)
    label: String,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    // 1. Context access (global state)
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");

    // 2. Derived signals (computed state)
    let is_active = Signal::derive(move || {
        pattern_signal.with(|p| p.tracks.len() > 0)
    });

    // 3. Event handlers
    let handle_click = move |_| {
        web_sys::console::log_1(&"Clicked!".into());
    };

    // 4. View macro (JSX-like template)
    view! {
        <button
            class="px-4 py-2 rounded bg-blue-500 hover:bg-blue-400"
            on:click=handle_click
            disabled=disabled
        >
            {label}
        </button>
    }
}
```

### Working with Signals

Signals are Leptos's reactive primitives. They automatically track dependencies and update the UI when changed.

#### Reading Signals

```rust
// Get current value (clones if T: Clone)
let value = signal.get();

// Borrow value (no clone, use for heavy structs)
signal.with(|value| {
    // Use value here (borrowed, not cloned)
    value.field
});
```

**Best Practice**: Use `.with()` for `Pattern` (16KB+ struct) to avoid expensive clones.

#### Writing Signals

```rust
// Replace entire value
set_signal.set(new_value);

// Mutate in place (preferred)
set_signal.update(|value| {
    value.field = new_field_value;
});
```

**Best Practice**: Use `.update()` for partial changes (more efficient).

#### Derived Signals

Computed values that automatically recompute when dependencies change:

```rust
let track_count = Signal::derive(move || {
    pattern_signal.with(|p| p.tracks.len())
});

// In view:
view! {
    <span>{move || track_count.get()}</span>
}
```

**When to use**:
- Transforming state for display (e.g., formatting, filtering)
- Combining multiple signals
- Expensive computations (cached until dependencies change)

### Context API (Global State)

FLUX uses Leptos context for global state instead of prop drilling.

**Providing Context** (in `app.rs`):

```rust
let (pattern_signal, set_pattern_signal) = signal(Pattern::default());
provide_context(pattern_signal);        // ReadSignal<Pattern>
provide_context(set_pattern_signal);    // WriteSignal<Pattern>
```

**Consuming Context** (in child components):

```rust
let pattern_signal = use_context::<ReadSignal<Pattern>>()
    .expect("Pattern context not found");

let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
    .expect("Pattern write context not found");
```

**Available Contexts in FLUX**:
- `ReadSignal<Pattern>` / `WriteSignal<Pattern>` - Sequencer pattern data
- `SequencerState` - Current step, selected step
- `ReadSignal<PlaybackState>` / `WriteSignal<PlaybackState>` - Playback position, playing state
- `ReadSignal<GridUIState>` / `WriteSignal<GridUIState>` - Grid-specific UI state (Grid component only)

### Styling with Tailwind

FLUX uses Tailwind CSS 4.x with a custom theme. All styles are utility classes.

#### Design Tokens

```css
/* Colors (defined in tailwind.config.js) */
bg-zinc-950         /* Background (dark) */
bg-zinc-900         /* Panels */
bg-zinc-800         /* Inactive buttons */
bg-blue-500         /* Active steps */
bg-amber-500        /* Musical accents (triggers, selections) */
text-zinc-400       /* Muted text */
text-white          /* Primary text */

/* Spacing (4/6/8 unit scale) */
gap-4               /* 1rem (16px) */
gap-6               /* 1.5rem (24px) */
px-4 py-2           /* Button padding */

/* Borders */
rounded-lg          /* 0.5rem radius */
border-zinc-700     /* Subtle borders */
```

#### Common Patterns

**Button (Primary)**:
```rust
view! {
    <button class="px-4 py-2 rounded-lg bg-blue-500 hover:bg-blue-400 transition-colors">
        "Click Me"
    </button>
}
```

**Button (Inactive/Secondary)**:
```rust
view! {
    <button class="px-4 py-2 rounded-lg bg-zinc-800 hover:bg-zinc-700 transition-colors">
        "Secondary"
    </button>
}
```

**Input Field**:
```rust
view! {
    <input
        type="number"
        class="px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-white focus:outline-none focus:ring focus:ring-amber-500"
    />
}
```

**Panel/Container**:
```rust
view! {
    <div class="p-6 bg-zinc-900 rounded-lg border border-zinc-800">
        // Content
    </div>
}
```

#### Dynamic Classes

Use signals to toggle classes based on state:

```rust
let is_active = Signal::derive(move || {
    pattern_signal.with(|p| p.tracks[0].is_active)
});

let button_classes = Signal::derive(move || {
    let base = "px-4 py-2 rounded-lg transition-colors";
    let state = if is_active.get() {
        "bg-blue-500 hover:bg-blue-400"
    } else {
        "bg-zinc-800 hover:bg-zinc-700"
    };
    format!("{} {}", base, state)
});

view! {
    <button class=move || button_classes.get()>
        "Toggle"
    </button>
}
```

**Best Practice**: Use `Signal::derive()` for complex class logic (avoids recomputation on every render).

### Event Handlers

Leptos uses `on:event=handler` syntax for DOM events.

#### Common Events

```rust
// Click
let handle_click = move |_| {
    web_sys::console::log_1(&"Clicked!".into());
};

view! {
    <button on:click=handle_click>"Click"</button>
}

// Input (real-time text changes)
let handle_input = move |ev| {
    let value = event_target_value(&ev);
    set_text.set(value);
};

view! {
    <input on:input=handle_input />
}

// Change (after blur/Enter)
let handle_change = move |ev| {
    let value = event_target_value(&ev);
    set_bpm.set(value.parse().unwrap_or(120.0));
};

view! {
    <input type="number" on:change=handle_change />
}

// Keyboard
let handle_keydown = move |ev: web_sys::KeyboardEvent| {
    if ev.key() == "Enter" {
        submit();
    }
};

view! {
    <input on:keydown=handle_keydown />
}
```

### Calling Backend Commands

Use `safe_invoke()` to send commands to the Tauri backend.

```rust
use crate::ui::tauri::safe_invoke;
use leptos::prelude::*;

let handle_play = move |_| {
    spawn_local(async move {
        match safe_invoke::<()>("set_playback_state", &[true.into()]).await {
            Ok(_) => web_sys::console::log_1(&"Playing".into()),
            Err(e) => web_sys::console::error_1(&format!("Error: {:?}", e).into()),
        }
    });
};

view! {
    <button on:click=handle_play>"Play"</button>
}
```

**Pattern**: Always wrap `safe_invoke()` in `spawn_local()` (async executor for WASM).

### Listening to Backend Events

Use `safe_listen_event()` to subscribe to backend events.

```rust
use crate::ui::tauri::safe_listen_event;
use crate::engine::domain::AudioSnapshot;

// In App.rs (component setup)
safe_listen_event("playback-status", move |event: AudioSnapshot| {
    set_current_step.set(event.current_step % 16);
    set_playback_state.update(|state| {
        state.is_playing = event.is_playing;
        state.current_position = event.current_step % 16;
    });
});
```

**Note**: Event listeners are set up once during component creation (not in event handlers).

### Component Development Checklist

Before submitting a new component:

- [ ] Component follows naming convention (`PascalCase`, descriptive)
- [ ] Uses context API for global state (no prop drilling)
- [ ] Uses `.with()` for reading `Pattern` (avoid clones)
- [ ] Derived signals for computed state (not in view! macro)
- [ ] Tailwind classes follow design system (zinc/amber palette)
- [ ] Event handlers are async-safe (`spawn_local` for IPC)
- [ ] Component exported in `mod.rs`
- [ ] No `console.log` statements in production code

---

## Adding New Features

A step-by-step guide to implementing new functionality in FLUX.

### Feature Development Workflow

**Overview**: Most features involve frontend UI, backend logic, and IPC communication.

**Typical Flow**:
1. Design data model (update `models.rs`)
2. Implement backend logic (audio engine, commands)
3. Create frontend UI (Leptos components)
4. Wire up IPC (Tauri commands, events)
5. Test in both desktop and browser modes
6. Commit changes

### Example: Adding a "Swing" Parameter

Let's walk through adding a swing/shuffle control to the sequencer.

#### Step 1: Update Data Model

```rust
// src/shared/models.rs

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub tracks: Vec<Track>,
    pub bpm: f32,
    pub master_length: u32,
    pub swing_amount: f32,  // NEW: 0.0 = none, 1.0 = max swing
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            tracks: Vec::new(),
            bpm: 120.0,
            master_length: 16,
            swing_amount: 0.0,  // NEW
        }
    }
}
```

**Why here?**: `models.rs` is shared between frontend and backend (single source of truth).

#### Step 2: Implement Backend Logic

```rust
// src-tauri/src/engine/kernel.rs

impl FluxKernel {
    fn process(&mut self, output_buffer: &mut [f32], channels: usize) {
        for frame in output_buffer.chunks_mut(channels) {
            self.step_phase += 1.0;

            // NEW: Apply swing to even-numbered 16th notes
            let swing_offset = if (self.current_step % 2) == 1 {
                self.samples_per_step * 0.1 * self.pattern.swing_amount
            } else {
                0.0
            };

            if self.step_phase >= (self.samples_per_step + swing_offset) {
                self.advance_step();
            }

            // ... (rest of audio processing)
        }
    }
}
```

**Key Concept**: Swing delays even 16th notes slightly, creating a "shuffle" feel.

#### Step 3: Add Tauri Command

```rust
// src-tauri/src/commands.rs

#[tauri::command]
pub fn set_swing_amount(
    swing_amount: f32,
    state: State<AppState>
) -> Result<(), String> {
    // Validate input
    if !(0.0..=1.0).contains(&swing_amount) {
        return Err("Swing amount must be 0.0-1.0".to_string());
    }

    // Send command to audio thread
    let mut producer = state.command_producer.lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    producer.push(AudioCommand::SetSwingAmount(swing_amount))
        .map_err(|_| "Command queue full".to_string())?;

    Ok(())
}

// Add to AudioCommand enum
#[derive(Clone, Debug)]
pub enum AudioCommand {
    // ... (existing commands)
    SetSwingAmount(f32),  // NEW
}
```

**Register Command**:

```rust
// src-tauri/src/lib.rs

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // ... (existing commands)
            commands::set_swing_amount,  // NEW
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Step 4: Create Frontend UI

```rust
// src/ui/components/swing_control.rs

use leptos::prelude::*;
use crate::shared::models::Pattern;
use crate::ui::tauri::safe_invoke;

#[component]
pub fn SwingControl() -> impl IntoView {
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write context not found");

    let swing_amount = Signal::derive(move || {
        pattern_signal.with(|p| p.swing_amount)
    });

    let handle_change = move |ev| {
        let value: f32 = event_target_value(&ev).parse().unwrap_or(0.0);

        // Update frontend state
        set_pattern_signal.update(|p| {
            p.swing_amount = value;
        });

        // Sync to backend
        spawn_local(async move {
            let _ = safe_invoke::<()>("set_swing_amount", &[value.into()]).await;
        });
    };

    view! {
        <div class="flex flex-col gap-2">
            <label class="text-sm text-zinc-400">"Swing Amount"</label>
            <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value=move || swing_amount.get()
                on:input=handle_change
                class="w-full accent-amber-500"
            />
            <span class="text-xs text-zinc-500">
                {move || format!("{:.0}%", swing_amount.get() * 100.0)}
            </span>
        </div>
    }
}
```

#### Step 5: Integrate Component

```rust
// src/ui/components/mod.rs
pub mod swing_control;
pub use swing_control::SwingControl;

// src/ui/components/toolbar.rs (or inspector.rs)
use crate::ui::components::SwingControl;

view! {
    <div class="toolbar">
        // ... (existing controls)
        <SwingControl />  // NEW
    </div>
}
```

#### Step 6: Test

**Desktop Mode**:
1. Run `npm run dev`
2. Adjust swing slider
3. Press Play
4. Listen for swing/shuffle timing (even steps delayed)
5. Verify `swing_amount` persists on Save/Load

**Browser Mode** (UI only):
1. Run `trunk serve`
2. Verify slider renders correctly
3. Check console for errors (backend calls will no-op)

#### Step 7: Commit

```bash
# Stage changes
git add src/shared/models.rs
git add src-tauri/src/engine/kernel.rs
git add src-tauri/src/commands.rs
git add src-tauri/src/lib.rs
git add src/ui/components/swing_control.rs
git add src/ui/components/mod.rs
git add src/ui/components/toolbar.rs  # (or inspector.rs)

# Commit with descriptive message
git commit -m "feat: add swing/shuffle control to sequencer

- Add swing_amount field to Pattern model
- Implement swing timing in audio kernel (delays even 16th notes)
- Create SwingControl component with 0-100% slider
- Add set_swing_amount Tauri command for backend sync

Swing applies to all tracks, adjusts timing of even-numbered steps.
Default is 0% (straight timing).

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

**Commit Message Format**:
```
<type>: <summary>

<body>

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```

**Types**: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

### Feature Complexity Matrix

| Feature Type | Touches | Example | Estimated Time |
|--------------|---------|---------|----------------|
| UI-only | Frontend | Add tooltip, change colors | 1-2 hours |
| Parameter change | Models, Commands, UI | Swing (above example) | 2-4 hours |
| New audio effect | Kernel, Commands, UI | Add reverb | 4-8 hours |
| New component type | Models, Kernel, MIDI, UI | Add drum machine | 1-2 days |
| Architecture change | Full stack | Multi-track support | 3-5 days |

### Testing Checklist

Before committing a feature:

- [ ] Desktop mode: Feature works as expected
- [ ] Browser mode: UI renders (or shows graceful degradation)
- [ ] No console errors in either mode
- [ ] Audio thread doesn't crash (check terminal logs)
- [ ] Pattern saves/loads correctly (if data model changed)
- [ ] Performance: No frame drops (check DevTools FPS counter)
- [ ] Keyboard shortcuts still work (Tab, Space, Esc)
- [ ] Code follows Rust conventions (`cargo clippy` passes)

---

## State Management Best Practices

FLUX uses Leptos signals for reactive state management. This section covers patterns, pitfalls, and performance tips.

### State Architecture Overview

**Three Layers of State**:

1. **Model State** (`Pattern`): The sequencer data (tracks, steps, parameters, LFOs)
2. **Playback State**: Real-time audio engine state (current step, playing/stopped)
3. **UI State**: Ephemeral UI state (hover effects, selected step, recent triggers)

**Data Flow**:

```
User Interaction → Signal Update → Derived Signal → DOM Update
                ↓
          Tauri Command → Audio Thread → State Snapshot → Event → Signal Update
```

For a detailed architecture explanation, see [ARCHITECTURE.md - State Management](ARCHITECTURE.md#state-management).

### Signal Patterns

#### Pattern 1: Reading Heavy Structs (Avoid Clones)

```rust
// ❌ BAD - Clones entire 16KB+ Pattern on every access
let is_active = move || {
    let pattern = pattern_signal.get();  // Clones!
    pattern.tracks[track_idx].steps[step_idx].is_active()
};

// ✅ GOOD - Borrows Pattern, extracts only what's needed
let is_active = Signal::derive(move || {
    pattern_signal.with(|p| {
        p.tracks.get(track_idx)
            .and_then(|t| t.subtracks.get(0))
            .and_then(|st| st.steps.get(step_idx))
            .map(|s| s.trig_type != TrigType::None)
            .unwrap_or(false)
    })
});
```

**Why**: `.with()` borrows the signal value (no heap allocation), `.get()` clones it.

#### Pattern 2: Batch Updates

```rust
// ❌ BAD - Triggers two reactive updates
set_playback_state.set(PlaybackState {
    is_playing: true,
    current_position: 0,
    triggered_tracks: [false; 4],
});
set_playback_state.set(PlaybackState {
    is_playing: true,
    current_position: 1,  // Changed
    triggered_tracks: [true, false, false, false],
});

// ✅ GOOD - Single update, mutate in place
set_playback_state.update(|state| {
    state.is_playing = true;
    state.current_position = 1;
    state.triggered_tracks = [true, false, false, false];
});
```

**Why**: `.update()` batches changes into one reactive notification (fewer DOM updates).

#### Pattern 3: Derived Signals for Computed State

```rust
// ❌ BAD - Recomputes on every render
view! {
    <div class=move || {
        if playback_state.get().is_playing { "playing" } else { "stopped" }
    }>
}

// ✅ GOOD - Cached, only recomputes when playback_state changes
let status_class = Signal::derive(move || {
    if playback_state.get().is_playing { "playing" } else { "stopped" }
});

view! {
    <div class=move || status_class.get()>
}
```

**Why**: Derived signals memoize results (only recompute when dependencies change).

#### Pattern 4: Conditional Rendering

```rust
// ✅ Preferred pattern for conditional UI
view! {
    {move || {
        let selected = sequencer_state.selected_step.get();
        if let Some((track_id, step_idx)) = selected {
            view! {
                <div class="step-inspector">
                    "Editing Track " {track_id + 1} ", Step " {step_idx + 1}
                </div>
            }.into_any()
        } else {
            view! {
                <div class="placeholder">"No step selected"</div>
            }.into_any()
        }
    }}
}
```

**Note**: Use `.into_any()` to unify types in `if/else` branches.

### Frontend ↔ Backend Sync

**Key Principle**: Frontend signals are the source of truth for UI. Backend is the source of truth for audio.

**Pattern**: Always sync both ways:

```rust
let handle_bpm_change = move |ev| {
    let value: f32 = event_target_value(&ev).parse().unwrap_or(120.0);

    // 1. Update frontend state (immediate UI feedback)
    set_pattern_signal.update(|p| {
        p.bpm = value;
    });

    // 2. Sync to backend (asynchronous)
    spawn_local(async move {
        let _ = safe_invoke::<()>("set_bpm", &[value.into()]).await;
    });
};
```

**Why Two Updates?**
- Frontend update: UI reflects change immediately (no lag)
- Backend sync: Audio engine adopts new BPM (eventual consistency)

**Edge Case**: If backend rejects the value (e.g., out of range), it won't update. Frontend is now out of sync. Solution: Listen to backend events to confirm changes.

### Performance Optimization

#### 1. Component-Local Context

Limit reactivity scope by providing context at component level (not globally):

```rust
// In Grid.rs (not App.rs)
let grid_ui_state = signal(GridUIState::default());
provide_context(grid_ui_state.0);  // Only Grid children react
provide_context(grid_ui_state.1);
```

**Why**: Changes to `GridUIState` only trigger re-renders in Grid subtree (not entire app).

#### 2. Avoid Unnecessary Effects

```rust
// ❌ BAD - Creates infinite loop (effect reads and writes same signal)
Effect::new(move |_| {
    let bpm = pattern_signal.with(|p| p.bpm);
    set_pattern_signal.update(|p| p.bpm = bpm * 2.0);  // INFINITE LOOP!
});

// ✅ GOOD - Derive a signal instead
let doubled_bpm = Signal::derive(move || {
    pattern_signal.with(|p| p.bpm * 2.0)
});
```

**Why**: Effects should only perform side effects (logging, DOM manipulation), not state updates.

#### 3. Throttle Event Listeners

```rust
// Backend throttles "playback-status" events (only emit on step change)
// Frontend: No additional throttling needed

safe_listen_event("playback-status", move |event: AudioSnapshot| {
    // Runs at most 16 times per bar (not 60 FPS)
    set_current_step.set(event.current_step % 16);
});
```

**Why**: Backend already throttles (see `src-tauri/src/lib.rs` sync thread).

### Common Pitfalls

#### Pitfall 1: Forgetting to Call `.get()` in View

```rust
// ❌ WRONG - Renders Signal object (not value)
view! {
    <div>{bpm_signal}</div>
}

// ✅ CORRECT - Call .get() to read value
view! {
    <div>{move || bpm_signal.get()}</div>
}
```

#### Pitfall 2: Mutating Signal Without `.update()`

```rust
// ❌ WRONG - Signal value mutated, but Leptos doesn't detect change
pattern_signal.with(|p| {
    p.bpm = 140.0;  // No effect! (read-only borrow)
});

// ✅ CORRECT - Use WriteSignal and .update()
set_pattern_signal.update(|p| {
    p.bpm = 140.0;
});
```

#### Pitfall 3: Cloning in Tight Loops

```rust
// ❌ SLOW - Clones Pattern 16 times
for step_idx in 0..16 {
    let is_active = pattern_signal.get().tracks[0].steps[step_idx].is_active();
}

// ✅ FAST - Borrow once, iterate inside
pattern_signal.with(|p| {
    for step_idx in 0..16 {
        let is_active = p.tracks[0].steps[step_idx].is_active();
    }
});
```

### State Management Checklist

When working with state:

- [ ] Use `.with()` for reading `Pattern` (avoid `.get()`)
- [ ] Use `.update()` for partial changes (avoid `.set()`)
- [ ] Derive signals for computed state (avoid recomputation)
- [ ] Sync frontend and backend on state changes
- [ ] Provide context at narrowest scope (not always globally)
- [ ] Avoid effects for state updates (use derived signals)
- [ ] Test reactivity (change signal, verify UI updates)

---

## Code Style & Contributing

Follow these conventions to maintain code quality and consistency across the project.

### Rust Code Style

FLUX follows standard Rust conventions with some project-specific rules.

#### Naming Conventions

```rust
// Types: PascalCase
struct FluxKernel { }
enum TrigType { }
trait Synthesizer { }

// Functions, variables: snake_case
fn process_audio_buffer() { }
let current_step = 0;

// Constants: SCREAMING_SNAKE_CASE
const SAMPLE_RATE: f32 = 44100.0;
const PARAM_PITCH: usize = 60;

// Modules: snake_case
mod audio_engine;
mod midi_output;
```

#### Formatting

Use `rustfmt` to auto-format code:

```bash
# Format entire project
cargo fmt

# Check formatting without changing files
cargo fmt -- --check
```

**Project Rules** (`.rustfmt.toml`):
- Max line width: 100 characters
- 4-space indentation
- Trailing commas in multi-line arrays/structs

#### Linting

Run `clippy` to catch common mistakes:

```bash
# Check all targets (frontend + backend)
cargo clippy --all-targets

# Auto-fix safe suggestions
cargo clippy --fix
```

**Common Clippy Warnings to Fix**:
- `needless_return`: Remove explicit `return` at end of function
- `redundant_clone`: Use references instead of cloning
- `useless_conversion`: Remove `.into()` when type is already correct

#### Documentation Comments

Use `///` for public APIs, `//` for implementation details:

```rust
/// Processes audio buffer and advances sequencer clock.
///
/// # Arguments
/// * `output_buffer` - Interleaved audio samples (stereo = 2 channels)
/// * `channels` - Number of channels (1 = mono, 2 = stereo)
///
/// # Safety
/// Must be called from real-time audio thread (no allocations).
pub fn process(&mut self, output_buffer: &mut [f32], channels: usize) {
    // Implementation detail: Step phase tracks sub-step position
    self.step_phase += 1.0;
}
```

#### Error Handling

**Frontend** (WASM): Use `Result` with `TauriError`:

```rust
use crate::ui::tauri::{safe_invoke, TauriError};

match safe_invoke::<()>("command", &args).await {
    Ok(_) => { /* success */ },
    Err(TauriError::NotAvailable) => {
        web_sys::console::warn_1(&"Running in browser mode".into());
    },
    Err(e) => {
        web_sys::console::error_1(&format!("Error: {:?}", e).into());
    },
}
```

**Backend** (Tauri): Return `Result<T, String>`:

```rust
#[tauri::command]
pub fn my_command(value: f32) -> Result<(), String> {
    if value < 0.0 {
        return Err("Value must be non-negative".to_string());
    }
    Ok(())
}
```

**Audio Thread**: Use `Option` or panic (no `Result` - can't handle errors in real-time):

```rust
fn get_step(&self, index: usize) -> Option<&AtomicStep> {
    self.pattern.tracks.get(0)?.steps.get(index)
}
```

### Commit Message Format

Use Conventional Commits format:

```
<type>(<scope>): <summary>

<body>

<footer>
```

**Types**:
- `feat`: New feature (e.g., `feat: add swing control`)
- `fix`: Bug fix (e.g., `fix: prevent audio dropouts on buffer underrun`)
- `refactor`: Code restructuring (no behavior change)
- `docs`: Documentation only
- `test`: Add/update tests
- `chore`: Tooling, dependencies (e.g., `chore: update Leptos to 0.7.1`)

**Scope** (optional): Affected module (e.g., `kernel`, `grid`, `midi`)

**Examples**:

```bash
# Simple feature
git commit -m "feat: add BPM input to toolbar"

# Bug fix with body
git commit -m "fix: resolve playhead desync at high BPM

The sequencer clock was accumulating floating-point error,
causing the playhead to drift from the audio engine.

Solution: Reset step_phase on transport stop to prevent
phase accumulation across play sessions."

# Breaking change
git commit -m "refactor!: change Pattern serialization format

BREAKING CHANGE: Old .flux files are incompatible.
Migration guide: Load in v0.1, re-save to upgrade."
```

**Footer**: Always include co-authorship tag:

```
Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```

### Pull Request Process

1. **Create Feature Branch**:
   ```bash
   git checkout -b feat/swing-control
   ```

2. **Make Changes**: Follow development workflow (see [Adding New Features](#adding-new-features))

3. **Test Locally**:
   - Desktop mode: `npm run dev`
   - Browser mode: `trunk serve`
   - Verify no console errors, features work as expected

4. **Format & Lint**:
   ```bash
   cargo fmt
   cargo clippy --fix
   ```

5. **Commit**:
   ```bash
   git add .
   git commit -m "feat: add swing/shuffle control

   - Add swing_amount to Pattern model
   - Implement swing timing in audio kernel
   - Create SwingControl UI component

   Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
   ```

6. **Push Branch**:
   ```bash
   git push -u origin feat/swing-control
   ```

7. **Open Pull Request**:
   - Use GitHub UI or `gh pr create`
   - Fill PR template (summary, test plan, screenshots)
   - Link related issues (e.g., "Closes #42")

8. **Address Review Feedback**:
   - Make requested changes
   - Commit with `fix: address review feedback` (or amend if minor)
   - Push updates

9. **Merge**:
   - Squash commits if multiple small fixes
   - Maintainer merges to `main`

### Code Review Checklist

**For Authors** (before requesting review):

- [ ] Feature works in desktop mode
- [ ] UI renders correctly in browser mode (or degrades gracefully)
- [ ] No new compiler warnings (`cargo build`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Commit messages follow convention
- [ ] PR description includes test plan

**For Reviewers**:

- [ ] Code follows Rust style guide
- [ ] No unnecessary clones or allocations (especially in audio code)
- [ ] Signals used correctly (`.with()` for heavy structs, derived signals for computed state)
- [ ] Tauri commands return `Result<T, String>`
- [ ] Frontend handles errors gracefully (no panics in WASM)
- [ ] Tailwind classes follow design system (zinc/amber palette)
- [ ] Feature tested in both desktop and browser modes

### Common Review Feedback

**Performance**:
- "Use `.with()` instead of `.get()` for Pattern reads" (avoid clones)
- "Move signal derivation outside view! macro" (avoid recomputation)
- "Batch signal updates with `.update()`" (reduce reactive notifications)

**Safety**:
- "Avoid `.unwrap()` in frontend (use `?` or `unwrap_or`)" (prevent WASM panics)
- "No allocations in audio thread" (use pre-allocated buffers)
- "Check ring buffer capacity before push" (handle backpressure)

**Style**:
- "Add doc comments to public functions" (help future developers)
- "Extract magic numbers to constants" (e.g., `const NUM_STEPS: usize = 16`)
- "Use `expect()` with descriptive message" (better than bare `.unwrap()`)

### Contributing Workflow Tips

**Incremental Commits**: Prefer small, focused commits over large "mega commits":

```bash
# Good: Logical progression
git commit -m "feat: add swing_amount to Pattern model"
git commit -m "feat: implement swing timing in audio kernel"
git commit -m "feat: create SwingControl UI component"

# Bad: Everything at once
git commit -m "feat: add swing feature" (changes 10 files, 500 lines)
```

**Draft PRs**: Use GitHub draft PRs for work-in-progress:

```bash
gh pr create --draft --title "WIP: Add swing control"
```

**Rebase Before Merge**: Keep history clean by rebasing on `main`:

```bash
git fetch origin
git rebase origin/main
git push --force-with-lease
```

### Resources

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design, lock-free communication
- [README.md](README.md) - Project overview, quick start
- [Leptos Book](https://leptos.dev/) - Leptos framework guide
- [Tauri Docs](https://tauri.app/) - Tauri API reference

**Rust Resources**:
- [The Rust Book](https://doc.rust-lang.org/book/) - Rust fundamentals
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/) - All Clippy rules

**Audio Programming**:
- [cpal Examples](https://github.com/RustAudio/cpal/tree/master/examples) - Audio I/O patterns
- [rtrb Docs](https://docs.rs/rtrb/latest/rtrb/) - Lock-free ring buffer usage

---

## Getting Help

**Stuck on a bug?**
1. Check console logs (browser DevTools + terminal)
2. Search issues on GitHub
3. Ask in project Discord/Slack (if available)
4. Open GitHub issue with reproduction steps

**Understanding the code?**
1. Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design
2. Trace signal flow from UI → Backend (use `web_sys::console::log_1()`)
3. Read inline comments in `kernel.rs` (audio logic)
4. Ask maintainers for clarification (open discussion issue)

**Contributing for the first time?**
1. Start with "good first issue" label on GitHub
2. Read this guide + ARCHITECTURE.md
3. Set up development environment (see [Setup & Installation](#setup--installation))
4. Make small PR (fix typo, improve docs) to learn workflow

---

**Last Updated**: 2026-02-14
**FLUX Version**: 0.1.0 (Phase 3 Complete)
**For Questions**: Open GitHub issue or contact maintainers
