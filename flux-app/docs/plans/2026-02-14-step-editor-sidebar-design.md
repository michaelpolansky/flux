# Step Editor Sidebar - Design Document

**Date:** 2026-02-14
**Status:** Approved
**Feature:** Left-column step editor sidebar for per-step parameter editing (Cubase-inspired layout)

---

## Overview

Move step editing controls to a dedicated left-column sidebar, creating a Cubase-style layout where the sequencer grid is flanked by a persistent step editor panel. The sidebar displays step-level parameters (pitch, velocity, length, probability, micro-timing) when a step is selected, and shows helpful placeholder text when nothing is selected.

---

## Requirements

### Functional Requirements

- **FR1:** Always-visible left sidebar (240px width) for step editing
- **FR2:** Display 5 step parameters when a step is selected: Note (Pitch), Velocity, Length, Probability, Micro-timing
- **FR3:** Show empty state with helpful text when no step selected
- **FR4:** Close button in sidebar header to deselect current step
- **FR5:** Direct field editing on `AtomicStep` struct (not P-Locks)
- **FR6:** Single-step selection for MVP (designed to support multi-step in future)
- **FR7:** All existing step selection methods work (click, right-click, ESC to deselect)

### Non-Functional Requirements

- **NFR1:** Desktop-focused design (no mobile/tablet responsive requirements)
- **NFR2:** Consistent FLUX zinc/dark theme styling
- **NFR3:** Reactive updates via Leptos signals
- **NFR4:** Smooth transitions for control updates
- **NFR5:** Future-proof architecture for multi-step editing

---

## User Decisions

### Question 1: Which Controls Move to Left Column?
**Answer:** Create unified step editor sidebar with step-specific parameters
**Rationale:** Matches Cubase pattern, separates step-level editing (left) from track-level controls (bottom Inspector)

### Question 2: What Parameters to Include?
**Answer:** 5 core step parameters from `AtomicStep` struct:
- Note (pitch) - 0-127 MIDI note
- Velocity - 0-127 note velocity
- Length - 0.1-4.0 step duration multiplier
- Probability - 0-100% trigger probability
- Micro-timing - -23 to +23 tick offset

**Rationale:** These are the most common per-step adjustments in hardware sequencers, all available as direct fields on the data model

### Question 3: Sidebar Visibility Behavior?
**Answer:** Always visible (shows empty state when no step selected)
**Rationale:** Consistent layout, clear visual anchor, encourages step editing workflow

### Question 4: Multi-Step Selection?
**Answer:** Single-step for MVP, designed to support multi-step later
**Rationale:** Simpler initial implementation, but architecture allows future enhancement

---

## Architecture

### Component Structure

**New Component:**
- `StepEditorSidebar` (`src/ui/components/step_editor_sidebar.rs`)
  - Reads `selected_step` from `SequencerState` context
  - Reads/writes `Pattern` signal for step data
  - Displays parameter controls or empty state
  - Handles input validation and clamping

**Modified Components:**
- `Grid` (`src/ui/components/grid.rs`)
  - Update layout to 2-column: `[Sidebar] [Grid+Labels]`
  - Sidebar always rendered, conditionally filled

- `App` (`src/app.rs`)
  - Remove `StepInspector` from Parameters section
  - Keep `Inspector` (track defaults + LFO) in bottom section

**Removed Component:**
- `StepInspector` - functionality merged into `StepEditorSidebar`

### Component Hierarchy

```
App
├── Header
├── Sequencer Grid Section
│   ├── StepEditorSidebar (NEW)
│   └── Grid
│       ├── Track Labels
│       ├── Grid Steps
│       └── Track Controls
└── Parameters Section
    └── Inspector (track defaults + LFO)
```

---

## Data Model

### AtomicStep Struct (Verified)

All required fields exist in `src/shared/models.rs`:

```rust
pub struct AtomicStep {
    pub trig_type: TrigType,
    pub note: u8,               // ← Pitch (0-127)
    pub velocity: u8,           // ← Velocity (0-127)
    pub length: f32,            // ← Length (step duration)
    pub micro_timing: i8,       // ← Offset (-23 to +23)
    pub condition: TrigCondition, // contains prob: u8 (0-100%)
    pub p_locks: ParameterLocks,
    // ... other fields
}
```

### Parameter Mapping

| UI Control    | Data Field              | Type  | Range      | UI Control Type        |
|---------------|-------------------------|-------|------------|------------------------|
| Pitch         | `step.note`             | `u8`  | 0-127      | Slider + number        |
| Velocity      | `step.velocity`         | `u8`  | 0-127      | Slider + number        |
| Length        | `step.length`           | `f32` | 0.1-4.0    | Slider + number        |
| Probability   | `step.condition.prob`   | `u8`  | 0-100      | Slider + number        |
| Micro-timing  | `step.micro_timing`     | `i8`  | -23 to +23 | Slider or +/- buttons  |

---

## Visual Design

### Layout Structure

```
┌─────────────────────────────────────────────────────┐
│  Header                                             │
├──────────────┬──────────────────────────────────────┤
│ Step Editor  │  Grid Section                        │
│ Sidebar      │  ┌────┬────────────────────┐         │
│              │  │ T1 │ [●●○●○○●●]         │         │
│ (240px)      │  │ T2 │ [○●●○○●○○]         │         │
│              │  └────┴────────────────────┘         │
│              │  Track Controls                      │
└──────────────┴──────────────────────────────────────┘
│  Parameters Section (Inspector + LFO)               │
└─────────────────────────────────────────────────────┘
```

### StepEditorSidebar Styling

**Container:**
- Width: `w-60` (240px)
- Background: `bg-zinc-900/50`
- Border: `border-r border-zinc-800`
- Padding: `p-4`
- Rounded: `rounded-l-lg`

**Header (when step selected):**
```
┌─────────────────────┐
│ EDITING STEP    [×] │
│ Track 2, Step 5     │
├─────────────────────┤
```
- Title: `text-xs font-bold text-zinc-400 uppercase tracking-wide`
- Track/Step info: `text-sm text-zinc-100`
- Close button: `text-xs text-zinc-500 hover:text-red-500`

**Parameter Controls:**
- Vertical stack with `gap-3`
- Each param: `InlineParam` with `ParamLabel` + input control
- Labels: `text-xs font-bold text-blue-400 uppercase` (when locked/active)
- Inputs: Same styling as Inspector (sliders + number inputs)

**Empty State (no step selected):**
```
┌─────────────────────┐
│                     │
│   Select a step     │
│   to edit           │
│   parameters        │
│                     │
│ Tip: Click or       │
│ right-click a step  │
└─────────────────────┘
```
- Centered text: `text-zinc-500 text-sm italic text-center`
- Padding: `py-8`

---

## State Management

### Reactive Data Flow

**Reading step data:**
```rust
let pitch_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks[track_id].subtracks[0].steps[step_idx].note
        })
    } else {
        60 // Default middle C
    }
});
```

**Writing step updates:**
```rust
let on_pitch_change = move |new_value: u8| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        set_pattern_signal.update(|pattern| {
            pattern.tracks[track_id].subtracks[0].steps[step_idx].note = new_value;
        });

        // Optional: Tauri backend sync
        spawn_local(async move {
            safe_invoke("update_step", args).await;
        });
    }
};
```

### Context Usage

**Existing contexts (no new contexts needed):**
- `SequencerState` - contains `selected_step: RwSignal<Option<(usize, usize)>>`
- `ReadSignal<Pattern>` - read pattern state
- `WriteSignal<Pattern>` - update pattern state

---

## User Interactions

### Selection Behavior

**Step selection (existing, unchanged):**
- Click step → selects
- Right-click step → selects
- ESC → deselects
- Click outside grid → deselects

**Sidebar response:**
- When step selected → shows parameter controls
- When deselected → shows empty state
- Always visible (never hidden)

### Close Button

- Small [×] button in sidebar header
- Clicking sets `selected_step.set(None)`
- Same effect as ESC or clicking outside grid

### Input Validation

**Value clamping:**
- Note: 0-127 (enforced by `u8`)
- Velocity: 0-127 (enforced by `u8`)
- Length: 0.1-4.0 (UI enforced, prevent 0 or negative)
- Probability: 0-100 (enforced by `u8` + UI max)
- Micro-timing: -23 to +23 (enforced by `i8` + UI min/max)

---

## Future Enhancements

### Multi-Step Editing (Not MVP)

**Architecture considerations:**
- Current: `selected_step: RwSignal<Option<(usize, usize)>>`
- Future option 1: `selected_steps: RwSignal<Vec<(usize, usize)>>`
- Future option 2: `selected_steps: RwSignal<HashSet<(usize, usize)>>`

**UI when multiple steps selected:**
```
┌─────────────────────┐
│ EDITING 3 STEPS     │
│ Track 2: Steps 1,3,5│
│                     │
│ ⚠️ Batch Edit Mode  │
│ Changes apply to    │
│ all selected steps  │
└─────────────────────┘
```

**Selection method:**
- Shift+click to add/remove steps from selection
- Cmd/Ctrl+click for non-contiguous selection

### Additional Features (Future)

- Keyboard navigation: Arrow keys to move between steps while sidebar open
- Tab to cycle through sidebar controls
- Step copy/paste from sidebar
- Conditional triggers UI (PRE, NEI, FILL logic operators)
- Retrig controls
- Sound lock selector

---

## Implementation Notes

### Phase 1 - Core Structure
1. Create `StepEditorSidebar` component with empty state
2. Modify `Grid` layout to accommodate sidebar
3. Remove `StepInspector` from `App`

### Phase 2 - Parameter Controls
4. Implement Note (Pitch) control with slider + number input
5. Implement Velocity control
6. Implement Length control with validation
7. Implement Probability control
8. Implement Micro-timing control

### Phase 3 - Polish
9. Add close button functionality
10. Smooth transitions and animations
11. Keyboard shortcuts (ESC already works)
12. Testing across different step selections

---

## Success Criteria

- ✅ Sidebar always visible with 240px width
- ✅ All 5 step parameters editable when step selected
- ✅ Empty state shows helpful message
- ✅ Close button deselects step
- ✅ All existing selection methods work unchanged
- ✅ Consistent FLUX styling throughout
- ✅ Reactive updates (no manual DOM manipulation)
- ✅ No regressions in grid or Inspector functionality

---

## Design Rationale

**Why left sidebar instead of bottom panel?**
- Matches Cubase pattern (industry standard)
- Keeps grid vertically centered and prominent
- Clear visual separation: left = step editing, bottom = track controls
- More ergonomic for frequent step parameter tweaks

**Why always visible instead of toggle?**
- Consistent layout (no jarring shifts when selecting/deselecting)
- Clear visual anchor for "where do I edit steps?"
- Empty state encourages interaction ("select a step...")

**Why these 5 parameters?**
- Core step properties from hardware sequencers (Elektron, Elektron Digitakt/Digitone patterns)
- All available as direct fields on `AtomicStep` (clean data model)
- Cover most common per-step adjustments
- Foundation for future expansion (retrig, conditions, sound locks)

---

**Approved by:** User
**Next steps:** Create implementation plan via writing-plans skill

---

## Implementation Notes (Final)

**Completed:** 2026-02-14

**Implementation Summary:**
- Created `StepEditorSidebar` component with 5 parameter controls
- Modified `Grid` layout to 2-column (sidebar + grid)
- Removed old `StepInspector` from App
- All parameters use direct field access on `AtomicStep`
- Smooth transitions and animations added
- Manual testing checklist created

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
