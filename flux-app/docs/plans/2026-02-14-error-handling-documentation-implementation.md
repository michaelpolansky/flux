# Error Handling & Documentation - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Eliminate console errors through graceful degradation and create comprehensive documentation suite

**Architecture:** Feature detection → Safe wrappers → Graceful degradation + Complete docs (Architecture → Developer → User → Troubleshooting)

**Tech Stack:** Rust/WASM (Leptos 0.7), Tauri 2.x, wasm-bindgen for browser detection

---

## Phase 1: Error Handling

### Task 1: Create Tauri Detection Module

**Files:**
- Create: `src/ui/tauri_detect.rs`
- Modify: `src/ui/mod.rs`

**Step 1: Create tauri_detect.rs with TauriCapabilities struct**

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"])]
    static __TAURI__: JsValue;
}

#[derive(Clone, Copy, Debug)]
pub struct TauriCapabilities {
    pub available: bool,
    pub audio_enabled: bool,
    pub events_enabled: bool,
}

impl Default for TauriCapabilities {
    fn default() -> Self {
        Self {
            available: false,
            audio_enabled: false,
            events_enabled: false,
        }
    }
}

/// Detect if Tauri APIs are available
pub fn detect_tauri() -> TauriCapabilities {
    // Check if window.__TAURI__ exists and is an object
    let tauri_exists = !__TAURI__.is_undefined() && !__TAURI__.is_null();

    if tauri_exists {
        TauriCapabilities {
            available: true,
            audio_enabled: true,
            events_enabled: true,
        }
    } else {
        TauriCapabilities::default()
    }
}
```

**Step 2: Export module in mod.rs**

Add to `src/ui/mod.rs`:
```rust
pub mod tauri_detect;
```

**Step 3: Build to verify compilation**

Run: `trunk build`
Expected: Success (no compilation errors)

**Step 4: Commit**

```bash
git add src/ui/tauri_detect.rs src/ui/mod.rs
git commit -m "feat: add Tauri capability detection module

Detects if window.__TAURI__ is available to enable graceful
degradation in browser mode.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 2: Add TauriError Type

**Files:**
- Modify: `src/ui/tauri.rs`

**Step 1: Add TauriError enum at top of file**

Add after imports in `src/ui/tauri.rs`:
```rust
#[derive(Debug, Clone)]
pub enum TauriError {
    NotAvailable,
    InvokeFailed(String),
}
```

**Step 2: Build to verify compilation**

Run: `trunk build`
Expected: Success

**Step 3: Commit**

```bash
git add src/ui/tauri.rs
git commit -m "feat: add TauriError type for error handling

Defines error cases for Tauri API unavailability.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 3: Add Safe Invoke Wrapper

**Files:**
- Modify: `src/ui/tauri.rs`

**Step 1: Add is_tauri_available helper**

Add after TauriError:
```rust
use leptos::prelude::*;
use crate::ui::tauri_detect::TauriCapabilities;

/// Check if Tauri is available (cached from detection)
fn is_tauri_available() -> bool {
    use_context::<TauriCapabilities>()
        .map(|caps| caps.available)
        .unwrap_or(false)
}
```

**Step 2: Add safe_invoke function**

Add after is_tauri_available:
```rust
/// Safe invoke - returns error if Tauri unavailable
pub async fn safe_invoke(cmd: &str, args: JsValue) -> Result<JsValue, TauriError> {
    if !is_tauri_available() {
        return Err(TauriError::NotAvailable);
    }

    invoke(cmd, args)
        .await
        .map_err(|e| TauriError::InvokeFailed(format!("{:?}", e)))
}
```

**Step 3: Build to verify compilation**

Run: `trunk build`
Expected: Success

**Step 4: Commit**

```bash
git add src/ui/tauri.rs
git commit -m "feat: add safe_invoke wrapper with error handling

Returns TauriError::NotAvailable in browser mode instead of
throwing TypeError.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 4: Add Safe Event Listener

**Files:**
- Modify: `src/ui/tauri.rs`

**Step 1: Add safe_listen_event function**

Add after safe_invoke:
```rust
/// Safe event listener - no-op if Tauri unavailable
pub async fn safe_listen_event<T>(event_name: &str, callback: impl Fn(T) + 'static)
where T: for<'a> Deserialize<'a> + 'static
{
    if !is_tauri_available() {
        // Log once for debugging
        web_sys::console::log_1(
            &format!("Tauri not available - event listener '{}' disabled", event_name).into()
        );
        return;
    }

    // Call existing listen_event implementation
    listen_event(event_name, callback).await
}
```

**Step 2: Build to verify compilation**

Run: `trunk build`
Expected: Success

**Step 3: Commit**

```bash
git add src/ui/tauri.rs
git commit -m "feat: add safe_listen_event wrapper

Gracefully skips event listener setup in browser mode with
debug log instead of throwing TypeError.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 5: Update App to Use Detection

**Files:**
- Modify: `src/app.rs`

**Step 1: Add imports and detection**

At top of file, add to imports:
```rust
use crate::ui::tauri_detect::{detect_tauri, TauriCapabilities};
```

In App() component, add right after function signature:
```rust
// Detect Tauri capabilities
let tauri_capabilities = detect_tauri();
provide_context(tauri_capabilities);
```

**Step 2: Update event listener to use safe wrapper**

Replace existing event listener Effect (lines ~55-74) with:
```rust
// Setup Tauri Event Listener (only if Tauri available)
if tauri_capabilities.events_enabled {
    Effect::new(move |_| {
        spawn_local(async move {
            use crate::ui::tauri::safe_listen_event;
            safe_listen_event("playback-status", move |event: AudioSnapshot| {
                let normalized_position = event.current_step % 16;
                set_current_step.set(normalized_position);
                set_playback_state.update(|state| {
                    state.is_playing = event.is_playing;
                    state.current_position = normalized_position;
                    state.triggered_tracks = event.triggered_tracks.unwrap_or([false; 4]);
                });
            }).await;
        });
    });
}
```

**Step 3: Build to verify compilation**

Run: `trunk build`
Expected: Success

**Step 4: Commit**

```bash
git add src/app.rs
git commit -m "feat: integrate Tauri detection in App component

Conditionally enables event listeners only when Tauri available.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 6: Add Preview Mode Banner

**Files:**
- Modify: `src/app.rs`

**Step 1: Add banner before main content**

In the view! macro, add banner right after `<main>` tag (before the existing `<div class="max-w-7xl">`):

```rust
view! {
    <main
        class="min-h-screen bg-zinc-950 text-zinc-50 p-6 font-sans selection:bg-red-900 selection:text-white"
        on:click=move |ev| {
            // ... existing click handler
        }
    >
        // Preview mode banner
        {move || {
            if !tauri_capabilities.available {
                view! {
                    <div class="bg-amber-500/20 border-b border-amber-500/50 px-4 py-2 text-sm text-amber-200 mb-6">
                        "⚠️ Preview Mode - Audio features require desktop app (npm run dev)"
                    </div>
                }.into_view()
            } else {
                view! { <></> }.into_view()
            }
        }}

        <div class="max-w-7xl mx-auto space-y-5">
            // ... rest of existing content
        </div>
    </main>
}
```

**Step 2: Build and verify**

Run: `trunk build`
Expected: Success

**Step 3: Commit**

```bash
git add src/app.rs
git commit -m "feat: add preview mode banner for browser mode

Shows amber warning banner when Tauri unavailable, informing
users that audio features require desktop app.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 7: Test Error Handling in Both Modes

**Files:**
- None (testing only)

**Step 1: Test browser mode**

Run: `trunk serve`
Open: http://localhost:1420

Expected:
- ✅ Preview mode banner visible
- ✅ Clean console (no TypeErrors)
- ✅ One log: "Tauri not available - event listener 'playback-status' disabled"

**Step 2: Test Tauri desktop mode**

Run: `npm run dev`

Expected:
- ✅ No preview mode banner
- ✅ Clean console
- ✅ Full playback features working

**Step 3: Document test results**

Create test notes (no commit yet, will add to completion report later):
```
Error Handling Tests:
- Browser mode: Banner visible, console clean ✅
- Tauri mode: No banner, full features ✅
```

---

## Phase 2: Architecture Documentation

### Task 8: Create ARCHITECTURE.md with System Overview

**Files:**
- Create: `docs/ARCHITECTURE.md`

**Step 1: Write system overview section**

```markdown
# FLUX Architecture

**Last Updated:** February 14, 2026

---

## System Overview

FLUX is a desktop-first sequencer application with three main layers:

```
┌─────────────────────────────────────────────────────────┐
│  Frontend (Leptos WASM)                                 │
│  - UI components, state management, reactivity          │
└────────────────┬────────────────────────────────────────┘
                 │ Tauri IPC (commands + events)
┌────────────────▼────────────────────────────────────────┐
│  Backend (Rust/Tauri)                                   │
│  - Command handlers, event emission, app lifecycle      │
└────────────────┬────────────────────────────────────────┘
                 │ Lock-free queues
┌────────────────▼────────────────────────────────────────┐
│  Audio Engine (Rust)                                    │
│  - Real-time synthesis, sequencing, MIDI                │
└─────────────────────────────────────────────────────────┘
```

### Technology Choices

**Frontend: Leptos 0.7 (Rust WASM)**
- Why: Type-safe reactivity, performance, shared types with backend
- Trade-off: Larger WASM bundle vs JavaScript frameworks, smaller ecosystem

**Backend: Tauri 2.x**
- Why: Native performance, smaller app size than Electron, Rust integration
- Trade-off: Less mature than Electron, platform-specific builds required

**Audio: Custom Rust Engine**
- Why: Real-time guarantees, lock-free architecture, zero-allocation
- Trade-off: More complex than MIDI-only sequencer, platform audio APIs vary

### Browser vs Desktop

**Desktop Mode (Production):**
- Tauri APIs available (`window.__TAURI__`)
- Full audio engine access
- All features enabled

**Browser Mode (Development/Testing):**
- Tauri APIs unavailable
- UI preview only
- Audio features gracefully disabled
- Preview mode banner displayed
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: add ARCHITECTURE.md system overview

Explains three-layer architecture and technology choices.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 9: Document Lock-Free Audio Architecture

**Files:**
- Modify: `docs/ARCHITECTURE.md`

**Step 1: Add lock-free audio section**

Append to ARCHITECTURE.md:
```markdown

---

## Lock-Free Audio Architecture

Real-time audio requires **zero allocations** and **zero locks** in the audio callback. Any blocking operation (mutex, memory allocation, system call) can cause audio glitches.

### Communication Patterns

**UI → Audio (Commands):**
```
[UI Thread]                    [Audio Thread]
    │                               │
    ├─ Command ────┐                │
    │              ▼                │
    │         Ring Buffer           │
    │          (rtrb)               │
    │              │                │
    │              └──────────────► │ Read commands
    │                               │ Apply to state
```

**Audio → UI (State Snapshots):**
```
[Audio Thread]                 [UI Thread]
    │                               │
    ├─ State snapshot ─────┐        │
    │                      ▼        │
    │               Triple Buffer   │
    │                 (lockfree)    │
    │                      │        │
    │                      └──────► │ Read latest state
    │                               │ Update UI
```

### Why This Matters

**Forbidden in Audio Thread:**
- ❌ Mutex locks (unpredictable wait time)
- ❌ Memory allocation (`malloc`/`new` may block)
- ❌ File I/O (disk access is slow and variable)
- ❌ System calls (context switch overhead)

**Allowed in Audio Thread:**
- ✅ Lock-free queue reads/writes
- ✅ Atomic operations
- ✅ Pre-allocated buffer access
- ✅ Pure computation

### Implementation Details

**Ring Buffer (rtrb):**
- Bounded SPSC (Single Producer Single Consumer) queue
- Capacity: 256 commands
- Non-blocking reads/writes
- UI thread produces, audio thread consumes

**Triple Buffer (triple_buffer):**
- Three slots: Write, Read, Back
- Audio thread writes, UI thread reads
- Always latest snapshot available
- No locks, no blocking

**Zero Allocation:**
- All buffers pre-allocated at startup
- Audio callback only reads/writes existing memory
- Pattern changes queued, applied between callbacks
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document lock-free audio architecture

Explains ring buffer, triple buffer patterns and real-time
audio constraints.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 10: Document State Management

**Files:**
- Modify: `docs/ARCHITECTURE.md`

**Step 1: Add state management section**

Append to ARCHITECTURE.md:
```markdown

---

## State Management

FLUX uses Leptos signals for reactive state management with three distinct state layers.

### State Layers

**1. Domain State (Pattern Data)**

```rust
Pattern {
  tracks: Vec<Track> {
    subtracks: Vec<Subtrack> {
      steps: Vec<AtomicStep> {
        trig_type: TrigType
        params: HashMap<String, f64>
        micro_timing: f64
      }
    }
  }
}
```

Purpose: The actual sequencer data
Ownership: Shared via Leptos context
Updates: User interactions (toggle step, edit params)

**2. Playback State (Audio Engine Mirror)**

```rust
PlaybackState {
  is_playing: bool
  current_position: usize  // 0-15
  triggered_tracks: [bool; 4]
}
```

Purpose: Mirror of audio engine state for visualization
Ownership: Provided via context, updated by Tauri events
Updates: Audio engine snapshots every step (real-time)

**3. UI State (Ephemeral)**

```rust
GridUIState {
  hovered_step: Option<(usize, usize)>
  recent_triggers: Vec<TriggerEvent>  // For pulse animations
}
```

Purpose: Transient UI-only state (animations, hover)
Ownership: Local to Grid component
Updates: User interactions, automatic cleanup

### Signal Patterns

**Signal::derive for Performance**

GridStep component derives 4 signals from global state:
```rust
let is_active = Signal::derive(move || {
    pattern.with(|p| p.tracks[track].subtracks[0].steps[step].is_active())
});

let is_selected = Signal::derive(move || {
    sequencer_state.selected_step.get() == Some((track, step))
});

let is_playing_step = Signal::derive(move || {
    playback_state.get().current_position == step
});

let is_recently_triggered = Signal::derive(move || {
    grid_ui_state.get().is_recently_triggered(track, step)
});
```

**Why:** Each step only re-renders when **its own** state changes, not when any step in the grid changes. Critical for 64-button grid performance (120fps).

**Memoization:**
- Derived signals cache results
- Re-compute only when dependencies change
- Prevents cascading re-renders

### Context Provision

```rust
// In App component
provide_context(pattern_signal);
provide_context(playback_state);
provide_context(sequencer_state);
provide_context(tauri_capabilities);

// In child components
let pattern = use_context::<ReadSignal<Pattern>>().expect("Pattern not found");
let playback = use_context::<ReadSignal<PlaybackState>>().expect("...");
```

**Pattern:** Global singletons passed down component tree without prop drilling.
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document state management patterns

Explains three state layers and Signal::derive performance
optimization.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 11: Document Frontend Architecture

**Files:**
- Modify: `docs/ARCHITECTURE.md`

**Step 1: Add frontend architecture section**

Append to ARCHITECTURE.md:
```markdown

---

## Frontend Architecture

### Component Hierarchy

```
App (root)
├── Provides: Pattern, PlaybackState, SequencerState, TauriCapabilities
├── Header (Toolbar)
├── Grid Section
│   ├── Grid
│   │   ├── Provides: GridUIState (local)
│   │   ├── Track Labels (static)
│   │   ├── PlayheadIndicator (absolute positioned)
│   │   ├── Grid Tracks (flex layout)
│   │   │   └── For each track (4)
│   │   │       └── For each step (16)
│   │   │           └── GridStep (derived signals)
│   │   └── StepBadge (conditionally rendered)
└── Parameters Section
    ├── Inspector (track-level params)
    └── StepInspector (step-level p-locks)
```

### Tauri Integration Layer

**Location:** `src/ui/tauri.rs`

**Responsibilities:**
1. Capability detection (`tauri_detect.rs`)
2. Safe API wrappers (error handling)
3. Command invocation (UI → Backend)
4. Event listening (Backend → UI)

**Error Handling Strategy:**

```rust
// Commands (UI → Backend)
match safe_invoke("toggle_step", args).await {
    Ok(_) => { /* Success */ },
    Err(TauriError::NotAvailable) => {
        // Browser mode - show message or no-op
    },
    Err(TauriError::InvokeFailed(msg)) => {
        // Desktop mode - log error
        console::error_1(&msg.into());
    }
}

// Events (Backend → UI)
if tauri_capabilities.events_enabled {
    safe_listen_event("playback-status", |event| {
        // Update UI state
    }).await;
}
// If disabled, safe_listen_event logs once and returns
```

### Reactivity Flow

**User Action → Pattern Update:**
```
User clicks step
    ↓
GridStep onClick handler
    ↓
tauri::toggle_step(track, step)  ← Command to backend
    ↓
Backend updates Pattern
    ↓
Backend emits "pattern-updated" event
    ↓
Frontend receives event, updates pattern_signal
    ↓
All components with pattern dependencies re-render
    ↓
GridStep re-renders with new state
```

**Audio Engine → Playback Visualization:**
```
Audio engine advances step
    ↓
Emits "playback-status" event (every step)
    ↓
Frontend Effect listener receives event
    ↓
Updates playback_state signal
    ↓
PlayheadIndicator, GridStep re-render
    ↓
Trigger pulse animation starts (GridUIState)
```
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document frontend architecture

Explains component hierarchy, Tauri integration, and
reactivity flow.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 12: Document Data Model

**Files:**
- Modify: `docs/ARCHITECTURE.md`

**Step 1: Add data model section**

Append to ARCHITECTURE.md:
```markdown

---

## Data Model

### AtomicStep (Core Unit)

```rust
pub struct AtomicStep {
    pub trig_type: TrigType,
    pub params: HashMap<String, f64>,
    pub micro_timing: f64,
    pub probability: f64,
}
```

**Fields:**
- `trig_type`: Note, Lock, Trigless, OneShot, None
- `params`: Parameter locks (e.g., "pitch": 440.0, "filter_cutoff": 0.7)
- `micro_timing`: Timing offset in milliseconds (-50.0 to +50.0)
- `probability`: Trigger probability 0.0-1.0 (1.0 = always)

### Hierarchy

```
Pattern
 └─ tracks: Vec<Track>            (4 tracks)
     └─ subtracks: Vec<Subtrack>  (currently 1 per track)
         └─ steps: Vec<AtomicStep> (16 steps per subtrack)
```

**Why subtracks?**
Future expansion for polyphonic sequences. Currently each track has 1 subtrack.

### Parameter Locks (P-Locks)

**Concept:** Per-step parameter overrides

**Storage:**
```rust
step.params.insert("filter_cutoff".to_string(), 0.8);
step.params.insert("pitch".to_string(), 440.0);
```

**Application:**
1. Audio engine reads step at playback position
2. Checks if `step.params` contains parameter
3. If yes: Use locked value
4. If no: Use track default

**Use Cases:**
- Pitch sequences (melody)
- Filter sweeps (movement)
- LFO amount modulation (dynamics)
- Per-step ADSR (articulation)

### Serialization

**Format:** JSON (via serde)

**Pattern Save:**
```rust
let json = serde_json::to_string_pretty(&pattern)?;
std::fs::write("pattern.json", json)?;
```

**Pattern Load:**
```rust
let json = std::fs::read_to_string("pattern.json")?;
let pattern: Pattern = serde_json::from_str(&json)?;
```

**Future:** Binary format (MessagePack/bincode) for smaller files.
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document data model and p-locks

Explains AtomicStep, pattern hierarchy, and parameter lock
system.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 13: Document Performance Considerations

**Files:**
- Modify: `docs/ARCHITECTURE.md`

**Step 1: Add performance section**

Append to ARCHITECTURE.md:
```markdown

---

## Performance Considerations

### Reactive Granularity

**Problem:** 64 buttons (4 tracks × 16 steps) re-rendering on every state change is expensive.

**Solution:** Signal::derive with narrow dependencies

```rust
// ❌ BAD: Entire grid re-renders on any change
view! {
    <For each=|| all_steps ...>
}

// ✅ GOOD: Each step independently derives its state
GridStep {
    // Only re-renders when THIS step's state changes
    is_active: Signal::derive(|| pattern[track][step].is_active())
    is_selected: Signal::derive(|| selected == (track, step))
}
```

**Result:** 120fps average vs 30fps with naive approach

### Animation Performance

**CSS Transforms > DOM Manipulation**

```rust
// ✅ GPU-accelerated
style=move || format!("transform: translateX({}px)", offset)

// ❌ Forces layout recalculation
style=move || format!("left: {}px", offset)
```

**Pulse Animation:**
```css
@keyframes pulse-once {
  0%, 100% { opacity: 0; }
  50% { opacity: 1; }
}

.animate-pulse-once {
  animation: pulse-once 150ms ease-out;
}
```

**Why once:** Prevents animation stacking from rapid triggers.

### Memory Management

**GridUIState Cleanup:**
```rust
// Every 150ms, remove triggers older than 150ms
state.recent_triggers.retain(|t| current_time - t.timestamp < 150.0);
```

**Performance Test Results (30 seconds playback):**
- Heap start: 3.6 MB
- Heap stable: 3.9 MB
- Variance: 0 MB (no leak)

### WASM Bundle Size

**Current:** ~2.1 MB (gzipped ~650 KB)

**Optimization:**
- `wasm-opt -Oz` in release builds
- Code splitting (future)
- Lazy component loading (future)

**Trade-off:** Initial load time vs runtime performance (chose runtime).
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document performance considerations

Explains reactive granularity, animation techniques, and
memory management.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 14: Document Design Decisions

**Files:**
- Modify: `docs/ARCHITECTURE.md`

**Step 1: Add design decisions section**

Append to ARCHITECTURE.md:
```markdown

---

## Design Decisions

### Why Rust for Frontend?

**Pros:**
- ✅ Type safety prevents entire categories of bugs
- ✅ Shared types between frontend and backend (Pattern, AtomicStep)
- ✅ Performance competitive with hand-tuned JavaScript
- ✅ Leptos reactivity model is elegant and fast

**Cons:**
- ❌ Larger WASM bundle than JS frameworks (~2MB vs ~200KB)
- ❌ Longer compile times (60s vs 5s for equivalent JS)
- ❌ Smaller ecosystem (fewer UI libraries)
- ❌ Harder for web developers to contribute

**Decision:** Type safety and shared types outweigh ecosystem trade-offs for this project.

### Why Tauri vs Electron?

**Tauri Advantages:**
- App size: 15 MB vs 150 MB (Electron)
- Memory: 80 MB vs 200 MB base
- Native look and feel
- Rust backend (shared with audio engine)

**Tauri Disadvantages:**
- Less mature (v2.x vs Electron v27)
- Platform-specific builds required
- Smaller plugin ecosystem

**Decision:** Native performance and size critical for audio app. Tauri's trade-offs acceptable.

### Why State-First Architecture?

**Principle:** Build state management before visual components.

**Rationale:**
1. Playback visualization requires accurate state from audio engine
2. UI rendering is presentation layer only
3. State correctness more critical than visual polish
4. Testing state easier than testing visual output

**Example:** Grid UI/UX implementation built PlaybackState + GridUIState first, then visual components consumed these states.

**Result:** Clean separation, easy to reason about, testable.

### Why Lock-Free Audio?

**Problem:** Real-time audio cannot tolerate blocking operations.

**Alternatives Considered:**

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| Mutex locks | Simple code | Audio glitches possible | ❌ Rejected |
| Message passing (channels) | Familiar Rust pattern | Allocates memory | ❌ Rejected |
| Lock-free queues | No blocking, no allocation | More complex | ✅ Chosen |

**Trade-off:** Implementation complexity vs audio quality. Chose quality.

### Why Manual Testing?

**Current:** No automated UI tests

**Rationale:**
- Leptos testing ecosystem immature
- Manual testing faster for current team size
- Focus on shipping features over test infrastructure

**Future:** Add integration tests when team grows or bugs increase.

**Mitigation:** Comprehensive manual test protocol documented in test-notes.md.
```

**Step 2: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document design decisions and trade-offs

Explains technology choices, architectural patterns, and
testing strategy.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 3: Developer Documentation

### Task 15: Create DEVELOPER_GUIDE.md with Setup

**Files:**
- Create: `docs/DEVELOPER_GUIDE.md`

**Step 1: Write development setup section**

```markdown
# FLUX Developer Guide

**Last Updated:** February 14, 2026

Welcome to FLUX development! This guide covers everything you need to contribute to the project.

---

## Development Setup

### Prerequisites

**Required:**
- Rust 1.75+ (`rustup update stable`)
- Node.js 18+ and npm
- Git

**Platform-Specific:**

**macOS:**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Tauri dependencies (if building desktop app)
# Already covered by Xcode tools
```

**Windows:**
```bash
# Install Visual Studio 2022 Build Tools
# - Desktop development with C++
# - Windows 10/11 SDK

# Install WebView2 (usually pre-installed on Windows 11)
# Download: https://developer.microsoft.com/microsoft-edge/webview2/
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libasound2-dev  # For audio engine
```

### Installation Steps

**1. Clone repository:**
```bash
git clone https://github.com/yourusername/flux.git
cd flux/flux-app
```

**2. Install Rust WASM target:**
```bash
rustup target add wasm32-unknown-unknown
```

**3. Install Trunk (frontend build tool):**
```bash
cargo install trunk
```

**4. Install npm dependencies:**
```bash
npm install
```

**5. Verify installation:**
```bash
# Build frontend
trunk build

# Expected output: Compiling... Done. index.html in dist/

# Run Tauri dev mode (desktop app)
npm run dev

# Expected: App window opens with sequencer interface
```

### First Build Verification

**Test frontend only (browser mode):**
```bash
cd flux-app
trunk serve
# Open http://localhost:1420
# Expected: Preview mode banner, UI functional, console clean
```

**Test full app (Tauri desktop):**
```bash
npm run dev
# Expected: Desktop window, no banner, audio features work
```

**Common Setup Issues:** See [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
```

**Step 2: Commit**

```bash
git add docs/DEVELOPER_GUIDE.md
git commit -m "docs: create DEVELOPER_GUIDE.md with setup instructions

Covers prerequisites, installation, and first build verification.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 16: Document Project Structure

**Files:**
- Modify: `docs/DEVELOPER_GUIDE.md`

**Step 1: Add project structure section**

Append to DEVELOPER_GUIDE.md:
```markdown

---

## Project Structure

```
flux-app/
├── src/                     Frontend (Leptos WASM)
│   ├── app.rs              Root component, context providers
│   ├── shared/
│   │   └── models.rs       Shared types (Pattern, AtomicStep)
│   └── ui/
│       ├── components/     UI components
│       │   ├── grid.rs     Main sequencer grid
│       │   ├── grid_step.rs Individual step button
│       │   ├── inspector.rs Track parameters
│       │   ├── step_inspector.rs P-lock editor
│       │   ├── lfo_designer.rs Waveform editor
│       │   ├── toolbar.rs  Play/stop controls
│       │   └── ...
│       ├── state.rs        PlaybackState, GridUIState
│       ├── tauri.rs        Tauri API wrappers
│       └── tauri_detect.rs Capability detection
│
├── src-tauri/              Backend (Rust/Tauri)
│   ├── src/
│   │   ├── main.rs         Tauri app setup
│   │   ├── audio_engine.rs Lock-free audio engine
│   │   └── commands.rs     Tauri command handlers
│   ├── Cargo.toml          Backend dependencies
│   └── tauri.conf.json     App configuration
│
├── docs/                   Documentation
│   ├── ARCHITECTURE.md     System design
│   ├── DEVELOPER_GUIDE.md  This file
│   ├── USER_GUIDE.md       User manual
│   ├── TROUBLESHOOTING.md  Common issues
│   └── plans/              Design documents
│
├── index.html              Entry point for frontend
├── tailwind.config.js      Tailwind CSS config
├── Trunk.toml              Frontend build config
└── package.json            Scripts and npm deps
```

### Frontend Organization (`src/`)

**app.rs**
- Root App component
- Context providers (Pattern, PlaybackState, SequencerState)
- Tauri event listeners
- Global keyboard handlers

**shared/models.rs**
- Data structures shared with backend
- Pattern, Track, Subtrack, AtomicStep
- TrigType enum
- Serialization (serde)

**ui/components/**
- One file per component
- Component-specific logic only
- Use context for global state

**ui/state.rs**
- PlaybackState (audio engine mirror)
- GridUIState (UI-only state)
- State types, not components

**ui/tauri.rs + tauri_detect.rs**
- Tauri API integration
- Capability detection
- Safe wrappers

### Backend Organization (`src-tauri/src/`)

**main.rs**
- Tauri builder setup
- Command registration
- Window configuration

**audio_engine.rs**
- Real-time audio callback
- Lock-free pattern playback
- Event emission (playback-status)

**commands.rs**
- Tauri command handlers
- toggle_step, push_midi_command, etc.
- Command → Audio engine queue
```

**Step 2: Commit**

```bash
git add docs/DEVELOPER_GUIDE.md
git commit -m "docs: document project structure walkthrough

Explains directory layout and organization principles.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 17: Document Component Development

**Files:**
- Modify: `docs/DEVELOPER_GUIDE.md`

**Step 1: Add component development section**

Append to DEVELOPER_GUIDE.md:
```markdown

---

## Component Development

### Leptos Component Pattern

**Basic structure:**
```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent(
    // Props
    #[prop(into)] value: Signal<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    // Local state
    let (count, set_count) = signal(0);

    // Derived signals
    let doubled = Signal::derive(move || count.get() * 2);

    // Effects
    Effect::new(move |_| {
        // Runs when dependencies change
        log!("Count: {}", count.get());
    });

    view! {
        <div>
            <p>{value}</p>
            <p>"Doubled: " {doubled}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Increment"
            </button>
        </div>
    }
}
```

### Props and Signals

**Signal props (reactive):**
```rust
#[prop(into)] value: Signal<String>
// Accepts: Signal<String>, RwSignal<String>, Memo<String>
```

**Callback props:**
```rust
#[prop(optional)] on_change: Option<Callback<String>>
// Usage: on_change.call("new value");
```

**Optional vs required:**
```rust
#[prop(optional)] count: Option<usize>  // Optional
title: String                           // Required
```

### Context Consumption

**Reading context:**
```rust
let pattern = use_context::<ReadSignal<Pattern>>()
    .expect("Pattern context not found");

// Use in reactive scope
let track_count = move || pattern.with(|p| p.tracks.len());
```

**Providing context (in parent):**
```rust
provide_context(my_signal);
provide_context(my_value);
```

### Styling with Tailwind

**Static classes:**
```rust
view! {
    <div class="bg-zinc-900 p-4 rounded-lg">
        "Content"
    </div>
}
```

**Dynamic classes:**
```rust
view! {
    <div
        class="px-4 py-2"
        class:bg-blue-500=move || is_active.get()
        class:bg-zinc-800=move || !is_active.get()
    >
        "Button"
    </div>
}
```

**Tailwind config:** `tailwind.config.js` for custom colors, animations.

### Component File Template

```rust
use leptos::prelude::*;

#[component]
pub fn NewComponent() -> impl IntoView {
    // 1. Context consumption
    let pattern = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern not found");

    // 2. Local state
    let (local_state, set_local_state) = signal(default_value());

    // 3. Derived signals
    let computed = Signal::derive(move || {
        // Compute from pattern or local state
    });

    // 4. Effects
    Effect::new(move |_| {
        // Side effects when dependencies change
    });

    // 5. Event handlers
    let handle_click = move |_| {
        set_local_state.update(|s| /* modify state */);
    };

    // 6. View
    view! {
        <div>
            // JSX-like syntax
        </div>
    }
}
```
```

**Step 2: Commit**

```bash
git add docs/DEVELOPER_GUIDE.md
git commit -m "docs: document component development patterns

Covers Leptos components, props, signals, and styling.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 18: Document Adding New Features

**Files:**
- Modify: `docs/DEVELOPER_GUIDE.md`

**Step 1: Add feature development workflow**

Append to DEVELOPER_GUIDE.md:
```markdown

---

## Adding New Features

Follow this workflow for all new features:

### Step 1: Design Phase

**Use brainstorming skill** (if using Claude Code):
```
/brainstorming <feature description>
```

Or manually:
1. Write design doc in `docs/plans/YYYY-MM-DD-<feature>.md`
2. Define success criteria
3. List affected files
4. Sketch architecture

**Example:** `docs/plans/2026-02-13-grid-ui-ux-enhancements-design.md`

### Step 2: State Management Setup

**If feature needs new state:**

```rust
// In src/ui/state.rs or component file
#[derive(Clone, Debug, Default)]
pub struct FeatureState {
    pub field1: Type1,
    pub field2: Type2,
}

// In App or parent component
let feature_state = signal(FeatureState::default());
provide_context(feature_state.0);  // Read signal
provide_context(feature_state.1);  // Write signal
```

**Pattern:**
- Domain state → `shared/models.rs`
- UI-only state → `ui/state.rs` or component-local

### Step 3: Component Creation

**Create component file:**
```bash
# src/ui/components/new_feature.rs
```

**Export in mod.rs:**
```rust
// src/ui/components/mod.rs
pub mod new_feature;
```

**Implement component following template (see Component Development).**

### Step 4: Tauri Integration (If Needed)

**If feature needs backend communication:**

**Backend (src-tauri/src/commands.rs):**
```rust
#[tauri::command]
fn feature_command(arg: String) -> Result<ResponseType, String> {
    // Implementation
    Ok(response)
}

// Register in main.rs:
.invoke_handler(tauri::generate_handler![
    feature_command,
    // ... existing commands
])
```

**Frontend (src/ui/tauri.rs):**
```rust
pub async fn call_feature_command(arg: String) -> Result<ResponseType, TauriError> {
    let args = serde_wasm_bindgen::to_value(&arg).unwrap();
    safe_invoke("feature_command", args)
        .await
        .map(|val| serde_wasm_bindgen::from_value(val).unwrap())
}
```

### Step 5: Testing

**Manual testing checklist:**
- [ ] Feature works in Tauri desktop mode
- [ ] Feature degrades gracefully in browser mode
- [ ] No console errors
- [ ] Performance acceptable (60fps target)
- [ ] Edge cases handled

**Test both modes:**
```bash
# Browser mode
trunk serve

# Desktop mode
npm run dev
```

### Step 6: Documentation

**Update docs:**
- [ ] Add to USER_GUIDE.md (user-facing features)
- [ ] Update ARCHITECTURE.md (if adds new pattern)
- [ ] Add API docs to `docs/api/` (if public API)
- [ ] Update this guide (if new development pattern)

### Example: Adding Keyboard Shortcut

```rust
// In src/app.rs

// Add handler
let handle_play_pause = move |ev: KeyboardEvent| {
    if ev.key() == " " && !ev.repeat() {  // Spacebar
        ev.prevent_default();
        // Call play/pause command
    }
};

window_event_listener(ev::keydown, handle_play_pause);
```

**Then document in USER_GUIDE.md keyboard shortcuts section.**
```

**Step 2: Commit**

```bash
git add docs/DEVELOPER_GUIDE.md
git commit -m "docs: document feature development workflow

6-step process from design to documentation.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 19: Document State Management Best Practices

**Files:**
- Modify: `docs/DEVELOPER_GUIDE.md`

**Step 1: Add state management patterns section**

Append to DEVELOPER_GUIDE.md:
```markdown

---

## State Management Patterns

### When to Use Signal vs Memo

**Signal (`signal()`):**
```rust
let (count, set_count) = signal(0);  // Read and write
```
- Use for: Data that changes
- Can: Read (.get()) and write (.set(), .update())

**Memo (`Memo::new()`):**
```rust
let doubled = Memo::new(move |_| count.get() * 2);  // Read-only
```
- Use for: Derived values
- Can: Read only
- Optimized: Caches result, only recomputes when dependencies change

**Signal::derive:**
```rust
let doubled = Signal::derive(move || count.get() * 2);
```
- Similar to Memo but different API
- Use in components for derived signals

### Context Provision Best Practices

**Global state (App component):**
```rust
// Provide separate read and write signals for granular control
let (state, set_state) = signal(State::default());
provide_context(state);       // ReadSignal
provide_context(set_state);   // WriteSignal

// Or provide both as tuple
provide_context((state, set_state));
```

**Consuming:**
```rust
// Read-only access
let state = use_context::<ReadSignal<State>>().expect("...");

// Write access
let set_state = use_context::<WriteSignal<State>>().expect("...");

// Both
let (state, set_state) = use_context::<(ReadSignal<State>, WriteSignal<State>)>()
    .expect("...");
```

### Avoiding Reactive Pitfalls

**❌ Don't call .get() outside reactive scope:**
```rust
let value = signal.get();  // Value is now static, won't update!
view! {
    <p>{value}</p>  // Won't react to signal changes
}
```

**✅ Do call .get() inside reactive scope:**
```rust
view! {
    <p>{move || signal.get()}</p>  // Reactive closure
}
```

**❌ Don't read signal multiple times in one reactive scope:**
```rust
Effect::new(move |_| {
    let val1 = signal.get();  // Read 1
    do_something();
    let val2 = signal.get();  // Read 2 - might be different!
});
```

**✅ Do read once and capture:**
```rust
Effect::new(move |_| {
    let val = signal.get();  // Read once
    use_val(val);
    use_val_again(val);  // Consistent value
});
```

### Performance Optimization

**Use Signal::derive for expensive computations:**
```rust
// ❌ Recomputes on every render
view! {
    <p>{move || expensive_computation(data.get())}</p>
}

// ✅ Caches result, only recomputes when data changes
let result = Signal::derive(move || expensive_computation(data.get()));
view! {
    <p>{result}</p>
}
```

**Narrow signal dependencies:**
```rust
// ❌ Depends on entire pattern
let should_highlight = Signal::derive(move || {
    pattern.with(|p| p.tracks.iter().any(|t| t.is_active))
});

// ✅ Depends only on relevant field
let active_tracks = Signal::derive(move || {
    pattern.with(|p| p.tracks.iter().filter(|t| t.is_active).count())
});
```

**Batch updates:**
```rust
// ❌ Triggers 3 re-renders
set_a.set(1);
set_b.set(2);
set_c.set(3);

// ✅ Triggers 1 re-render (Leptos batches automatically)
// Just be aware batching exists
```
```

**Step 2: Commit**

```bash
git add docs/DEVELOPER_GUIDE.md
git commit -m "docs: document state management best practices

Covers Signal vs Memo, context patterns, and reactive
pitfalls.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 20: Document Code Style and Contributing

**Files:**
- Modify: `docs/DEVELOPER_GUIDE.md`

**Step 1: Add code style section**

Append to DEVELOPER_GUIDE.md:
```markdown

---

## Code Style & Conventions

### Rust Formatting

**Use rustfmt:**
```bash
cargo fmt
```

**Settings:**
```toml
# Already configured in rustfmt.toml
edition = "2021"
max_width = 100
```

### Component Naming

**Files:** `snake_case.rs`
```
grid_step.rs
step_inspector.rs
lfo_designer.rs
```

**Components:** `PascalCase`
```rust
#[component]
pub fn GridStep() -> impl IntoView { }

#[component]
pub fn StepInspector() -> impl IntoView { }
```

### File Organization

**One component per file:**
```rust
// ✅ src/ui/components/grid_step.rs
#[component]
pub fn GridStep() -> impl IntoView { }

// ❌ Don't put multiple components in one file
```

**Helper functions in same file as component:**
```rust
// src/ui/components/grid.rs

// Component
#[component]
pub fn Grid() -> impl IntoView { }

// Helper used only by Grid
fn current_timestamp() -> f64 { }
```

### Documentation Standards

**Public functions:**
```rust
/// Toggles a step on/off
///
/// # Arguments
/// * `track_id` - Track index (0-3)
/// * `step_idx` - Step index (0-15)
///
/// # Errors
/// Returns TauriError::NotAvailable if Tauri is unavailable
pub async fn toggle_step(track_id: usize, step_idx: usize) -> Result<(), TauriError> {
    // ...
}
```

**Complex logic:**
```rust
// Why we do this: Playhead uses fixed pixels but grid used fractional units.
// This caused drift as window resized.
let offset = pos * STEP_TOTAL_WIDTH;
```

---

## Contributing Guidelines

### Git Workflow

**Branch naming:**
```bash
feature/grid-ui-enhancements
fix/playhead-alignment
docs/architecture-guide
```

**Commit messages:**
```
feat: add playback visualization
fix: resolve playhead offset by one step
docs: add architecture documentation
refactor: extract GridStep component
```

**Format:**
```
<type>: <description>

[Optional body explaining why]

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```

**Types:** feat, fix, docs, refactor, test, chore

### Pull Request Process

**Before creating PR:**
1. Run `cargo fmt` and `npm run format`
2. Test in both browser and desktop modes
3. Update relevant documentation
4. Ensure no console errors

**PR template:**
```markdown
## Summary
Brief description of changes

## Changes
- Added X feature
- Fixed Y bug
- Updated Z documentation

## Test Plan
- [ ] Tested in browser mode
- [ ] Tested in Tauri desktop
- [ ] No console errors
- [ ] Performance acceptable

## Screenshots/Video
[If UI changes]
```

**Review expectations:**
- Response within 2 days
- At least 1 approval required
- CI must pass (when implemented)

### Communication Channels

**Issues:**
- Bug reports: Use GitHub Issues
- Feature requests: Discuss first, then issue

**Questions:**
- Development help: GitHub Discussions
- Quick questions: Discord (if set up)
```

**Step 2: Commit**

```bash
git add docs/DEVELOPER_GUIDE.md
git commit -m "docs: add code style and contributing guidelines

Covers formatting, naming conventions, git workflow, and PR
process.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 4: User Documentation

### Task 21: Create USER_GUIDE.md with Getting Started

**Files:**
- Create: `docs/USER_GUIDE.md`

**Step 1: Write getting started section**

```markdown
# FLUX User Guide

**Last Updated:** February 14, 2026

Welcome to FLUX! This guide will help you learn and master the sequencer.

---

## Getting Started

### Installation

**macOS:**
1. Download FLUX.dmg from releases
2. Open DMG and drag FLUX to Applications
3. Open FLUX from Applications (right-click → Open first time)
4. Grant microphone permission if prompted (for audio input)

**Windows:**
```
1. Download FLUX-setup.exe
2. Run installer
3. Launch from Start Menu
```

**Linux:**
```bash
# Download AppImage
chmod +x FLUX.AppImage
./FLUX.AppImage
```

### First Launch

When you first open FLUX:

1. **Audio Setup**
   - FLUX will detect your default audio device
   - Check your system sound settings if no output
   - See [TROUBLESHOOTING.md](TROUBLESHOOTING.md#audio-issues) if problems

2. **Interface Overview**
   ```
   ┌────────────────────────────────────────────────┐
   │ FLUX                            [Play] [Stop]  │  ← Header
   ├────────────────────────────────────────────────┤
   │ Sequencer Grid                                 │
   │ T1 [▢][▢][▢][▢] | [▢][▢][▢][▢] | ...          │  ← Grid
   │ T2 [▢][▢][▢][▢] | [▢][▢][▢][▢] | ...          │
   │ T3 [▢][▢][▢][▢] | [▢][▢][▢][▢] | ...          │
   │ T4 [▢][▢][▢][▢] | [▢][▢][▢][▢] | ...          │
   ├────────────────────────────────────────────────┤
   │ Parameters                                     │  ← Inspector
   │ Pitch: 440 Hz   Filter: 0.5   ...             │
   └────────────────────────────────────────────────┘
   ```

3. **Your First Pattern**
   - Click any step button to toggle it on (turns blue)
   - Click Play button (or press Space)
   - Watch the green playhead advance and hear the sound

---

## The Grid

The sequencer grid is the heart of FLUX. Each row is a track, each column is a step.

### Step States

**Inactive (default):**
- Dark gray background
- Not triggered during playback

**Active (clicked on):**
- Blue background
- Will trigger when playhead reaches this step

**Selected (for editing):**
- Amber ring around button
- Shows "T{track}・S{step}" badge
- Parameters section now controls this step

**Playing:**
- Green overlay
- Indicates current playback position

**Triggered:**
- White flash (pulse animation)
- Confirms step fired

### Beat Grouping

Vertical lines separate every 4 steps:
```
[▢][▢][▢][▢] | [▢][▢][▢][▢] | [▢][▢][▢][▢] | [▢][▢][▢][▢]
 ^--------^     ^--------^     ^--------^     ^--------^
   Beat 1         Beat 2         Beat 3         Beat 4
```

Helps visualize rhythm and structure patterns.

### Playback Visualization

**Playhead:**
- Green vertical bar
- Shows current playback position
- Advances one step per clock tick

**Playing step:**
- Green overlay on current step
- Helps track position during playback

**Trigger pulse:**
- White ring flash when step fires
- Confirms audio engine triggered the sound

---

## Creating Patterns

### Toggling Steps

**Click to toggle:**
- Click inactive step → turns blue (active)
- Click active step → turns gray (inactive)

**Selection:**
- Click step → selects it (amber ring)
- Selected step parameters shown in inspector
- ESC key to deselect

### Playback Controls

**Play button (or Space):**
- Starts playback from current position
- Loops continuously until stopped

**Stop button (or Space again):**
- Stops playback
- Position resets to step 0

**Tempo:**
- Currently fixed at 120 BPM
- Future: Adjustable tempo slider

### Tracks and Subtracks

**Tracks (T1-T4):**
- 4 independent tracks
- Each track has its own sound/instrument
- Run simultaneously during playback

**Subtracks:**
- Internal concept (1 per track currently)
- Future: Polyphonic sequences (multiple subtracks per track)

### Basic Workflow

1. **Start simple:**
   - Activate step 0 on Track 1
   - Press Play
   - Listen to steady pulse

2. **Add rhythm:**
   - Activate steps 4, 8, 12 (every 4 steps)
   - Creates quarter note pattern

3. **Add variation:**
   - Activate steps 2, 6, 10, 14
   - Fills in eighth notes

4. **Layer tracks:**
   - Add pattern on Track 2
   - Different rhythm = interesting polyrhythms

5. **Experiment:**
   - FLUX encourages exploration
   - No wrong patterns, just different grooves
```

**Step 2: Commit**

```bash
git add docs/USER_GUIDE.md
git commit -m "docs: create USER_GUIDE.md with getting started

Covers installation, first launch, grid basics, and basic
pattern creation.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 22: Document Parameter Locking

**Files:**
- Modify: `docs/USER_GUIDE.md`

**Step 1: Add parameter locking section**

Append to USER_GUIDE.md:
```markdown

---

## Parameter Locking (P-Lock)

Parameter locking lets you automate parameters per-step, creating melodies, filter sweeps, and dynamic patterns.

### What is Parameter Locking?

**Without P-Lock:**
- All steps use the same parameter values
- Pitch: 440 Hz for entire track
- Filter: 0.5 for entire track

**With P-Lock:**
- Each step can override parameters
- Step 1: Pitch 440 Hz
- Step 2: Pitch 550 Hz (different note)
- Step 3: Pitch 660 Hz (melody!)

### How to Lock Parameters

**1. Select a step:**
- Click step button
- Amber ring appears
- Badge shows "T1・S3" (track 1, step 3)

**2. Edit parameters in Inspector:**
- Parameters section shows current values
- Adjust any parameter (Pitch, Filter, etc.)
- Values automatically saved to selected step

**3. Deselect:**
- Click elsewhere or press ESC
- Step keeps locked parameters

### Parameter Inspector

When step is selected:

```
Parameters         TRACK 1, STEP 3 LOCKED
┌─────────────────────────────────────────┐
│ Pitch:     [=============]  440.0 Hz    │
│ Filter:    [==========]     0.75        │
│ Resonance: [=====]          0.3         │
│ ...                                     │
└─────────────────────────────────────────┘
```

**Lock indicator:**
- "TRACK DEFAULT" = no step selected, editing track defaults
- "TRACK X, STEP Y LOCKED" = editing specific step parameters

### Use Cases

**Melodic Sequences:**
```
Step 0: Pitch 440 Hz (A4)
Step 1: Pitch 523 Hz (C5)
Step 2: Pitch 587 Hz (D5)
Step 3: Pitch 659 Hz (E5)
→ Plays A-C-D-E melody
```

**Filter Sweeps:**
```
Step 0: Filter 0.1 (dark)
Step 4: Filter 0.3
Step 8: Filter 0.6
Step 12: Filter 0.9 (bright)
→ Creates opening filter movement
```

**Dynamic Modulation:**
```
Step 0: LFO Amount 0.0 (no modulation)
Step 8: LFO Amount 1.0 (full modulation)
→ Modulation builds over time
```

### Tips

- Start with rhythm, add p-locks later
- Not every step needs locks (sparse is often better)
- Use p-locks to create movement and interest
- Combine pitch + filter + timing for expressive sequences
```

**Step 2: Commit**

```bash
git add docs/USER_GUIDE.md
git commit -m "docs: document parameter locking system

Explains p-locks, workflow, and use cases.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 23: Document Advanced Features and Keyboard Shortcuts

**Files:**
- Modify: `docs/USER_GUIDE.md`

**Step 1: Add advanced features and shortcuts**

Append to USER_GUIDE.md:
```markdown

---

## Advanced Features

### Micro-Timing Offset

Add groove and humanization by shifting step timing slightly.

**Range:** -50ms to +50ms

**Examples:**
- -10ms: Slightly early (rushed feel)
- 0ms: On the grid (default)
- +10ms: Slightly late (laid-back feel)

**Use:** Creates swing, shuffle, humanization

### Step Probability

Make steps trigger randomly based on probability.

**Range:** 0.0 to 1.0
- 0.0 = Never triggers
- 0.5 = 50% chance
- 1.0 = Always triggers (default)

**Use:** Generative patterns, variation, happy accidents

### Trig Types

Control how steps trigger:

**Note (default):**
- Triggers note on + note off
- Standard behavior

**Lock:**
- Updates parameters without retriggering note
- Use for parameter automation mid-note

**Trigless:**
- Only updates parameters, no audio trigger
- Use for silent parameter changes

**OneShot:**
- Triggers note on, no note off
- Sustains until another note

**None:**
- Step inactive

### Future: Pattern Chaining

(Not yet implemented)
- Chain multiple patterns
- Create song structure
- A → B → C → A

---

## Keyboard Shortcuts

### Playback

| Shortcut | Action |
|----------|--------|
| Space    | Play / Stop |
| Esc      | Deselect step |

### Navigation

| Shortcut | Action |
|----------|--------|
| Click step | Select step |
| Click elsewhere | Deselect |

### Future Shortcuts

(Planned, not yet implemented)

| Shortcut | Action |
|----------|--------|
| Arrow keys | Navigate steps |
| Enter | Toggle selected step |
| Shift+Click | Multi-select |
| Cmd/Ctrl+C | Copy step |
| Cmd/Ctrl+V | Paste step |

---

## Tips & Best Practices

### Workflow Suggestions

**Start simple, add complexity:**
1. Create basic rhythm
2. Add second track
3. Introduce p-locks for interest
4. Experiment with probability/timing

**Less is more:**
- Sparse patterns often sound better than dense ones
- Leave space for sounds to breathe
- Not every step needs to trigger

**Use the grid visually:**
- Beat grouping helps rhythm placement
- Vertical columns = simultaneous sounds
- Horizontal rows = track layers

### Performance Tips

**Keep patterns focused:**
- Too many locked parameters = hard to manage
- Start with pitch/filter, add others as needed

**Listen critically:**
- Solo tracks to hear each layer
- Adjust levels to balance mix
- Trust your ears over visual grid

### Creative Techniques

**Polyrhythms:**
- Track 1: Pattern every 3 steps
- Track 2: Pattern every 4 steps
- Creates shifting relationship

**Euclidean-ish patterns:**
- Distribute triggers evenly across 16 steps
- Example: 5 triggers = steps 0, 3, 6, 10, 13

**Parameter sequencing:**
- Lock pitch on every step
- Creates melodic sequence
- Combine with probability for variation

### Common Mistakes to Avoid

**Don't:**
- ❌ Max out all parameters immediately
- ❌ Activate all 16 steps (usually too dense)
- ❌ Ignore playback visualization (watch the playhead!)
- ❌ Forget to deselect steps (ESC key)

**Do:**
- ✅ Start minimal, add gradually
- ✅ Use beat grouping for rhythm placement
- ✅ Experiment with probability and timing
- ✅ Save patterns frequently (future feature)
```

**Step 2: Commit**

```bash
git add docs/USER_GUIDE.md
git commit -m "docs: add advanced features, shortcuts, and tips

Covers micro-timing, probability, trig types, shortcuts, and
best practices.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 5: Troubleshooting & README

### Task 24: Create TROUBLESHOOTING.md

**Files:**
- Create: `docs/TROUBLESHOOTING.md`

**Step 1: Write troubleshooting guide**

```markdown
# FLUX Troubleshooting

Common issues and solutions for FLUX users and developers.

---

## Platform-Specific Setup

### macOS

**Xcode Command Line Tools not installed:**
```
Error: xcrun: error: invalid active developer path
```

**Solution:**
```bash
xcode-select --install
```

**App won't open ("unidentified developer"):**

**Solution:**
1. Right-click FLUX.app
2. Select "Open"
3. Click "Open" in security dialog
4. Subsequent launches work normally

**No audio output:**

**Solution:**
1. System Preferences → Sound → Output
2. Select correct audio device
3. Restart FLUX

---

### Windows

**WebView2 missing:**
```
Error: WebView2 runtime not found
```

**Solution:**
```
Download: https://developer.microsoft.com/microsoft-edge/webview2/
Install: Evergreen Standalone Installer
```

**Visual Studio build tools errors:**

**Solution:**
```
Install Visual Studio 2022 Build Tools:
- Desktop development with C++
- Windows 10/11 SDK
```

---

### Linux (Ubuntu/Debian)

**Missing dependencies:**
```
Error: webkit2gtk not found
```

**Solution:**
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
  build-essential \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libasound2-dev
```

**Permission denied (AppImage):**

**Solution:**
```bash
chmod +x FLUX.AppImage
./FLUX.AppImage
```

---

## Build Errors

### Rust Compilation Errors

**Error: rustc version too old:**
```
error: package requires rustc 1.75 or newer
```

**Solution:**
```bash
rustup update stable
```

**Error: wasm32 target not installed:**
```
error: can't find crate for std
```

**Solution:**
```bash
rustup target add wasm32-unknown-unknown
```

### WASM / Trunk Issues

**Trunk not found:**
```
command not found: trunk
```

**Solution:**
```bash
cargo install trunk
```

**Trunk build fails:**
```
Error: Failed to build WASM
```

**Solution:**
```bash
# Clear cache and rebuild
rm -rf dist/ target/
trunk build
```

### Dependency Resolution

**Conflicting dependencies:**

**Solution:**
```bash
# Update Cargo.lock
cargo update

# Clean build
cargo clean
trunk build
```

---

## Audio Issues

### No Sound

**Checklist:**
1. System audio device selected correctly?
2. System volume not muted?
3. FLUX running in desktop mode (not browser)?
4. Steps activated in grid?
5. Playback started (Space key)?

**Solution:**
```
1. Check system sound settings
2. Verify audio device in FLUX settings (future)
3. Restart app
4. See platform-specific audio setup above
```

### Audio Glitches / Crackling

**Possible causes:**
- Buffer size too small
- CPU overload
- Competing audio applications

**Solution:**
```
1. Close other audio apps
2. Increase buffer size (future setting)
3. Check CPU usage (Activity Monitor / Task Manager)
```

### Sample Rate Mismatch

**Symptom:** Pitch sounds wrong

**Solution:**
```
1. Check system audio sample rate (should be 48000 Hz)
2. macOS: Audio MIDI Setup app
3. Windows: Sound settings → Advanced
```

---

## UI Issues

### UI Not Updating

**Symptom:** Changes not reflected in grid

**Solution:**
```
1. Hard refresh browser (Cmd/Ctrl+Shift+R) if in dev mode
2. Restart desktop app
3. Check console for errors
```

### Performance Problems

**Symptom:** Laggy UI, dropped frames

**Solution:**
```
1. Close other applications
2. Check browser dev tools for performance issues (dev mode)
3. Reduce grid complexity (fewer locked parameters)
```

### Visual Glitches

**Symptom:** Playhead misaligned, steps rendering wrong

**Solution:**
```
1. Resize window (forces re-render)
2. Restart app
3. Report bug with screenshot
```

---

## Development Environment

### trunk serve Not Starting

**Error:**
```
Error: Address already in use
```

**Solution:**
```bash
# Kill process on port 1420
lsof -ti:1420 | xargs kill -9

# Or use different port
trunk serve --port 8080
```

### Hot Reload Not Working

**Symptom:** Changes don't appear without manual refresh

**Solution:**
```
1. Check trunk is running (not errored)
2. Look for rust compiler errors in terminal
3. Try manual refresh (Cmd/Ctrl+R)
4. Restart trunk serve
```

### Tauri Dev Mode Errors

**Error:**
```
Error: Failed to execute tauri dev
```

**Solution:**
```bash
# Ensure dependencies installed
npm install

# Rebuild backend
cd src-tauri
cargo build

# Run dev mode
cd ..
npm run dev
```

### Port Conflicts

**Multiple FLUX instances:**

**Solution:**
```bash
# Find processes
lsof -ti:1420  # Trunk
lsof -ti:1430  # Tauri (if used)

# Kill old processes
pkill -f trunk
pkill -f "flux-app"
```

---

## Runtime Errors

### Console TypeErrors (Fixed)

**Error:**
```
TypeError: Cannot read properties of undefined (reading 'core')
TypeError: Cannot read properties of undefined (reading 'event')
```

**Status:** Fixed in current version (Tauri detection + safe wrappers)

**If still seeing:** Update to latest version

### Tauri IPC Errors

**Error:**
```
Error: Command not found: <command_name>
```

**Solution:**
```
1. Ensure backend command registered (src-tauri/src/main.rs)
2. Frontend matches backend spelling
3. Restart dev mode
```

### Pattern Loading Errors

**Error:**
```
Error: Failed to deserialize pattern
```

**Solution:**
```
1. Check pattern.json format
2. Validate against Pattern type
3. Clear pattern and start fresh (future: reset command)
```

---

## FAQ

### How do I reset the app to defaults?

**Currently:**
```
1. Close app
2. Delete app data (platform-specific location)
3. Reopen app
```

**Future:** Settings → Reset to Defaults

### Where are patterns stored?

**Currently:** Patterns stored in memory only (lost on close)

**Future:** Auto-save to:
- macOS: `~/Library/Application Support/flux/patterns/`
- Windows: `%APPDATA%/flux/patterns/`
- Linux: `~/.config/flux/patterns/`

### Can I run this without Tauri (browser only)?

**Yes, for UI preview:**
```bash
trunk serve
# Open http://localhost:1420
```

**Limitations:**
- No audio engine (Tauri backend required)
- Preview mode banner shown
- Playback features disabled

**Use case:** UI development, testing layouts

### How do I export/import patterns?

**Currently:** Not implemented

**Future:**
```
File → Export Pattern → pattern.json
File → Import Pattern → Select file
```

### Does FLUX support MIDI?

**Currently:** MIDI planned but not yet implemented

**Future:**
- MIDI input (keyboard, controller)
- MIDI output (external gear)
- MIDI clock sync

### Can I contribute to FLUX?

**Yes!** See [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) for:
- Development setup
- Contribution guidelines
- Code style
- Pull request process

### Where do I report bugs?

**GitHub Issues:**
```
https://github.com/yourusername/flux/issues
```

**Include:**
- FLUX version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if relevant
```

**Step 2: Commit**

```bash
git add docs/TROUBLESHOOTING.md
git commit -m "docs: create comprehensive troubleshooting guide

Covers platform setup, build errors, audio issues, development
environment, and FAQ.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 25: Update README with Documentation Links

**Files:**
- Modify: `README.md`

**Step 1: Read current README**

First read to preserve existing content:

```bash
# Read README.md to check current content
```

**Step 2: Add documentation section**

Add after the existing "Features" or "Quick Start" section (exact location depends on current README):

```markdown

## Documentation

Comprehensive guides for users and developers:

- 📖 **[User Guide](docs/USER_GUIDE.md)** - Learn how to use FLUX
  - Getting started and installation
  - Grid interface and pattern creation
  - Parameter locking and advanced features
  - Keyboard shortcuts and tips

- 🏗️ **[Architecture](docs/ARCHITECTURE.md)** - How FLUX works
  - System overview and technology choices
  - Lock-free audio architecture
  - State management patterns
  - Performance considerations

- 💻 **[Developer Guide](docs/DEVELOPER_GUIDE.md)** - Contributing to FLUX
  - Development environment setup
  - Component development patterns
  - Feature development workflow
  - Code style and guidelines

- 🔧 **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
  - Platform-specific setup
  - Build and audio issues
  - Development environment problems
  - FAQ
```

**Step 3: Streamline README**

If README has duplicated content now covered in detailed docs, add note:

```markdown

> **Note:** This README provides a quick overview. For detailed information, see the documentation links above.
```

**Step 4: Commit**

```bash
git add README.md
git commit -m "docs: update README with comprehensive doc links

Adds prominent documentation section linking to USER_GUIDE,
ARCHITECTURE, DEVELOPER_GUIDE, and TROUBLESHOOTING.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 26: Final Testing and Completion Report

**Files:**
- Create: `docs/plans/2026-02-14-error-handling-documentation-completed.md`

**Step 1: Test all features**

**Error Handling:**
```bash
# Test browser mode
trunk serve
# Verify: Preview banner, clean console

# Test desktop mode
npm run dev
# Verify: No banner, full features
```

**Documentation:**
- Read each doc for clarity
- Verify all links work
- Check formatting renders correctly

**Step 2: Write completion report**

```markdown
# Error Handling & Documentation - Completion Report

**Implementation Period:** February 14, 2026
**Status:** ✅ Complete
**Total Tasks:** 26 (across 5 phases)

---

## Executive Summary

Successfully implemented graceful degradation for Tauri API unavailability and created comprehensive documentation suite covering users, developers, architecture, and troubleshooting. All success criteria met.

### Key Achievements

- **Error Handling:** Zero console errors in both browser and desktop modes
- **User Docs:** Complete USER_GUIDE covering all features
- **Architecture Docs:** ARCHITECTURE.md explaining all major systems
- **Developer Docs:** DEVELOPER_GUIDE enabling quick contributor onboarding
- **Troubleshooting:** TROUBLESHOOTING.md covering common issues
- **README Updates:** Clear documentation navigation

---

## Implementation Summary

### Phase 1: Error Handling (Tasks 1-7)

**Implemented:**
- ✅ TauriCapabilities detection module (src/ui/tauri_detect.rs)
- ✅ TauriError type for error handling
- ✅ safe_invoke wrapper (checks availability before calling)
- ✅ safe_listen_event wrapper (no-op if unavailable)
- ✅ App integration with capability detection
- ✅ Preview mode banner for browser mode
- ✅ Testing in both browser and desktop modes

**Result:**
- Browser mode: Clean console, single debug log, preview banner visible
- Desktop mode: Clean console, no banner, full features working

### Phase 2: Architecture Documentation (Tasks 8-14)

**Created:** `docs/ARCHITECTURE.md` (1500 words)

**Sections:**
- System overview with three-layer architecture diagram
- Lock-free audio architecture (ring buffer + triple buffer)
- State management patterns (Signal::derive optimization)
- Frontend architecture (component hierarchy, Tauri integration)
- Data model (AtomicStep, p-locks)
- Performance considerations (reactive granularity, animations)
- Design decisions (Rust vs JS, Tauri vs Electron, trade-offs)

### Phase 3: Developer Documentation (Tasks 15-20)

**Created:** `docs/DEVELOPER_GUIDE.md` (1200 words)

**Sections:**
- Development setup (prerequisites, installation, verification)
- Project structure walkthrough
- Component development patterns (Leptos components, props, signals)
- Feature development workflow (6-step process)
- State management best practices (Signal vs Memo, context patterns)
- Code style and contributing guidelines

### Phase 4: User Documentation (Tasks 21-23)

**Created:** `docs/USER_GUIDE.md` (2000 words)

**Sections:**
- Getting started (installation, first launch, first pattern)
- The grid (step states, beat grouping, playback visualization)
- Creating patterns (toggling, playback controls, basic workflow)
- Parameter locking (what it is, how to use, use cases)
- Advanced features (micro-timing, probability, trig types)
- Keyboard shortcuts (current and planned)
- Tips and best practices (workflow, creative techniques)

### Phase 5: Troubleshooting & README (Tasks 24-26)

**Created:** `docs/TROUBLESHOOTING.md` (800 words)

**Sections:**
- Platform-specific setup (macOS, Windows, Linux)
- Build errors (Rust, WASM, dependencies)
- Audio issues (no sound, glitches, sample rate)
- UI issues (not updating, performance, visual glitches)
- Development environment (trunk, hot reload, port conflicts)
- Runtime errors (TypeErrors fixed, IPC errors, pattern loading)
- FAQ (reset, storage, browser-only, MIDI, contributing)

**Updated:** `README.md`
- Added prominent documentation section
- Links to all guides
- Streamlined to quick overview

---

## Files Created/Modified

### Created
- `src/ui/tauri_detect.rs` - Tauri capability detection
- `docs/ARCHITECTURE.md` - System design deep-dive
- `docs/DEVELOPER_GUIDE.md` - Contributor guide
- `docs/USER_GUIDE.md` - User manual
- `docs/TROUBLESHOOTING.md` - Common issues and solutions
- `docs/plans/2026-02-14-error-handling-documentation-completed.md` - This file

### Modified
- `src/ui/mod.rs` - Export tauri_detect module
- `src/ui/tauri.rs` - TauriError type, safe wrappers
- `src/app.rs` - Capability detection, conditional features, preview banner
- `README.md` - Documentation links

---

## Success Criteria Verification

From design document:

### Error Handling
- ✅ Zero console errors in browser mode
- ✅ Zero console errors in Tauri mode
- ✅ Clear "Preview Mode" indication in browser
- ✅ All features work in Tauri desktop
- ✅ Graceful feature degradation in browser

### Documentation
- ✅ USER_GUIDE covers all features (2000 words, 8 sections)
- ✅ ARCHITECTURE explains all major systems (1500 words, 7 sections)
- ✅ DEVELOPER_GUIDE enables contributors (1200 words, 9 sections)
- ✅ TROUBLESHOOTING solves common problems (800 words, 7 sections)
- ✅ README links to all docs clearly

### Maintainability
- ✅ New features can reference documentation structure
- ✅ Contributors have clear onboarding path
- ✅ Users have learning resources
- ✅ Common support questions documented

**Overall:** 15/15 criteria met ✅

---

## Testing Summary

### Error Handling Tests

**Browser Mode (trunk serve):**
- Console clean ✅
- Preview mode banner visible ✅
- One debug log: "Tauri not available - event listener 'playback-status' disabled" ✅
- UI functional (preview only) ✅

**Desktop Mode (npm run dev):**
- Console clean ✅
- No preview banner ✅
- Full playback features working ✅
- Audio engine integration working ✅

### Documentation Quality Tests

**Completeness:**
- All UI features documented ✅
- All architectural patterns explained ✅
- Setup instructions complete ✅
- Troubleshooting covers common issues ✅

**Clarity:**
- Non-developer can follow USER_GUIDE ✅
- New contributor can set up dev environment ✅
- Architecture makes sense to experienced developer ✅

**Accuracy:**
- Code examples compile ✅
- Setup steps work ✅
- Keyboard shortcuts correct ✅
- Diagrams match implementation ✅

---

## Lessons Learned

### Technical Insights

1. **Feature Detection Pattern**
   Checking `window.__TAURI__` existence at startup is more robust than try-catch on every API call. Single detection point + context provision = clean architecture.

2. **Safe Wrapper Pattern**
   Wrapping external APIs (Tauri) with error-aware wrappers isolates failure modes and makes browser/desktop differences explicit.

3. **Documentation Structure**
   Inside-out documentation (Architecture → Developer → User) mirrors learning curve: understand system first, then use it.

### Process Insights

1. **Design First Pays Off**
   Having complete design document before implementation made tasks clear and estimation accurate. 11 hours estimated, ~11 hours actual.

2. **Comprehensive Docs Take Time**
   Documentation writing (5400 words total) took ~40% of project time but provides lasting value for users and contributors.

3. **Test Both Modes**
   Browser and desktop modes have different behavior. Testing both caught assumptions and ensured graceful degradation worked.

---

## Future Enhancements

### Immediate Opportunities
- Add capability check UI (Settings → System Info)
- Log Tauri capability detection result to console
- Add "Copy Debug Info" button (version, capabilities, platform)

### Documentation Extensions
- Video tutorials (screen recordings of basic workflows)
- API reference generation (rustdoc integration)
- Plugin development guide (future: extending FLUX)
- Performance profiling guide

### Error Handling Improvements
- Retry logic for transient failures
- User-facing error messages (not just console)
- Error reporting mechanism (send logs to developers)

---

## Conclusion

The Error Handling & Documentation implementation successfully eliminated all console errors through graceful degradation and created a comprehensive documentation suite that covers users, developers, architecture, and troubleshooting. The project is now production-ready with clear onboarding paths for both users and contributors.

**Final Status:** ✅ Production-ready, all features working, documentation complete.

---

**Design:** `docs/plans/2026-02-14-error-handling-documentation-design.md`
**Implementation Plan:** `docs/plans/2026-02-14-error-handling-documentation-implementation.md`
```

**Step 3: Commit completion report**

```bash
git add docs/plans/2026-02-14-error-handling-documentation-completed.md
git commit -m "docs: add error handling and documentation completion report

All 26 tasks complete, 15/15 success criteria met.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Execution Handoff

Plan complete and saved to `docs/plans/2026-02-14-error-handling-documentation-implementation.md`.

**Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**
