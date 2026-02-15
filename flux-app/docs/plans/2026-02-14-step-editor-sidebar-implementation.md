# Step Editor Sidebar Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement left-column step editor sidebar for per-step parameter editing with Cubase-inspired layout.

**Architecture:** Create new `StepEditorSidebar` component that reads `selected_step` from context and displays 5 step parameters (Note, Velocity, Length, Probability, Micro-timing) using direct field access on `AtomicStep`. Modify `Grid` component to use 2-column layout. Remove existing `StepInspector` component.

**Tech Stack:** Leptos 0.7 (reactive Rust WASM), Tailwind CSS, existing form controls (`InlineParam`, `ParamLabel`, `NumberInput`)

---

## Task 1: Create StepEditorSidebar Component Skeleton

**Files:**
- Create: `src/ui/components/step_editor_sidebar.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create component file with empty state**

Create `src/ui/components/step_editor_sidebar.rs`:

```rust
use leptos::prelude::*;
use crate::app::SequencerState;

#[component]
pub fn StepEditorSidebar() -> impl IntoView {
    let sequencer_state = use_context::<SequencerState>()
        .expect("SequencerState context not found");
    let selected_step = sequencer_state.selected_step;

    view! {
        <div class="w-60 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col">
            {move || {
                if let Some((track_id, step_idx)) = selected_step.get() {
                    view! {
                        <div>
                            <div class="flex items-center justify-between mb-4">
                                <div class="flex flex-col">
                                    <span class="text-xs font-bold text-zinc-400 uppercase tracking-wide">"EDITING STEP"</span>
                                    <span class="text-sm text-zinc-100">
                                        {format!("Track {}, Step {}", track_id + 1, step_idx + 1)}
                                    </span>
                                </div>
                                <button
                                    class="text-xs text-zinc-500 hover:text-red-500 transition-colors focus:outline-none"
                                    on:click=move |_| selected_step.set(None)
                                >
                                    "×"
                                </button>
                            </div>

                            <div class="text-zinc-500 text-xs italic">
                                "Parameters coming soon..."
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex flex-col items-center justify-center py-8 text-center">
                            <p class="text-zinc-500 text-sm italic mb-2">
                                "Select a step to"
                            </p>
                            <p class="text-zinc-500 text-sm italic mb-4">
                                "edit parameters"
                            </p>
                            <p class="text-zinc-600 text-xs">
                                "Tip: Click or right-click a step"
                            </p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
```

**Step 2: Register component in mod.rs**

Modify `src/ui/components/mod.rs`:

```rust
pub mod grid;
pub mod grid_step;
pub mod inspector;
pub mod step_badge;
pub mod playhead_indicator;
pub mod machine_selector;
pub mod track_controls;
pub mod remove_track_button;
pub mod confirm_dialog;
pub mod lfo_designer;
pub mod lfo_draw;
pub mod form_controls;
pub mod toolbar;
pub mod step_inspector;
pub mod step_editor_sidebar;  // ADD THIS LINE
```

**Step 3: Verify component compiles**

Run: `npm run dev` or `trunk serve`
Expected: Compiles successfully (component not yet rendered, but should compile)

**Step 4: Commit skeleton**

```bash
git add src/ui/components/step_editor_sidebar.rs src/ui/components/mod.rs
git commit -m "feat: add StepEditorSidebar component skeleton with empty state

- Create new component with selected/unselected states
- Close button to deselect step
- Placeholder text for parameters
- Empty state with helpful tip

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Integrate Sidebar into Grid Layout

**Files:**
- Modify: `src/ui/components/grid.rs`

**Step 1: Import StepEditorSidebar**

Add to imports in `src/ui/components/grid.rs`:

```rust
use super::step_editor_sidebar::StepEditorSidebar;
```

**Step 2: Modify Grid layout to 2-column**

Find the main `view!` block (around line 114) and replace the grid section with:

```rust
view! {
    <div class="sequencer-grid">
        // NEW: 2-column layout with sidebar
        <div class="flex gap-4">
            // Left: Step Editor Sidebar
            <StepEditorSidebar />

            // Right: Grid portion
            <div class="flex-1">
                <div class="flex">
                    // Track labels on the left (dynamic)
                    <div class="flex flex-col gap-[2px] mr-2">
                        <For
                            each=move || {
                                pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                            }
                            key=|track_idx| *track_idx
                            children=move |track_idx| {
                                view! {
                                    <div class="h-10 flex items-center justify-start gap-1 px-1">
                                        <RemoveTrackButton
                                            track_idx=track_idx
                                            show_confirm=set_show_confirm_dialog
                                        />
                                        <div class="text-xs text-zinc-400 w-6">
                                            {format!("T{}", track_idx + 1)}
                                        </div>
                                        <MachineSelector track_idx=track_idx />
                                    </div>
                                }
                            }
                        />
                    </div>

                    // Grid of tracks × 16 steps
                    <div class="flex flex-col gap-[2px] relative">
                        <PlayheadIndicator
                            position=playback_position
                            is_playing=is_playing
                        />
                        <For
                            each=move || {
                                pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                            }
                            key=|track_idx| *track_idx
                            children=move |track_idx| {
                                view! {
                                    <div class="flex gap-[2px]">
                                        <For
                                            each=move || {
                                                (0..16).into_iter()
                                            }
                                            key=|step_idx| *step_idx
                                            children=move |step_idx| {
                                                view! {
                                                    <GridStep track_idx=track_idx step_idx=step_idx />
                                                }
                                            }
                                        />
                                    </div>
                                }
                            }
                        />
                    </div>

                    <StepBadge
                        track=selected_track
                        step=selected_step_idx
                        visible=badge_visible
                    />
                </div>

                // Track controls below grid
                <TrackControls />
            </div>
        </div>
    </div>

    // Confirmation dialog (modal overlay outside grid container)
    <ConfirmDialog
        visible=Signal::derive(move || show_confirm_dialog.get().is_some())
        on_confirm=on_confirm_remove
        on_cancel=on_cancel_remove
        title="Confirm Removal"
        message=confirm_message
    />
}
```

**Step 3: Test sidebar appears in UI**

Run: `npm run dev`
Expected: Sidebar appears on left with "Select a step to edit parameters" message

Action: Click a step in the grid
Expected: Sidebar shows "EDITING STEP - Track 1, Step 1" with close button

Action: Click close button (×)
Expected: Sidebar returns to empty state

**Step 4: Commit layout integration**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: integrate StepEditorSidebar into Grid layout

- Add 2-column layout: sidebar + grid
- Sidebar always visible, shows selected step info
- Grid shifts right to accommodate sidebar
- Test: sidebar appears and responds to selection

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Remove Old StepInspector from App

**Files:**
- Modify: `src/app.rs`

**Step 1: Remove StepInspector import**

Remove this line from `src/app.rs`:

```rust
use crate::ui::components::step_inspector::StepInspector;
```

**Step 2: Remove StepInspector from Parameters section**

Find the Parameters section (around line 136-155) and remove the `<StepInspector />` line:

```rust
<section class="bg-zinc-900/50 rounded-lg p-4">
    <div class="flex items-center justify-between mb-4">
        <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Parameters"</h2>
         <div class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full"
                class:bg-blue-500=move || selected_step.get().is_some()
                class:bg-zinc-800=move || selected_step.get().is_none()
            ></span>
            <span class="text-xs font-mono text-zinc-500">
                {move || if let Some((track_id, step_idx)) = selected_step.get() {
                    format!("TRACK {}, STEP {} LOCKED", track_id + 1, step_idx + 1)
                } else {
                    "TRACK DEFAULT".to_string()
                }}
            </span>
        </div>
    </div>
    <Inspector />
    // REMOVE THIS LINE: <StepInspector />
</section>
```

**Step 3: Verify old component is removed**

Run: `npm run dev`
Expected: Compiles successfully, no StepInspector in bottom section, only sidebar

**Step 4: Commit removal**

```bash
git add src/app.rs
git commit -m "refactor: remove StepInspector from Parameters section

- Remove import and usage of StepInspector component
- Step editing now handled by StepEditorSidebar
- Inspector remains for track defaults and LFO controls

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Add Note (Pitch) Parameter Control

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Import Pattern context and form controls**

Add to imports:

```rust
use crate::shared::models::Pattern;
use crate::ui::components::form_controls::*;
```

**Step 2: Add Pattern signals to component**

After getting `selected_step`, add:

```rust
let pattern_signal = use_context::<ReadSignal<Pattern>>()
    .expect("Pattern context not found");
let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
    .expect("Pattern write signal not found");
```

**Step 3: Create Note parameter reactive signal and handler**

Before the `view!` macro, add:

```rust
// Get current note value
let note_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.note as f64)
                .unwrap_or(60.0)
        })
    } else {
        60.0
    }
});

// Note change handler
let on_note_change = move |val: f64| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        let clamped = (val.round() as u8).clamp(0, 127);
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(0) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        step.note = clamped;
                    }
                }
            }
        });
    }
};
```

**Step 4: Replace placeholder with Note control**

Replace the placeholder `"Parameters coming soon..."` with:

```rust
<div class="flex flex-col gap-3">
    <InlineParam>
        <ParamLabel text="Note (Pitch)" locked=Signal::derive(|| false) />
        <NumberInput
            min="0"
            max="127"
            step="1"
            value=Signal::derive(move || format!("{}", note_value.get() as u8))
            on_input=on_note_change
        />
        <div class="flex justify-between text-xs text-zinc-500 font-mono mt-1">
            <span>"0 (C-1)"</span>
            <span>{move || {
                let note = note_value.get() as u8;
                format!("{}", note)
            }}</span>
            <span>"127 (G9)"</span>
        </div>
    </InlineParam>
</div>
```

**Step 5: Test Note parameter**

Run: `npm run dev`

Action: Select a step
Expected: Sidebar shows Note slider with current value

Action: Drag slider or edit number
Expected: Value updates reactively

Action: Open browser console, check pattern state updates
Expected: `step.note` value changes in Pattern signal

**Step 6: Commit Note parameter**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add Note (Pitch) parameter control to sidebar

- Add Pattern signal context access
- Implement reactive note_value signal
- Add NumberInput control with 0-127 range
- Display current note with min/max labels
- Test: note updates reactively when editing

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Add Velocity Parameter Control

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Create Velocity reactive signal and handler**

After the `on_note_change` handler, add:

```rust
// Get current velocity value
let velocity_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.velocity as f64)
                .unwrap_or(100.0)
        })
    } else {
        100.0
    }
});

// Velocity change handler
let on_velocity_change = move |val: f64| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        let clamped = (val.round() as u8).clamp(0, 127);
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(0) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        step.velocity = clamped;
                    }
                }
            }
        });
    }
};
```

**Step 2: Add Velocity control after Note**

Add to the parameters div:

```rust
<InlineParam>
    <ParamLabel text="Velocity" locked=Signal::derive(|| false) />
    <NumberInput
        min="0"
        max="127"
        step="1"
        value=Signal::derive(move || format!("{}", velocity_value.get() as u8))
        on_input=on_velocity_change
    />
    <div class="flex justify-between text-xs text-zinc-500 font-mono mt-1">
        <span>"0 (Silent)"</span>
        <span>{move || format!("{}", velocity_value.get() as u8)}</span>
        <span>"127 (Max)"</span>
    </div>
</InlineParam>
```

**Step 3: Test Velocity parameter**

Run: `npm run dev`

Action: Select a step, adjust Velocity slider
Expected: Value updates 0-127, different from Note control

**Step 4: Commit Velocity parameter**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add Velocity parameter control to sidebar

- Implement velocity_value reactive signal
- Add NumberInput with 0-127 range
- Display velocity with Silent/Max labels
- Test: velocity updates independently of note

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Add Length Parameter Control

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Create Length reactive signal and handler**

After the `on_velocity_change` handler, add:

```rust
// Get current length value
let length_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.length as f64)
                .unwrap_or(1.0)
        })
    } else {
        1.0
    }
});

// Length change handler
let on_length_change = move |val: f64| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        let clamped = (val as f32).clamp(0.1, 4.0);
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(0) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        step.length = clamped;
                    }
                }
            }
        });
    }
};
```

**Step 2: Add Length control**

```rust
<InlineParam>
    <ParamLabel text="Length" locked=Signal::derive(|| false) />
    <NumberInput
        min="0.1"
        max="4.0"
        step="0.1"
        value=Signal::derive(move || format!("{:.1}", length_value.get()))
        on_input=on_length_change
    />
    <div class="flex justify-between text-xs text-zinc-500 font-mono mt-1">
        <span>"0.1 (Short)"</span>
        <span>{move || format!("{:.1}x", length_value.get())}</span>
        <span>"4.0 (Long)"</span>
    </div>
</InlineParam>
```

**Step 3: Test Length parameter**

Run: `npm run dev`

Action: Select step, adjust Length slider
Expected: Values 0.1 to 4.0, displayed with 1 decimal place

**Step 4: Commit Length parameter**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add Length parameter control to sidebar

- Implement length_value reactive signal with f32
- Add NumberInput with 0.1-4.0 range
- Clamp to prevent 0 or negative values
- Display with 1 decimal precision
- Test: length updates with decimal precision

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Add Probability Parameter Control

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Create Probability reactive signal and handler**

After the `on_length_change` handler, add:

```rust
// Get current probability value
let probability_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.condition.prob as f64)
                .unwrap_or(100.0)
        })
    } else {
        100.0
    }
});

// Probability change handler
let on_probability_change = move |val: f64| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        let clamped = (val.round() as u8).clamp(0, 100);
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(0) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        step.condition.prob = clamped;
                    }
                }
            }
        });
    }
};
```

**Step 2: Add Probability control**

```rust
<InlineParam>
    <ParamLabel text="Probability" locked=Signal::derive(|| false) />
    <NumberInput
        min="0"
        max="100"
        step="1"
        value=Signal::derive(move || format!("{}", probability_value.get() as u8))
        on_input=on_probability_change
    />
    <div class="flex justify-between text-xs text-zinc-500 font-mono mt-1">
        <span>"0% (Never)"</span>
        <span>{move || format!("{}%", probability_value.get() as u8)}</span>
        <span>"100% (Always)"</span>
    </div>
</InlineParam>
```

**Step 3: Test Probability parameter**

Run: `npm run dev`

Action: Select step, adjust Probability slider
Expected: Values 0-100%, displayed with % symbol

**Step 4: Commit Probability parameter**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add Probability parameter control to sidebar

- Implement probability_value via condition.prob
- Add NumberInput with 0-100 range
- Display as percentage with % symbol
- Test: probability updates via nested TrigCondition

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Add Micro-timing (Offset) Parameter Control

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Create Micro-timing reactive signal and handler**

After the `on_probability_change` handler, add:

```rust
// Get current micro-timing value
let micro_timing_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.micro_timing as f64)
                .unwrap_or(0.0)
        })
    } else {
        0.0
    }
});

// Micro-timing change handler
let on_micro_timing_change = move |val: f64| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        let clamped = (val.round() as i8).clamp(-23, 23);
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(0) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        step.micro_timing = clamped;
                    }
                }
            }
        });
    }
};
```

**Step 2: Add Micro-timing control**

```rust
<InlineParam>
    <ParamLabel text="Micro-timing" locked=Signal::derive(|| false) />
    <NumberInput
        min="-23"
        max="23"
        step="1"
        value=Signal::derive(move || format!("{}", micro_timing_value.get() as i8))
        on_input=on_micro_timing_change
    />
    <div class="flex justify-between text-xs text-zinc-500 font-mono mt-1">
        <span>"-23 (Early)"</span>
        <span>{move || {
            let val = micro_timing_value.get() as i8;
            if val > 0 {
                format!("+{}", val)
            } else {
                format!("{}", val)
            }
        }}</span>
        <span>"+23 (Late)"</span>
    </div>
</InlineParam>
```

**Step 3: Test Micro-timing parameter**

Run: `npm run dev`

Action: Select step, adjust Micro-timing slider
Expected: Values -23 to +23, negative values show -, positive show +

**Step 4: Commit Micro-timing parameter**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add Micro-timing (Offset) parameter control

- Implement micro_timing_value with i8 type
- Add NumberInput with -23 to +23 range
- Display with +/- prefix for clarity
- Test: micro-timing updates with signed values
- All 5 core parameters now implemented

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Add Smooth Transitions and Polish

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Add transition classes to selected state**

Update the selected step view wrapper to include animation:

```rust
view! {
    <div class="animate-in fade-in slide-in-from-top-2 duration-200">
        <div class="flex items-center justify-between mb-4">
            // ... rest of header
        </div>

        <div class="flex flex-col gap-3">
            // ... parameters
        </div>
    </div>
}.into_any()
```

**Step 2: Add transition to empty state**

Update empty state wrapper:

```rust
view! {
    <div class="flex flex-col items-center justify-center py-8 text-center animate-in fade-in duration-300">
        // ... empty state content
    </div>
}.into_any()
```

**Step 3: Add hover effect to close button**

Update close button styling:

```rust
<button
    class="text-xs text-zinc-500 hover:text-red-500 transition-colors duration-150 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:ring-offset-zinc-900 rounded px-1"
    on:click=move |_| selected_step.set(None)
>
    "×"
</button>
```

**Step 4: Test transitions**

Run: `npm run dev`

Action: Select step → deselect → select different step
Expected: Smooth fade-in animations between states

Action: Hover close button
Expected: Color transitions smoothly to red

**Step 5: Commit polish**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "polish: add transitions and animations to sidebar

- Fade-in animation when switching between states
- Smooth color transition on close button hover
- Focus ring for accessibility on close button
- Test: animations feel smooth and polished

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Manual Testing Verification

**Files:**
- None (testing only)

**Step 1: Test empty state**

Action: Launch app with no step selected
Expected: Sidebar shows "Select a step to edit parameters" with tip

**Step 2: Test step selection**

Action: Click various steps (different tracks, different positions)
Expected: Sidebar updates to show correct Track N, Step M

**Step 3: Test all 5 parameters**

Action: For a selected step, adjust each parameter:
- Note: 0-127
- Velocity: 0-127
- Length: 0.1-4.0
- Probability: 0-100%
- Micro-timing: -23 to +23

Expected: All values update reactively, display correctly

**Step 4: Test close button**

Action: Click × button in sidebar
Expected: Sidebar returns to empty state, step deselected in grid

**Step 5: Test ESC key**

Action: Select step, press ESC
Expected: Sidebar returns to empty state (existing behavior still works)

**Step 6: Test persistence**

Action: Set step parameters, select different step, return to original step
Expected: Parameters preserved (values stored in Pattern signal)

**Step 7: Test grid layout**

Action: Verify grid shifted right to accommodate sidebar
Expected: Track labels, steps, track controls all visible and functional

**Step 8: Test Inspector still works**

Action: Open Parameters section at bottom
Expected: Inspector still edits track defaults and LFO, no StepInspector present

**Step 9: Document test results**

Create verification checklist file (optional):

```bash
echo "# Step Editor Sidebar Testing Checklist

## Empty State
- [ ] Shows 'Select a step to edit parameters'
- [ ] Shows helpful tip text
- [ ] Smooth fade-in animation

## Step Selection
- [ ] Clicking step shows sidebar controls
- [ ] Header displays correct Track/Step number
- [ ] Close button visible
- [ ] Animation smooth

## Parameter Controls
- [ ] Note: 0-127 range works
- [ ] Velocity: 0-127 range works
- [ ] Length: 0.1-4.0 range works
- [ ] Probability: 0-100% range works
- [ ] Micro-timing: -23 to +23 range works

## Interactions
- [ ] Close button deselects step
- [ ] ESC key deselects step
- [ ] Click outside grid deselects step
- [ ] Parameter changes persist

## Layout
- [ ] Sidebar 240px width
- [ ] Grid shifted right appropriately
- [ ] Track labels visible
- [ ] Track controls functional

## Integration
- [ ] Inspector still works (bottom section)
- [ ] No StepInspector in bottom section
- [ ] Grid selection unchanged
- [ ] No console errors

All tests passed: _____ (Date/Time)
" > docs/step-editor-sidebar-testing.md

git add docs/step-editor-sidebar-testing.md
git commit -m "docs: add step editor sidebar testing checklist

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: Final Documentation Update

**Files:**
- Modify: `docs/plans/2026-02-14-step-editor-sidebar-design.md`

**Step 1: Add implementation notes to design doc**

Append to design doc:

```markdown

---

## Implementation Notes (Final)

**Completed:** 2026-02-14

**Implementation Summary:**
- Created `StepEditorSidebar` component with 5 parameter controls
- Modified `Grid` layout to 2-column (sidebar + grid)
- Removed old `StepInspector` from App
- All parameters use direct field access on `AtomicStep`
- Smooth transitions and animations added
- Manual testing verified all functionality

**Code Statistics:**
- New files: 1 (`step_editor_sidebar.rs`)
- Modified files: 3 (`grid.rs`, `app.rs`, `mod.rs`)
- Lines of code: ~300 (sidebar component)
- Parameters implemented: 5/5 (Note, Velocity, Length, Probability, Micro-timing)

**Known Limitations:**
- No Tauri backend sync for step updates (future enhancement)
- Single-step selection only (multi-step designed but not implemented)
- Manual testing only (no automated UI tests)

**Next Steps:**
- Add Tauri backend sync if needed
- Consider keyboard navigation (arrow keys between steps)
- Implement multi-step editing when needed
- Add conditional triggers UI (PRE, NEI, FILL)
```

**Step 2: Commit documentation update**

```bash
git add docs/plans/2026-02-14-step-editor-sidebar-design.md
git commit -m "docs: add implementation notes to design document

- Mark feature as completed
- Add implementation summary and stats
- Document known limitations
- List potential next steps

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

**Step 3: Create implementation summary commit**

Final commit summarizing the whole feature:

```bash
git commit --allow-empty -m "feat: complete step editor sidebar implementation

Summary of implementation:
- Created StepEditorSidebar component (240px left column)
- Added 5 parameter controls: Note, Velocity, Length, Probability, Micro-timing
- Modified Grid layout to 2-column structure
- Removed old StepInspector from Parameters section
- Added smooth transitions and animations
- Manual testing verified all functionality

Key architectural decisions:
- Direct field access on AtomicStep (not P-Locks)
- Always-visible sidebar (shows empty state when unselected)
- Single-step selection for MVP (designed for multi-step future)
- Reactive updates via Leptos signals
- Consistent FLUX zinc/dark theme

Files changed:
- Created: src/ui/components/step_editor_sidebar.rs
- Modified: src/ui/components/grid.rs, src/app.rs, src/ui/components/mod.rs
- Docs: docs/plans/2026-02-14-step-editor-sidebar-design.md

Testing: All manual tests passed (see testing checklist)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Success Criteria Verification

After completing all tasks, verify these criteria are met:

- ✅ Sidebar always visible with 240px width
- ✅ All 5 step parameters editable when step selected
- ✅ Empty state shows helpful message
- ✅ Close button deselects step
- ✅ All existing selection methods work unchanged
- ✅ Consistent FLUX styling throughout
- ✅ Reactive updates (no manual DOM manipulation)
- ✅ No regressions in grid or Inspector functionality

---

## Common Issues and Solutions

**Issue:** Sidebar too wide, grid cramped
**Solution:** Verify `w-60` (240px) class on sidebar container, ensure `flex-1` on grid wrapper

**Issue:** Parameters not updating reactively
**Solution:** Check Pattern signal context is available, verify `set_pattern_signal.update()` calls

**Issue:** Close button not working
**Solution:** Verify `selected_step.set(None)` callback, check SequencerState context

**Issue:** Wrong step values displayed
**Solution:** Check track_id and step_idx extraction from `selected_step.get()`

**Issue:** Compilation errors on AtomicStep fields
**Solution:** Verify field names match models.rs: `note`, `velocity`, `length`, `micro_timing`, `condition.prob`

---

## Total Estimated Time

- Task 1: 5 minutes (skeleton component)
- Task 2: 5 minutes (layout integration)
- Task 3: 2 minutes (remove old component)
- Task 4: 10 minutes (Note parameter)
- Task 5: 5 minutes (Velocity parameter)
- Task 6: 5 minutes (Length parameter)
- Task 7: 5 minutes (Probability parameter)
- Task 8: 5 minutes (Micro-timing parameter)
- Task 9: 5 minutes (transitions/polish)
- Task 10: 10 minutes (manual testing)
- Task 11: 5 minutes (documentation)

**Total: ~60 minutes** (1 hour implementation)
