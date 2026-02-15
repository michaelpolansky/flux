# Inline Velocity Lanes Design

**Date:** 2026-02-14
**Status:** Approved
**Approach:** Separate VelocityLanes Component (Approach 1)

## Problem Statement

The current FLUX sequencer shows parameter values only in the sidebar when a step is selected. This requires clicking each step individually to see velocity automation, making it difficult to scan and edit velocity patterns across multiple steps at a glance. Compared to DAWs like Cubase with inline parameter lanes, FLUX has lower information density and slower velocity editing workflows.

## Solution Overview

Add an always-visible velocity lane section below the step grid, showing velocity values for all 16 steps across all tracks. Users can see velocity patterns at a glance and edit values via vertical drag interaction. This improves information density and workflow efficiency while maintaining the clean, compact aesthetic.

---

## Design Decisions

### Requirements Gathered

Through collaborative design discussion, the following requirements were established:

1. **Parameter Scope:** Start with velocity only (can expand to filter, tuning later)
2. **Visual Format:** Numbers only (no bar graphs - prioritize density)
3. **Visibility:** Always visible (no toggle/collapse needed)
4. **Editing:** Click and drag vertically to adjust values
5. **Inactive Steps:** Show dimmed track default value
6. **P-Lock Indication:** Amber/yellow text for P-locked values
7. **Layout:** Velocity lanes grouped at bottom of grid (not per-track)
8. **Track Labels:** Repeated on left side of velocity section
9. **Row Height:** Same as step grid rows (~40px)

### Approach Selection

**Chosen: Separate VelocityLanes Component**

Create a new `VelocityLanes` component that sits below the step grid, accesses the same `pattern_signal`, and handles its own drag interactions.

**Rationale:**
- Clean separation of concerns (step triggers vs. parameter automation)
- Easy to extend with additional parameter lanes (filter, tuning) later
- Isolated drag interaction logic (doesn't complicate grid.rs)
- Reusable component architecture

**Alternatives Considered:**
- Integrated Grid Extension (mixes concerns, harder to maintain)
- VelocityCell Component Library (over-engineered for MVP, too granular)

---

## Component Structure & Layout

### Layout Structure

```
┌─────────────────────────────────────────┐
│ Sidebar      Grid Section               │
│              ┌──────────────────────────┐│
│              │ Track Labels | Steps     ││
│              │ T1  [x]      [●][○][●].. ││
│              │ T2  [x]      [●][●][○].. ││
│              ├──────────────────────────┤│
│              │ VELOCITY                  ││
│              │ T1  [127][--][100][--].. ││
│              │ T2  [64][80][90][--]...  ││
│              └──────────────────────────┘│
└─────────────────────────────────────────┘
```

### Component Hierarchy

```
Grid (grid.rs)
├── StepEditorSidebar (existing)
└── GridSection
    ├── StepGrid (existing structure)
    │   ├── TrackLabels (T1, T2...)
    │   └── Grid of GridStep components
    └── VelocityLanes (NEW COMPONENT)
        ├── Section header ("VELOCITY")
        ├── TrackLabels (T1, T2...)
        └── Grid of velocity cells (rendered inline)
```

### New File: `velocity_lanes.rs`

```rust
#[component]
pub fn VelocityLanes() -> impl IntoView {
    // Access shared context
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Drag state (which cell is being dragged)
    let (drag_state, set_drag_state) = signal::<Option<(usize, usize)>>(None);
    let (drag_start_y, set_drag_start_y) = signal::<Option<f64>>(None);
    let (drag_start_value, set_drag_start_value) = signal::<Option<u8>>(None);

    // Render logic
    view! {
        <div class="velocity-lanes">
            <div class="section-header">VELOCITY</div>
            <div class="lanes-grid">
                <For each=move || pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                    key=|track_idx| *track_idx
                    children=move |track_idx| {
                        view! {
                            <div class="velocity-lane">
                                // Track label
                                <div class="track-label">{format!("T{}", track_idx + 1)}</div>

                                // Velocity cells (16 steps)
                                <div class="velocity-cells">
                                    <For each=move || (0..16)
                                        key=|step_idx| *step_idx
                                        children=move |step_idx| {
                                            render_velocity_cell(track_idx, step_idx, ...)
                                        }
                                    />
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
```

**Helper Functions:**
- `get_velocity_value(track_idx, step_idx)` - Returns velocity (P-lock or default)
- `is_velocity_locked(track_idx, step_idx)` - Checks if step has velocity P-lock
- `is_step_active(track_idx, step_idx)` - Checks if step's trig_type != None
- `handle_velocity_drag(track_idx, step_idx, new_value)` - Updates velocity on drag

**No new context needed** - uses existing `pattern_signal` from Grid.

---

## Data Flow & Calculations

### Reading Velocity Values

```rust
fn get_velocity_value(
    pattern: &Pattern,
    track_idx: usize,
    step_idx: usize
) -> u8 {
    pattern.tracks.get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| {
            // Check for velocity P-lock first (parameter index 1)
            step.p_locks[1]
                .map(|v| (v * 127.0) as u8)  // Convert f32 to u8
                .unwrap_or(step.velocity)     // Fall back to step.velocity
        })
        .unwrap_or(100)  // Default if track/step doesn't exist
}
```

### Detecting P-Locks

```rust
fn is_velocity_locked(
    pattern: &Pattern,
    track_idx: usize,
    step_idx: usize
) -> bool {
    pattern.tracks.get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .and_then(|step| step.p_locks[1])  // Velocity is param index 1
        .is_some()
}
```

### Writing Velocity Changes (Drag)

```rust
fn handle_velocity_drag(
    track_idx: usize,
    step_idx: usize,
    new_velocity: u8,
    set_pattern_signal: WriteSignal<Pattern>
) {
    set_pattern_signal.update(|pattern| {
        if let Some(step) = pattern.tracks.get_mut(track_idx)
            .and_then(|t| t.subtracks.get_mut(0))
            .and_then(|st| st.steps.get_mut(step_idx))
        {
            // Create P-lock (normalized to 0.0-1.0)
            step.p_locks[1] = Some(new_velocity as f32 / 127.0);
        }
    });
}
```

### Key Points

- Velocity stored as `f32` in p_locks (0.0-1.0), displayed as `u8` (0-127)
- Velocity P-lock is **parameter index 1** in the p_locks array
- Drag creates P-lock (always modifies p_locks[1], not step.velocity)
- Falls back to step.velocity if no P-lock exists

### Reactive Updates

- Velocity lanes rendered inside `move || { ... }` closure
- Reacts to `pattern_signal` changes (steps toggled, tracks added/removed)
- Drag updates trigger pattern signal update → automatic re-render
- Only affected lane row re-renders (not entire grid)

---

## Interaction Model

### Drag Interaction Flow

**1. Mouse Down** (on velocity cell)
- Capture starting Y position
- Capture current velocity value
- Set drag state to `Some((track_idx, step_idx))`
- Add `cursor-ns-resize` class

**2. Mouse Move** (while dragging)
- Calculate delta: `delta_y = start_y - current_y`
- Calculate new velocity: `new_vel = start_value + (delta_y / sensitivity)`
- Clamp to 0-127 range
- Update pattern (create/update P-lock)
- Display updates reactively

**3. Mouse Up** (anywhere)
- Clear drag state
- Remove cursor class
- Finalize P-lock value

### Drag Sensitivity

- **1px vertical movement = 1 velocity unit** (127 total range)
- Drag up = increase velocity
- Drag down = decrease velocity
- Typical drag range: ~50-100px for full sweep

### Visual Feedback During Drag

- Number updates live as you drag
- Color remains amber (indicates active P-lock)
- Cursor changes to `ns-resize` (north-south arrows)
- Optional: darken background of cell being dragged

### Event Handlers

```rust
on:mousedown=move |ev| {
    set_drag_state.set(Some((track_idx, step_idx)));
    set_drag_start_y.set(Some(ev.client_y() as f64));
    set_drag_start_value.set(Some(get_velocity_value(...)));
}

on:mousemove=move |ev| {
    if let Some((t, s)) = drag_state.get() {
        let delta = drag_start_y.get().unwrap() - ev.client_y() as f64;
        let new_vel = (drag_start_value.get().unwrap() as i32 + delta as i32).clamp(0, 127);
        handle_velocity_drag(t, s, new_vel as u8, set_pattern_signal);
    }
}

on:mouseup=move |_| {
    set_drag_state.set(None);
}
```

**Important:** Mouse move and mouse up handlers attached to parent container or document, not individual cells (prevents losing drag if cursor moves outside cell).

---

## Visual Design Specifications

### Section Layout

```
┌─────────────────────────────────────────┐
│ VELOCITY                    ← Header    │
├─────────────────────────────────────────┤
│ T1  [127][64][100][--][80]...           │ ← ~40px height
│ T2  [64][80][--][127][90]...            │
│ T3  [--][--][--][--][--]...             │
│ T4  [100][127][64][80][--]...           │
└─────────────────────────────────────────┘
```

### Section Header

- Text: `"VELOCITY"`
- Style: `text-xs font-bold text-zinc-400 uppercase tracking-wide`
- Spacing: `py-2 px-2` (matches track controls styling)
- Border: `border-t border-zinc-800` (separates from step grid)

### Track Labels

- Text: `"T1"`, `"T2"`, etc.
- Style: `text-xs text-zinc-400 w-6`
- Alignment: Vertically centered in 40px row
- Spacing: Same left margin as step grid labels (consistent alignment)

### Velocity Cells

- Size: `w-10 h-10` (40px × 40px, same as GridStep)
- Gap: `gap-[2px]` between cells (matches step grid)
- Background: `bg-zinc-800/30` (subtle, not as prominent as steps)
- Border: `border border-zinc-700/50`
- Hover: `hover:bg-zinc-700/50` (indicates interactive)

### Text Styling (Value Display)

**Active step (has trigger):**
- With P-lock: `text-amber-400 font-medium` (amber = locked)
- No P-lock: `text-zinc-100` (white = default)

**Inactive step (no trigger):**
- `text-zinc-600 text-xs` (dimmed, smaller)
- Shows track default velocity (or "--" if no default set)

**Drag State:**
- Cursor: `cursor-ns-resize` during drag
- Optional: `bg-zinc-700` background highlight on dragged cell

### Typography

- Font size: `text-sm` (14px) for active steps
- Font size: `text-xs` (12px) for inactive steps
- Center-aligned in cell
- Monospace feel (numbers align nicely)

### Spacing

- 16px gap between section header and first velocity lane
- 2px gap between velocity lanes (matches track rows)
- Aligns exactly with step columns above (vertical alignment critical)

---

## Edge Cases & Considerations

### Empty/New Tracks

- Track with no active steps shows all `"--"` in dimmed text
- Still interactive - can drag to set velocity before activating steps
- P-locks created even on inactive steps (ready when step activated)

### Pattern Changes

- Velocity lanes react to `pattern_signal` changes automatically
- Adding track → new velocity lane appears
- Removing track → velocity lane disappears
- No special handling needed (reactive signals handle it)

### During Playback

- Velocity editing works normally during playback
- Changes apply immediately to playing notes (via existing P-lock system)
- No conflicts - drag interaction independent of playback state

### Drag Boundaries

- Dragging past 127 → clamps to 127
- Dragging below 0 → clamps to 0
- Visual feedback: value stops increasing/decreasing at limits
- No error states needed

### Step Selection Integration

- Velocity lanes don't affect step selection (read-only regarding selection)
- Clicking velocity cell does NOT select the step
- If step is selected, its velocity shows in both sidebar AND velocity lane
- Both stay in sync (same signal source)

### Performance

- 4 tracks × 16 steps = 64 velocity cells (very lightweight)
- Drag updates single cell at a time (no mass re-renders)
- Pattern signal updates trigger only affected lane row to re-render
- No performance concerns expected

### Accessibility

- Drag-only interaction (no keyboard support in MVP)
- Future: Arrow keys could adjust selected step's velocity
- Future: Click to type (double-click opens input)

### Removing P-Locks

- To remove velocity P-lock: drag back to track default value
- Alternative: Use sidebar "Reset" button (when step selected)
- Velocity lane shows change immediately (white text = no P-lock)

### Multi-Track Alignment

- Velocity cells align perfectly with step columns above
- Track labels align with step grid track labels
- Visual scanning: straight vertical line from step to velocity

---

## Implementation Details

### Files to Create

**New:**
- `flux-app/src/ui/components/velocity_lanes.rs` - Main component

### Files to Modify

**Modify:**
- `flux-app/src/ui/components/grid.rs` - Add VelocityLanes component to layout
- `flux-app/src/ui/components/mod.rs` - Export VelocityLanes module

### Integration into Grid

In `grid.rs`, add VelocityLanes below the step grid:

```rust
view! {
    <div class="sequencer-grid">
        <div class="flex gap-4">
            <StepEditorSidebar />

            <div class="flex-1">
                // Existing step grid
                <div class="flex">
                    // Track labels and steps (existing)
                </div>

                // NEW: Velocity lanes section
                <VelocityLanes />

                // Track controls (existing)
                <TrackControls />
            </div>
        </div>
    </div>
}
```

---

## Testing Strategy

### Manual Testing Scenarios

1. **Display accuracy:**
   - Velocity values match sidebar values when step selected
   - Active steps show correct velocity (P-lock or default)
   - Inactive steps show dimmed "--" or track default
   - P-locked values appear in amber

2. **Drag interaction:**
   - Click and drag up increases velocity
   - Click and drag down decreases velocity
   - Values clamp at 0 and 127
   - Dragging creates P-lock (amber text)
   - Release mouse finalizes value

3. **Reactive updates:**
   - Adding track → new velocity lane appears
   - Removing track → velocity lane disappears
   - Activating step → value changes from dimmed to normal
   - Deactivating step → value changes to dimmed

4. **Integration:**
   - Velocity changes in lane reflect in sidebar
   - Velocity changes in sidebar reflect in lane
   - Both stay synchronized
   - No conflicts during playback

5. **Visual verification:**
   - Section header styling matches "SOUND PARAMETERS" style
   - Cells align vertically with step grid
   - Track labels align with step grid labels
   - Hover states work correctly

### Edge Case Testing

- Empty pattern (0 tracks)
- Pattern with many tracks (10+)
- All steps inactive
- All steps active with P-locks
- Rapid dragging (stress test)
- Drag during playback

---

## Success Criteria

- [ ] VelocityLanes component created and integrated into grid
- [ ] Velocity values display correctly for all steps
- [ ] Active steps show white text (no P-lock) or amber (P-locked)
- [ ] Inactive steps show dimmed text
- [ ] Drag interaction creates/updates P-locks
- [ ] Drag up increases velocity, drag down decreases
- [ ] Values clamp at 0-127 boundaries
- [ ] Visual alignment with step grid perfect
- [ ] Section header matches existing design tokens
- [ ] No layout shifts or regressions
- [ ] Performance remains smooth (no lag during drag)
- [ ] Velocity lane syncs with sidebar values

---

## Future Enhancements

Not in this implementation, but potential improvements:

1. **Additional parameter lanes:** Filter Freq, Tuning, Resonance
2. **Toggle visibility:** Collapse/expand velocity section
3. **Keyboard editing:** Arrow keys to adjust values
4. **Click-to-type:** Double-click opens numeric input
5. **Visual curves:** Option to show bar graphs instead of numbers
6. **Per-step indicators:** Mini dots on steps showing P-lock presence
7. **Zoom controls:** Compress lanes vertically for more density
8. **Copy/paste:** Copy velocity pattern, paste to another track

---

## Notes

- This design maintains the existing sidebar and doesn't replace it
- Velocity lane is complementary (at-a-glance view + quick editing)
- Sidebar still used for precise editing and other parameters
- Minimal code changes required (one new component, small grid.rs update)
- Leverages existing reactive signals and design tokens
- Can be implemented and tested quickly (~3-5 tasks)
