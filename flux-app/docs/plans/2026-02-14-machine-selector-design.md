# Machine Selector for Each Track - Design Document

**Date:** 2026-02-14
**Status:** Approved
**Feature:** Per-track machine type selector with dropdown UI

---

## Overview

Add a machine type selector next to each track label, allowing users to choose between 7 machine types (OneShot, Werp, Slice, FmTone, Subtractive, TonverkBus, MidiCC) for each track. The selector uses a compact dropdown design with abbreviations to fit within the existing track label area.

---

## Requirements

### Functional Requirements

- **FR1:** Display current machine type as 2-3 letter abbreviation next to track label
- **FR2:** Click abbreviation to open dropdown showing all 7 machine types
- **FR3:** Select machine type from dropdown to update track
- **FR4:** Preserve all existing track data (steps, p-locks) when changing machine type
- **FR5:** Close dropdown on: selection, ESC key, click outside
- **FR6:** Only one dropdown open at a time across all tracks
- **FR7:** Keyboard navigation support (arrow keys, Enter, ESC)

### Non-Functional Requirements

- **NFR1:** Compact design - fits next to track label without expanding grid width
- **NFR2:** Consistent FLUX zinc/dark theme styling
- **NFR3:** Smooth transitions and hover states
- **NFR4:** Reactive updates via Leptos signals (no manual DOM manipulation)

---

## User Decisions

### Question 1: UI Placement
**Answer:** Next to track label (T1, T2, etc.)
**Rationale:** Always visible, quick access, minimal clicks

### Question 2: Interaction Style
**Answer:** Dropdown menu
**Rationale:** Shows all options at once, familiar pattern, clear selection

### Question 3: Visual Display
**Answer:** Short abbreviations (OS, WP, SL, etc.)
**Rationale:** Compact, fits well in limited space, tech aesthetic

### Question 4: Data Behavior on Change
**Answer:** Preserve all data
**Rationale:** Non-destructive, allows experimentation, matches hardware sequencer behavior

---

## Approach Selected

**Approach 1: Dedicated MachineSelector Component**

Create a focused, reusable component following the same pattern as `RemoveTrackButton` and `TrackControls`. This maintains architectural consistency and keeps the grid component clean.

**Rejected Approaches:**
- Inline dropdown in Grid: Violates single responsibility, makes grid complex
- Generic Dropdown<T>: Over-engineering (YAGNI), added complexity for current needs

---

## Component Architecture

### New Component: `MachineSelector`

**Location:** `src/ui/components/machine_selector.rs`

**Props:**
- `track_idx: usize` - Index of track this selector controls

**Internal State:**
- `is_open: RwSignal<bool>` - Dropdown open/closed state

**Context Dependencies:**
- `ReadSignal<Pattern>` - Read current machine type
- `WriteSignal<Pattern>` - Update machine type

**Component Structure:**
```rust
#[component]
pub fn MachineSelector(
    track_idx: usize,
) -> impl IntoView {
    // Context access
    let pattern_signal = use_context::<ReadSignal<Pattern>>();
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>();

    // Local state
    let (is_open, set_is_open) = signal(false);

    // Read current machine
    let current_machine = move || {
        pattern_signal.with(|p| {
            p.tracks.get(track_idx)
                .map(|t| t.machine)
                .unwrap_or(MachineType::OneShot)
        })
    };

    // Update machine
    let set_machine = move |new_machine: MachineType| {
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_idx) {
                track.machine = new_machine;
            }
        });
        set_is_open.set(false);
    };

    // Toggle dropdown
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    // Render button + dropdown
    view! { /* ... */ }
}
```

### Integration Point

**File:** `src/ui/components/grid.rs`

**Location:** Track label row (line ~126-134)

**Updated Structure:**
```rust
<div class="flex items-center gap-1">
    <div class="text-xs text-zinc-400">
        {format!("T{}", track_idx + 1)}
    </div>
    <MachineSelector track_idx=track_idx />  // NEW
    <RemoveTrackButton
        track_idx=track_idx
        show_confirm=set_show_confirm_dialog
    />
</div>
```

---

## Machine Type Abbreviations

| Machine Type | Abbreviation | Description |
|-------------|--------------|-------------|
| OneShot     | **OS**       | Digitakt II one-shot sample |
| Werp        | **WP**       | Digitakt II werp engine |
| Slice       | **SL**       | Octatrack slice mode |
| FmTone      | **FM**       | Digitone FM synthesis |
| Subtractive | **SUB**      | Analog Four subtractive synth |
| TonverkBus  | **TNV**      | Tonverk bus processing |
| MidiCC      | **CC**       | External MIDI control |

**Mapping Function:**
```rust
fn machine_abbreviation(machine: MachineType) -> &'static str {
    match machine {
        MachineType::OneShot => "OS",
        MachineType::Werp => "WP",
        MachineType::Slice => "SL",
        MachineType::FmTone => "FM",
        MachineType::Subtractive => "SUB",
        MachineType::TonverkBus => "TNV",
        MachineType::MidiCC => "CC",
    }
}

fn machine_full_name(machine: MachineType) -> &'static str {
    match machine {
        MachineType::OneShot => "OneShot",
        MachineType::Werp => "Werp",
        MachineType::Slice => "Slice",
        MachineType::FmTone => "FmTone",
        MachineType::Subtractive => "Subtractive",
        MachineType::TonverkBus => "TonverkBus",
        MachineType::MidiCC => "MidiCC",
    }
}
```

---

## Data Flow & State Management

### Reading Current Machine Type

```rust
let current_machine = move || {
    pattern_signal.with(|p| {
        p.tracks.get(track_idx)
            .map(|t| t.machine)
            .unwrap_or(MachineType::OneShot)
    })
};
```

**Behavior:**
- Reads from Pattern context
- Uses `.get()` for safety (track might not exist)
- Defaults to OneShot if track missing

### Updating Machine Type

```rust
let set_machine = move |new_machine: MachineType| {
    set_pattern_signal.update(|pattern| {
        if let Some(track) = pattern.tracks.get_mut(track_idx) {
            track.machine = new_machine;
        }
    });
    set_is_open.set(false); // Close dropdown
};
```

**Behavior:**
- Updates Pattern signal (triggers reactive re-render)
- Only updates if track exists
- Closes dropdown after selection
- **Preserves all existing data** (steps, p-locks, LFOs, parameters)

### Dropdown State Management

**Local State:**
- `is_open: RwSignal<bool>` - Component-local state
- Not shared across tracks (each has own dropdown state)

**Open/Close Triggers:**
- **Open:** Click button, toggles state
- **Close:**
  - Select option
  - ESC key press
  - Click outside dropdown
  - Open another dropdown (future: global state coordination)

**Click Outside Handler:**
```rust
// Window-level click listener
window_event_listener(ev::click, move |event| {
    if is_open.get() {
        // Check if click target is outside dropdown
        // Close if outside
    }
});
```

---

## UI/UX Design

### Closed State (Button)

**Visual Appearance:**
- Abbreviation displayed (e.g., "OS", "FM", "SUB")
- Down arrow indicator: `▾`
- Combined: "OS ▾"

**Styling:**
```rust
class="px-1.5 py-0.5 text-[10px] font-mono bg-zinc-700 text-zinc-300
       border border-zinc-600 rounded hover:bg-zinc-600 transition-colors
       cursor-pointer"
```

**Dimensions:**
- Width: ~30-35px (auto based on content)
- Height: ~18px (fits with track label)

### Open State (Dropdown Menu)

**Visual Appearance:**
- List of all 7 machine types with full names
- Current selection highlighted
- Hover state on each option

**Styling:**
```rust
// Dropdown container
class="absolute top-full left-0 mt-1 bg-zinc-800 border border-zinc-600
       rounded shadow-lg z-50 min-w-[120px]"

// Each option
class="px-3 py-1.5 text-sm text-zinc-300 hover:bg-zinc-700 cursor-pointer
       transition-colors"

// Current selection highlight
class="bg-blue-900/30"
```

**Positioning:**
- Absolute positioned below button
- `z-50` ensures it appears above grid
- `min-w-[120px]` ensures readable option names
- Future: Upward positioning if near screen bottom

### Interaction States

**Button States:**
- Normal: `bg-zinc-700`
- Hover: `bg-zinc-600`
- Active (dropdown open): `bg-zinc-600 border-blue-500`

**Dropdown Option States:**
- Normal: `bg-zinc-800`
- Hover: `bg-zinc-700`
- Current: `bg-blue-900/30`
- Selected (on click): `bg-blue-800` briefly, then closes

### Accessibility

**Keyboard Navigation:**
- **Tab:** Focus button
- **Enter/Space:** Open dropdown
- **Arrow Up/Down:** Navigate options (when open)
- **Enter:** Select highlighted option
- **ESC:** Close dropdown
- **Tab (when open):** Close and move focus

**Visual Feedback:**
- Clear hover states
- Current selection highlighted
- Focus ring on button when keyboard-focused

---

## Error Handling & Edge Cases

### 1. Invalid Track Index

**Scenario:** Component receives `track_idx` that doesn't exist in pattern

**Handling:**
```rust
let current_machine = move || {
    pattern_signal.with(|p| {
        p.tracks.get(track_idx)
            .map(|t| t.machine)
            .unwrap_or(MachineType::OneShot) // Fallback
    })
};
```

**Behavior:** Display fallback (OneShot), disable interaction if track truly missing

### 2. Track Removal While Dropdown Open

**Scenario:** User opens dropdown, then removes the track

**Handling:**
- Track removal triggers grid re-render
- Component unmounts with the track
- Leptos cleanup handles event listener removal
- No memory leaks or stale state

**Prevention:** Could disable track removal while any dropdown is open (optional)

### 3. Multiple Dropdowns Open

**Current:** Each component manages own `is_open` state (multiple can be open)

**Future Enhancement:** Global dropdown coordinator
```rust
// Shared context
let (active_dropdown, set_active_dropdown) = signal::<Option<usize>>(None);

// In MachineSelector
let toggle = move |_| {
    if active_dropdown.get() == Some(track_idx) {
        set_active_dropdown.set(None); // Close self
    } else {
        set_active_dropdown.set(Some(track_idx)); // Close others, open self
    }
};
```

**Decision:** Start simple (allow multiple), add coordinator if UX issue

### 4. Rapid Clicks

**Scenario:** User clicks button rapidly

**Handling:**
- Leptos signal updates are batched and efficient
- Toggle state handles rapid updates gracefully
- No debouncing needed

### 5. Dropdown Clipping/Overflow

**Scenario:** Dropdown near screen edge gets cut off

**Initial:** Always render downward (simple)

**Future Enhancement:** Detect position, render upward if needed
```rust
let position = move || {
    // Check available space below
    // Return "down" or "up"
};
```

### 6. Pattern Context Missing

**Scenario:** Component renders without Pattern context (shouldn't happen)

**Handling:**
```rust
let pattern_signal = use_context::<ReadSignal<Pattern>>()
    .expect("Pattern context not found");
```

**Behavior:** Panic in development (clear error), would need try_context for graceful degradation

### 7. Concurrent Machine Changes

**Scenario:** User changes machine while backend is processing

**Handling:**
- Frontend updates immediately (optimistic)
- Backend receives updated pattern on next save/playback
- No conflicts (frontend is source of truth for pattern editing)

---

## Testing Strategy

### Manual Testing Checklist

#### Basic Functionality
- [ ] Click machine selector → dropdown opens
- [ ] Click "OneShot" option → track.machine updates to OneShot, dropdown closes
- [ ] Abbreviation on button updates to "OS"
- [ ] Click selector again → dropdown opens showing "OneShot" highlighted
- [ ] Test all 7 machine types: OS, WP, SL, FM, SUB, TNV, CC
- [ ] Each selection updates correctly

#### Data Persistence
- [ ] Add 5 steps to a track (OneShot)
- [ ] Change machine to Subtractive
- [ ] Verify all 5 steps still present
- [ ] Change back to OneShot
- [ ] Verify steps still intact
- [ ] Add p-locks and LFOs, change machine → all preserved

#### Dropdown Behavior
- [ ] Click outside dropdown → closes without selection
- [ ] Press ESC → closes without selection
- [ ] Open dropdown on Track 1, click Track 2 selector → Track 1 closes, Track 2 opens (if global state implemented)
- [ ] Current machine highlighted in dropdown with blue background
- [ ] Hover over option → background changes to zinc-700

#### Integration with Track Management
- [ ] Add new track → machine selector shows "OS" (default)
- [ ] Click selector on new track → all 7 options available
- [ ] Remove a track with dropdown open → dropdown closes gracefully
- [ ] Re-index tracks (remove T2) → machine selectors update correctly

#### Visual & Styling
- [ ] Button size doesn't expand track label area
- [ ] Abbreviations legible at 10px font size
- [ ] Dropdown appears above grid rows (z-50 works)
- [ ] Hover states smooth and visible
- [ ] Colors match FLUX zinc theme
- [ ] Dropdown doesn't clip at screen edges (test near top/bottom)

#### Keyboard Navigation
- [ ] Tab to button → focus visible
- [ ] Enter/Space → opens dropdown
- [ ] Arrow down → highlights first option
- [ ] Arrow up/down → navigates options
- [ ] Enter on option → selects and closes
- [ ] ESC → closes dropdown
- [ ] Tab when dropdown open → closes and moves focus

#### Edge Cases
- [ ] Rapid clicking button → no visual glitches
- [ ] Click button 10x fast → state stable
- [ ] Open all dropdowns (4+ tracks) → all work independently
- [ ] Change machine during playback → playback continues
- [ ] Save pattern, reload → machine types persist correctly

#### Performance
- [ ] Dropdown opens instantly (<50ms perceived)
- [ ] Machine change updates UI immediately
- [ ] No lag with 10+ tracks open
- [ ] Smooth hover transitions

### Code Review Checklist

- [ ] Component follows Leptos patterns (like RemoveTrackButton)
- [ ] Proper use of `use_context` for Pattern signals
- [ ] Signal management clean (no memory leaks)
- [ ] Event listeners properly registered and cleaned up
- [ ] Component exported in `mod.rs`
- [ ] Abbreviation mapping function correct
- [ ] Full name mapping function correct
- [ ] Error handling for missing tracks
- [ ] Consistent styling with FLUX theme
- [ ] No hardcoded colors (use Tailwind classes)

### Automated Tests (Future)

**Unit Tests:**
- Test abbreviation mapping function
- Test full name mapping function
- Test all 7 machine types convert correctly

**Integration Tests:**
- Test machine type updates Pattern signal
- Test data preservation on machine change
- Test dropdown state management

---

## Implementation Files

### Files to Create
1. `src/ui/components/machine_selector.rs` - Main component

### Files to Modify
1. `src/ui/components/mod.rs` - Export MachineSelector
2. `src/ui/components/grid.rs` - Add MachineSelector to track labels (import + render)

### Optional Future Files
1. `src/ui/components/dropdown.rs` - Generic dropdown if we need more dropdowns later

---

## Future Enhancements (Out of Scope)

### Phase 2: Enhanced UX
- **Global dropdown coordinator** - Only one dropdown open at a time
- **Smart positioning** - Render upward if near screen bottom
- **Machine icons** - Visual indicators for each machine type
- **Grouped categories** - Sample machines vs synth machines vs MIDI

### Phase 3: Advanced Features
- **Machine-specific parameter presets** - Load default params when switching machines
- **Machine compatibility warnings** - "This machine doesn't support p-locks" etc.
- **Bulk machine assignment** - Select multiple tracks, set all to same machine
- **Machine type colors** - Color-code tracks by machine type

### Phase 4: Backend Integration
- **Machine type affects audio engine** - Actually use different synthesis/sampling engines
- **Machine-specific parameters** - Different param mappings per machine type
- **Machine validation** - Backend verifies machine type is valid for operations

---

## Success Criteria

The machine selector feature is successful when:

1. **Usability:** Users can change machine type in 2 clicks (open + select)
2. **Clarity:** Current machine type always visible via abbreviation
3. **Safety:** Data never lost when changing machines
4. **Polish:** Smooth interactions, clear visual feedback, FLUX aesthetic
5. **Reliability:** No crashes, no visual glitches, handles all edge cases
6. **Integration:** Works seamlessly with add/remove tracks, pattern save/load

---

## Approval

**Design Status:** ✅ Approved
**Approved By:** User
**Date:** 2026-02-14

**Next Step:** Create implementation plan using `writing-plans` skill.
