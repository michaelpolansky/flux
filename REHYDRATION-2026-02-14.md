# FLUX Sequencer - Session Rehydration Prompt (2026-02-14)

## Session Summary

**Goal:** Fix critical Tauri desktop app bug preventing Play button from working, then implement Play/Pause toggle UI.

**Status:** ✅ COMPLETE - Play button works, toggles to Pause during playback, all features functional.

---

## What We Fixed

### Critical Bug: Tauri Detection Timing Issue

**Symptom:**
- Desktop app (`npm run dev`) showed "Tauri not available - playback command disabled" in console
- Play button did nothing when clicked
- No errors in Rust code, no panics, just silent failure

**Root Cause:**
Initialization order race condition:
1. Tauri desktop app loads HTML
2. Leptos WASM initializes
3. `App()` component mounts and calls `detect_tauri()`
4. `detect_tauri()` checks `window.__TAURI__` ← **doesn't exist yet!**
5. Result cached in Leptos context as "not available"
6. Tauri injects `window.__TAURI__` ← happens AFTER detection
7. All subsequent feature checks use cached "false" value
8. Play button thinks Tauri unavailable, refuses to send commands

**Solution Applied:**
Changed from **cached detection** to **runtime detection**:

```rust
// src/ui/tauri.rs - BEFORE (broken)
fn is_tauri_available() -> bool {
    use_context::<TauriCapabilities>()
        .map(|caps| caps.available)
        .unwrap_or(false)  // Uses cached value from component mount
}

// AFTER (fixed)
fn is_tauri_available() -> bool {
    // Check at call time, not at component mount
    crate::ui::tauri_detect::detect_tauri().available
}
```

**Additional Fixes:**
1. Inlined JavaScript wrappers in `index.html` (trunk wasn't serving external script)
2. Fixed `.unwrap()` panic risk in `src/services/audio.rs`
3. Added proper error handling for serialization failures

**Commits:**
- `676f4c4` - fix: resolve Tauri detection timing issue preventing Play button
- `b743268` - feat: add Play/Pause toggle button with reactive state

---

## Feature: Play/Pause Toggle Button

**Implementation:**
Replaced separate static Play and Stop buttons with dynamic Play/Pause toggle.

**Code Pattern:**
```rust
// src/ui/components/toolbar.rs
let playback_state = use_context::<ReadSignal<crate::ui::state::PlaybackState>>()
    .expect("PlaybackState context not found");

<button
    on:click=move |_| {
        let is_playing = playback_state.get().is_playing;
        leptos::task::spawn_local(async move {
            crate::services::audio::set_playback_state(!is_playing).await;
        });
    }
    class=move || {
        if playback_state.get().is_playing {
            "... bg-amber-600 ..."  // Pause button (amber)
        } else {
            "... bg-green-600 ..."  // Play button (green)
        }
    }
>
    {move || if playback_state.get().is_playing { "⏸" } else { "▶" }}
</button>
```

**Behavior:**
- Shows **▶ Play** (green) when stopped
- Shows **⏸ Pause** (amber) when playing
- Toggles playback state on click
- Updates reactively when backend sends `playback-status` events
- Stop button (■) remains separate

**Note on Leptos Type Compatibility:**
Cannot use `if/else` with two different `view!` macros (creates incompatible closure types). Solution: single button with reactive properties (`class=move ||`, text content with closures).

---

## Current Architecture State

### Tauri Integration (Dual-Mode Support)

**Detection Flow:**
1. `tauri_detect.rs` - exports `detect_tauri()` function
2. Checks `window.__TAURI__` existence at runtime
3. Returns `TauriCapabilities { available: bool, audio_enabled: bool, events_enabled: bool }`
4. Used by safety wrappers to determine desktop vs browser mode

**Safety Wrappers (Two-Layer Defense):**

**Layer 1: JavaScript FFI Boundary** (`index.html` inline script)
```javascript
window.__TAURI_SAFE__ = {
  invoke: async function(cmd, args) {
    if (typeof window.__TAURI__ === 'undefined' || ...) {
      throw new Error('Tauri not available');
    }
    return await window.__TAURI__.core.invoke(cmd, args);
  },
  // ... dialogSave, dialogOpen, listen
};
```

**Layer 2: Rust Safety Wrappers** (`src/ui/tauri.rs`)
```rust
pub async fn safe_invoke(cmd: &str, args: JsValue) -> Result<JsValue, TauriError> {
    if !is_tauri_available() {
        return Err(TauriError::NotAvailable);
    }
    invoke_with_error(cmd, args).await
        .map_err(|e| TauriError::InvokeFailed(format!("{:?}", e)))
}
```

**Why Two Layers:**
- JavaScript layer catches errors before WASM executes (prevents TypeErrors)
- Rust layer provides typed error handling and graceful degradation
- Browser mode: Both layers return errors, UI shows "Preview Mode" banner
- Desktop mode: Both layers succeed, full features enabled

### Event Flow (Playback State Updates)

```
Backend (Rust)                Frontend (Leptos WASM)
──────────────                ─────────────────────

[AudioEngine]
    │
    ├─> Heartbeat every 1000 ticks
    │   (shows in logs: "Heartbeat: Tick 1000, Drift: 0.000 ms")
    │
    └─> On playback change:
        emit("playback-status", AudioSnapshot {
            current_step: usize,
            is_playing: bool,
            triggered_tracks: Option<[bool; 4]>
        })
                │
                │ (Tauri IPC)
                ▼
        window.__TAURI__.event.listen("playback-status", ...)
                │
                ▼
        safe_listen_event() catches event
                │
                ▼
        Updates Leptos signals:
        - set_current_step.set(normalized_position)
        - set_playback_state.update(|state| {
            state.is_playing = event.is_playing;
            ...
          })
                │
                ▼
        UI reactively updates:
        - Playhead indicator moves
        - Play button → Pause button
        - Button color green → amber
```

### File Structure (Key Files)

```
flux-app/
├── index.html                      # Inlined __TAURI_SAFE__ wrappers
├── src/
│   ├── app.rs                      # Main component, provides context
│   ├── services/
│   │   └── audio.rs                # set_playback_state() - fixed .unwrap()
│   └── ui/
│       ├── tauri.rs                # safe_invoke(), is_tauri_available()
│       ├── tauri_detect.rs         # detect_tauri() - runtime check
│       ├── state.rs                # PlaybackState struct
│       └── components/
│           └── toolbar.rs          # Play/Pause toggle button
├── TROUBLESHOOTING.md              # Updated with timing issue docs
└── ARCHITECTURE.md                 # Full system documentation
```

---

## Critical Patterns & Decisions

### 1. Runtime Detection Over Cached Detection

**Why:** Tauri's initialization timing is unpredictable relative to WASM load.

**Pattern:**
```rust
// ❌ WRONG - Caches at component mount
let capabilities = detect_tauri();
provide_context(capabilities);
// Later: use_context() returns stale value

// ✅ RIGHT - Check when needed
fn is_tauri_available() -> bool {
    detect_tauri().available  // Fresh check every time
}
```

### 2. Error Handling Without Unwrap

**Why:** `.unwrap()` in WASM causes silent panics, breaking entire UI.

**Pattern:**
```rust
// ❌ WRONG
let args = serde_wasm_bindgen::to_value(&data).unwrap();

// ✅ RIGHT
let args = match serde_wasm_bindgen::to_value(&data) {
    Ok(v) => v,
    Err(e) => {
        web_sys::console::error_1(&format!("Serialization failed: {:?}", e).into());
        return;
    }
};
```

### 3. Leptos Reactive Properties

**Why:** `if/else` with different `view!` macros creates incompatible types.

**Pattern:**
```rust
// ❌ WRONG - Type error
if condition {
    view! { <Button on:click=|_| {...} /> }
} else {
    view! { <Button on:click=|_| {...} /> }
}

// ✅ RIGHT - Single view with reactive properties
<Button
    class=move || if condition { "class-a" } else { "class-b" }
    on:click=move |_| { /* handler */ }
>
    {move || if condition { "Text A" } else { "Text B" }}
</Button>
```

### 4. Async Command Handling in Leptos

**Pattern:**
```rust
on:click=move |_| {
    leptos::task::spawn_local(async move {
        crate::services::audio::set_playback_state(true).await;
    });
}
```

**Why:** Tauri commands are async, must spawn local task in event handler.

---

## Verification & Testing

**Desktop Mode (Full Features):**
```bash
npm run dev  # Starts Tauri desktop app
```

**Tests to run:**
1. ✅ Click Play → Button changes to Pause (amber), logs show "Step: 0 [TRIG]"
2. ✅ Click Pause → Button changes to Play (green), logs show heartbeat only
3. ✅ Click Stop → Playback stops, button returns to Play
4. ✅ Save button → Native save dialog opens
5. ✅ Load button → Native load dialog opens
6. ✅ Grid clicks → Steps toggle, MIDI commands logged
7. ✅ No console errors about "Tauri not available"

**Browser Mode (Preview Only):**
```bash
trunk serve  # Opens http://localhost:1420
```

**Expected behavior:**
- Yellow "Preview Mode" banner visible
- Play button shows "Tauri not available" in console (expected)
- UI renders correctly for development
- No TypeErrors (graceful degradation working)

**Console Commands for Debugging:**
```javascript
// Check Tauri availability
typeof window.__TAURI__        // Desktop: "object", Browser: "undefined"
typeof window.__TAURI_SAFE__   // Both: "object" (inlined in HTML)

// Check playback state
// (Open Leptos DevTools if available, or add console.log in code)
```

---

## Known Issues & Next Steps

### Fixed in This Session ✅
- Tauri detection timing race condition
- Play button not working in desktop mode
- Missing Play/Pause toggle UI
- `.unwrap()` panic risk in audio.rs
- JavaScript wrapper loading issue (trunk serve)

### Remaining Known Issues
- **MIDI thread priority warning:** "Failed to set MIDI Engine thread priority: OS(1)"
  - Non-critical: macOS requires elevated permissions for realtime priority
  - Workaround: Accept warning or run with sudo (not recommended)

- **High jitter warnings:** "WARNING: High Jitter Detected (>0.5ms)"
  - Expected during startup as system stabilizes
  - Not critical for functionality, monitoring for performance

- **Unused imports warnings:**
  - `TauriCapabilities` in app.rs (line 13)
  - `wasm_bindgen::prelude::*` in toolbar.rs (line 2)
  - Non-critical: can run `cargo fix` to clean up

### Potential Future Improvements
- Add visual feedback for playhead during playback (current step highlight)
- Parameter locking UI (backend supports, frontend incomplete)
- LFO waveform designer integration
- MIDI input/output configuration UI
- BPM adjustment controls
- Pattern save/load with file browser
- Keyboard shortcuts for playback control

---

## Tech Stack Reference

**Frontend:**
- Leptos 0.7 (Rust WASM reactive framework)
- Tailwind CSS 4.1.18 (utility-first CSS)
- Trunk 0.21.14 (WASM bundler/dev server)

**Desktop Runtime:**
- Tauri 2.10.2 (Rust-based Electron alternative)
- wry 0.54.1 (WebView wrapper)
- tauri-runtime-wry 2.10.0

**Audio Engine:**
- cpal 0.15.3 (cross-platform audio I/O)
- coremidi 0.6.0 (macOS MIDI)
- rtrb 0.3.2 (lock-free ring buffer)
- triple_buffer 8.1.1 (wait-free triple buffering)

**Build Tools:**
- Rust 1.85+ (stable toolchain)
- cargo (Rust package manager)
- rustup (Rust version manager)
- npm (Node package manager for Tauri CLI)

---

## Git History (This Session)

```
b743268 feat: add Play/Pause toggle button with reactive state
676f4c4 fix: resolve Tauri detection timing issue preventing Play button
e6149c3 (previous HEAD before session)
```

**Previous Work (Before This Session):**
- 17 commits of error handling and documentation
- Tasks 56-88 completed (error handling + comprehensive docs)
- All unsafe Tauri invoke calls migrated to safe wrappers
- Created ARCHITECTURE.md (1,408 lines)
- Created DEVELOPER_GUIDE.md (1,483 lines)
- Created USER_GUIDE.md (349 lines)
- Created TROUBLESHOOTING.md (626 lines → now 677 lines)
- Created LOCK_FREE_AUDIO.md (602 lines)

---

## How to Use This Rehydration Prompt

**When starting a new session:**
1. Share this entire file with Claude
2. Say: "Continue working on FLUX sequencer. Read REHYDRATION-2026-02-14.md for context."
3. Claude will have full context of what was accomplished and current state

**Key phrases for quick context:**
- "We fixed the Tauri detection timing issue"
- "Play/Pause toggle is implemented and working"
- "All desktop features functional, tested and verified"
- "Documentation updated in TROUBLESHOOTING.md"

**Current working directory:** `/Users/michaelpolansky/Development/flux/flux-app`

**Current branch:** `main` (all changes committed)

**App status:** Running in background (task b4fa8d7), playback functional

---

## Last Verified State

**Date:** 2026-02-14
**Time:** ~18:08 PST
**Commits:** All work committed to main branch
**App State:** Running via `npm run dev`, sequencer triggering steps
**Tests:** Play/Pause toggle verified working
**Logs:** "Step: 0 [TRIG] Freq: 261.63" messages confirming playback

---

**End of Rehydration Prompt**
