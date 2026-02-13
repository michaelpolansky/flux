# 4-Track Dense Grid Design

## Problem Statement

The current sequencer grid displays a single track with 16 steps in an 8×2 layout (64×64px buttons, ~240px height). This uses significant vertical space and limits the user's ability to see and edit multiple tracks simultaneously. Users familiar with DAWs like Cubase expect a more compact, multi-track grid view.

**User Request:** "Can we make the grid more dense, like cubase? I'm thinking 4 tracks of 16 steps?"

## Solution Overview

**Approach: Simple Vertical Stacking**

Display 4 tracks vertically, each with 16 steps horizontally, using 40×40px buttons. This creates a compact grid (~175px height) that shows more musical information while fitting comfortably in the established viewport budget.

**Key Design Decisions:**
- **Button size:** 40×40px (down from 64×64px) for density while maintaining usability
- **Track labels:** Simple "T1", "T2", "T3", "T4" labels on the left for orientation
- **Selection model:** Single selection across all 4 tracks (consistent with current UX)
- **Inspector integration:** Extends existing pattern to show "Track N, Step M"

---

## Architecture

### Grid Structure

The grid will display 4 tracks vertically, each with 16 steps horizontally. Instead of the current 8×2 layout (64×64px buttons), we'll use a 4×16 layout with 40×40px buttons. This gives us approximately 175px total height (4 tracks × 40px + spacing) compared to the current 240px.

### Track Labels

A new component on the left side will display track labels ("T1", "T2", "T3", "T4"). These are purely visual indicators, not interactive in this design.

### Selection Model

The current `selected_step: RwSignal<Option<usize>>` will be extended to track both track and step:

```rust
selected_step: RwSignal<Option<(usize, usize)>>  // (track_id, step_idx)
```

This single selection spans all 4 tracks. Clicking any step on any track sets the selection, and the inspector shows that specific step's parameters.

### Inspector Integration

The inspector header will be updated to show: `"Editing: Track {track_id + 1}, Step {step_idx + 1}"` or `"Editing: Track Defaults"` when no step is selected. All existing parameter controls, Active toggle, and p-lock functionality remain unchanged—they just operate on the selected (track, step) tuple.

---

## Components

### Grid Component (`flux-app/src/ui/components/grid.rs`)

**Changes:**
- Change from `grid-cols-8` to `grid-cols-16` (16 columns for 16 steps)
- Add 4 rows, one per track (currently displays only track 0, subtrack 0)
- Loop over tracks 0-3, then steps 0-15 within each track
- Button size: `w-10 h-10` (40×40px) instead of current `w-16 h-16` (64×64px)
- Selection logic updates from `Some(idx)` to `Some((track_id, step_idx))`
- Visual indicator for selected step spans both track and step dimensions

### New Track Labels Component

A simple component positioned to the left of the grid:

```rust
<div class="flex flex-col gap-[2px] mr-2">
    <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T1</div>
    <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T2</div>
    <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T3</div>
    <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T4</div>
</div>
```

### Inspector Component (`flux-app/src/ui/components/inspector.rs`)

**Changes:**
- Header text changes from `format!("Editing: Step {}", step_idx + 1)` to `format!("Editing: Track {}, Step {}", track_id + 1, step_idx + 1)`
- All parameter controls and Active toggle remain unchanged—they already receive `track_id` and operate on the correct track
- P-lock display continues to work as-is (shows locks for the selected step)

---

## Data Flow

### Selection State Flow

```
User clicks step button
  → on:click handler fires with (track_id, step_idx)
  → sequencer_state.selected_step.set(Some((track_id, step_idx)))
  → Inspector reactively reads selected_step signal
  → Inspector header displays "Editing: Track {track_id + 1}, Step {step_idx + 1}"
  → Inspector parameter controls operate on pattern.tracks[track_id].subtracks[0].steps[step_idx]
```

### Parameter Modifications

```
User adjusts parameter slider in inspector
  → Parameter handler calls set_pattern_signal.update()
  → Updates pattern.tracks[track_id].subtracks[0].steps[step_idx].p_locks[param_id]
  → Spawns async Tauri command to sync to backend
  → Grid reactively re-renders (button styling reflects active/inactive state)
```

### Active Toggle Flow

```
User clicks Active toggle in inspector
  → toggle_step(track_id, step_idx) function fires
  → Updates step.trig_type (Note ↔ None)
  → Spawns async Tauri command toggle_step(track_id, step_idx)
  → Grid button visual updates (filled circle vs empty circle)
```

### Deselection Flow

```
ESC key pressed OR click outside grid
  → set_selected_step.set(None)
  → Inspector header shows "Editing: Track Defaults"
  → Inspector controls operate on track.default_params instead
  → Grid buttons show no selection highlight
```

All existing async communication with the Tauri backend (via `invoke()`) remains unchanged—commands already accept `track_id` and `step_idx` parameters.

---

## Visual Design

### Grid Layout

- **Container:** `flex` row with track labels on left, grid on right
- **Grid:** 4 rows × 16 columns, `gap-[2px]` between buttons (matches current spacing)
- **Total dimensions:** ~650px wide × ~175px tall (vs current ~530px × ~240px)
- **Viewport fit:** Fits comfortably in the ~900px viewport budget established in the no-scroll layout

### Button Styling

```rust
class="w-10 h-10 rounded-lg border transition-all duration-150"
```

- **Size:** 40×40px (down from 64×64px)
- **Corners:** Same rounded corners and transitions as current design
- **Active step:** `bg-blue-500 border-blue-400` with filled circle indicator
- **Inactive step:** `bg-zinc-800 border-zinc-700` with empty circle indicator
- **Selected step:** `ring-2 ring-blue-400` (existing selection highlight)
- **Hover:** `hover:bg-zinc-700` (for inactive steps)

### Track Labels

- **Size:** Width 32px (w-8), Height 40px (h-10) to align with grid rows
- **Text:** `text-xs text-zinc-400` (small, subtle)
- **Spacing:** `mr-2` (8px margin) from grid
- **Interaction:** Non-interactive, purely informational

### Color Consistency

All colors remain consistent with the existing design system (zinc/blue palette from Tailwind). The only change is scale—everything gets proportionally smaller to accommodate 4 tracks.

---

## Testing & Edge Cases

### Manual Testing Checklist

1. ✓ Click steps on different tracks—verify selection moves correctly and inspector header updates
2. ✓ Toggle Active on/off for steps across all 4 tracks—verify each track maintains independent active/inactive states
3. ✓ Adjust parameters on selected steps—verify p-locks save correctly per track
4. ✓ Press ESC—verify deselection works and inspector shows "Track Defaults"
5. ✓ Click outside grid—verify same deselection behavior
6. ✓ Click between tracks rapidly—verify selection indicator moves smoothly
7. ✓ Verify visual consistency (button sizes, spacing, alignment with track labels)

### Edge Cases

- **No selection state:** When `selected_step == None`, inspector operates on track 0's default_params (current behavior, unchanged)
- **Track boundary:** Only tracks 0-3 are rendered; the pattern supports up to 16 tracks, but we're only displaying the first 4
- **Step count:** All 4 tracks display 16 steps (the default subtrack length)
- **Selection persistence:** Selection survives parameter changes—only cleared by ESC, click-away, or selecting a different step
- **Visual feedback:** Selected step shows ring, active steps show filled circle, inactive show empty circle—all indicators work independently

### No Breaking Changes

- All existing Tauri backend commands remain compatible (they already accept `track_id`)
- P-lock system works unchanged
- Parameter ranges and defaults unchanged
- LFO system unaffected

---

## Implementation Notes

- Grid component requires updating selection signal type from `Option<usize>` to `Option<(usize, usize)>`
- All components that read `selected_step` need to destructure the tuple
- Track labels can be a simple inline component (no separate file needed initially)
- Visual testing should verify alignment between track labels and grid rows
