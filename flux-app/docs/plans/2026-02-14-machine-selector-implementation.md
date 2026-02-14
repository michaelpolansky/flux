# Machine Selector Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a machine type selector dropdown next to each track label for choosing between 7 machine types with abbreviations.

**Architecture:** Create a dedicated `MachineSelector` component following the same pattern as `RemoveTrackButton`. Uses Leptos signals to read/write machine type from Pattern context. Dropdown with click-outside and ESC-to-close handlers.

**Tech Stack:** Leptos 0.7 (reactive WASM framework), Tailwind CSS, Rust

---

## Task 1: Create Machine Selector Component File with Helper Functions

**Files:**
- Create: `flux-app/src/ui/components/machine_selector.rs`

**Step 1: Create file with helper functions**

Create the new file with abbreviation and full name mapping functions:

```rust
use leptos::prelude::*;
use crate::shared::models::{MachineType, Pattern};

/// Convert MachineType to 2-3 letter abbreviation for compact display
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

/// Convert MachineType to full name for dropdown options
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

/// Get all machine types in order for dropdown
fn all_machine_types() -> [MachineType; 7] {
    [
        MachineType::OneShot,
        MachineType::Werp,
        MachineType::Slice,
        MachineType::FmTone,
        MachineType::Subtractive,
        MachineType::TonverkBus,
        MachineType::MidiCC,
    ]
}
```

**Step 2: Verify file compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles without errors (file isn't imported yet, but syntax should be valid)

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/machine_selector.rs
git commit -m "feat: add machine selector helper functions

Add abbreviation and full name mapping for 7 machine types.
Provides foundation for MachineSelector component.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Implement MachineSelector Component Base Structure

**Files:**
- Modify: `flux-app/src/ui/components/machine_selector.rs`

**Step 1: Add component skeleton**

Add the component structure with context access and signals:

```rust
#[component]
pub fn MachineSelector(
    track_idx: usize,
) -> impl IntoView {
    // Access Pattern context for reading/writing machine type
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Local state for dropdown open/closed
    let (is_open, set_is_open) = signal(false);

    // Read current machine type for this track
    let current_machine = move || {
        pattern_signal.with(|p| {
            p.tracks.get(track_idx)
                .map(|t| t.machine)
                .unwrap_or(MachineType::OneShot)
        })
    };

    // Update machine type and close dropdown
    let set_machine = move |new_machine: MachineType| {
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_idx) {
                track.machine = new_machine;
            }
        });
        set_is_open.set(false);
    };

    // Toggle dropdown open/closed
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    // Placeholder view - we'll build this in next tasks
    view! {
        <div class="relative">
            "MachineSelector placeholder"
        </div>
    }
}
```

**Step 2: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles without errors

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/machine_selector.rs
git commit -m "feat: add MachineSelector component skeleton

Component structure with Pattern context access, local state,
and handlers for reading/updating machine type.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Implement Button (Closed State)

**Files:**
- Modify: `flux-app/src/ui/components/machine_selector.rs`

**Step 1: Replace placeholder with button**

Replace the `view!` block with the button implementation:

```rust
    view! {
        <div class="relative">
            <button
                on:click=toggle_dropdown
                class=move || {
                    let base = "px-1.5 py-0.5 text-[10px] font-mono border rounded \
                                hover:bg-zinc-600 transition-colors cursor-pointer";
                    if is_open.get() {
                        format!("{} bg-zinc-600 border-blue-500 text-zinc-300", base)
                    } else {
                        format!("{} bg-zinc-700 border-zinc-600 text-zinc-300", base)
                    }
                }
            >
                {move || format!("{} ▾", machine_abbreviation(current_machine()))}
            </button>
        </div>
    }
```

**Step 2: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/machine_selector.rs
git commit -m "feat: add machine selector button UI

Compact button showing abbreviation + dropdown arrow.
Reactive styling based on open/closed state.
FLUX zinc theme colors with hover states.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Implement Dropdown Menu (Open State)

**Files:**
- Modify: `flux-app/src/ui/components/machine_selector.rs`

**Step 1: Add dropdown menu below button**

Update the `view!` block to include the dropdown:

```rust
    view! {
        <div class="relative">
            <button
                on:click=toggle_dropdown
                class=move || {
                    let base = "px-1.5 py-0.5 text-[10px] font-mono border rounded \
                                hover:bg-zinc-600 transition-colors cursor-pointer";
                    if is_open.get() {
                        format!("{} bg-zinc-600 border-blue-500 text-zinc-300", base)
                    } else {
                        format!("{} bg-zinc-700 border-zinc-600 text-zinc-300", base)
                    }
                }
            >
                {move || format!("{} ▾", machine_abbreviation(current_machine()))}
            </button>

            // Dropdown menu - only render when open
            <Show when=move || is_open.get()>
                <div class="absolute top-full left-0 mt-1 bg-zinc-800 border border-zinc-600 rounded shadow-lg z-50 min-w-[120px]">
                    {move || {
                        all_machine_types().iter().map(|&machine_type| {
                            let is_current = current_machine() == machine_type;
                            view! {
                                <div
                                    on:click=move |_| set_machine(machine_type)
                                    class=move || {
                                        let base = "px-3 py-1.5 text-sm text-zinc-300 cursor-pointer transition-colors";
                                        if is_current {
                                            format!("{} bg-blue-900/30 hover:bg-zinc-700", base)
                                        } else {
                                            format!("{} hover:bg-zinc-700", base)
                                        }
                                    }
                                >
                                    {machine_full_name(machine_type)}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </Show>
        </div>
    }
```

**Step 2: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/machine_selector.rs
git commit -m "feat: add dropdown menu with all 7 machine types

Dropdown shows full names for all machine types.
Current selection highlighted with blue background.
Hover states and smooth transitions.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Add Click-Outside-to-Close Handler

**Files:**
- Modify: `flux-app/src/ui/components/machine_selector.rs`

**Step 1: Add window click listener**

Add the click-outside handler after the signal definitions and before the `view!` block:

```rust
    // Close dropdown when clicking outside
    let dropdown_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if is_open.get() {
            let close_on_click = move |event: web_sys::MouseEvent| {
                if let Some(dropdown_el) = dropdown_ref.get() {
                    if let Some(target) = event.target() {
                        let target_el = target.dyn_into::<web_sys::Element>().ok();
                        if let Some(target_el) = target_el {
                            // Close if click is outside the dropdown
                            if !dropdown_el.contains(Some(&target_el)) {
                                set_is_open.set(false);
                            }
                        }
                    }
                }
            };

            let listener = leptos::ev::click;
            let window = web_sys::window().expect("window not found");
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(close_on_click) as Box<dyn FnMut(_)>);
            window.add_event_listener_with_callback(listener.name(), closure.as_ref().unchecked_ref())
                .expect("failed to add event listener");
            closure.forget(); // Keep listener alive
        }
    });
```

Then update the outer `<div>` to use the ref:

```rust
    view! {
        <div class="relative" node_ref=dropdown_ref>
```

**Step 2: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/machine_selector.rs
git commit -m "feat: add click-outside-to-close handler

Window-level click listener closes dropdown when clicking outside.
Uses NodeRef to detect click target and close appropriately.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Add ESC Key Handler

**Files:**
- Modify: `flux-app/src/ui/components/machine_selector.rs`

**Step 1: Add keydown listener for ESC**

Add ESC key handler after the click-outside handler:

```rust
    // Close dropdown on ESC key
    Effect::new(move |_| {
        if is_open.get() {
            let close_on_esc = move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape" {
                    set_is_open.set(false);
                }
            };

            let listener = leptos::ev::keydown;
            let window = web_sys::window().expect("window not found");
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(close_on_esc) as Box<dyn FnMut(_)>);
            window.add_event_listener_with_callback(listener.name(), closure.as_ref().unchecked_ref())
                .expect("failed to add event listener");
            closure.forget();
        }
    });
```

**Step 2: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/machine_selector.rs
git commit -m "feat: add ESC key to close dropdown

Keydown listener closes dropdown when ESC is pressed.
Improves keyboard accessibility.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Export MachineSelector in Module

**Files:**
- Modify: `flux-app/src/ui/components/mod.rs`

**Step 1: Add module declaration and export**

Find the existing module declarations (likely near `remove_track_button`, `track_controls`) and add:

```rust
pub mod machine_selector;
pub use machine_selector::MachineSelector;
```

**Step 2: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add flux-app/src/ui/components/mod.rs
git commit -m "feat: export MachineSelector component

Add module declaration and public export for MachineSelector.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Integrate MachineSelector into Grid

**Files:**
- Modify: `flux-app/src/ui/components/grid.rs`

**Step 1: Add import**

Add to imports at top of file:

```rust
use super::machine_selector::MachineSelector;
```

**Step 2: Update track label structure**

Find the track label rendering (around line 126-134) and update:

```rust
<div class="w-8 h-10 flex items-center justify-center gap-1">
    <div class="flex items-center gap-1">
        <div class="text-xs text-zinc-400">
            {format!("T{}", track_idx + 1)}
        </div>
        <MachineSelector track_idx=track_idx />
        <RemoveTrackButton
            track_idx=track_idx
            show_confirm=set_show_confirm_dialog
        />
    </div>
</div>
```

**Step 3: Verify compiles**

Run: `cd flux-app && npm run build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add flux-app/src/ui/components/grid.rs
git commit -m "feat: integrate MachineSelector into grid track labels

Add MachineSelector between track label and RemoveTrackButton.
Compact design fits in existing track label area.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Manual Testing and Verification

**Files:**
- Create: `flux-app/docs/MACHINE_SELECTOR_TESTING.md`

**Step 1: Create testing checklist document**

```markdown
# Machine Selector Manual Testing Checklist

**Date:** 2026-02-14
**Tester:** [Your Name]
**Build:** [Commit SHA]

---

## Basic Functionality

- [ ] Click machine selector button → dropdown opens
- [ ] Click "OneShot" option → track.machine updates, dropdown closes
- [ ] Button abbreviation updates to "OS"
- [ ] Click selector again → dropdown opens with "OneShot" highlighted
- [ ] Test all 7 machine types: OS, WP, SL, FM, SUB, TNV, CC
- [ ] Each selection updates correctly

## Data Persistence

- [ ] Add 5 steps to a track (OneShot)
- [ ] Change machine to Subtractive
- [ ] Verify all 5 steps still present
- [ ] Change back to OneShot
- [ ] Verify steps still intact

## Dropdown Behavior

- [ ] Click outside dropdown → closes without selection
- [ ] Press ESC → closes without selection
- [ ] Current machine highlighted in dropdown with blue background
- [ ] Hover over option → background changes to zinc-700

## Integration with Track Management

- [ ] Add new track → machine selector shows "OS" (default)
- [ ] Click selector on new track → all 7 options available
- [ ] Remove a track with dropdown open → dropdown closes gracefully
- [ ] Re-index tracks (remove T2) → machine selectors update correctly

## Visual & Styling

- [ ] Button size doesn't expand track label area
- [ ] Abbreviations legible at 10px font size
- [ ] Dropdown appears above grid rows (z-50 works)
- [ ] Hover states smooth and visible
- [ ] Colors match FLUX zinc theme

## Edge Cases

- [ ] Rapid clicking button → no visual glitches
- [ ] Click button 10x fast → state stable
- [ ] Open multiple dropdowns → all work independently
- [ ] Change machine during playback → playback continues

---

## Notes

[Add any issues, bugs, or observations here]
```

**Step 2: Run the app and test**

Run: `cd flux-app && npm run dev`

Work through the checklist systematically.

**Step 3: Commit testing document**

```bash
git add flux-app/docs/MACHINE_SELECTOR_TESTING.md
git commit -m "docs: add machine selector testing checklist

Comprehensive manual testing checklist covering functionality,
data persistence, dropdown behavior, integration, styling, and edge cases.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Implementation Complete

All tasks complete! The machine selector feature is now fully implemented with:

✅ MachineSelector component with abbreviation display
✅ Dropdown menu with all 7 machine types
✅ Click-outside and ESC-to-close handlers
✅ Integration into grid track labels
✅ Manual testing checklist

**Next Steps:**
- Run manual tests to verify functionality
- Report any issues discovered
- Consider future enhancements (keyboard navigation, global dropdown coordinator)
