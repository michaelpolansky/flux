# Grid UI/UX Enhancements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement comprehensive grid UI/UX enhancements with state-first architecture for clear visual feedback, selection indication, and playback visualization.

**Architecture:** State-first approach - build PlaybackState and GridUIState layers, then create visual components that reactively consume this state. Extract step rendering into dedicated component for better organization.

**Tech Stack:** Leptos 0.7, Tailwind CSS v4, Rust signals, Tauri events

---

## Phase 1: State Foundation

### Task 1: Create PlaybackState Type

**Files:**
- Create: `src/ui/state.rs`
- Modify: `src/ui/mod.rs`

**Step 1: Create state module file**

Create `src/ui/state.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,        // 0-15
    pub triggered_tracks: [bool; 4],    // Which tracks fired this step
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            current_position: 0,
            triggered_tracks: [false; 4],
        }
    }
}
```

**Step 2: Add module declaration**

In `src/ui/mod.rs`, add after existing module declarations:
```rust
pub mod state;
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 4: Commit**

```bash
git add src/ui/state.rs src/ui/mod.rs
git commit -m "feat: add PlaybackState type for playback visualization"
```

---

### Task 2: Add PlaybackState Context to App

**Files:**
- Modify: `src/app.rs:1-60`

**Step 1: Import PlaybackState**

Add to imports at top of `src/app.rs`:
```rust
use crate::ui::state::PlaybackState;
```

**Step 2: Create PlaybackState signal**

Add after creating `selected_step` signal (around line 29):
```rust
let (playback_state, set_playback_state) = signal(PlaybackState::new());
```

**Step 3: Provide PlaybackState context**

Add after providing pattern signal (around line 37):
```rust
provide_context(playback_state);
provide_context(set_playback_state);
```

**Step 4: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 5: Commit**

```bash
git add src/app.rs
git commit -m "feat: add PlaybackState signal and context to app"
```

---

### Task 3: Update Tauri Event Listener for PlaybackState

**Files:**
- Modify: `src/app.rs:13-17,50-59`

**Step 1: Extend AudioSnapshot struct**

Modify `AudioSnapshot` struct to include triggered tracks:
```rust
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct AudioSnapshot {
    current_step: usize,
    is_playing: bool,
    triggered_tracks: Option<[bool; 4]>,  // Optional for backward compatibility
}
```

**Step 2: Update Tauri event listener**

Modify the Effect that listens to playback events (around line 50-59):
```rust
Effect::new(move |_| {
    spawn_local(async move {
        use crate::ui::tauri::listen_event;
        listen_event("playback-status", move |event: AudioSnapshot| {
            // Update current_step (existing)
            set_current_step.set(event.current_step);

            // Update PlaybackState (new)
            set_playback_state.update(|state| {
                state.is_playing = event.is_playing;
                state.current_position = event.current_step;
                state.triggered_tracks = event.triggered_tracks.unwrap_or([false; 4]);
            });
        }).await;
    });
});
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 4: Commit**

```bash
git add src/app.rs
git commit -m "feat: wire PlaybackState to Tauri playback events"
```

---

### Task 4: Create GridUIState Type

**Files:**
- Modify: `src/ui/state.rs`

**Step 1: Add GridUIState types**

Append to `src/ui/state.rs`:
```rust
#[derive(Clone, Debug)]
pub struct TriggerEvent {
    pub track: usize,
    pub step: usize,
    pub timestamp: f64,  // Using f64 for JavaScript timestamp compatibility
}

#[derive(Clone, Debug, Default)]
pub struct GridUIState {
    pub hovered_step: Option<(usize, usize)>,
    pub recent_triggers: Vec<TriggerEvent>,
}

impl GridUIState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_trigger(&mut self, track: usize, step: usize, timestamp: f64) {
        self.recent_triggers.push(TriggerEvent { track, step, timestamp });
    }

    pub fn cleanup_old_triggers(&mut self, current_time: f64, max_age_ms: f64) {
        self.recent_triggers.retain(|trigger| {
            current_time - trigger.timestamp < max_age_ms
        });
    }
}
```

**Step 2: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 3: Commit**

```bash
git add src/ui/state.rs
git commit -m "feat: add GridUIState for UI-only state management"
```

---

## Phase 2: Visual Refinement

### Task 5: Update Grid Color Palette and Selection Ring

**Files:**
- Modify: `src/ui/components/grid.rs:54-73`

**Step 1: Update selection ring to amber**

Find the selection ring code (around line 67-70) and change:
```rust
let selection_classes = if is_selected {
    "ring-3 ring-amber-400"  // Changed from ring-2 ring-blue-400
} else {
    ""
};
```

**Step 2: Add hover scale to base classes**

Update base_classes (around line 54):
```rust
let base_classes = "w-10 h-10 rounded-lg transition-all duration-100 flex items-center justify-center select-none active:scale-95 hover:scale-105 focus:outline-none border";
```

**Step 3: Test visually**

Run trunk serve and verify:
- Selection ring is now amber and thicker
- Hover scales step slightly

**Step 4: Commit**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: enhance selection ring to amber ring-3 and add hover scale"
```

---

### Task 6: Add Beat Grouping Markers

**Files:**
- Modify: `src/ui/components/grid.rs:39-91`

**Step 1: Add beat marker logic to step rendering**

Modify the button classes computation to include beat markers:
```rust
let base_classes = "w-10 h-10 rounded-lg transition-all duration-100 flex items-center justify-center select-none active:scale-95 hover:scale-105 focus:outline-none border";

// Beat grouping: add right border every 4 steps (except last)
let beat_marker = if (step_idx + 1) % 4 == 0 && step_idx < 15 {
    "border-r-2 border-zinc-600"
} else {
    ""
};

let is_active_note = is_active();
let is_selected = sequencer_state.selected_step.get()
    .map(|(tid, sidx)| tid == track_idx && sidx == step_idx)
    .unwrap_or(false);

let state_classes = if is_active_note {
    "bg-blue-500 border-blue-400 hover:bg-blue-400"
} else {
    "bg-zinc-800 border-zinc-700 hover:bg-zinc-700"
};

let selection_classes = if is_selected {
    "ring-3 ring-amber-400"
} else {
    ""
};

format!("{} {} {} {}", base_classes, state_classes, selection_classes, beat_marker)
```

**Step 2: Test visually**

Run trunk serve and verify:
- Vertical line appears after every 4th step
- Visual rhythm: ●●●● | ●●●● | ●●●● | ●●●●

**Step 3: Commit**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: add beat grouping markers every 4 steps"
```

---

## Phase 3: New Components

### Task 7: Extract GridStep Component

**Files:**
- Create: `src/ui/components/grid_step.rs`
- Modify: `src/ui/components/mod.rs`
- Modify: `src/ui/components/grid.rs:39-91`

**Step 1: Create GridStep component**

Create `src/ui/components/grid_step.rs`:
```rust
use leptos::prelude::*;

#[component]
pub fn GridStep(
    track_idx: usize,
    step_idx: usize,
) -> impl IntoView {
    // Use contexts
    let sequencer_state = use_context::<crate::app::SequencerState>()
        .expect("SequencerState context not found");
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>()
        .expect("Pattern context not found");

    // Hardcode subtrack 0
    let subtrack_id = 0;

    // Reactive check for active state
    let is_active = move || {
        pattern_signal.with(|p| {
            p.tracks.get(track_idx)
                .and_then(|t| t.subtracks.get(subtrack_id))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.trig_type != crate::shared::models::TrigType::None)
                .unwrap_or(false)
        })
    };

    view! {
        <button
            class=move || {
                let base_classes = "w-10 h-10 rounded-lg transition-all duration-100 flex items-center justify-center select-none active:scale-95 hover:scale-105 focus:outline-none border";

                // Beat grouping marker
                let beat_marker = if (step_idx + 1) % 4 == 0 && step_idx < 15 {
                    "border-r-2 border-zinc-600"
                } else {
                    ""
                };

                let is_active_note = is_active();
                let is_selected = sequencer_state.selected_step.get()
                    .map(|(tid, sidx)| tid == track_idx && sidx == step_idx)
                    .unwrap_or(false);

                let state_classes = if is_active_note {
                    "bg-blue-500 border-blue-400 hover:bg-blue-400"
                } else {
                    "bg-zinc-800 border-zinc-700 hover:bg-zinc-700"
                };

                let selection_classes = if is_selected {
                    "ring-3 ring-amber-400"
                } else {
                    ""
                };

                format!("{} {} {} {}", base_classes, state_classes, selection_classes, beat_marker)
            }
            on:click=move |_| {
                sequencer_state.selected_step.set(Some((track_idx, step_idx)));
            }
        >
            <span class=move || {
                if is_active() {
                    "text-white text-lg"
                } else {
                    "text-zinc-600 text-lg"
                }
            }>
                {move || if is_active() { "●" } else { "○" }}
            </span>
        </button>
    }
}
```

**Step 2: Add module declaration**

In `src/ui/components/mod.rs`, add:
```rust
pub mod grid_step;
```

**Step 3: Update grid.rs to use GridStep**

Replace the button view in grid.rs (around lines 51-90) with:
```rust
view! {
    <crate::ui::components::grid_step::GridStep
        track_idx=track_idx
        step_idx=step_idx
    />
}
```

**Step 4: Verify compilation and functionality**

Run: `cargo check && trunk serve`
Expected: Builds successfully, grid works same as before

**Step 5: Commit**

```bash
git add src/ui/components/grid_step.rs src/ui/components/mod.rs src/ui/components/grid.rs
git commit -m "refactor: extract GridStep component for cleaner organization"
```

---

### Task 8: Create StepBadge Component

**Files:**
- Create: `src/ui/components/step_badge.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create StepBadge component**

Create `src/ui/components/step_badge.rs`:
```rust
use leptos::prelude::*;

#[component]
pub fn StepBadge(
    #[prop(into)] track: Signal<usize>,
    #[prop(into)] step: Signal<usize>,
    #[prop(into)] visible: Signal<bool>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let base = "absolute -top-6 left-1/2 -translate-x-1/2 bg-zinc-900/90 backdrop-blur text-amber-400 text-xs px-2 py-0.5 rounded whitespace-nowrap transition-opacity duration-150 pointer-events-none";
                let opacity = if visible.get() {
                    "opacity-100"
                } else {
                    "opacity-0"
                };
                format!("{} {}", base, opacity)
            }
        >
            {move || format!("T{}・S{}", track.get() + 1, step.get() + 1)}
        </div>
    }
}
```

**Step 2: Add module declaration**

In `src/ui/components/mod.rs`, add:
```rust
pub mod step_badge;
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 4: Commit**

```bash
git add src/ui/components/step_badge.rs src/ui/components/mod.rs
git commit -m "feat: create StepBadge component for selection feedback"
```

---

### Task 9: Add StepBadge to Grid

**Files:**
- Modify: `src/ui/components/grid.rs:14-98`

**Step 1: Import StepBadge**

Add import at top of grid.rs:
```rust
use crate::ui::components::step_badge::StepBadge;
```

**Step 2: Add StepBadge rendering in grid**

After the grid tracks container closing tag (around line 97), add before the outer closing tag:
```rust
        </div>

        // Step info badge (conditionally rendered when step selected)
        {move || {
            sequencer_state.selected_step.get().map(|(track, step)| {
                view! {
                    <div class="relative">
                        <StepBadge
                            track=Signal::derive(move || track)
                            step=Signal::derive(move || step)
                            visible=Signal::derive(move || sequencer_state.selected_step.get().is_some())
                        />
                    </div>
                }.into_any()
            })
        }}
    </div>
```

**Step 3: Test visually**

Run trunk serve and verify:
- Clicking a step shows "T1・S3" badge above grid
- Badge fades in/out smoothly

**Step 4: Commit**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: add StepBadge to grid for selection feedback"
```

---

### Task 10: Create PlayheadIndicator Component

**Files:**
- Create: `src/ui/components/playhead_indicator.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create PlayheadIndicator component**

Create `src/ui/components/playhead_indicator.rs`:
```rust
use leptos::prelude::*;

#[component]
pub fn PlayheadIndicator(
    #[prop(into)] position: Signal<usize>,
    #[prop(into)] is_playing: Signal<bool>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let base = "absolute top-0 bg-emerald-500/20 w-10 h-full transition-transform duration-100 pointer-events-none";
                let visibility = if is_playing.get() {
                    "opacity-100"
                } else {
                    "opacity-0"
                };
                format!("{} {}", base, visibility)
            }
            style:transform=move || {
                // Calculate pixel offset: 40px (step width) + 2px (gap) per step
                let offset = position.get() * 42; // 40px + 2px gap
                format!("translateX({}px)", offset)
            }
        />
    }
}
```

**Step 2: Add module declaration**

In `src/ui/components/mod.rs`, add:
```rust
pub mod playhead_indicator;
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 4: Commit**

```bash
git add src/ui/components/playhead_indicator.rs src/ui/components/mod.rs
git commit -m "feat: create PlayheadIndicator component for playback visualization"
```

---

## Phase 4: Playback Visualization

### Task 11: Add PlayheadIndicator to Grid

**Files:**
- Modify: `src/ui/components/grid.rs:1-25`

**Step 1: Import PlayheadIndicator and PlaybackState**

Add imports at top of grid.rs:
```rust
use crate::ui::components::playhead_indicator::PlayheadIndicator;
use crate::ui::state::PlaybackState;
```

**Step 2: Consume PlaybackState context**

Add after consuming pattern_signal (around line 9):
```rust
let playback_state = use_context::<ReadSignal<PlaybackState>>()
    .expect("PlaybackState context not found");
```

**Step 3: Add PlayheadIndicator to grid**

Wrap the grid tracks container in a relative positioned div and add playhead:
```rust
// Grid of 4 tracks × 16 steps
<div class="relative">  // Add this wrapper
    <PlayheadIndicator
        position=Signal::derive(move || playback_state.get().current_position)
        is_playing=Signal::derive(move || playback_state.get().is_playing)
    />

    <div class="flex flex-col gap-[2px]">
        // ... existing For loop for tracks ...
    </div>
</div>
```

**Step 4: Test visually**

Run trunk serve and start playback:
- Green bar should move across grid
- Should only be visible when playing

**Step 5: Commit**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: add playhead indicator to grid for playback position"
```

---

### Task 12: Add Playing Step Highlight

**Files:**
- Modify: `src/ui/components/grid_step.rs:6-65`

**Step 1: Import and consume PlaybackState**

Add import at top of grid_step.rs:
```rust
use crate::ui::state::PlaybackState;
```

Add context consumption after pattern_signal:
```rust
let playback_state = use_context::<ReadSignal<PlaybackState>>()
    .expect("PlaybackState context not found");
```

**Step 2: Add playing state check**

After the is_active check, add:
```rust
let is_playing_here = move || {
    playback_state.get().is_playing &&
    playback_state.get().current_position == step_idx
};
```

**Step 3: Add playing highlight to classes**

Update the class computation to include playing state:
```rust
let state_classes = if is_active_note {
    "bg-blue-500 border-blue-400 hover:bg-blue-400"
} else {
    "bg-zinc-800 border-zinc-700 hover:bg-zinc-700"
};

// Playing highlight (subtle green underlay)
let playing_classes = if is_playing_here() {
    "bg-emerald-500/30"
} else {
    ""
};

let selection_classes = if is_selected {
    "ring-3 ring-amber-400"
} else {
    ""
};

format!("{} {} {} {} {}", base_classes, playing_classes, state_classes, selection_classes, beat_marker)
```

**Step 4: Test with playback**

Run trunk serve and start playback:
- Current playing step should have subtle green tint
- Should move with playhead

**Step 5: Commit**

```bash
git add src/ui/components/grid_step.rs
git commit -m "feat: add playing step highlight with emerald tint"
```

---

### Task 13: Add Trigger Pulse Animation

**Files:**
- Modify: `src/ui/components/grid_step.rs:1-80`
- Modify: `tailwind.config.js`

**Step 1: Add GridUIState to grid_step**

Import and use GridUIState:
```rust
use crate::ui::state::GridUIState;
use leptos::wasm_bindgen::JsCast;
use web_sys::window;
```

Add context (this will be created in grid.rs first, so this step depends on Task 14):
```rust
let grid_ui_state = use_context::<RwSignal<GridUIState>>()
    .expect("GridUIState context not found");
```

**Step 2: Check if recently triggered**

Add reactive check for trigger animation:
```rust
let is_triggered = move || {
    grid_ui_state.with(|state| {
        state.recent_triggers.iter().any(|t| {
            t.track == track_idx && t.step == step_idx
        })
    })
};
```

**Step 3: Add trigger flash classes**

Update class computation to include trigger:
```rust
// Trigger pulse animation
let trigger_classes = if is_triggered() {
    "ring-2 ring-white/50 animate-pulse-once"
} else {
    ""
};

format!("{} {} {} {} {} {}", base_classes, playing_classes, state_classes, selection_classes, beat_marker, trigger_classes)
```

**Step 4: Add custom animation to Tailwind**

In `tailwind.config.js`, add to theme.extend:
```js
module.exports = {
  theme: {
    extend: {
      animation: {
        'pulse-once': 'pulse 0.15s ease-out',
      },
    },
  },
  // ... rest of config
}
```

**Step 5: Verify compilation**

Run: `cargo check`
Expected: Builds successfully (may need Task 14 first)

**Step 6: Commit**

```bash
git add src/ui/components/grid_step.rs tailwind.config.js
git commit -m "feat: add trigger pulse animation to steps"
```

---

### Task 14: Add GridUIState to Grid and Trigger Logic

**Files:**
- Modify: `src/ui/components/grid.rs:1-15`

**Step 1: Import and create GridUIState**

Add import:
```rust
use crate::ui::state::GridUIState;
use leptos::wasm_bindgen::JsCast;
use web_sys::window;
```

Create local state after consuming contexts:
```rust
let grid_ui_state = RwSignal::new(GridUIState::new());
provide_context(grid_ui_state);
```

**Step 2: Add effect to track triggers**

Add Effect to detect position changes and add triggers:
```rust
Effect::new(move |prev_position: Option<usize>| {
    let current_pos = playback_state.get().current_position;

    // When position changes, check for triggered steps
    if let Some(prev) = prev_position {
        if prev != current_pos {
            // Get current timestamp
            let timestamp = window()
                .and_then(|w| w.performance())
                .map(|p| p.now())
                .unwrap_or(0.0);

            // Check each track for active steps at current position
            pattern_signal.with(|p| {
                for track_idx in 0..4 {
                    if let Some(track) = p.tracks.get(track_idx) {
                        if let Some(subtrack) = track.subtracks.get(0) {
                            if let Some(step) = subtrack.steps.get(current_pos) {
                                if step.trig_type != crate::shared::models::TrigType::None {
                                    grid_ui_state.update(|state| {
                                        state.add_trigger(track_idx, current_pos, timestamp);
                                    });
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    current_pos
});
```

**Step 3: Add cleanup effect for old triggers**

Add another Effect to clean up triggers after 150ms:
```rust
Effect::new(move |_| {
    use leptos::set_interval;
    use std::time::Duration;

    set_interval(
        move || {
            let current_time = window()
                .and_then(|w| w.performance())
                .map(|p| p.now())
                .unwrap_or(0.0);

            grid_ui_state.update(|state| {
                state.cleanup_old_triggers(current_time, 150.0);
            });
        },
        Duration::from_millis(50),
    );
});
```

**Step 4: Verify compilation**

Run: `cargo check`
Expected: Builds successfully

**Step 5: Commit**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: add trigger detection and cleanup logic to grid"
```

---

## Phase 5: Polish & Testing

### Task 15: Fix PlayheadIndicator Positioning

**Files:**
- Modify: `src/ui/components/playhead_indicator.rs:13-21`

**Step 1: Adjust positioning calculation**

The playhead needs to account for track labels. Update the transform:
```rust
style:transform=move || {
    // Account for track labels (32px + 8px margin = 40px)
    // Each step is 40px wide + 2px gap
    let label_offset = 40;  // Track label width + margin
    let step_offset = position.get() * 42; // 40px step + 2px gap
    format!("translateX({}px)", label_offset + step_offset)
}
```

**Step 2: Test visually**

Run trunk serve and verify playhead aligns correctly with steps

**Step 3: Commit**

```bash
git add src/ui/components/playhead_indicator.rs
git commit -m "fix: adjust playhead position to account for track labels"
```

---

### Task 16: Test All State Combinations

**Files:**
- Manual testing

**Step 1: Test inactive step**
- Click on empty step → should show amber ring + badge
- Hover → should scale slightly

**Step 2: Test active step**
- Click on step with note → should show amber ring + badge + blue background
- Hover → should scale and brighten

**Step 3: Test playback**
- Start playback → green playhead should move
- Active steps should pulse when triggered
- Playing step should have green tint

**Step 4: Test multiple states**
- Select a step
- Start playback when playhead is on that step
- Should see: blue background + green tint + amber ring + badge

**Step 5: Test beat grouping**
- Visual lines should appear after steps 4, 8, 12
- Should create clear visual rhythm

**Step 6: Document any issues**

Create issues for any bugs found

**Step 7: Commit manual test notes**

```bash
echo "Manual testing completed - all visual states working" > docs/test-notes.md
git add docs/test-notes.md
git commit -m "docs: add manual test notes for grid UI/UX"
```

---

### Task 17: Performance Check

**Files:**
- Manual testing

**Step 1: Profile with browser DevTools**

1. Open browser DevTools (F12)
2. Go to Performance tab
3. Start recording
4. Start playback in FLUX
5. Let it run for 10-15 seconds
6. Stop recording

**Step 2: Check frame rate**

- Look for dropped frames
- Verify consistent 60fps during playback
- Check CPU usage is reasonable

**Step 3: Check memory**

- Take heap snapshot before playback
- Start playback, run for 30 seconds
- Take another snapshot
- Verify no memory leaks (trigger cleanup working)

**Step 4: Document results**

```bash
echo "Performance check: 60fps maintained, no memory leaks detected" >> docs/test-notes.md
git add docs/test-notes.md
git commit -m "docs: add performance test results"
```

---

### Task 18: Edge Case Testing

**Files:**
- Manual testing

**Step 1: Test playhead wrap**

- Let playback reach step 15
- Verify smooth transition to step 0
- No visual glitches

**Step 2: Test rapid selection changes**

- Rapidly click different steps
- Badge should update smoothly
- No lag or flicker

**Step 3: Test with no active steps**

- Clear all triggers
- Start playback
- Playhead should move but no pulses
- No errors in console

**Step 4: Test step badge positioning**

- Select steps at different positions
- Badge should always be visible and centered
- Shouldn't overflow grid bounds

**Step 5: Document edge cases**

```bash
echo "Edge cases tested: playhead wrap, rapid selection, empty pattern, badge positioning" >> docs/test-notes.md
git add docs/test-notes.md
git commit -m "docs: add edge case test results"
```

---

## Success Criteria Verification

After completing all tasks, verify:

- [ ] Grid clearly shows which step is selected (amber ring + badge)
- [ ] Playback position is instantly visible (emerald playhead + step highlight)
- [ ] Steps pulse when triggered (white flash animation)
- [ ] Beat groupings provide visual rhythm (every 4 steps)
- [ ] All interactions feel smooth and responsive (60fps)
- [ ] State management is clean and extensible

---

## Notes

- Tasks 1-4: Foundation - can be done independently
- Tasks 5-6: Visual refinement - builds on foundation
- Tasks 7-10: Components - can be done in parallel after foundation
- Tasks 11-14: Playback - requires foundation and components
- Tasks 15-18: Polish - final integration and testing

**Estimated total time:** 4-6 hours for implementation, 1-2 hours for testing/polish

**Dependencies:**
- Requires existing Leptos 0.7, Tailwind v4, Tauri setup
- May need backend update for `triggered_tracks` in playback events (currently optional)
