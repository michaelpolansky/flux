# Step Editing UX Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign step selection to use left-click with clear visual feedback and dedicated toggle button.

**Architecture:** Modify Grid component to select on left-click (not toggle), add Inspector header showing editing context, add Active toggle button for step activation, add ESC and click-away deselection handlers.

**Tech Stack:** Leptos 0.7 (Rust → WASM), Tailwind CSS v4

---

## Task 1: Grid - Change Left-Click to Selection

**Files:**
- Modify: `flux-app/src/ui/components/grid.rs:43-98`

**Step 1: Remove toggle from left-click handler**

In `grid.rs`, change the `on:click` handler from calling `toggle_step(idx)` to just selecting the step.

**Current code (lines 95-98):**
```rust
on:mousedown=move |_| handle_mouse_down(idx)
on:mouseup=move |e| handle_mouse_up(e)
on:mouseleave=move |e| handle_mouse_up(e)
on:click=move |_| toggle_step(idx)
```

**New code:**
```rust
on:mousedown=move |_| handle_mouse_down(idx)
on:mouseup=move |e| handle_mouse_up(e)
on:mouseleave=move |e| handle_mouse_up(e)
on:click=move |_| {
    sequencer_state.selected_step.set(Some(idx));
}
```

**Step 2: Remove right-click handler**

Remove the `on:contextmenu` handler (lines 99-102) as it's no longer needed.

**Delete:**
```rust
on:contextmenu=move |e| {
    e.prevent_default();
    sequencer_state.selected_step.set(Some(idx));
}
```

**Step 3: Remove unused toggle_step function**

Remove the entire `toggle_step` closure (lines 17-40) since we'll move step activation to the inspector toggle button.

**Step 4: Test manually**

Run: `cd flux-app && trunk serve`
Expected:
- Left-click a step → blue ring appears (step selected)
- Left-click another step → selection moves to new step
- Right-click → no special behavior (browser context menu appears)
- Steps no longer toggle on/off via click

**Step 5: Commit**

```bash
git add flux-app/src/ui/components/grid.rs
git commit -m "refactor: change grid left-click to selection only

Remove step toggle from click handler. Toggle will move to
inspector Active button in next task.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Inspector - Add Header Section

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:96-151`

**Step 1: Add header view above parameter grid**

Insert header section before the parameter grid in the inspector view.

**Location:** After line 96 (`view! {`), before line 97 (`<div class="bg-zinc-900...">`)

**Add:**
```rust
view! {
    <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800 shadow-xl mt-4">
        // Header section
        <div class="flex items-center justify-between mb-4 pb-3 border-b border-zinc-800 bg-zinc-800/50 -mx-4 -mt-4 px-4 pt-4 rounded-t-xl">
            <div class="text-sm text-zinc-300">
                {move || {
                    if let Some(step_idx) = sequencer_state.selected_step.get() {
                        format!("Editing: Step {}", step_idx + 1)
                    } else {
                        "Editing: Track Defaults".to_string()
                    }
                }}
            </div>
        </div>

        // Parameter grid (existing code continues here)
        <div class="grid grid-cols-4 gap-x-6 gap-y-2">
```

**Step 2: Update closing div structure**

Ensure the closing `</div>` tags are properly nested. The structure should be:
```rust
view! {
    <div class="bg-zinc-900...">  // Main container
        <div class="flex...">      // Header
            ...
        </div>
        <div class="grid...">      // Parameters
            ...
        </div>
        // LFO section...
    </div>
}
```

**Step 3: Test manually**

Run: `trunk serve` (should still be running, auto-reloads)
Expected:
- Inspector shows "Editing: Track Defaults" when no step selected
- Click a step → header updates to "Editing: Step 5" (or whichever step)
- Click another step → header updates reactively
- Header has subtle background (zinc-800/50) and border below it

**Step 4: Commit**

```bash
git add flux-app/src/ui/components/inspector.rs
git commit -m "feat: add inspector header showing editing context

Display 'Editing: Step N' when step selected, 'Editing: Track
Defaults' otherwise. Provides clear feedback about what's being
edited.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Inspector - Add Active Toggle Button

**Files:**
- Modify: `flux-app/src/ui/components/inspector.rs:22-54` (add toggle handler)
- Modify: `flux-app/src/ui/components/inspector.rs:96-110` (add toggle to header)

**Step 1: Create toggle_step handler function**

Add this after the `handle_input` function (around line 54).

**Add:**
```rust
let toggle_step = move |step_idx: usize| {
    set_pattern_signal.update(|p| {
        if let Some(track) = p.tracks.get_mut(track_id) {
            if let Some(subtrack) = track.subtracks.get_mut(subtrack_id) {
                if let Some(step) = subtrack.steps.get_mut(step_idx) {
                    use crate::shared::models::TrigType;
                    if step.trig_type == TrigType::None {
                        step.trig_type = TrigType::Note;
                    } else {
                        step.trig_type = TrigType::None;
                    }

                    spawn_local(async move {
                        use crate::ui::tauri::toggle_step;
                        toggle_step(track_id, step_idx).await;
                    });
                }
            }
        }
    });
};

let is_step_active = move |step_idx: usize| {
    pattern_signal.with(|p| {
        p.tracks.get(track_id)
            .and_then(|t| t.subtracks.get(subtrack_id))
            .and_then(|st| st.steps.get(step_idx))
            .map(|s| s.trig_type != crate::shared::models::TrigType::None)
            .unwrap_or(false)
    })
};
```

**Step 2: Add Active toggle button to header**

Modify the header section to include the toggle button when a step is selected.

**Replace header div (from Task 2) with:**
```rust
<div class="flex items-center justify-between mb-4 pb-3 border-b border-zinc-800 bg-zinc-800/50 -mx-4 -mt-4 px-4 pt-4 rounded-t-xl">
    <div class="text-sm text-zinc-300">
        {move || {
            if let Some(step_idx) = sequencer_state.selected_step.get() {
                format!("Editing: Step {}", step_idx + 1)
            } else {
                "Editing: Track Defaults".to_string()
            }
        }}
    </div>

    // Active toggle button (only when step selected)
    {move || {
        if let Some(step_idx) = sequencer_state.selected_step.get() {
            let is_active = is_step_active(step_idx);
            view! {
                <button
                    class=move || {
                        let base = "px-3 py-1 rounded-lg text-xs font-medium transition-all duration-150 flex items-center gap-2";
                        let state = if is_step_active(step_idx) {
                            "bg-amber-500 text-black hover:bg-amber-400"
                        } else {
                            "bg-zinc-700 text-zinc-400 hover:bg-zinc-600"
                        };
                        format!("{} {}", base, state)
                    }
                    on:click=move |_| toggle_step(step_idx)
                >
                    <span class="text-base">{move || if is_step_active(step_idx) { "●" } else { "○" }}</span>
                    "Active"
                </button>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    }}
</div>
```

**Step 3: Test manually**

Expected:
- With no step selected: No toggle button visible
- Select a step: "Active" toggle appears in header
- Inactive step: Button shows "○ Active" with gray background
- Click toggle: Step activates, button shows "● Active" with amber background
- Grid step changes from zinc-800 to amber-500 when activated
- Click toggle again: Step deactivates, colors revert

**Step 4: Commit**

```bash
git add flux-app/src/ui/components/inspector.rs
git commit -m "feat: add Active toggle button to inspector header

When step selected, show toggle to activate/deactivate it.
Replaces the old left-click toggle behavior with explicit
control.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: App - Add ESC Key Handler

**Files:**
- Modify: `flux-app/src/app.rs:15-35` (add ESC listener)

**Step 1: Import keyboard event types**

Add at the top of `app.rs` after existing imports:

```rust
use leptos::ev::KeyboardEvent;
use wasm_bindgen::JsCast;
```

**Step 2: Add ESC key listener**

Find where `SequencerState` is created and provided as context (around line 20-35). After providing the context signals, add a keyboard event listener.

**Add after context provisions:**
```rust
// ESC key handler to deselect step
let handle_escape = move |ev: KeyboardEvent| {
    if ev.key() == "Escape" {
        set_selected_step.set(None);
    }
};

// Attach to window
use leptos::ev::EventListenerOptions;
window_event_listener(ev::keydown, handle_escape);
```

**Step 3: Test manually**

Expected:
- Select a step (blue ring appears)
- Press ESC key → selection clears, header shows "Editing: Track Defaults"
- Works from any part of the app (global listener)

**Step 4: Commit**

```bash
git add flux-app/src/app.rs
git commit -m "feat: add ESC key handler to deselect step

Global keyboard listener allows deselecting current step by
pressing Escape from anywhere in the app.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: App - Add Click-Away Handler

**Files:**
- Modify: `flux-app/src/app.rs:60-120` (wrap content in click-away container)

**Step 1: Add click-away container wrapper**

Find the main app layout (around line 60-120 where the Grid and Inspector are rendered). Wrap the entire content in a click-away container.

**Before:**
```rust
view! {
    <div class="min-h-screen bg-zinc-950 text-zinc-50 p-6">
        <div class="max-w-7xl mx-auto space-y-5">
            <Toolbar/>
            <Grid/>
            <Inspector/>
        </div>
    </div>
}
```

**After:**
```rust
view! {
    <div
        class="min-h-screen bg-zinc-950 text-zinc-50 p-6"
        on:click=move |ev| {
            // Get the clicked element
            let target = ev.target();
            if let Some(element) = target.and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                // Check if click is outside the grid
                // If the clicked element or its ancestors don't have class "grid"
                let mut current = Some(element);
                let mut found_grid = false;

                while let Some(el) = current {
                    let class_list = el.class_list();
                    if class_list.contains("grid") && class_list.contains("grid-cols-8") {
                        found_grid = true;
                        break;
                    }
                    current = el.parent_element();
                }

                if !found_grid {
                    set_selected_step.set(None);
                }
            }
        }
    >
        <div class="max-w-7xl mx-auto space-y-5">
            <Toolbar/>
            <Grid/>
            <Inspector/>
        </div>
    </div>
}
```

**Step 2: Import web_sys types**

Add to imports at top of `app.rs`:

```rust
use wasm_bindgen::JsCast;
```

**Step 3: Test manually**

Expected:
- Select a step (blue ring, header shows "Editing: Step 5")
- Click on parameter controls → step stays selected
- Click in empty space (between components, in margins) → step deselects
- Click on toolbar → step deselects
- Click on LFO section → step deselects
- Clicking directly on a grid step still selects it (doesn't deselect)

**Step 4: Commit**

```bash
git add flux-app/src/app.rs
git commit -m "feat: add click-away handler to deselect step

Clicking anywhere outside the grid deselects the current step.
Completes the selection workflow: click to select, ESC or
click-away to deselect.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Integration Testing & Verification

**No file changes - manual testing only**

**Step 1: Test complete workflow**

Run full workflow test:

1. **Basic Selection:**
   - [ ] Left-click Step 3 → selected (blue ring)
   - [ ] Header shows "Editing: Step 3"
   - [ ] Left-click Step 7 → Step 3 deselects, Step 7 selected
   - [ ] Header updates to "Editing: Step 7"

2. **Deselection:**
   - [ ] Click empty space → step deselects
   - [ ] Header shows "Editing: Track Defaults"
   - [ ] Select Step 5, press ESC → deselects
   - [ ] Header shows "Editing: Track Defaults"

3. **Step Activation:**
   - [ ] Select inactive step → toggle shows "○ Active" (gray)
   - [ ] Click toggle → step activates, turns amber, toggle shows "● Active"
   - [ ] Click toggle again → step deactivates, turns zinc-800, toggle shows "○ Active"
   - [ ] Step stays selected during activation changes

4. **Parameter Editing:**
   - [ ] No step selected → edit Tuning → track default changes
   - [ ] Select Step 2 → edit Tuning → P-lock created (label turns amber)
   - [ ] Deselect → label returns to zinc (track default shown)
   - [ ] Reselect Step 2 → label amber again, P-locked value shown

5. **Edge Cases:**
   - [ ] Click selected step again → stays selected (no deselect)
   - [ ] Click between grid buttons (gaps) → no deselection
   - [ ] Edit parameters while playback running → works normally
   - [ ] Playhead highlighting (amber-300) + selection ring (blue) both visible
   - [ ] Deactivate selected step → step stays selected, can edit P-locks on inactive step

6. **Layout:**
   - [ ] Inspector header doesn't break compact layout
   - [ ] No vertical scroll added (still fits in ~900px viewport)
   - [ ] LFO section collapse/expand still works
   - [ ] All spacing preserved from previous work

**Step 2: Record any issues**

If any tests fail, document in issue tracker or fix immediately.

**Step 3: Final commit (if needed)**

If any bugs were fixed during testing:

```bash
git add <affected-files>
git commit -m "fix: resolve [specific issue] in step editing

Description of what was wrong and how it was fixed.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Success Criteria Checklist

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

---

## Rollback Plan

If critical issues arise:

```bash
# Revert all commits from this feature
git log --oneline  # Find commit hash before Task 1
git reset --hard <commit-hash>

# Or revert specific commits
git revert <commit-hash-task-5>
git revert <commit-hash-task-4>
# ... etc
```

---

## Notes for Implementation

- **No automated tests:** This project doesn't have a test suite configured. All verification is manual via `trunk serve` and browser testing.
- **Leptos reactivity:** All signals update automatically. No manual re-rendering needed.
- **Tailwind classes:** Use exact classes from design spec. Tailwind v4 syntax (`@import "tailwindcss"`).
- **Frequent commits:** One commit per task minimum. Atomic, revertable changes.
- **YAGNI:** Don't add features beyond the spec (no multi-select, no keyboard navigation, no P-lock indicators on grid).
