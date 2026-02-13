# No-Scroll Layout Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Eliminate vertical scrolling by replacing parameter sliders with numeric inputs and making LFO section collapsible.

**Architecture:** Update Leptos components in `inspector.rs` and `app.rs` to use compact numeric inputs and collapsible LFO section. No backend/engine changes required.

**Tech Stack:** Leptos 0.7 (WASM), Tailwind CSS 4.x, existing component structure

---

## Phase 1: Reduce Padding

### Task 1: Update Section Padding in App.rs

**Files:**
- Modify: `flux-app/src/app.rs:61-88`

**Step 1: Update grid section padding**

Change line 61 from:
```rust
<section class="bg-zinc-900/50 rounded-lg p-6">
```

To:
```rust
<section class="bg-zinc-900/50 rounded-lg p-5">
```

**Step 2: Update parameters section padding**

Change line 69 from:
```rust
<section class="bg-zinc-900/50 rounded-lg p-6">
```

To:
```rust
<section class="bg-zinc-900/50 rounded-lg p-5">
```

**Step 3: Visual test in browser**

Run: `source ~/.cargo/env && npm run dev` (if not already running)
Open: http://localhost:1420
Expected: Sections have slightly less padding (20px vs 24px)

**Step 4: Commit padding changes**

```bash
git add flux-app/src/app.rs
git commit -m "refactor: reduce section padding for compact layout

- Update p-6 to p-5 on grid and parameters sections
- Saves ~16px vertical space
- Part of no-scroll layout redesign

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 2: Replace Parameter Sliders with Numeric Inputs

### Task 2: Update Parameter Input Rendering

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:95-140`

**Step 1: Locate current slider rendering**

Find the section around line 95-140 that renders the 8 parameter sliders in a `grid grid-cols-4` layout. Current structure has:
- Label
- `<input type="range">`
- Value display div

**Step 2: Replace slider structure with numeric inputs**

Replace the entire parameter rendering section (inside the `grid grid-cols-4` div) with:

```rust
<div class="grid grid-cols-4 gap-x-6 gap-y-3">
    {params.into_iter().enumerate().map(|(idx, name)| {
        let handle_input = handle_input.clone();
        let name_str = name.to_string();
        view! {
            <div class="flex flex-col gap-1">
                <label class=move || {
                    let base = "text-xs font-medium uppercase tracking-wide";
                    let color = if sequencer_state.selected_step.get().is_some() && is_locked(idx) {
                        "text-amber-400"
                    } else {
                        "text-zinc-400"
                    };
                    format!("{} {}", base, color)
                }>
                    {name}
                </label>
                <input
                    type="number"
                    min="0"
                    max="1"
                    step="0.01"
                    prop:value=move || format!("{:.2}", get_value(idx))
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                        let clamped = val.clamp(0.0, 1.0);
                        handle_input(idx, clamped, name_str.clone());
                    }
                    on:keydown=move |ev| {
                        let key = ev.key();
                        match key.as_str() {
                            "ArrowUp" => {
                                ev.prevent_default();
                                let current = get_value(idx);
                                let new_val = (current + 0.01).clamp(0.0, 1.0);
                                handle_input(idx, new_val, name_str.clone());
                            }
                            "ArrowDown" => {
                                ev.prevent_default();
                                let current = get_value(idx);
                                let new_val = (current - 0.01).clamp(0.0, 1.0);
                                handle_input(idx, new_val, name_str.clone());
                            }
                            _ => {}
                        }
                    }
                    class="w-full text-xs text-center bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-zinc-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900 transition-colors"
                />
            </div>
        }
    }).collect::<Vec<_>>()}
</div>
```

**Step 3: Update gap-y spacing**

Note: The `gap-y-4` is changed to `gap-y-3` in the grid div to reduce vertical spacing.

**Step 4: Visual test in browser**

Run: Reload http://localhost:1420 (should auto-reload)
Expected:
- 8 numeric input boxes in 2×4 grid
- Can type values directly
- Arrow keys increment/decrement
- Values clamp to 0.0-1.0 range
- Labels turn amber when step is P-locked

**Step 5: Functional test**

Actions:
1. Click a grid step (right-click to P-lock)
2. Type a value in a parameter input (e.g., "0.75")
3. Press Enter
4. Play the sequence
5. Verify parameter changes are audible

Expected: Parameter editing works identically to slider version

**Step 6: Commit numeric inputs**

```bash
git add flux-app/src/ui/components/inspector.rs
git commit -m "feat: replace parameter sliders with numeric inputs

- Replace range sliders with number input fields
- Reduce gap-y from 4 to 3 for compact spacing
- Add arrow key increment/decrement support
- Saves ~100px vertical space
- Prepares for real-time modulation visualization

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 3: Add Collapsible LFO Section

### Task 3: Add LFO Toggle Button to Header

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:70-85` (parameters section header in app.rs)

**Context:** We need to add the toggle button in the parameters section header. This is actually in `app.rs`, not `inspector.rs`.

**Step 1: Add LFO toggle signal in App component**

In `flux-app/src/app.rs`, after line 26 where `selected_step` is created, add:

```rust
let (show_lfo, set_show_lfo) = signal(false); // LFO collapsed by default
```

**Step 2: Provide LFO toggle to context**

After line 33, add:

```rust
provide_context(show_lfo);
provide_context(set_show_lfo);
```

**Step 3: Add toggle button to parameters header**

In `flux-app/src/app.rs`, around line 70-84, update the parameters section header:

Change from:
```rust
<div class="flex items-center justify-between mb-4">
    <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Parameters"</h2>
    <div class="flex items-center gap-2">
        // ... step lock indicator ...
    </div>
</div>
```

To:
```rust
<div class="flex items-center justify-between mb-4">
    <div class="flex items-center gap-3">
        <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Parameters"</h2>
        <button
            on:click=move |_| set_show_lfo.update(|v| *v = !*v)
            class="text-xs bg-zinc-800 px-3 py-1 rounded hover:bg-zinc-700 cursor-pointer transition-colors active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
        >
            {move || if show_lfo.get() { "LFO ▲" } else { "LFO ▼" }}
        </button>
    </div>
    <div class="flex items-center gap-2">
        // ... step lock indicator (unchanged) ...
    </div>
</div>
```

**Step 4: Visual test**

Run: Reload browser
Expected:
- "LFO ▼" button appears next to "PARAMETERS" header
- Clicking toggles between "LFO ▼" and "LFO ▲"
- Button has hover and active states

**Step 5: Commit toggle button**

```bash
git add flux-app/src/app.rs
git commit -m "feat: add LFO toggle button to parameters header

- Add show_lfo signal to App state
- Add toggle button next to Parameters title
- Button shows ▼ when closed, ▲ when open
- Provides context for LFO section collapsing

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

### Task 4: Make LFO Section Collapsible in Inspector

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:143-289`

**Step 1: Get LFO toggle state from context**

At the top of the `Inspector` component function (around line 17), add:

```rust
let show_lfo = use_context::<ReadSignal<bool>>().expect("show_lfo context not found");
```

**Step 2: Wrap LFO section in conditional rendering**

Find the LFO section (starting around line 143, `<div class="mt-4 pt-4 border-t border-zinc-800">`).

Wrap the entire LFO section div in a conditional:

```rust
{move || {
    if show_lfo.get() {
        view! {
            <div class="mt-4 pt-4 border-t border-zinc-800 transition-all duration-200">
                // ... existing LFO content ...
            </div>
        }.into_any()
    } else {
        view! { <div></div> }.into_any()
    }
}}
```

**Step 3: Visual test**

Run: Reload browser
Expected:
- LFO section hidden by default
- Clicking "LFO ▼" shows LFO section
- Clicking "LFO ▲" hides LFO section
- Smooth transition (200ms)

**Step 4: Functional test**

Actions:
1. Open LFO section
2. Change LFO settings (shape, destination, amount, speed)
3. Close LFO section
4. Play sequence
5. Verify LFO modulation is still active

Expected: LFO works whether section is visible or not

**Step 5: Commit collapsible LFO**

```bash
git add flux-app/src/ui/components/inspector.rs
git commit -m "feat: make LFO section collapsible

- Get show_lfo signal from context
- Wrap LFO section in conditional rendering
- Add smooth transition animation
- Saves ~400px when collapsed

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 4: Compact LFO Layout

### Task 5: Reorganize LFO to Inline Layout

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:143-289`

**Step 1: Restructure LFO grid to inline controls**

Replace the current 2-column LFO grid (`grid grid-cols-2`) with a more compact layout:

**Find the section starting with:**
```rust
<div class="grid grid-cols-2 gap-4">
```

**Replace entire LFO section content with:**

```rust
<div class="mt-4 pt-4 border-t border-zinc-800 transition-all duration-200">
    <h3 class="text-sm font-bold text-zinc-400 mb-3">LFO 1</h3>

    // Inline controls row
    <div class="grid grid-cols-4 gap-4 mb-3">
        // Shape
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-500">Shape</label>
            <select
                class="bg-zinc-800 text-zinc-300 text-xs rounded p-1 border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_pattern_signal.update(|p| {
                       if let Some(track) = p.tracks.get_mut(track_id) {
                           if let Some(lfo) = track.lfos.get_mut(0) {
                                match val.as_str() {
                                    "Sine" => lfo.shape = crate::shared::models::LFOShape::Sine,
                                    "Triangle" => lfo.shape = crate::shared::models::LFOShape::Triangle,
                                    "Square" => lfo.shape = crate::shared::models::LFOShape::Square,
                                    "Random" => lfo.shape = crate::shared::models::LFOShape::Random,
                                    "Designer" => lfo.shape = crate::shared::models::LFOShape::Designer([0.0; 16].to_vec()),
                                    _ => {}
                                }
                            }
                       }
                    });
                }
            >
                <option value="Sine">Sine</option>
                <option value="Triangle" selected>Triangle</option>
                <option value="Square">Square</option>
                <option value="Random">Random</option>
                <option value="Designer">Designer</option>
            </select>
        </div>

        // Destination
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-500">Destination</label>
            <select
                class="bg-zinc-800 text-zinc-300 text-xs rounded p-1 border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
                on:change=move |ev| {
                    let val = event_target_value(&ev).parse::<u8>().unwrap_or(74);
                    set_pattern_signal.update(|p| {
                       if let Some(track) = p.tracks.get_mut(track_id) {
                           if let Some(lfo) = track.lfos.get_mut(0) {
                               lfo.destination = val;
                           }
                       }
                    });
                }
            >
                <option value="74" selected>Filter Cutoff</option>
                <option value="71">Resonance</option>
                <option value="1">Mod Wheel</option>
                <option value="10">Pan</option>
            </select>
        </div>

        // Amount
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-500">Amount</label>
            <input
                type="number"
                min="-1"
                max="1"
                step="0.01"
                prop:value=move || {
                    pattern_signal.with(|p| {
                        p.tracks.get(track_id)
                            .and_then(|t| t.lfos.get(0))
                            .map(|l| format!("{:.2}", l.amount))
                            .unwrap_or("0.00".to_string())
                    })
                }
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse::<f32>().unwrap_or(0.0).clamp(-1.0, 1.0);
                    set_pattern_signal.update(|p| {
                        if let Some(track) = p.tracks.get_mut(track_id) {
                             if let Some(lfo) = track.lfos.get_mut(0) {
                                 lfo.amount = val;
                             }
                        }
                    });
                }
                class="w-full text-xs text-center bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-zinc-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
            />
        </div>

        // Speed
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-500">Speed</label>
            <input
                type="number"
                min="0.1"
                max="4.0"
                step="0.1"
                prop:value=move || {
                    pattern_signal.with(|p| {
                        p.tracks.get(track_id)
                            .and_then(|t| t.lfos.get(0))
                            .map(|l| format!("{:.1}", l.speed))
                            .unwrap_or("1.0".to_string())
                    })
                }
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse::<f32>().unwrap_or(1.0).clamp(0.1, 4.0);
                    set_pattern_signal.update(|p| {
                        if let Some(track) = p.tracks.get_mut(track_id) {
                             if let Some(lfo) = track.lfos.get_mut(0) {
                                 lfo.speed = val;
                             }
                        }
                    });
                }
                class="w-full text-xs text-center bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-zinc-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
            />
        </div>
    </div>

    // Designer section (unchanged logic, just repositioned)
    <div>
        {move || {
             let is_designer = pattern_signal.with(|p| {
                 p.tracks.get(track_id)
                    .and_then(|t| t.lfos.get(0))
                    .map(|l| matches!(l.shape, crate::shared::models::LFOShape::Designer(_)))
                    .unwrap_or(false)
             });

             if is_designer {
                 view! {
                     <div class="flex flex-col gap-2">
                         <label class="text-xs text-zinc-500">Waveform Designer</label>
                         <crate::ui::components::lfo_designer::LfoDesigner
                            track_id=Signal::derive(move || track_id)
                            lfo_index=Signal::derive(move || 0)
                            value=Signal::derive(move || {
                                pattern_signal.with(|p| {
                                    p.tracks.get(track_id)
                                    .and_then(|t| t.lfos.get(0))
                                    .and_then(|l| {
                                        if let crate::shared::models::LFOShape::Designer(v) = &l.shape {
                                            Some(v.to_vec())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| vec![0.0; 16])
                                })
                            })
                            on_change=Callback::new(move |new_val: Vec<f32>| {
                                if new_val.len() == 16 {
                                    let mut arr = [0.0; 16];
                                    arr.copy_from_slice(&new_val);
                                    set_pattern_signal.update(|p| {
                                        if let Some(track) = p.tracks.get_mut(track_id) {
                                            if let Some(lfo) = track.lfos.get_mut(0) {
                                                lfo.shape = crate::shared::models::LFOShape::Designer(arr.to_vec());
                                            }
                                        }
                                    });
                                }
                            })
                         />
                     </div>
                 }.into_any()
             } else {
                 view! {
                     <div class="w-full h-32 flex items-center justify-center text-zinc-600 text-xs border border-zinc-800 rounded bg-zinc-900/50">
                         "Select 'Designer' shape to draw"
                     </div>
                 }.into_any()
             }
        }}
    </div>
</div>
```

**Step 2: Visual test**

Run: Reload browser
Expected:
- LFO controls in single row (4 columns: Shape, Dest, Amount, Speed)
- Waveform designer below controls
- Total LFO section height ~180px (vs previous ~400px)
- Dropdowns and numeric inputs work

**Step 3: Functional test**

Actions:
1. Open LFO section
2. Change Shape to "Sine"
3. Set Amount to 0.8
4. Set Speed to 2.0
5. Play sequence
6. Verify filter modulation is audible

Expected: LFO works identically despite compact layout

**Step 4: Measure viewport height**

Run: Open browser DevTools, check computed height
Expected: Total page height ≤ 900px with LFO open, ~500px with LFO closed

**Step 5: Commit compact LFO**

```bash
git add flux-app/src/ui/components/inspector.rs
git commit -m "refactor: compact LFO layout to inline controls

- Reorganize LFO from 2-column to 4-column inline layout
- Controls: Shape, Destination, Amount, Speed in one row
- Designer section below controls (unchanged functionality)
- Reduces LFO height from ~400px to ~180px
- Total page fits in ~650px with LFO open

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 5: Final Testing & Documentation

### Task 6: Comprehensive Visual and Functional Testing

**Files:**
- None (testing only)

**Step 1: Viewport height verification**

Actions:
1. Open browser DevTools
2. Set viewport to 1920×1080 (simulating 1080p display)
3. Measure page height with LFO closed
4. Measure page height with LFO open

Expected:
- LFO closed: ~440-500px (well under 900px)
- LFO open: ~620-680px (well under 900px)
- No vertical scrollbar in either state

**Step 2: Parameter interaction testing**

Test all parameters with numeric inputs:
1. Type values: 0.0, 0.5, 1.0
2. Arrow up/down keys
3. Tab navigation between parameters
4. Focus rings visible
5. Right-click grid step to P-lock
6. Verify label turns amber when locked
7. Edit P-locked parameter
8. Play sequence, verify parameter changes audible

Expected: All interactions work smoothly

**Step 3: LFO interaction testing**

Test LFO controls:
1. Toggle LFO open/close (smooth animation)
2. Change Shape dropdown (all options)
3. Change Destination dropdown
4. Edit Amount numeric input (type and arrows)
5. Edit Speed numeric input
6. Select "Designer" shape
7. Draw in waveform designer
8. Play sequence, verify LFO modulation audible

Expected: All LFO functionality preserved

**Step 4: Cross-browser testing**

Test in:
- Chrome/Arc (primary)
- Safari (WebKit differences)
- Firefox (Gecko differences)

Expected: Layout consistent across browsers

**Step 5: Responsive testing**

Test at various viewport heights:
- 900px (target minimum)
- 1080px (common laptop)
- 1440px (larger display)

Expected: No scrolling at any size ≥900px

**Step 6: Document results**

Create testing checklist in commit message for final commit.

---

### Task 7: Update Documentation

**Files:**
- Create: `docs/plans/2026-02-13-no-scroll-layout-completed.md`

**Step 1: Document completed changes**

```markdown
# No-Scroll Layout - Completion Report

**Date:** 2026-02-13
**Status:** Completed

## Changes Implemented

### 1. Padding Reduction
- Section padding: p-6 → p-5 (saves ~16px)
- Files: `flux-app/src/app.rs`

### 2. Numeric Parameter Inputs
- Replaced sliders with number inputs
- Added arrow key support
- Reduced gap-y spacing
- Saves ~100px
- Files: `flux-app/src/ui/components/inspector.rs`

### 3. Collapsible LFO
- Added toggle button in header
- LFO hidden by default
- Smooth collapse/expand animation
- Saves ~400px when closed
- Files: `flux-app/src/app.rs`, `flux-app/src/ui/components/inspector.rs`

### 4. Compact LFO Layout
- 4 controls inline (Shape, Dest, Amount, Speed)
- Designer section below
- Reduces LFO from ~400px to ~180px
- Files: `flux-app/src/ui/components/inspector.rs`

## Results

- **LFO closed:** ~440px total height
- **LFO open:** ~620px total height
- **Target:** ~900px
- **Status:** ✅ Exceeds target, no scrolling required

## Testing Completed

- ✅ Viewport height measurement
- ✅ Parameter numeric input interactions
- ✅ LFO toggle and controls
- ✅ Cross-browser compatibility
- ✅ Responsive behavior

## Known Issues

None identified.

## Future Enhancements

- Persist LFO toggle state to localStorage
- Add scroll wheel support for numeric inputs
- Animate numeric value changes (smooth transitions)
- Modulation depth visualization (border pulse)
```

**Step 2: Commit documentation**

```bash
git add docs/plans/2026-02-13-no-scroll-layout-completed.md
git commit -m "docs: add no-scroll layout completion report

- Document all implemented changes
- Confirm viewport height targets met
- Record testing results

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Summary

**Total commits:** 7
- Reduce padding
- Numeric parameter inputs
- LFO toggle button
- Collapsible LFO section
- Compact LFO layout
- Testing verification
- Documentation

**Files modified:**
- `flux-app/src/app.rs` (padding, toggle button, context)
- `flux-app/src/ui/components/inspector.rs` (numeric inputs, collapsible LFO, compact layout)

**Testing focus:**
- Visual: Viewport height, layout appearance
- Functional: Parameter editing, LFO controls, P-locks
- Cross-browser: Chrome, Safari, Firefox
- Responsive: Various viewport heights

**Success criteria:**
- ✅ No scrolling at ≥900px viewport
- ✅ All functionality preserved
- ✅ Smooth interactions
- ✅ Professional appearance
