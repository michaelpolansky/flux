# Compact Step Editor Sidebar Design

**Date:** 2026-02-15
**Status:** Approved
**Approach:** Aggressive Compression

## Problem Statement

The Step Editor sidebar has too LOW information density. The current design uses large fonts (12-14px), generous spacing (8-16px gaps), and includes a complex LFO Designer component that takes significant vertical space. This results in excessive scrolling and makes it difficult to see multiple parameters at once.

Additionally, the sidebar doesn't scroll independently - the entire window scrolls instead, which is disorienting and non-standard for a panel UI.

## Solution Overview

Aggressively compress all visual elements (fonts, spacing, padding) to maximize information density while maintaining readability. Simplify the LFO section by removing the Designer component. Fix the scrolling behavior so the sidebar scrolls independently within the viewport.

**Density improvement:** ~40% more parameters visible at once, reducing scrolling and improving workflow efficiency.

---

## Design Decisions

### Requirements Gathered

Through collaborative design discussion:

1. **Information density:** TOO LOW - needs MORE compression
2. **Scrolling:** Sidebar should scroll independently, not entire window
3. **Controls:** Keep simple number inputs (no fancy sliders/visual controls)
4. **LFO:** Simplify - remove Designer, keep basic 4 shapes
5. **Sections:** Keep collapsible structure, just make headers more compact
6. **Parameters:** All equally important, no prioritization needed
7. **Pattern Overview:** Keep it, just make it more compact

### Approach Selection

**Chosen: Aggressive Compression**

Dramatically reduce all spacing, padding, and font sizes to achieve maximum information density.

**Rationale:**
- Achieves ~40% density improvement
- 10px fonts still very readable (same as professional DAWs)
- Simple implementation (just update sizing values)
- Aligns with pro audio tool expectations

**Alternatives Considered:**
- Moderate Compression (~25% improvement, less dramatic)
- Hybrid Layout (two-column grid, more complex)

---

## Layout & Scrolling Fix

### Problem
Sidebar has `overflow-y-auto` but no height constraint, causing the entire window to scroll instead of just the sidebar.

### Solution
Use flexbox structure with explicit height constraints:

```rust
<div class="w-80 h-screen flex flex-col bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg">
    <!-- Header area (fixed, no scroll) -->
    <div class="flex-shrink-0 p-2">
        Pattern Overview OR Step Header
    </div>

    <!-- Content area (scrolls independently) -->
    <div class="flex-1 overflow-y-auto px-2 pb-2">
        Collapsible sections with parameters
    </div>
</div>
```

**Key changes:**
- `h-screen` - Constrains sidebar to viewport height
- `flex flex-col` - Vertical flex layout
- `flex-shrink-0` on header - Prevents header from being compressed
- `flex-1 overflow-y-auto` on content - Fills remaining space and scrolls

---

## Spacing & Padding Compression

### Sidebar Padding
- **Current:** `p-4` (16px all sides)
- **New:** Header `p-2`, content `px-2 pb-2` (8px, split for flex structure)

### Section Gaps
- **Current:** `gap-2` (8px between sections)
- **New:** `gap-1` (4px between sections)

### Parameter Gaps (within sections)
- **Current:** `gap-1.5` (6px between parameters)
- **New:** `gap-0.5` (2px between parameters)

### Section Padding
- **Current:** `pb-3` (12px bottom padding)
- **New:** `pb-1` (4px bottom padding)

### Header Margins
- **Current:** `mb-4` (16px below header)
- **New:** `mb-2` (8px below header)

### InlineParam Component
- **Current:** Default flex gap
- **New:** `gap-0.5` (2px between label and input)

**Vertical space saved:** ~40-50px, allowing 6-8 more parameters visible

---

## Component Sizing

### Typography Compression

**Section Headers (CollapsibleSection):**
- **Current:** `text-xs font-bold` (12px)
- **New:** `text-[10px] font-bold` (10px)
- Arrow indicator: Keep current size for clickability

**Parameter Labels (ParamLabel):**
- **Current:** `text-[10px] w-24` (10px text, 96px width)
- **New:** `text-[10px] w-20` (10px text, 80px width)
- Keep uppercase, tracking

**Number Inputs (NumberInput):**
- **Current:** `text-xs px-1.5 py-0.5 w-16` (12px text, 64px width)
- **New:** `text-[10px] px-1 py-0.5 w-14` (10px text, 56px width)

**Dropdowns (Dropdown):**
- **Current:** `text-xs px-1.5 py-0.5` (12px)
- **New:** `text-[10px] px-1 py-0.5` (10px)

**Header Text ("EDITING STEP"):**
- **Current:** `text-xs` (12px)
- **New:** `text-[10px]` (10px)
- Track/Step label: `text-sm` (14px) → `text-xs` (12px)

**Pattern Overview Table:**
- Header: `text-xs` (12px) → `text-[10px]` (10px)
- Body: `text-sm` (14px) → `text-xs` (12px)
- Row padding: `py-2` → `py-1`

**Close Button:**
- Keep current size (clickability)

**Height saved:** ~30-40px from smaller fonts and tighter controls

---

## LFO Simplification

### Current LFO Section
- Shape dropdown: 5 options (Sine, Triangle, Square, Random, **Designer**)
- Amount: -1 to 1
- Speed: 0.1 to 4.0
- Destination: Filter Cutoff, Resonance, etc.
- **Designer component:** Visual waveform editor (conditional rendering, ~100px tall)

### Simplified LFO Section
- Shape dropdown: **4 options** (Sine, Triangle, Square, Random)
- Amount: Keep as-is
- Speed: Keep as-is
- Destination: Keep as-is
- **Remove Designer component entirely**

### Code Changes
1. Remove `is_designer` signal derivation (lines 362-371)
2. Remove "Designer" option from Shape dropdown
3. Remove Designer case from `on_shape_change` handler (line 384)
4. Remove conditional Designer component rendering (lines 592-635)
5. Remove `LfoDesigner` import (line 5)

**Space saved:** ~150px vertical space
**Code removed:** ~50 lines

### Rationale
- 4 basic shapes cover 95% of LFO use cases
- Designer is complex, niche feature
- Removes significant visual clutter
- Dramatically improves density

---

## Additional Polish & Details

### Animations
- Keep slide-in animations, reduce duration
- **Current:** `duration-200`
- **New:** `duration-150` (snappier)

### P-Lock Badge
- Reduce to match section header size
- **Current:** `text-xs` (12px)
- **New:** `text-[10px]` (10px)
- Keep amber color and positioning

### Focus States
- Keep all focus rings (accessibility)
- No size changes to focus indicators

### Borders & Separators
- Keep all current borders
- Visual separation more important with denser content

### Hover States
- Keep all hover effects
- Pattern Overview: Keep `hover:bg-zinc-800/30`

### Text Truncation
- Pattern Overview machine names: Keep `truncate` class
- Prevents layout issues

### Visual Hierarchy
- Section headers remain distinct (bold, uppercase, borders)
- Parameters within sections uniform (no prioritization)
- P-lock amber indicators stand out

---

## Implementation Details

### Files to Modify

**Components:**
1. `flux-app/src/ui/components/step_editor_sidebar.rs` - Main sidebar component
2. `flux-app/src/ui/components/collapsible_section.rs` - Section header sizing
3. `flux-app/src/ui/components/form_controls.rs` - Label/input sizing

### Step Editor Sidebar Changes

**Layout structure:**
```rust
<div class="w-80 h-screen flex flex-col bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg">
    {move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            // STEP EDITOR MODE
            view! {
                <div class="flex flex-col h-full">
                    // Header (fixed)
                    <div class="flex-shrink-0 p-2">
                        <div class="flex items-center justify-between mb-2">
                            // ... header content
                        </div>
                    </div>

                    // Content (scrolls)
                    <div class="flex-1 overflow-y-auto px-2 pb-2">
                        <div class="flex flex-col gap-1">
                            // Collapsible sections
                        </div>
                    </div>
                </div>
            }
        } else {
            // PATTERN OVERVIEW MODE
            view! {
                <div class="flex flex-col h-full">
                    // Header (fixed)
                    <div class="flex-shrink-0 p-2 mb-2">
                        <h3 class="text-[10px] font-bold text-zinc-400 uppercase tracking-wide">
                            "PATTERN OVERVIEW"
                        </h3>
                    </div>

                    // Table (scrolls)
                    <div class="flex-1 overflow-y-auto px-2 pb-2">
                        // ... table content
                    </div>
                </div>
            }
        }
    }}
</div>
```

**Section gap:**
```rust
<div class="flex flex-col gap-1">  // Changed from gap-2
    <CollapsibleSection ...>
    <CollapsibleSection ...>
</div>
```

**Remove LFO Designer:**
- Remove `is_designer` signal
- Update Shape dropdown options (remove "Designer")
- Remove conditional Designer rendering block
- Simplify `on_shape_change` handler

### CollapsibleSection Changes

**Header:**
```rust
<div class="flex items-center justify-between px-2 py-1 cursor-pointer ...">
    <div class="flex items-center gap-2">
        <span class="text-zinc-400 text-[10px]">  // Changed from text-xs
            {move || if is_open.get() { "▼" } else { "▶" }}
        </span>
        <span class="text-[10px] font-bold text-zinc-400 uppercase tracking-wide">  // Changed from text-xs
            {title}
        </span>
        {move || {
            if let Some(count_signal) = badge_count {
                let count = count_signal.get();
                if count > 0 {
                    view! {
                        <span class="text-[10px] text-amber-400 ml-1">  // Changed from text-xs
                            {format!("({})", count)}
                        </span>
                    }
                }
            }
        }}
    </div>
</div>
```

**Content:**
```rust
<div class=move || {
    if is_open.get() {
        "flex flex-col gap-0.5 mt-2 transition-all duration-150 ..."  // Changed gap-1.5 → gap-0.5, duration-200 → 150
    } else {
        "hidden"
    }
}>
```

**Border:**
```rust
<div class="flex flex-col pb-1 border-b border-zinc-800/50 last:border-b-0">  // Changed pb-3 → pb-1
```

### Form Controls Changes

**ParamLabel:**
```rust
<label class=move || {
    let base = "text-[10px] font-medium uppercase tracking-tight flex-shrink-0 w-20";  // Changed w-24 → w-20
    // ... color logic
}>
```

**NumberInput:**
```rust
<input
    type="number"
    class="w-14 text-[10px] text-center bg-zinc-800 border border-zinc-700 rounded px-1 py-0.5 ..."  // Changed w-16 → w-14, text-xs → text-[10px], px-1.5 → px-1
/>
```

**Dropdown:**
```rust
<select
    class="bg-zinc-800 text-zinc-50 text-[10px] rounded px-1 py-0.5 border ..."  // Changed text-xs → text-[10px], px-1.5 → px-1
>
```

**InlineParam:**
```rust
<div class="flex items-center gap-0.5">  // Changed default gap → gap-0.5
```

### Pattern Overview Table Changes

**Header:**
```rust
<h3 class="text-[10px] font-bold text-zinc-400 uppercase tracking-wide">  // Changed text-xs → text-[10px]
```

**Table header:**
```rust
<thead>
    <tr class="text-[10px] text-zinc-500 border-b border-zinc-800">  // Changed text-xs → text-[10px]
        <th class="text-left pb-2 pr-2">"Track"</th>
        // ...
    </tr>
</thead>
```

**Table body:**
```rust
<tr class="border-b border-zinc-800/50 hover:bg-zinc-800/30 transition-colors">
    <td class="py-1 pr-2 text-xs text-zinc-100">  // Changed py-2 → py-1, text-sm → text-xs
```

---

## Testing Strategy

### Visual Verification
1. **Scrolling behavior:**
   - Sidebar scrolls independently
   - Window does not scroll when scrolling sidebar
   - Header remains fixed at top
   - Smooth scroll performance

2. **Density improvement:**
   - Count visible parameters before/after
   - Verify ~40% improvement (should see 6-8 more params)

3. **Typography readability:**
   - 10px fonts are readable at normal viewing distance
   - Labels align properly with inputs
   - No text clipping or overflow

4. **Spacing consistency:**
   - All gaps are uniform (0.5px, 1px, 2px system)
   - No awkward cramping or overlapping
   - Visual hierarchy clear despite compression

5. **LFO section:**
   - Designer option removed from dropdown
   - Designer component never renders
   - 4 shapes work correctly (Sine, Triangle, Square, Random)

### Functional Testing
1. **All controls work:**
   - Number inputs accept values
   - Dropdowns change values
   - P-locks create/remove correctly
   - Collapsible sections expand/collapse

2. **Pattern Overview:**
   - Table displays all tracks
   - Stats calculate correctly
   - Hover states work

3. **Empty state:**
   - Pattern Overview shows when no step selected
   - Transitions smoothly between modes

### Edge Cases
1. **Long machine names** - Truncate properly in table
2. **Many tracks** - Scrolling works smoothly
3. **Many P-locks** - Badge displays correctly
4. **Rapid clicking** - Animations don't break

---

## Success Criteria

- [x] Design approved by user
- [ ] Sidebar scrolls independently (not entire window)
- [ ] ~40% density improvement achieved (6-8 more parameters visible)
- [ ] All fonts 10-12px (readable, compact)
- [ ] All spacing compressed (0.5-2px gaps)
- [ ] LFO Designer removed, 4 basic shapes work
- [ ] Collapsible sections maintain functionality
- [ ] Pattern Overview table more compact
- [ ] No visual regressions (borders, colors, hierarchy intact)
- [ ] All controls remain functional

---

## Future Enhancements

Not in this implementation:

1. **Keyboard shortcuts** - Arrow keys to adjust values
2. **Visual controls** - Sliders, knobs, graphs
3. **Smart defaults** - Show most-used params first
4. **Multi-parameter editing** - Edit multiple steps at once
5. **Preset system** - Save/load parameter presets

---

## Notes

- This design prioritizes information density over visual polish
- 10px fonts are standard in pro audio tools (Ableton, Cubase, etc.)
- Removing LFO Designer is a trade-off for density (can be added back if needed)
- Aggressive compression requires good eyesight and monitor
- Users can still scroll to access all parameters, just more are visible at once
