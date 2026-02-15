# Inline Velocity Lanes - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add always-visible velocity lanes below the step grid showing velocity values for all tracks with drag-to-edit interaction.

**Architecture:** New VelocityLanes component accesses shared pattern_signal, renders velocity values in a grid layout grouped below step grid, handles drag interactions to create/update velocity P-locks.

**Tech Stack:** Leptos 0.7 (Rust WASM), Tailwind CSS 4.x

**Design Document:** `docs/plans/2026-02-14-inline-velocity-lanes-design.md`

---

## Task 1: Create VelocityLanes Component File

**Files:**
- Create: `flux-app/src/ui/components/velocity_lanes.rs`
- Modify: `flux-app/src/ui/components/mod.rs`

**Step 1: Create the component file**

Create `flux-app/src/ui/components/velocity_lanes.rs` with basic structure:

```rust
use leptos::prelude::*;

#[component]
pub fn VelocityLanes() -> impl IntoView {
    // Access shared context
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>()
        .expect("Pattern context not found");

    view! {
        <div class="velocity-lanes border-t border-zinc-800 mt-4">
            <div class="py-2 px-2">
                <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                    "VELOCITY"
                </h3>
            </div>
            <div class="velocity-grid">
                <p class="text-zinc-500 text-sm p-4">"Velocity lanes component"</p>
            </div>
        </div>
    }
}
```

**Step 2: Export the module**

In `flux-app/src/ui/components/mod.rs`, add:

```rust
pub mod velocity_lanes;
pub use velocity_lanes::VelocityLanes;
```

Find the existing `pub mod` declarations and add `velocity_lanes` alphabetically. Then find the `pub use` declarations and add the VelocityLanes export.

**Step 3: Verify compilation**

```bash
cd flux-app
npm run dev
```

Expected: Compiles successfully (component not yet used, but should compile)
Note: Warning about unused VelocityLanes is expected

**Step 4: Commit**

```bash
git add src/ui/components/velocity_lanes.rs src/ui/components/mod.rs
git commit -m "feat: create VelocityLanes component stub

- Add basic component structure
- Export from mod.rs
- Section header with VELOCITY label

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Add Helper Functions

**Files:**
- Modify: `flux-app/src/ui/components/velocity_lanes.rs:1-50`

**Step 1: Add helper functions**

Add these functions after the imports, before the component:

```rust
use leptos::prelude::*;

/// Get velocity value for a specific step (P-lock or default)
fn get_velocity_value(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> u8 {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| {
            // Check for velocity P-lock first (parameter index 1)
            step.p_locks[1]
                .map(|v| (v * 127.0) as u8) // Convert f32 to u8
                .unwrap_or(step.velocity) // Fall back to step.velocity
        })
        .unwrap_or(100) // Default if track/step doesn't exist
}

/// Check if velocity is P-locked for a specific step
fn is_velocity_locked(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> bool {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .and_then(|step| step.p_locks[1]) // Velocity is param index 1
        .is_some()
}

/// Check if step is active (has trigger)
fn is_step_active(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> bool {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| step.trig_type != crate::shared::models::TrigType::None)
        .unwrap_or(false)
}

#[component]
pub fn VelocityLanes() -> impl IntoView {
    // ... existing code
}
```

**Step 2: Verify compilation**

```bash
cd flux-app
npm run dev
```

Expected: Compiles successfully
Note: Warnings about unused functions are expected

**Step 3: Commit**

```bash
git add src/ui/components/velocity_lanes.rs
git commit -m "feat: add velocity helper functions

- get_velocity_value: Read P-lock or default
- is_velocity_locked: Check P-lock presence
- is_step_active: Check if step has trigger

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Implement Velocity Grid Display

**Files:**
- Modify: `flux-app/src/ui/components/velocity_lanes.rs:50-150`

**Step 1: Replace stub with velocity grid**

Replace the component's view! macro with the full grid implementation:

```rust
#[component]
pub fn VelocityLanes() -> impl IntoView {
    // Access shared context
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>()
        .expect("Pattern context not found");

    view! {
        <div class="velocity-lanes border-t border-zinc-800 mt-4">
            // Section header
            <div class="py-2 px-2">
                <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                    "VELOCITY"
                </h3>
            </div>

            // Velocity grid
            <div class="flex">
                // Track labels column
                <div class="flex flex-col gap-[2px] mr-2">
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="h-10 flex items-center justify-start px-1">
                                    <div class="text-xs text-zinc-400 w-6">
                                        {format!("T{}", track_idx + 1)}
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>

                // Velocity cells grid
                <div class="flex flex-col gap-[2px]">
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="flex gap-[2px]">
                                    <For
                                        each=move || (0..16)
                                        key=|step_idx| *step_idx
                                        children=move |step_idx| {
                                            let value_signal = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    get_velocity_value(p, track_idx, step_idx)
                                                })
                                            });

                                            let is_locked = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    is_velocity_locked(p, track_idx, step_idx)
                                                })
                                            });

                                            let is_active = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    is_step_active(p, track_idx, step_idx)
                                                })
                                            });

                                            view! {
                                                <div class="w-10 h-10 bg-zinc-800/30 border border-zinc-700/50 flex items-center justify-center hover:bg-zinc-700/50 transition-colors">
                                                    <span class=move || {
                                                        let base = "text-center";
                                                        let active_class = if is_active.get() {
                                                            if is_locked.get() {
                                                                "text-amber-400 font-medium text-sm"
                                                            } else {
                                                                "text-zinc-100 text-sm"
                                                            }
                                                        } else {
                                                            "text-zinc-600 text-xs"
                                                        };
                                                        format!("{} {}", base, active_class)
                                                    }>
                                                        {move || {
                                                            if is_active.get() {
                                                                format!("{}", value_signal.get())
                                                            } else {
                                                                "--".to_string()
                                                            }
                                                        }}
                                                    </span>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
```

**Step 2: Verify compilation**

```bash
cd flux-app
npm run dev
```

Expected: Compiles successfully with hot reload

**Step 3: Commit**

```bash
git add src/ui/components/velocity_lanes.rs
git commit -m "feat: implement velocity grid display

- Track labels column (T1, T2, etc.)
- Velocity cells grid (16 steps per track)
- Show velocity value or \"--\" for inactive steps
- Amber text for P-locked values
- White text for active non-locked
- Dimmed text for inactive steps

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Integrate VelocityLanes into Grid

**Files:**
- Modify: `flux-app/src/ui/components/grid.rs:185-190`

**Step 1: Add VelocityLanes import**

At the top of `grid.rs`, add to the imports from `super::`:

```rust
use super::velocity_lanes::VelocityLanes;
```

**Step 2: Add VelocityLanes to layout**

Find the section around line 185 where `<TrackControls />` is rendered. Add VelocityLanes before TrackControls:

```rust
                    </div>

                    // NEW: Velocity lanes
                    <VelocityLanes />

                    // Track controls below velocity lanes
                    <TrackControls />
                </div>
```

The full context should look like:

```rust
                        </div>

                        <StepBadge
                            track=selected_track
                            step=selected_step_idx
                            visible=badge_visible
                        />
                    </div>

                    // Velocity lanes
                    <VelocityLanes />

                    // Track controls below grid
                    <TrackControls />
                </div>
```

**Step 3: Verify in browser**

```bash
cd flux-app
npm run dev
```

Open browser to `http://localhost:1420`

Expected:
- Velocity section appears below step grid
- Shows "VELOCITY" header
- Shows T1, T2, T3, T4 labels on left
- Shows velocity values or "--" for each step
- Layout aligns with step grid columns

**Step 4: Commit**

```bash
git add src/ui/components/grid.rs
git commit -m "feat: integrate VelocityLanes into grid layout

- Add VelocityLanes below step grid
- Positioned before TrackControls
- Section now visible in UI

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Add Drag Interaction State

**Files:**
- Modify: `flux-app/src/ui/components/velocity_lanes.rs:50-100`

**Step 1: Add drag state signals**

In the VelocityLanes component function, add drag state after pattern_signal:

```rust
#[component]
pub fn VelocityLanes() -> impl IntoView {
    // Access shared context
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<crate::shared::models::Pattern>>()
        .expect("Pattern write signal not found");

    // Drag state
    let (drag_state, set_drag_state) = signal::<Option<(usize, usize)>>(None);
    let (drag_start_y, set_drag_start_y) = signal::<Option<f64>>(None);
    let (drag_start_value, set_drag_start_value) = signal::<Option<u8>>(None);

    // ... rest of component
```

**Step 2: Verify compilation**

```bash
cd flux-app
npm run dev
```

Expected: Compiles successfully
Note: Warnings about unused signals expected

**Step 3: Commit**

```bash
git add src/ui/components/velocity_lanes.rs
git commit -m "feat: add drag interaction state

- drag_state: Track which cell is being dragged
- drag_start_y: Capture starting Y position
- drag_start_value: Capture starting velocity value

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Implement Mouse Event Handlers

**Files:**
- Modify: `flux-app/src/ui/components/velocity_lanes.rs:100-200`

**Step 1: Add event handlers to velocity cells**

Modify the velocity cell `<div>` to add mouse event handlers:

```rust
<div
    class=move || {
        let base = "w-10 h-10 bg-zinc-800/30 border border-zinc-700/50 flex items-center justify-center hover:bg-zinc-700/50 transition-colors";
        let cursor = if drag_state.get().is_some() {
            "cursor-ns-resize"
        } else {
            "cursor-ns-resize"
        };
        format!("{} {}", base, cursor)
    }
    on:mousedown=move |ev| {
        ev.prevent_default();
        set_drag_state.set(Some((track_idx, step_idx)));
        set_drag_start_y.set(Some(ev.client_y() as f64));
        let current_value = pattern_signal.with(|p| get_velocity_value(p, track_idx, step_idx));
        set_drag_start_value.set(Some(current_value));
    }
>
```

**Step 2: Add global mouse handlers**

Add these handlers to the component's outer `<div class="velocity-lanes">`:

```rust
<div
    class="velocity-lanes border-t border-zinc-800 mt-4"
    on:mousemove=move |ev| {
        if let Some((t_idx, s_idx)) = drag_state.get() {
            if let (Some(start_y), Some(start_val)) = (drag_start_y.get(), drag_start_value.get()) {
                let delta = start_y - ev.client_y() as f64;
                let new_velocity = ((start_val as i32 + delta as i32).clamp(0, 127)) as u8;

                // Update pattern
                set_pattern_signal.update(|pattern| {
                    if let Some(step) = pattern
                        .tracks
                        .get_mut(t_idx)
                        .and_then(|t| t.subtracks.get_mut(0))
                        .and_then(|st| st.steps.get_mut(s_idx))
                    {
                        // Create P-lock (normalized to 0.0-1.0)
                        step.p_locks[1] = Some(new_velocity as f32 / 127.0);
                    }
                });
            }
        }
    }
    on:mouseup=move |_| {
        set_drag_state.set(None);
        set_drag_start_y.set(None);
        set_drag_start_value.set(None);
    }
>
```

**Step 3: Test drag interaction**

```bash
cd flux-app
npm run dev
```

Open browser, test:
1. Click and hold on a velocity cell
2. Drag up → velocity increases
3. Drag down → velocity decreases
4. Release mouse → value finalizes

Expected:
- Number updates live during drag
- Value clamps at 0 and 127
- Color changes to amber (P-lock created)

**Step 4: Commit**

```bash
git add src/ui/components/velocity_lanes.rs
git commit -m "feat: implement drag interaction for velocity editing

- Mouse down captures cell and starting values
- Mouse move calculates delta and updates velocity
- Mouse up finalizes drag and clears state
- Creates P-lock on drag (always stores in p_locks[1])
- Clamps velocity to 0-127 range

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Manual Testing - Display Verification

**Files:**
- Test: Browser at `http://localhost:1420`

**Step 1: Test initial display**

Actions:
1. Open browser to `http://localhost:1420`
2. Observe velocity lanes section below step grid

Verify:
- ✓ "VELOCITY" header visible with correct styling
- ✓ Track labels (T1, T2, T3, T4) on left side
- ✓ Velocity cells align with step columns above
- ✓ Active steps show velocity numbers
- ✓ Inactive steps show "--" in dimmed text

**Step 2: Test P-lock indication**

Actions:
1. Click a step in the grid to select it
2. In sidebar, adjust a parameter (e.g., Tuning)
3. Return to velocity lane

Verify:
- ✓ If step has velocity P-lock, shows amber text
- ✓ If step has no velocity P-lock, shows white text
- ✓ Sidebar and velocity lane show same value

**Step 3: Test inactive steps**

Actions:
1. Find or create an inactive step (no trigger)
2. Observe its velocity lane cell

Verify:
- ✓ Shows "--" text
- ✓ Text is dimmed (text-zinc-600)
- ✓ Smaller font size (text-xs)

**Step 4: Document findings**

Create a checklist note:
```markdown
## Display Verification Testing

- [ ] Section header styled correctly
- [ ] Track labels align with step grid
- [ ] Velocity cells align vertically
- [ ] Active steps show correct values
- [ ] Inactive steps show "--"
- [ ] P-locked values show amber
- [ ] Non-locked values show white

Issues found: [None or describe]
```

---

## Task 8: Manual Testing - Drag Interaction

**Files:**
- Test: Browser at `http://localhost:1420`

**Step 1: Test basic drag**

Actions:
1. Click and hold on an active step's velocity cell
2. Drag mouse up slowly
3. Observe value changing
4. Release mouse

Verify:
- ✓ Cursor changes to ns-resize
- ✓ Value increases as you drag up
- ✓ Number updates live during drag
- ✓ Value finalizes on release
- ✓ Text color changes to amber (P-lock created)

**Step 2: Test drag down**

Actions:
1. Click and hold on a high velocity value
2. Drag mouse down
3. Observe value decreasing

Verify:
- ✓ Value decreases as you drag down
- ✓ Updates smoothly

**Step 3: Test boundary clamping**

Actions:
1. Find a step with low velocity (e.g., 10)
2. Drag down past 0
3. Observe clamping at 0

Actions:
4. Find a step with high velocity (e.g., 120)
5. Drag up past 127
6. Observe clamping at 127

Verify:
- ✓ Value stops at 0 (doesn't go negative)
- ✓ Value stops at 127 (doesn't exceed)

**Step 4: Test inactive step drag**

Actions:
1. Click and drag on an inactive step (shows "--")
2. Drag up/down

Verify:
- ✓ Can still drag (creates P-lock)
- ✓ Value appears (no longer shows "--")
- ✓ Shows amber text (P-locked)
- ✓ Step remains inactive (no trigger)

**Step 5: Document findings**

```markdown
## Drag Interaction Testing

- [ ] Drag up increases velocity
- [ ] Drag down decreases velocity
- [ ] Values clamp at 0 and 127
- [ ] Live update during drag
- [ ] Creates P-lock (amber text)
- [ ] Inactive steps can be dragged
- [ ] Cursor changes to ns-resize

Issues found: [None or describe]
```

---

## Task 9: Manual Testing - Integration & Edge Cases

**Files:**
- Test: Browser at `http://localhost:1420`

**Step 1: Test sidebar sync**

Actions:
1. Drag a velocity cell to change value (e.g., to 85)
2. Click that step to select it
3. Check sidebar velocity parameter

Verify:
- ✓ Sidebar shows same value (85)
- ✓ Both stay synchronized

Actions:
4. In sidebar, change velocity to different value (e.g., 100)
5. Check velocity lane

Verify:
- ✓ Velocity lane updates to 100
- ✓ Both stay synchronized

**Step 2: Test track add/remove**

Actions:
1. Note current number of velocity lanes
2. Click "[+ Add Track]" button
3. Observe velocity lanes

Verify:
- ✓ New velocity lane appears
- ✓ Shows "T5" label (or appropriate number)
- ✓ Shows "--" for all steps (inactive)

Actions:
4. Remove a track
5. Observe velocity lanes

Verify:
- ✓ Corresponding lane disappears
- ✓ Track labels re-index correctly

**Step 3: Test during playback**

Actions:
1. Start playback (click play button)
2. Drag velocity values while playing
3. Listen for changes

Verify:
- ✓ Drag interaction works during playback
- ✓ Velocity changes apply to playing notes
- ✓ No conflicts or lag

**Step 4: Test visual alignment**

Actions:
1. Visually scan from step grid to velocity lane
2. Check vertical alignment

Verify:
- ✓ Each velocity cell aligns with step column above
- ✓ Track labels align with step grid track labels
- ✓ No offset or misalignment
- ✓ Gap spacing matches (2px)

**Step 5: Document findings**

```markdown
## Integration & Edge Cases Testing

- [ ] Sidebar and velocity lane stay synced
- [ ] Adding track creates new lane
- [ ] Removing track removes lane
- [ ] Drag works during playback
- [ ] Vertical alignment perfect
- [ ] Track labels align correctly

Issues found: [None or describe]
```

---

## Task 10: Polish and Final Commit

**Files:**
- Review: All changes in `flux-app/src/ui/components/velocity_lanes.rs` and `grid.rs`

**Step 1: Review code formatting**

Check:
- Code is properly formatted
- No debug prints or commented code
- Clear variable names
- Proper spacing

Run `cargo fmt` if needed:
```bash
cd flux-app/src-tauri
cargo fmt
```

**Step 2: Verify no regressions**

Test:
1. Step selection still works normally
2. Parameter editing in sidebar unchanged
3. Grid interaction unchanged
4. P-Lock creation/removal in sidebar still works

Expected: All existing functionality intact

**Step 3: Test full workflow**

Actions:
1. Create a new pattern
2. Activate some steps
3. Use velocity lanes to create velocity pattern
4. Select steps and verify in sidebar
5. Edit parameters in sidebar
6. Verify velocity lane reflects changes

Verify:
- ✓ Complete workflow works end-to-end
- ✓ No errors in console
- ✓ Smooth performance

**Step 4: Final verification checklist**

Before marking complete, verify all success criteria:

- [ ] VelocityLanes component created and integrated
- [ ] Velocity values display correctly
- [ ] Active steps show white (no P-lock) or amber (P-locked)
- [ ] Inactive steps show dimmed "--"
- [ ] Drag up increases velocity
- [ ] Drag down decreases velocity
- [ ] Values clamp at 0-127
- [ ] Visual alignment perfect
- [ ] Section header styled correctly
- [ ] Track labels align properly
- [ ] No layout shifts or regressions
- [ ] Performance smooth during drag
- [ ] Sidebar and velocity lane stay synced

**Step 5: Create completion summary**

Document what was built:
```markdown
# Inline Velocity Lanes Implementation - Complete

## What Changed
- Created VelocityLanes component (velocity_lanes.rs)
- Added helper functions for reading/checking velocity values
- Implemented grid display with track labels and cells
- Added drag interaction for velocity editing
- Integrated into Grid component below step grid

## Files Modified
- `src/ui/components/velocity_lanes.rs` (new, ~200 lines)
- `src/ui/components/grid.rs` (+2 lines)
- `src/ui/components/mod.rs` (+2 lines)

## Testing Completed
- Display verification (alignment, styling, values)
- Drag interaction (up/down, clamping, P-lock creation)
- Integration (sidebar sync, track add/remove, playback)
- Edge cases (inactive steps, boundaries, performance)

## Performance
- Lightweight rendering (64 cells for 4 tracks)
- Reactive updates only on affected lanes
- Smooth drag interaction with no lag

## Next Steps (Future Enhancements)
- Add Filter Freq lane
- Add Tuning lane
- Add toggle to collapse/expand section
- Add keyboard shortcuts (arrow keys)
- Add click-to-type (double-click input)
```

---

## Verification Checklist

Before considering this complete, verify:

- [ ] VelocityLanes component file created
- [ ] Helper functions implemented and working
- [ ] Velocity grid displays correctly
- [ ] Track labels align with step grid
- [ ] Velocity cells align vertically with steps
- [ ] Active steps show velocity numbers
- [ ] Inactive steps show "--" in dimmed text
- [ ] P-locked values show amber text
- [ ] Non-locked values show white text
- [ ] Drag interaction creates P-locks
- [ ] Drag up increases velocity
- [ ] Drag down decreases velocity
- [ ] Values clamp at 0 and 127
- [ ] Cursor changes to ns-resize during drag
- [ ] Sidebar and velocity lane stay synchronized
- [ ] Adding track creates new velocity lane
- [ ] Removing track removes velocity lane
- [ ] Drag works during playback
- [ ] No layout shifts or visual regressions
- [ ] No performance issues or lag
- [ ] All commits follow conventional commit format
- [ ] Code is clean and formatted

---

## Notes

- This implementation adds one new component file (~200 lines)
- Minimal changes to existing files (grid.rs, mod.rs)
- No new dependencies required
- Uses existing Leptos reactive signals and Tailwind design tokens
- Manual testing required (no automated UI tests for Leptos components)
- Future enhancement: Add more parameter lanes (filter, tuning, etc.)
