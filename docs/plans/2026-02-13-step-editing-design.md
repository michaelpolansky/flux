# Step Editing UX Redesign

**Date:** 2026-02-13
**Status:** Approved
**Approach:** Streamlined Selection (Approach 1)

## Problem Statement

The current step editing workflow is unclear and awkward:
- Right-click to select a step is not discoverable
- No clear indication when editing a specific step vs. track defaults
- The right-click → edit → click away flow feels unintuitive

## Solution Overview

Redesign the step selection workflow to use left-click for selection, with clear visual feedback and a dedicated toggle button for step activation.

---

## 1. UX Flow

### Step Selection
1. User left-clicks a grid step → step becomes selected (blue ring appears)
2. Inspector header updates to show "Editing: Step 5"
3. Parameter values update to show step's P-locks (or track defaults if no P-locks)
4. Parameter labels turn amber for any parameters that have P-locks on this step

### Step Activation
1. With step selected, "Active" toggle button appears at top of inspector
2. Toggle shows current state (on/off based on `TrigType::Note` vs `TrigType::None`)
3. User clicks toggle to activate/deactivate the step

### Deselection
1. User clicks empty space outside grid → selection clears
2. OR user presses ESC key → selection clears
3. Inspector header updates to "Editing: Track Defaults"
4. Parameter values revert to showing track defaults

### Multiple Step Selection
1. User clicks Step 5 → Step 5 selected
2. User clicks Step 8 → Step 5 deselects, Step 8 selected
3. Only one step selected at a time

---

## 2. Component Changes

### Grid Component (`grid.rs`)
- Change left-click behavior from `toggle_step` to `select_step`
- Remove right-click handler (no longer needed)
- Keep visual feedback: blue ring on selected step (already exists)
- Add click-outside-grid handler to deselect (may need wrapper div)

### Inspector Component (`inspector.rs`)
- Add header section showing "Editing: Step 5" or "Editing: Track Defaults"
- Add "Active" toggle button (only visible when step selected)
- Toggle button controls `step.trig_type` (Note ↔ None)
- Keep existing parameter inputs (no changes needed)
- Keep amber label behavior (already works correctly)

### App Component (`app.rs`)
- Add ESC key listener at app level
- ESC sets `sequencer_state.selected_step` to None

### No Changes Needed
- LFO section (unchanged)
- Step inspector (unchanged)
- Parameter P-lock logic (already correct)

---

## 3. State Management

### Existing State (no changes needed)
- `sequencer_state.selected_step: ReadSignal<Option<usize>>` already exists
- Set via `set_selected_step: WriteSignal<Option<usize>>`
- Already provided as context in `app.rs`

### State Flow
1. **Click step** → `set_selected_step(Some(idx))`
2. **Click away / ESC** → `set_selected_step(None)`
3. **Click different step** → `set_selected_step(Some(new_idx))` (replaces previous)

### Reactive Updates
- Inspector header reacts to `selected_step` changes (shows step number)
- Parameter `get_value()` already checks `selected_step` (returns P-lock or default)
- Parameter `is_locked()` already checks `selected_step` (shows amber labels)
- Active toggle visibility reacts to `selected_step.is_some()`

**No new signals needed** - the existing `selected_step` signal does everything we need.

---

## 4. Visual Design

### Inspector Header
```
┌─────────────────────────────────────┐
│ Editing: Step 5          [●  Active]│  ← Header bar
├─────────────────────────────────────┤
│ TUNING        FILTER FREQ  ...      │  ← Parameters
```

**Styling:**
- Header background: `bg-zinc-800/50` (subtle contrast)
- Text: "Editing: Step 5" in `text-sm text-zinc-300`
- When no step selected: "Editing: Track Defaults"
- Active toggle: Switch-style button, only visible when step selected
  - On state: `bg-amber-500` (matches step color)
  - Off state: `bg-zinc-700`

### Grid Step Selection
- Selected step keeps current blue ring: `ring-2 ring-blue-500`
- No changes to step colors (amber = active, zinc = inactive)
- Playhead highlighting (amber-300 scale) unchanged

### Click-Away Target
- Wrap grid in container div with padding
- Clicking the padding area triggers deselect
- OR: add click handler to app background

---

## 5. Edge Cases

### Deactivating the Selected Step
- User selects Step 5, clicks Active toggle to OFF
- Step remains selected (can still edit P-locks on inactive steps)
- Visual: Step loses amber color, keeps blue selection ring
- Allows editing parameters before activating

### During Playback
- Editing selected step's parameters works normally
- P-locks apply in real-time (already implemented via `push_midi_command`)
- Playhead highlighting (amber-300) overlays selection ring (blue)
- No conflicts - both can be visible simultaneously

### Pattern/Track Changes
- If pattern switches, keep `selected_step` index
- If selected index > new pattern's step count, auto-deselect
- Simple bounds checking in `get_value()` / `is_locked()`

### Click on Currently Selected Step
- Clicking selected step again does nothing (stays selected)
- Only deselect via: click away, ESC, or selecting different step
- Prevents accidental deselection while adjusting parameters

### Empty Grid Clicks
- Clicks between step buttons (in grid gaps) → no change
- Only clicks fully outside grid container → deselect

---

## Trade-offs

**Advantages:**
- ✅ Clear, discoverable workflow (left-click is standard)
- ✅ Minimal code changes (reuses existing state)
- ✅ Explicit feedback (header text + toggle button)
- ✅ Stays within compact layout budget

**Limitations:**
- ⚠️ No at-a-glance view of which steps have P-locks without selecting them
- ⚠️ Single-step selection only (no multi-select for batch editing)

**Future Enhancements:**
- Could add small amber dot indicators on steps with P-locks (Approach 2)
- Could add keyboard shortcuts for faster step navigation (arrow keys)
- Could add multi-select with Shift+click for batch parameter editing

---

## Success Criteria

- [ ] Left-click selects a step (stays selected)
- [ ] Inspector header shows "Editing: Step X" or "Editing: Track Defaults"
- [ ] Active toggle button appears when step selected
- [ ] Active toggle controls step activation (on/off)
- [ ] Click outside grid deselects step
- [ ] ESC key deselects step
- [ ] Parameter editing works correctly for selected step (P-locks)
- [ ] Parameter editing works correctly when no step selected (track defaults)
- [ ] Amber labels indicate P-locked parameters on selected step
- [ ] No visual or functional regressions in compact layout
