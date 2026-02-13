# Flux UI Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign Flux sequencer UI with Ableton-inspired professional layout, improved visual polish, and optimized information density.

**Architecture:** Update existing Tailwind components following systematic design principles. Three-zone layout (header, grid, parameters) with consistent spacing scale and color system. No new dependencies.

**Tech Stack:** Leptos 0.7, Tailwind CSS 4.x, existing component structure

---

## Phase 1: Core Layout

### Task 1: Update App.rs Layout Structure

**Files:**
- Modify: `flux-app/src/app.rs:48-99`

**Step 1: Update main container width and padding**

```rust
view! {
    <main class="min-h-screen bg-zinc-950 text-zinc-50 p-6 font-sans selection:bg-red-900 selection:text-white">
        <div class="max-w-7xl mx-auto space-y-6">
```

**Step 2: Update header background and spacing**

```rust
            <header class="flex items-center justify-between bg-zinc-900 border-b border-zinc-800 px-6 py-4 rounded-t-lg h-16">
```

**Step 3: Wrap grid section in card**

```rust
            <section class="bg-zinc-900/50 rounded-lg p-6">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Sequencer Grid"</h2>
                    <div class="text-xs font-mono text-zinc-600">"TRACK 1 - LEAD SYNTH"</div>
                </div>
                <Grid />
            </section>
```

**Step 4: Wrap parameters section in card**

```rust
            <section class="bg-zinc-900/50 rounded-lg p-6">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">"Parameters"</h2>
                    <div class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full"
                            class:bg-blue-500=move || selected_step.get().is_some()
                            class:bg-zinc-800=move || selected_step.get().is_none()
                        ></span>
                        <span class="text-xs font-mono text-zinc-500">
                            {move || if let Some(step) = selected_step.get() {
                                format!("STEP {} LOCKED", step + 1)
                            } else {
                                "TRACK DEFAULT".to_string()
                            }}
                        </span>
                    </div>
                </div>
                <Inspector />
                <StepInspector />
            </section>
        </div>
    </main>
}
```

**Step 5: Test visual changes in browser**

Run dev server if not running: `source ~/.cargo/env && npm run dev`
Open: http://localhost:1420
Expected: Wider layout, darker header, card backgrounds on sections

**Step 6: Commit layout changes**

```bash
git add flux-app/src/app.rs
git commit -m "refactor: update main layout structure

- Widen container to max-w-7xl
- Reduce padding to p-6 for tighter spacing
- Add card backgrounds to grid and parameters sections
- Update spacing scale to gap-6 between sections

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 2: Header Redesign

### Task 2: Consolidate Header Component

**Files:**
- Modify: `flux-app/src/app.rs:51-66`
- Modify: `flux-app/src/ui/components/toolbar.rs`

**Step 1: Remove duplicate PLAY/STOP buttons from app.rs**

Find and remove these lines (around line 59-63):

```rust
                        <div class="w-px h-8 bg-zinc-800 mx-2"></div>
                        <button class="px-4 py-2 bg-zinc-900 border border-zinc-800 rounded hover:border-zinc-700 text-xs font-bold transition-colors">
                            "PLAY"
                        </button>
                        <button class="px-4 py-2 bg-zinc-900 border border-zinc-800 rounded hover:border-zinc-700 text-xs font-bold transition-colors">
                            "STOP"
                        </button>
```

**Step 2: Update header layout**

Replace header section with:

```rust
            <header class="flex items-center justify-between bg-zinc-900 border-b border-zinc-800 px-6 h-16">
                <div class="flex flex-col">
                    <h1 class="text-xl font-bold tracking-tight text-zinc-50">FLUX</h1>
                    <p class="text-xs text-zinc-500 font-mono">"Audio Engine"</p>
                </div>
                <div class="flex items-center gap-4">
                    <Toolbar />
                </div>
            </header>
```

**Step 3: Update Toolbar component styling**

```rust
// In flux-app/src/ui/components/toolbar.rs line 130
view! {
    <div class="flex items-center gap-2">
        <button
            on:click=save_project
            class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95"
        >
            SAVE
        </button>
        <button
            on:click=load_project
            class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95"
        >
            LOAD
        </button>

        <div class="w-px h-6 bg-zinc-700 mx-2"></div>

        <div class="text-sm font-mono text-zinc-400 px-3">
            "120 BPM"
        </div>

        <div class="w-px h-6 bg-zinc-700"></div>

        <button
            on:click=move |_| {
                leptos::task::spawn_local(async {
                    crate::services::audio::set_playback_state(true).await;
                });
            }
            class="h-10 px-4 bg-green-600 hover:bg-green-500 rounded-md text-sm font-medium text-white transition-colors active:scale-95"
        >
            ▶
        </button>
        <button
            on:click=move |_| {
                leptos::task::spawn_local(async {
                    crate::services::audio::set_playback_state(false).await;
                });
            }
            class="h-10 px-4 bg-zinc-800 hover:bg-zinc-700 rounded-md text-sm font-medium text-zinc-300 transition-colors active:scale-95"
        >
            ■
        </button>
    </div>
}
```

**Step 4: Test header in browser**

Expected: Clean header with left-aligned title, right-aligned controls, no duplicates

**Step 5: Commit header changes**

```bash
git add flux-app/src/app.rs flux-app/src/ui/components/toolbar.rs
git commit -m "refactor: redesign header component

- Consolidate duplicate transport controls
- Add BPM display
- Update button styling with new color system
- Add tactile press feedback (active:scale-95)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 3: Grid Redesign

### Task 3: Update Grid Layout

**Files:**
- Modify: `flux-app/src/ui/components/grid.rs:51-112`

**Step 1: Update grid container to 2 rows × 8 columns**

```rust
view! {
    <div class="grid grid-cols-8 gap-3">
        <For
            each=move || {
                (0..16).into_iter()
            }
            key=|idx| *idx
            children=move |idx| {
```

**Step 2: Update step button sizing and styling**

```rust
                view! {
                    <button
                        class=move || {
                            let base_classes = "w-16 h-16 rounded-lg transition-all duration-100 flex items-center justify-center text-xs font-mono select-none";

                            let is_current_step = sequencer_state.current_step.get() == idx;
                            let is_active_note = is_active();
                            let is_selected = sequencer_state.selected_step.get() == Some(idx);

                            let state_classes = if is_current_step {
                                "bg-amber-300 text-black shadow-lg scale-110 transition-transform duration-75"
                            } else if is_active_note {
                                "bg-amber-500 text-black shadow-md"
                            } else {
                                "bg-zinc-800 text-zinc-600 hover:bg-zinc-700"
                            };

                            let selection_classes = if is_selected {
                                "ring-2 ring-blue-500 ring-offset-2 ring-offset-zinc-900"
                            } else {
                                ""
                            };

                            format!("{} {} {}", base_classes, state_classes, selection_classes)
                        }
                        on:mousedown=move |_| handle_mouse_down(idx)
                        on:mouseup=move |e| handle_mouse_up(e)
                        on:mouseleave=move |e| handle_mouse_up(e)
                        on:click=move |_| toggle_step(idx)
                        on:contextmenu=move |e| {
                            e.prevent_default();
                            sequencer_state.selected_step.set(Some(idx));
                        }
                    >
                        {idx + 1}
                    </button>
                }
```

**Step 3: Test grid in browser**

Expected:
- 2 rows of 8 buttons
- Larger 64px square buttons
- Amber color for active notes
- Brighter amber + scale for playing step
- Blue ring for selected step

**Step 4: Commit grid changes**

```bash
git add flux-app/src/ui/components/grid.rs
git commit -m "refactor: update grid to 2x8 layout with new styling

- Change to 2 rows × 8 columns layout
- Increase button size to 64px (w-16 h-16)
- Update colors: amber for active, blue ring for selected
- Add scale effect for currently playing step
- Improve transition smoothness

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 4: Parameters Redesign

### Task 4: Update Inspector Layout

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:95-130`

**Step 1: Update container to 2×4 grid**

```rust
view! {
    <div class="grid grid-cols-4 gap-x-6 gap-y-4">
        {params.into_iter().enumerate().map(|(idx, name)| {
```

**Step 2: Update parameter control styling**

```rust
            view! {
                <div class="flex flex-col gap-2">
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
                        type="range"
                        min="0"
                        max="1"
                        step="0.01"
                        prop:value=move || get_value(idx)
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            handle_input(idx, val, name_str.clone());
                        }
                        class=move || {
                            let base = "w-full h-2 bg-zinc-800 rounded-full appearance-none cursor-pointer transition-all";
                            let track_color = if sequencer_state.selected_step.get().is_some() {
                                "accent-amber-500"
                            } else {
                                "accent-amber-500"
                            };
                            format!("{} {}", base, track_color)
                        }
                    />
                </div>
            }
```

**Step 3: Test parameters in browser**

Expected:
- 2 rows of 4 parameters
- Amber labels when step locked
- Clean slider styling
- Consistent spacing

**Step 4: Commit parameter changes**

```bash
git add flux-app/src/ui/components/inspector.rs
git commit -m "refactor: update parameters to 2x4 grid layout

- Reorganize to 2 rows × 4 columns
- Update label styling with amber for locked state
- Improve slider appearance
- Add consistent gap spacing

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 5: Polish & Interactive States

### Task 5: Add Transition Effects

**Files:**
- Modify: `flux-app/src/ui/components/toolbar.rs:130-165`
- Modify: `flux-app/src/ui/components/grid.rs:72-110`

**Step 1: Verify all buttons have active:scale-95**

Check that all button classes include:
```rust
"active:scale-95"
```

**Step 2: Verify grid has smooth transitions**

Check grid button classes include:
```rust
"transition-all duration-100"
```

And playing step has:
```rust
"transition-transform duration-75"
```

**Step 3: Test all interactive states**

Manual testing checklist:
- [ ] Buttons scale down on click
- [ ] Grid steps transition smoothly
- [ ] Playing step scales up smoothly
- [ ] Hover states work on all buttons
- [ ] Selected step ring appears instantly

**Step 4: Add focus rings for accessibility**

Update any button missing focus styles:
```rust
"focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950"
```

**Step 5: Test keyboard navigation**

- [ ] Tab through all controls
- [ ] Focus rings visible
- [ ] Enter/Space activates buttons

**Step 6: Commit polish changes**

```bash
git add flux-app/src/ui/components/
git commit -m "polish: add interactive states and transitions

- Add tactile feedback (scale-95 on press) to all buttons
- Ensure smooth transitions on grid and controls
- Add focus rings for keyboard accessibility
- Verify all hover states

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Phase 6: Final Verification

### Task 6: Visual QA and Testing

**Step 1: Full visual inspection**

Open http://localhost:1420 and verify:
- [ ] Layout hierarchy clear (grid dominates view)
- [ ] Spacing consistent (gap-2, gap-4, gap-6)
- [ ] Colors match design (zinc-950, amber-500, blue-500)
- [ ] Header height correct (~64px)
- [ ] Grid buttons large (64px)
- [ ] Parameters organized in 2×4 grid

**Step 2: Interaction testing**

- [ ] Click grid steps - toggles work
- [ ] Right-click step - selection works
- [ ] Press PLAY - playback indicator works
- [ ] Adjust sliders - smooth movement
- [ ] All buttons respond to clicks

**Step 3: Responsive behavior check**

Resize browser window:
- [ ] Layout stays centered (max-w-7xl)
- [ ] No horizontal scroll
- [ ] Components remain readable

**Step 4: Browser compatibility**

Test in:
- [ ] Safari (primary browser)
- [ ] Chrome (if available)
- [ ] Firefox (if available)

**Step 5: Screenshot for documentation**

Take screenshot of final UI for future reference

**Step 6: Final commit**

```bash
git add .
git commit -m "docs: update UI redesign completion

All phases complete:
- ✅ Core layout restructured
- ✅ Header redesigned
- ✅ Grid updated to 2x8 layout
- ✅ Parameters reorganized to 2x4
- ✅ Interactive polish applied
- ✅ Accessibility verified

Matches approved design document.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Success Criteria Checklist

After completing all tasks, verify:

- [ ] Visual hierarchy is clear (grid dominates)
- [ ] Information density feels balanced
- [ ] Professional appearance (Ableton-level polish)
- [ ] All interactive states provide clear feedback
- [ ] No new dependencies added
- [ ] Responsive behavior maintained
- [ ] Accessibility preserved (keyboard nav, focus rings)

## Notes for Implementation

- **No tests required:** This is purely visual/UI work. Manual testing in browser is sufficient.
- **Iteration expected:** Visual polish often requires tweaking. Don't hesitate to adjust spacing/colors if something looks off.
- **Reference design doc:** Keep `/docs/plans/2026-02-13-ui-redesign-design.md` open for color values and measurements.
- **Hot reload:** Trunk watch will auto-reload - refresh browser to see changes.
- **Commits:** Small, frequent commits help track what worked visually.

## Troubleshooting

**If layout breaks:**
1. Check Tailwind classes are spelled correctly
2. Verify no typos in grid-cols or gap values
3. Check browser console for Leptos errors

**If colors look wrong:**
1. Verify zinc shades: 950 (darkest), 900, 800, 700, etc.
2. Check amber-500 vs amber-300 (playing step is lighter)
3. Confirm blue-500 for selection rings

**If buttons don't respond:**
1. Check event handlers still attached
2. Verify async spawns for audio commands
3. Check browser console for JS errors
