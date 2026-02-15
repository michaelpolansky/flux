# Unified Step Editor Sidebar - Design Document

**Date:** 2026-02-14
**Status:** Approved
**Feature:** Unify step editor sidebar with P-Lock parameters using collapsible sections

---

## Overview

Consolidate all step-level editing into a single unified sidebar by merging the current Step Editor Sidebar with the Inspector's P-Lock parameters. Use collapsible sections to organize step properties, sound parameters, and LFO controls in one place.

**Goal:** Eliminate the conceptual split where step-specific data (P-Locks) lives in the bottom Inspector instead of with other step editing controls.

---

## User Decision

**Question:** How should we unify step editing?

**Answer:** Approach 1 - Expanded Sidebar with Collapsible Sections
- Expand sidebar to 280-320px width
- Three collapsible sections: Step Properties, Sound Parameters, LFO
- Remove bottom Inspector section entirely
- All step/sound editing in one place

**Rationale:**
- Single location for all step-related controls
- Collapsible sections manage complexity without tabs
- Slight width increase (240px → 320px) is reasonable trade-off for unified workflow

---

## Requirements

### Functional Requirements

- **FR1:** Sidebar expands to 320px width (from 240px)
- **FR2:** Three collapsible sections with independent expand/collapse state
- **FR3:** Step Properties section (5 params) - existing functionality
- **FR4:** Sound Parameters section (8 P-lockable synthesis params)
- **FR5:** LFO section (4 controls + designer)
- **FR6:** P-Lock count badge in Sound Parameters header
- **FR7:** Automatic P-Lock creation when editing sound params
- **FR8:** Remove bottom Inspector section entirely
- **FR9:** Empty state when no step selected (hides all sections)

### Non-Functional Requirements

- **NFR1:** Smooth expand/collapse animations (200ms transitions)
- **NFR2:** Scrollable sidebar if content exceeds viewport height
- **NFR3:** Collapse state persists during session
- **NFR4:** Consistent FLUX styling throughout
- **NFR5:** No data model changes (UI reorganization only)

---

## Architecture

### Component Structure

**Modified Component:**
- `StepEditorSidebar` (`src/ui/components/step_editor_sidebar.rs`)
  - Expand from 240px to 320px width
  - Add three collapsible sections
  - Merge P-Lock logic from Inspector
  - Merge LFO controls from Inspector

**New Component:**
- `CollapsibleSection` (`src/ui/components/collapsible_section.rs`)
  - Generic collapsible container
  - Props: title, default_open, optional badge_count
  - State: `RwSignal<bool>` for expanded/collapsed
  - Renders clickable header + conditional content

**Removed Component:**
- `Inspector` (`src/ui/components/inspector.rs`) - deleted entirely

**Updated:**
- `Grid` - no changes (already accommodates sidebar)
- `App` - remove Parameters section entirely

### Component Hierarchy

```
App
├── Header
├── Sequencer Grid Section
│   ├── StepEditorSidebar (320px, collapsible sections)
│   │   ├── Header ("Editing Step" + close button)
│   │   ├── CollapsibleSection: "Step Properties"
│   │   │   ├── Note (Pitch)
│   │   │   ├── Velocity
│   │   │   ├── Length
│   │   │   ├── Probability
│   │   │   └── Micro-timing
│   │   ├── CollapsibleSection: "Sound Parameters (N)"
│   │   │   ├── Tuning
│   │   │   ├── Filter Freq
│   │   │   ├── Resonance
│   │   │   ├── Drive
│   │   │   ├── Decay
│   │   │   ├── Sustain
│   │   │   ├── Reverb
│   │   │   └── Delay
│   │   └── CollapsibleSection: "LFO"
│   │       ├── Shape
│   │       ├── Amount
│   │       ├── Speed
│   │       ├── Destination
│   │       └── Designer (conditional)
│   └── Grid (takes remaining width)
└── [Bottom section removed]
```

---

## Visual Design

### Sidebar Container

```rust
view! {
    <div class="w-80 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col overflow-y-auto">
        // Header with "EDITING STEP" + close button (existing)

        // Collapsible sections
    </div>
}
```

**Dimensions:**
- Width: `w-80` (320px, up from 240px)
- Padding: `p-4` (16px, same as current)
- Available internal width: 320px - 32px = 288px
- Scrollable: `overflow-y-auto` if content exceeds viewport

### Collapsible Section Design

**Header (collapsed):**
```
┌─────────────────────────────────┐
│ ▶ SOUND PARAMETERS (3)          │  ← Collapsed state
└─────────────────────────────────┘
```

**Header (expanded):**
```
┌─────────────────────────────────┐
│ ▼ STEP PROPERTIES               │  ← Expanded state
├─────────────────────────────────┤
│ NOTE (PITCH)            [60]    │
│ VELOCITY                [100]   │
│ ...                             │
└─────────────────────────────────┘
```

**Header Styling:**
- Container: `flex items-center justify-between px-2 py-1.5 cursor-pointer hover:bg-zinc-800/50 rounded transition-colors duration-150`
- Text: `text-xs font-bold text-zinc-400 uppercase tracking-wide`
- Indicator: `▼` (expanded) or `▶` (collapsed)
- Badge (P-Lock count): `text-amber-400 text-xs ml-1` when count > 0

**Content Styling:**
- Container: `flex flex-col gap-3 mt-2` (when expanded)
- Animation: `animate-in slide-in-from-top-2 fade-in duration-200` (expand)
- Animation: `animate-out slide-out-to-top-2 fade-out duration-200` (collapse)

### Section Default States

- **Step Properties:** Expanded by default (`default_open=true`)
- **Sound Parameters:** Expanded by default (`default_open=true`)
- **LFO:** Collapsed by default (`default_open=false`)

---

## State Management

### Collapse State

Each section maintains independent expand/collapse state:

```rust
let step_props_expanded = RwSignal::new(true);
let sound_params_expanded = RwSignal::new(true);
let lfo_expanded = RwSignal::new(false);
```

**Persistence:** Session-only (resets on page reload for MVP)

### Parameter Values

**Step Properties (existing logic):**
```rust
let note_value = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks[track_id].subtracks[0].steps[step_idx].note
        })
    } else {
        60
    }
});
```

**Sound Parameters (adapted from Inspector):**
```rust
let get_param_value = move |param_idx: usize| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            let track = &p.tracks[track_id];
            let step = &track.subtracks[0].steps[step_idx];

            // Check P-Lock first, fallback to track default
            step.p_locks[param_idx]
                .unwrap_or(track.default_params[param_idx])
        })
    } else {
        0.0
    }
};

let is_param_locked = move |param_idx: usize| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks[track_id].subtracks[0].steps[step_idx]
                .p_locks[param_idx].is_some()
        })
    } else {
        false
    }
};
```

**LFO Parameters (track-level):**
```rust
let lfo_shape = Signal::derive(move || {
    let track_id = get_track_id();
    pattern_signal.with(|p| {
        p.tracks[track_id].lfos[0].shape.clone()
    })
});
```

### P-Lock Count Badge

```rust
let p_lock_count = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks[track_id].subtracks[0].steps[step_idx]
                .p_locks.iter()
                .filter(|p| p.is_some())
                .count()
        })
    } else {
        0
    }
});
```

### Context Usage

**Existing contexts (unchanged):**
- `SequencerState` - for `selected_step: RwSignal<Option<(usize, usize)>>`
- `ReadSignal<Pattern>` - read pattern state
- `WriteSignal<Pattern>` - update pattern state

**No new contexts needed.**

---

## User Interactions

### Section Expand/Collapse

**Click header to toggle:**
- Visual: `▼` rotates to `▶`, content slides in/out
- Keyboard: Space/Enter on focused header toggles
- CSS transition: `duration-200` for smooth animation

### Parameter Editing

**Step Properties:**
- Same as current: NumberInput controls
- Direct field updates on AtomicStep
- Immediate reactive updates

**Sound Parameters:**
- When user edits while step selected:
  - Compare new value to track default
  - If different → create P-Lock automatically
  - If same → remove P-Lock (if exists)
- Label turns amber when P-Locked
- No explicit "lock" button (hardware-inspired)

**LFO:**
- Track-level (not step-specific)
- Changes apply to entire track
- Designer UI appears when Shape = "Designer"

### Step Selection

**Existing behavior unchanged:**
- Click/right-click step → selects, shows sections
- ESC or close button → deselects, shows empty state
- Empty state hides all three sections

### Scrolling

- Sidebar scrolls if all sections expanded exceed viewport height
- Scroll position resets when selecting different step (optional)

---

## Data Model

### No Changes Required

**AtomicStep** (unchanged):
```rust
pub struct AtomicStep {
    pub note: u8,
    pub velocity: u8,
    pub length: f32,
    pub micro_timing: i8,
    pub condition: TrigCondition,
    pub p_locks: [Option<f32>; 128], // ← P-Locks for sound params
    // ... other fields
}
```

**Track** (unchanged):
```rust
pub struct Track {
    pub default_params: [f32; 128], // ← Track defaults
    pub lfos: Vec<LFO>,
    // ... other fields
}
```

**Parameter Mapping:**

| UI Control        | Data Field                       | Type    | Range      |
|-------------------|----------------------------------|---------|------------|
| **Step Properties** |                                  |         |            |
| Note (Pitch)      | `step.note`                      | `u8`    | 0-127      |
| Velocity          | `step.velocity`                  | `u8`    | 0-127      |
| Length            | `step.length`                    | `f32`   | 0.1-4.0    |
| Probability       | `step.condition.prob`            | `u8`    | 0-100      |
| Micro-timing      | `step.micro_timing`              | `i8`    | -23 to +23 |
| **Sound Parameters** (P-Lockable) |                    |         |            |
| Tuning            | `step.p_locks[0]` / `track.default_params[0]` | `f32` | 0.0-1.0 |
| Filter Freq       | `step.p_locks[1]` / `track.default_params[1]` | `f32` | 0.0-1.0 |
| Resonance         | `step.p_locks[2]` / `track.default_params[2]` | `f32` | 0.0-1.0 |
| Drive             | `step.p_locks[3]` / `track.default_params[3]` | `f32` | 0.0-1.0 |
| Decay             | `step.p_locks[4]` / `track.default_params[4]` | `f32` | 0.0-1.0 |
| Sustain           | `step.p_locks[5]` / `track.default_params[5]` | `f32` | 0.0-1.0 |
| Reverb            | `step.p_locks[6]` / `track.default_params[6]` | `f32` | 0.0-1.0 |
| Delay             | `step.p_locks[7]` / `track.default_params[7]` | `f32` | 0.0-1.0 |
| **LFO** (Track-level) |                                |         |            |
| Shape             | `track.lfos[0].shape`            | enum    | Sine/Triangle/Square/Random/Designer |
| Amount            | `track.lfos[0].amount`           | `f32`   | -1.0 to 1.0 |
| Speed             | `track.lfos[0].speed`            | `f32`   | 0.1-4.0    |
| Destination       | `track.lfos[0].destination`      | `u8`    | MIDI CC#   |

---

## Bottom Section Changes

### Inspector Removed

- **Delete:** `src/ui/components/inspector.rs`
- **Delete:** Inspector usage in `src/app.rs`
- **Result:** Grid section expands vertically to fill space

### Track Defaults Access

**MVP Decision:** Don't show track defaults in UI

**Rationale:**
- Track defaults are fallback values (rarely edited directly)
- Primary workflow: select step → edit sound (creates P-Locks)
- Can add "Track Settings" modal later if needed

**Future Enhancement:**
- Add "Track Defaults" button in sidebar header
- Opens modal showing all parameters for track-level editing
- Not needed for MVP

---

## Edge Cases

### No Step Selected

- Sidebar shows empty state message: "Select a step to edit parameters"
- All three collapsible sections hidden
- Same behavior as current implementation

### Step with No P-Locks

- Sound Parameters section shows track default values
- All parameter labels in normal zinc-400 color
- Badge shows "(0)" or hidden
- Editing any parameter creates P-Lock automatically

### Step with Some P-Locks

- P-Locked parameters show amber labels
- Non-P-Locked parameters show normal labels + track defaults
- Badge shows count: "SOUND PARAMETERS (3)"
- Editing P-Locked param updates P-Lock
- Editing non-P-Locked param creates new P-Lock

### All Sections Expanded Exceeds Viewport

- Sidebar becomes scrollable (`overflow-y-auto`)
- User scrolls to access parameters below fold
- Collapse sections to reduce vertical space

---

## Migration & Compatibility

### Data Migration

- **None required** - data model unchanged
- Existing P-Locks continue to work
- Existing track defaults preserved

### Code Migration

- Move P-Lock logic from `inspector.rs` to `step_editor_sidebar.rs`
- Move LFO controls from `inspector.rs` to `step_editor_sidebar.rs`
- Delete `inspector.rs` and its imports
- Update `app.rs` to remove Parameters section

### User Migration

- Users familiar with bottom Inspector will find controls in sidebar instead
- P-Lock workflow identical (edit param while step selected → creates lock)
- More intuitive: all step editing in one place

---

## Implementation Phases

### Phase 1: Create CollapsibleSection Component
1. Create `src/ui/components/collapsible_section.rs`
2. Implement expand/collapse logic with animations
3. Add to `mod.rs`

### Phase 2: Expand Sidebar with Sections
4. Increase sidebar width to 320px
5. Wrap existing step properties in CollapsibleSection
6. Test expand/collapse behavior

### Phase 3: Add Sound Parameters Section
7. Move P-Lock logic from Inspector to StepEditorSidebar
8. Create Sound Parameters collapsible section
9. Add 8 parameter controls with P-Lock detection
10. Implement P-Lock count badge

### Phase 4: Add LFO Section
11. Move LFO logic from Inspector to StepEditorSidebar
12. Create LFO collapsible section
13. Add 4 LFO controls + Designer UI

### Phase 5: Remove Inspector
14. Delete `inspector.rs`
15. Remove Inspector from `app.rs`
16. Update Grid layout if needed
17. Test entire flow

### Phase 6: Polish
18. Smooth animations
19. Keyboard navigation
20. Scrolling behavior
21. Visual polish

---

## Success Criteria

- ✅ Sidebar expanded to 320px width
- ✅ Three collapsible sections functional (Step Properties, Sound Parameters, LFO)
- ✅ P-Lock creation/removal works automatically
- ✅ P-Lock count badge shows accurate count
- ✅ Amber labels indicate P-Locked parameters
- ✅ LFO controls work identically to Inspector
- ✅ Bottom Inspector section removed
- ✅ Empty state shows when no step selected
- ✅ Smooth expand/collapse animations
- ✅ Scrollable sidebar when content exceeds viewport
- ✅ No regressions in step editing functionality
- ✅ All existing P-Locks preserved and functional

---

## Design Rationale

**Why unify instead of keeping separate sections?**
- Conceptual consistency: P-Locks ARE step-specific data
- Single source of truth for step editing
- Reduces cognitive load (look in one place, not two)
- Clearer mental model: "When I select a step, I edit ALL its properties here"

**Why collapsible sections instead of tabs?**
- No context switching (all sections visible when expanded)
- Clear indication of P-Lock count without opening tab
- Flexibility to expand multiple sections simultaneously
- Simpler implementation than tab state management

**Why 320px sidebar width?**
- Accommodates parameter labels + controls comfortably
- Leaves sufficient space for grid (especially on 1920px+ displays)
- Similar to Cubase inspector width (proportionally)

**Why remove track defaults from UI?**
- Simplifies MVP scope
- Track defaults are rarely edited directly
- P-Lock workflow doesn't require seeing defaults
- Can add later via modal if users request it

---

**Approved by:** User
**Next steps:** Create implementation plan via writing-plans skill

---
