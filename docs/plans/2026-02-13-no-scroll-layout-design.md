# Flux No-Scroll Layout Redesign - Design Document

**Date:** 2026-02-13
**Status:** Approved
**Design Goal:** Fit entire sequencer interface in ~900px viewport height (no scrolling required)

## Executive Summary

Redesign the parameter and LFO sections to eliminate vertical scrolling on all common screen sizes. Current layout requires ~956px, exceeding the ~900px target for universal compatibility (1080p laptops with browser chrome).

**Key Changes:**
1. Replace parameter sliders with compact numeric inputs (~100px saved)
2. Make LFO section collapsible, hidden by default (~400px saved when closed)
3. Reduce section padding modestly (~16px saved)
4. Compact LFO layout when open (~220px additional savings)

**Result:** ~500px when LFO closed, ~650px when LFO open (both well within ~900px target)

## Problem Statement

### Current State
- Header: ~64px
- Grid section: ~228px
- Parameters section: ~616px (8 sliders + LFO)
- **Total: ~956px** (requires scroll on 1080p displays after browser chrome)

### Root Cause
The LFO section (~400px) pushes total height beyond comfortable viewport limits. Parameter sliders with value displays also consume significant vertical space (~160px for 8 parameters).

### Target
~900px total height to fit all common screen sizes without scrolling.

## Design Rationale

### Why This Approach?

**Chosen:** Hybrid compact layout with numeric inputs + collapsible LFO

**Alternative Approaches Considered:**
1. **Compact everything only** - Saved ~130px but still required scroll
2. **Collapsible LFO only** - Worked, but missed opportunity for cleaner parameter interface

**Why hybrid wins:**
- Numeric inputs save ~100px AND provide better modulation visualization (numbers changing in real-time)
- Collapsible LFO makes sense for occasionally-used controls (~400px when closed)
- Still fits comfortably even with LFO open (~650px vs ~900px target)
- Unique interaction model (numeric vs traditional sliders)
- Future-proof with generous headroom

## Section 1: Layout & Spacing

### Padding Reduction

**Change:** Section padding from `p-6` (24px) to `p-5` (20px)

**Affected sections:**
- Grid section card
- Parameters section card

**Impact:**
- Vertical space saved: ~16px (8px × 2 sections)
- Still maintains visual breathing room
- Changes are subtle, not cramped

**Implementation:**
```rust
// flux-app/src/app.rs
<section class="bg-zinc-900/50 rounded-lg p-5">  // was p-6
```

### Spacing Between Sections

**No change:** Keep `space-y-6` (24px) between major sections
- Maintains visual hierarchy
- Clear separation between grid and parameters

## Section 2: Parameter Numeric Inputs

### Overview

Replace range sliders with compact numeric text inputs for all 8 parameters.

### Current vs New Structure

**Current (per parameter):**
```
Label (text-xs)       ~16px
Slider (h-2)          ~8px
Value display         ~16px
Gap                   ~16px
Total per param:      ~56px
```

**New (per parameter):**
```
Label (text-xs)       ~16px
Input (number)        ~24px
Total per param:      ~40px
```

**Savings:** ~16px per parameter × 8 = ~128px total

### Layout Design

**Grid structure:**
- Maintain 2×4 grid: `grid grid-cols-4 gap-x-6 gap-y-3`
- Reduced gap-y from 4 (16px) to 3 (12px)
- Total height: ~60px (down from ~160px)

**Visual mockup:**
```
┌─────────────────────────────────────────┐
│ PARAMETERS    [LFO ▼]    [STEP 4 LOCKED]│
├─────────────────────────────────────────┤
│ TUNING    FILTER   RESONANCE   DRIVE   │
│ [0.50]    [0.50]   [0.50]      [0.50]  │
│                                         │
│ DECAY     SUSTAIN  REVERB      DELAY   │
│ [0.50]    [0.50]   [0.50]      [0.50]  │
└─────────────────────────────────────────┘
```

### Input Specifications

**HTML attributes:**
- Type: `number`
- Step: `0.01`
- Min: `0`
- Max: `1`
- Pattern: Two decimal places (0.00 to 1.00)

**Styling:**
- Labels: `text-xs uppercase tracking-wide text-zinc-400`
- Inputs: `text-xs text-center bg-zinc-800 border border-zinc-700 rounded px-2 py-1`
- P-lock active: Label changes to `text-amber-400`
- Focus: `focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900`

### Modulation Visualization

**Real-time value updates:**
- When LFO/automation is active, input value updates continuously
- Shows actual modulated value, not base value
- Optional enhancement: Add subtle amber border pulse when modulated
  - `class:border-amber-500=is_modulated`
  - `class:animate-pulse=is_modulated`

**Benefits:**
- Immediate visual feedback (numbers changing)
- Clear indication of modulation depth
- Unique aesthetic compared to traditional slider-based sequencers

### Interaction Behavior

**Click to edit:**
- Input becomes focused and selectable
- Text highlights for quick replacement
- Type new value, press Enter or blur to commit

**Keyboard controls:**
- Arrow Up/Down: Increment/decrement by step (0.01)
- Page Up/Down: Larger increments (0.10)
- Home/End: Jump to min (0.0) or max (1.0)

**Validation:**
- Invalid input: Clamp to min/max range
- Empty input: Defaults to 0.0
- Out of range: Clamps silently on blur

**Engine sync:**
- On change: Update pattern signal
- Fire Tauri command to sync with audio engine
- Same logic as current slider implementation

## Section 3: Collapsible LFO Section

### Overview

Hide LFO controls by default (occasionally used), reveal via toggle button in parameters header.

### Toggle Button Design

**Location:** Parameters section header, between title and step lock indicator

**Visual design:**
```
PARAMETERS    [LFO ▼]    [STEP 4 LOCKED]
              ^^^^^^
              Toggle button
```

**Styling:**
- Closed: "LFO ▼"
- Open: "LFO ▲"
- Classes: `text-xs bg-zinc-800 px-3 py-1 rounded hover:bg-zinc-700 cursor-pointer transition-colors`
- Active state: `active:scale-95`

**State management:**
```rust
let (show_lfo, set_show_lfo) = signal(false);  // Collapsed by default
```

### LFO Section Layout (Compact)

**When open, use compact inline layout:**

```
┌─ LFO 1 ────────────────────────────────┐
│ SHAPE      DEST         AMOUNT   SPEED │
│ [Triangle▼][Filter▼]    [0.50]  [1.0] │
│                                        │
│ [Waveform designer - 128px height]    │
└────────────────────────────────────────┘
```

**Structure:**
1. **Top row - 4 controls inline:**
   - Shape: Dropdown (Triangle, Sine, Square, Random, Designer)
   - Destination: Dropdown (Filter Cutoff, Resonance, Mod Wheel, Pan)
   - Amount: Numeric input (-1.0 to 1.0)
   - Speed: Numeric input (0.1 to 4.0)

2. **Bottom section - Waveform designer:**
   - Shown when Shape = "Designer"
   - Fixed height: `h-32` (128px)
   - Otherwise: Placeholder message

**Total height when open:** ~180px
- Control row: ~40px
- Designer: ~128px
- Padding/gaps: ~12px

**Comparison:**
- Current 2-column layout: ~400px
- New compact layout: ~180px
- **Savings: ~220px**

### Animation

**Expand/collapse transition:**
```rust
class="transition-all duration-200 ease-in-out overflow-hidden"
class:max-h-0=move || !show_lfo.get()
class:max-h-96=move || show_lfo.get()
```

**Smooth animation:**
- Duration: 200ms
- Easing: ease-in-out
- Prevents layout shift jarring

## Section 4: Interactive Behavior

### LFO Toggle Interaction

**Click behavior:**
- Click "LFO ▼": Expands section with smooth animation
- Click "LFO ▲": Collapses section with smooth animation
- State: Toggles `show_lfo` signal

**Visual feedback:**
- Hover: `hover:bg-zinc-700` (lighten button)
- Active: `active:scale-95` (tactile press effect)
- Transition: `transition-colors duration-150`

**State persistence:**
- Session-only (does not persist across page reloads)
- Resets to collapsed on fresh load
- Future enhancement: Save to localStorage

### Numeric Input Interaction

**Focus & editing:**
- Click input: Focuses and selects all text
- Type: Replaces selected value
- Enter: Commits value, blurs input
- Escape: Reverts to previous value, blurs input
- Tab: Moves to next parameter in grid order

**Keyboard adjustments:**
- Arrow Up: Increment by 0.01
- Arrow Down: Decrement by 0.01
- Page Up: Increment by 0.10
- Page Down: Decrement by 0.10
- Home: Jump to 0.0
- End: Jump to 1.0

**Scroll wheel (optional):**
- Hover over input + scroll: Adjust value
- Scroll up: Increment
- Scroll down: Decrement
- Requires focus or explicit hover state

### Modulation Visualization

**Real-time updates:**
- When LFO is active and modulating a parameter
- Numeric input value updates continuously (e.g., 30-60 FPS)
- Shows modulated value, not base value

**Visual enhancement (optional):**
- Add `border-amber-500` when modulated
- Add `animate-pulse` for subtle pulsing effect
- Makes modulation immediately obvious

**Implementation note:**
- Listen to same playback-status events that update grid
- Derive modulated values from LFO state + base parameter value
- Update input `prop:value` reactively

### P-Lock Behavior (Unchanged)

**Right-click step:**
- Selects step for P-lock editing
- Updates `selected_step` signal
- Header shows "STEP X LOCKED"

**Visual feedback:**
- Parameter labels turn amber when step locked
- Numeric inputs now edit per-step values
- Same logic as current implementation, different UI

### Focus & Accessibility

**Tab order:**
- Parameters: Left to right, top to bottom (grid order)
- LFO controls: Shape → Destination → Amount → Speed
- Skip designer (complex interaction, separate tab order internally)

**Focus rings:**
- All inputs: `focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900`
- Consistent with grid button focus rings
- High contrast for visibility

**Screen readers:**
- Labels properly associated with inputs
- Aria-labels for toggle button ("Toggle LFO section")
- Current values announced on focus

## Implementation Notes

### Files to Modify

1. **flux-app/src/app.rs**
   - Update section padding: `p-6` → `p-5`

2. **flux-app/src/ui/components/inspector.rs**
   - Replace slider rendering with numeric inputs
   - Add LFO toggle button in header
   - Add `show_lfo` signal
   - Reorganize LFO layout to compact inline
   - Implement collapsible animation

### Breaking Changes

**None.** This is purely a UI refactor:
- No changes to data models
- No changes to Tauri commands
- No changes to audio engine
- Pattern/LFO state structures unchanged

### Testing Checklist

**Functional:**
- [ ] Parameter inputs update pattern state correctly
- [ ] LFO toggle expands/collapses smoothly
- [ ] P-lock still works with numeric inputs
- [ ] Real-time value updates during playback
- [ ] LFO controls work in compact layout
- [ ] Waveform designer shows/hides correctly

**Visual:**
- [ ] Total height fits in ~900px (LFO closed)
- [ ] Total height fits in ~900px (LFO open)
- [ ] No scrolling on 1080p laptop displays
- [ ] Spacing looks balanced, not cramped
- [ ] Focus rings visible on all inputs

**Interaction:**
- [ ] Tab navigation works in logical order
- [ ] Keyboard shortcuts (arrows, page up/down) work
- [ ] Input validation clamps to min/max
- [ ] Hover states provide clear feedback

## Success Criteria

1. **No vertical scrolling** on screens ≥900px viewport height
2. **All functionality preserved** (no features lost)
3. **Modulation visible** (numeric values update in real-time)
4. **Professional feel** (not cramped or cluttered)
5. **Smooth interactions** (animations, focus states, feedback)

## Future Enhancements (Out of Scope)

- Persist LFO toggle state to localStorage
- Scroll wheel input adjustment
- Keyboard shortcuts for quick parameter access
- Animated value changes (smooth number transitions)
- Modulation depth visualization (border thickness/pulse speed)
- Multiple LFO sections (LFO 2, LFO 3, etc.)

## Appendix: Height Calculations

### Before (Current)
- Header: 64px
- Grid section: 228px
- Parameters: 160px (sliders)
- LFO: 400px
- Padding: 48px
- Spacing: 48px
- **Total: ~948px** ❌ Requires scroll

### After (LFO Closed)
- Header: 64px
- Grid section: 228px
- Parameters: 60px (numeric inputs)
- LFO: 0px (collapsed)
- Padding: 40px
- Spacing: 48px
- **Total: ~440px** ✅ Plenty of room

### After (LFO Open)
- Header: 64px
- Grid section: 228px
- Parameters: 60px (numeric inputs)
- LFO: 180px (compact)
- Padding: 40px
- Spacing: 48px
- **Total: ~620px** ✅ Comfortable fit

**Margin:** ~280px headroom even with LFO open
