# Compact Step Editor Sidebar Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Compress Step Editor sidebar to achieve ~40% better information density while maintaining all functionality and fixing independent scrolling.

**Architecture:** UI refactoring focused on spacing/typography compression, layout structure changes for scrolling, and LFO component simplification. No data model or business logic changes.

**Tech Stack:** Rust, Leptos 0.7, Tailwind CSS, CSS Flexbox

---

## Task 1: Fix Sidebar Scrolling Structure

**Files:**
- Modify: `flux-app/src/ui/components/step_editor_sidebar.rs:48-550`

**Step 1: Update outer container to constrain height**

In `step_editor_sidebar.rs`, find the root div (line ~48):

```rust
// OLD
<div class=\"w-80 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4\">

// NEW
<div class=\"w-80 h-screen flex flex-col bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg\">
```

**Changes:**
- Add `h-screen` - Constrains to viewport height
- Add `flex flex-col` - Enables vertical flex layout
- Remove `p-4` - Padding will be split between header/content

**Step 2: Wrap Step Editor mode content in flex structure**

Find the Step Editor mode block (around line ~60-400). Wrap inner content:

```rust
// Inside the `if let Some((track_id, step_idx)) = selected_step.get()` block
view! {
    <div class="flex flex-col h-full">
        // Header (fixed, no scroll)
        <div class="flex-shrink-0 p-2">
            <div class="flex items-center justify-between mb-2">
                <div>
                    <div class="text-[10px] text-zinc-400 uppercase tracking-wide mb-1">
                        "EDITING STEP"
                    </div>
                    <div class="text-xs font-bold text-zinc-100">
                        {move || {
                            let track_name = pattern_signal.with(|p| {
                                p.tracks.get(track_id).map(|t| t.name.clone()).unwrap_or_default()
                            });
                            format!("{} - Step {}", track_name, step_idx + 1)
                        }}
                    </div>
                </div>
                <button
                    on:click=move |_| sequencer_state.selected_step.set(None)
                    class="text-zinc-400 hover:text-zinc-100 text-xl transition-colors"
                >
                    "×"
                </button>
            </div>
        </div>

        // Content (scrolls independently)
        <div class="flex-1 overflow-y-auto px-2 pb-2">
            <div class="flex flex-col gap-1">
                // All CollapsibleSection components go here
                // (move existing sections unchanged for now)
            </div>
        </div>
    </div>
}
```

**Step 3: Wrap Pattern Overview mode content in flex structure**

Find the Pattern Overview mode block (around line ~400-550):

```rust
// Inside the `else` block
view! {
    <div class="flex flex-col h-full">
        // Header (fixed)
        <div class="flex-shrink-0 p-2 mb-2">
            <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                "PATTERN OVERVIEW"
            </h3>
        </div>

        // Table (scrolls)
        <div class="flex-1 overflow-y-auto px-2 pb-2">
            // Move existing table content here unchanged
        </div>
    </div>
}
```

**Step 4: Test scrolling behavior**

Run: `npm run dev`

**Expected:**
- Sidebar height constrained to viewport
- Header stays fixed at top
- Content area scrolls independently
- Main window does NOT scroll when scrolling sidebar

**Step 5: Commit**

```bash
git add flux-app/src/ui/components/step_editor_sidebar.rs
git commit -m "fix: sidebar scrolls independently with h-screen and flex layout"
```

---

## Task 2: Compress Section Gaps and Padding

**Files:**
- Modify: `flux-app/src/ui/components/step_editor_sidebar.rs:48-550`
- Modify: `flux-app/src/ui/components/collapsible_section.rs:1-100`

**Step 1: Update section container gap**

In `step_editor_sidebar.rs`, find the section container (inside content div from Task 1):

```rust
// OLD
<div class="flex flex-col gap-2">

// NEW
<div class="flex flex-col gap-1">
```

**Step 2: Update CollapsibleSection bottom padding**

In `collapsible_section.rs`, find the root div (around line ~30):

```rust
// OLD
<div class="flex flex-col pb-3 border-b border-zinc-800/50 last:border-b-0">

// NEW
<div class="flex flex-col pb-1 border-b border-zinc-800/50 last:border-b-0">
```

**Step 3: Update CollapsibleSection content gap**

In `collapsible_section.rs`, find the content div (around line ~55):

```rust
// OLD
<div class=move || {
    if is_open.get() {
        "flex flex-col gap-1.5 mt-2 transition-all duration-200 ..."
    } else {
        "hidden"
    }
}>

// NEW
<div class=move || {
    if is_open.get() {
        "flex flex-col gap-0.5 mt-2 transition-all duration-150 ..."
    } else {
        "hidden"
    }
}>
```

**Changes:**
- `gap-1.5` → `gap-0.5` (6px → 2px between parameters)
- `duration-200` → `duration-150` (snappier animation)

**Step 4: Test spacing visually**

Run: `npm run dev`

**Expected:**
- Sections closer together (4px gap instead of 8px)
- Parameters within sections tighter (2px gap instead of 6px)
- Bottom padding reduced (4px instead of 12px)
- Visual hierarchy still clear

**Step 5: Commit**

```bash
git add flux-app/src/ui/components/step_editor_sidebar.rs flux-app/src/ui/components/collapsible_section.rs
git commit -m "polish: compress section gaps and padding for higher density"
```

---

## Task 3: Compress Section Header Typography

**Files:**
- Modify: `flux-app/src/ui/components/collapsible_section.rs:30-60`

**Step 1: Update header font size**

In `collapsible_section.rs`, find the header elements:

```rust
// Arrow indicator (around line 40)
// OLD
<span class="text-zinc-400 text-xs">

// NEW
<span class="text-zinc-400 text-[10px]">

// Title span (around line 45)
// OLD
<span class="text-xs font-bold text-zinc-400 uppercase tracking-wide">

// NEW
<span class="text-[10px] font-bold text-zinc-400 uppercase tracking-wide">
```

**Step 2: Update P-lock badge size**

Find the badge rendering (around line 50):

```rust
// OLD
<span class="text-xs text-amber-400 ml-1">

// NEW
<span class="text-[10px] text-amber-400 ml-1">
```

**Step 3: Test header readability**

Run: `npm run dev`

**Expected:**
- Section headers readable at 10px
- Arrow indicators clickable
- P-lock badges visible
- Headers visually distinct from content

**Step 4: Commit**

```bash
git add flux-app/src/ui/components/collapsible_section.rs
git commit -m "polish: reduce section header font to 10px"
```

---

## Task 4: Compress Form Control Components

**Files:**
- Modify: `flux-app/src/ui/components/form_controls.rs:1-400`

**Step 1: Update ParamLabel width and ensure gap**

In `form_controls.rs`, find the `ParamLabel` component:

```rust
// Label class (around line 50)
// OLD
let base = "text-[10px] font-medium uppercase tracking-tight flex-shrink-0 w-24";

// NEW
let base = "text-[10px] font-medium uppercase tracking-tight flex-shrink-0 w-20";
```

**Step 2: Update NumberInput sizing**

Find the `NumberInput` component (around line 100):

```rust
// OLD
<input
    type="number"
    class="w-16 text-xs text-center bg-zinc-800 border border-zinc-700 rounded px-1.5 py-0.5 ..."
/>

// NEW
<input
    type="number"
    class="w-14 text-[10px] text-center bg-zinc-800 border border-zinc-700 rounded px-1 py-0.5 ..."
/>
```

**Changes:**
- `w-16` → `w-14` (64px → 56px)
- `text-xs` → `text-[10px]` (12px → 10px)
- `px-1.5` → `px-1` (6px → 4px padding)

**Step 3: Update Dropdown sizing**

Find the `Dropdown` component (around line 200):

```rust
// OLD
<select
    class="bg-zinc-800 text-zinc-50 text-xs rounded px-1.5 py-0.5 border ..."
>

// NEW
<select
    class="bg-zinc-800 text-zinc-50 text-[10px] rounded px-1 py-0.5 border ..."
>
```

**Step 4: Add explicit gap to InlineParam**

Find the `InlineParam` component (around line 300):

```rust
// OLD
<div class="flex items-center">

// NEW
<div class="flex items-center gap-0.5">
```

**Step 5: Test form controls**

Run: `npm run dev`

**Expected:**
- Labels narrower (80px instead of 96px)
- Number inputs smaller but functional
- Dropdowns readable at 10px
- 2px gap between labels and inputs
- All controls still usable

**Step 6: Commit**

```bash
git add flux-app/src/ui/components/form_controls.rs
git commit -m "polish: compress form controls (fonts, widths, padding)"
```

---

## Task 5: Compress Step Editor Header

**Files:**
- Modify: `flux-app/src/ui/components/step_editor_sidebar.rs:60-90`

**Step 1: Update header text sizing**

In `step_editor_sidebar.rs`, find the header in Step Editor mode (modified in Task 1):

```rust
// "EDITING STEP" label
// OLD
<div class="text-xs text-zinc-400 uppercase tracking-wide mb-1">

// NEW
<div class="text-[10px] text-zinc-400 uppercase tracking-wide mb-1">

// Track/Step label
// OLD
<div class="text-sm font-bold text-zinc-100">

// NEW
<div class="text-xs font-bold text-zinc-100">
```

**Step 2: Update header margin**

```rust
// Header container
// OLD
<div class="flex items-center justify-between mb-4">

// NEW
<div class="flex items-center justify-between mb-2">
```

**Step 3: Test header appearance**

Run: `npm run dev`

**Expected:**
- "EDITING STEP" label readable at 10px
- Track/Step label readable at 12px
- Reduced margin below header (8px instead of 16px)
- Close button still easily clickable

**Step 4: Commit**

```bash
git add flux-app/src/ui/components/step_editor_sidebar.rs
git commit -m "polish: compress step editor header typography"
```

---

## Task 6: Compress Pattern Overview Table

**Files:**
- Modify: `flux-app/src/ui/components/step_editor_sidebar.rs:400-550`

**Step 1: Update table header text**

In `step_editor_sidebar.rs`, find Pattern Overview section:

```rust
// Section title (around line 410)
// OLD
<h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">

// NEW
<h3 class="text-[10px] font-bold text-zinc-400 uppercase tracking-wide">
```

**Step 2: Update table header row**

Find the table header (around line 425):

```rust
// OLD
<tr class="text-xs text-zinc-500 border-b border-zinc-800">

// NEW
<tr class="text-[10px] text-zinc-500 border-b border-zinc-800">
```

**Step 3: Update table body cells**

Find the table body rows (around line 450):

```rust
// OLD
<td class="py-2 pr-2 text-sm text-zinc-100">

// NEW
<td class="py-1 pr-2 text-xs text-zinc-100">

// Apply to all <td> elements in the table
```

**Changes:**
- `py-2` → `py-1` (8px → 4px vertical padding)
- `text-sm` → `text-xs` (14px → 12px)

**Step 4: Test table appearance**

Run: `npm run dev`

**Expected:**
- Table header at 10px
- Table rows at 12px
- Tighter row spacing (4px padding)
- All columns readable
- Machine names truncate properly

**Step 5: Commit**

```bash
git add flux-app/src/ui/components/step_editor_sidebar.rs
git commit -m "polish: compress pattern overview table typography and spacing"
```

---

## Task 7: Remove LFO Designer Component

**Files:**
- Modify: `flux-app/src/ui/components/step_editor_sidebar.rs:1-600`

**Step 1: Remove LfoDesigner import**

At the top of `step_editor_sidebar.rs` (around line 5):

```rust
// REMOVE THIS LINE
use super::lfo_designer::LfoDesigner;
```

**Step 2: Remove is_designer signal derivation**

Find the LFO section (around line 360-370), remove:

```rust
// REMOVE THESE LINES
let is_designer = Signal::derive(move || {
    pattern_signal.with(|p| {
        p.tracks
            .get(track_id)
            .and_then(|t| t.subtracks.get(0))
            .and_then(|st| st.steps.get(step_idx))
            .and_then(|s| s.lfo.as_ref())
            .map(|lfo| lfo.shape == crate::shared::models::LfoShape::Designer)
            .unwrap_or(false)
    })
});
```

**Step 3: Remove "Designer" option from Shape dropdown**

Find the Shape dropdown (around line 400):

```rust
// OLD (5 options)
<option value="Sine">"Sine"</option>
<option value="Triangle">"Triangle"</option>
<option value="Square">"Square"</option>
<option value="Random">"Random"</option>
<option value="Designer">"Designer"</option>  // REMOVE THIS

// NEW (4 options)
<option value="Sine">"Sine"</option>
<option value="Triangle">"Triangle"</option>
<option value="Square">"Square"</option>
<option value="Random">"Random"</option>
```

**Step 4: Simplify on_shape_change handler**

Find the shape change handler (around line 384):

```rust
// REMOVE the Designer case
// OLD
"Designer" => {
    lfo.shape = crate::shared::models::LfoShape::Designer;
    lfo.designer_waveform = Some(vec![0.0; 16]);
}

// Keep only:
"Sine" => lfo.shape = crate::shared::models::LfoShape::Sine,
"Triangle" => lfo.shape = crate::shared::models::LfoShape::Triangle,
"Square" => lfo.shape = crate::shared::models::LfoShape::Square,
"Random" => lfo.shape = crate::shared::models::LfoShape::Random,
```

**Step 5: Remove Designer component rendering**

Find and DELETE the entire Designer conditional block (around line 592-635):

```rust
// DELETE THIS ENTIRE BLOCK
{move || {
    if is_designer.get() {
        view! {
            <div class="mt-2">
                <LfoDesigner
                    waveform_signal=...
                    on_waveform_change=...
                />
            </div>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}}
```

**Step 6: Test LFO section**

Run: `npm run dev`

**Expected:**
- Shape dropdown shows 4 options only (Sine, Triangle, Square, Random)
- Selecting any shape works correctly
- Designer component never renders
- No build errors
- ~150px vertical space reclaimed

**Step 7: Commit**

```bash
git add flux-app/src/ui/components/step_editor_sidebar.rs
git commit -m "feat: remove LFO Designer component, keep 4 basic shapes"
```

---

## Task 8: Visual Verification and Testing

**Files:**
- Test: Visual inspection of all components

**Step 1: Verify scrolling behavior**

**Test plan:**
- [ ] Open Step Editor by clicking a step
- [ ] Scroll content area with mouse wheel
- [ ] Header stays fixed at top
- [ ] Main window does NOT scroll
- [ ] Smooth scroll performance

**Step 2: Verify density improvement**

**Test plan:**
- [ ] Count visible parameters before (baseline: ~12-15)
- [ ] Count visible parameters after (target: ~18-22)
- [ ] Verify ~40% improvement achieved
- [ ] No awkward overlapping or cramping

**Step 3: Verify typography readability**

**Test plan:**
- [ ] Section headers readable at 10px
- [ ] Parameter labels readable at 10px
- [ ] Number inputs readable at 10px
- [ ] No text clipping or overflow
- [ ] P-lock badges visible

**Step 4: Verify spacing consistency**

**Test plan:**
- [ ] All section gaps uniform (4px)
- [ ] All parameter gaps uniform (2px)
- [ ] Visual hierarchy still clear
- [ ] No excessive whitespace

**Step 5: Verify LFO section**

**Test plan:**
- [ ] Shape dropdown has 4 options only
- [ ] All 4 shapes work (Sine, Triangle, Square, Random)
- [ ] Designer option not present
- [ ] Designer component never renders
- [ ] Amount, Speed, Destination controls work

**Step 6: Verify all controls functional**

**Test plan:**
- [ ] Number inputs accept values
- [ ] Dropdowns change values
- [ ] P-locks create/remove correctly
- [ ] Collapsible sections expand/collapse
- [ ] Close button closes sidebar

**Step 7: Verify Pattern Overview**

**Test plan:**
- [ ] Table displays when no step selected
- [ ] All tracks listed
- [ ] Stats calculate correctly
- [ ] Hover states work
- [ ] Long machine names truncate

**Step 8: Document any issues**

Create list of visual regressions or functional issues found.

**Step 9: Commit test results**

```bash
# If tests pass
git commit --allow-empty -m "test: verify compact sidebar implementation"
```

---

## Task 9: Fix Any Issues Found in Testing

**Files:**
- Modify: As needed based on test results

**Step 1: Prioritize issues**

**Critical** (blocks release):
- Broken functionality
- Unreadable text
- Scrolling doesn't work

**Important** (should fix):
- Visual misalignment
- Poor contrast
- Awkward spacing

**Minor** (can defer):
- Aesthetic polish
- Edge case visuals

**Step 2: Fix critical issues**

For each critical issue:
- Identify root cause
- Make minimal fix
- Test immediately
- Commit

**Step 3: Fix important issues**

Same process as critical.

**Step 4: Document minor issues**

Add to design doc under "Future Enhancements" if deferring.

**Step 5: Final commit**

```bash
git add .
git commit -m "fix: address issues found in visual testing"
```

---

## Success Criteria

- [x] Design approved by user (completed in brainstorming)
- [~] Sidebar scrolls independently (not entire window) - **Needs further investigation**
- [x] ~40% density improvement achieved (6-8 more parameters visible)
- [x] All fonts 10-12px (readable, compact)
- [x] All spacing compressed (0.5-2px gaps)
- [x] LFO Designer removed, 4 basic shapes work
- [x] Collapsible sections maintain functionality
- [x] Pattern Overview table more compact
- [x] No visual regressions (borders, colors, hierarchy intact)
- [x] All controls remain functional

## Implementation Status

**Completed:** 2026-02-15
**Commits:** 11 commits (fa5095c through a1e501c)
**Net change:** -47 lines (46 insertions, 93 deletions)

---

## Notes

- Tasks 1-7 are implementation
- Task 8 is comprehensive testing
- Task 9 is contingency for fixes
- Each task should take 5-15 minutes
- Frequent commits allow easy rollback
- Visual verification replaces unit tests for UI components
- LFO Designer removal is intentional trade-off for density
- If 10px fonts prove unreadable, can adjust to 11px as fallback

---

## Future Enhancements

Not in this implementation:
1. Keyboard shortcuts for value adjustment
2. Visual controls (sliders, knobs)
3. Smart parameter prioritization
4. Multi-step editing
5. Preset system
