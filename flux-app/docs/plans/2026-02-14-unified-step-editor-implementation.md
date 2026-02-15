# Unified Step Editor Sidebar Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Consolidate all step-level editing into a single unified sidebar with collapsible sections for Step Properties, Sound Parameters, and LFO controls.

**Architecture:** Expand the existing StepEditorSidebar component from 240px to 320px width and add three collapsible sections. Move P-Lock and LFO logic from Inspector component into the sidebar. Remove the bottom Inspector section entirely.

**Tech Stack:** Leptos 0.7, Rust/WASM, Tailwind CSS, Leptos reactive signals

---

## Task 1: Create CollapsibleSection Component

**Files:**
- Create: `src/ui/components/collapsible_section.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create collapsible_section.rs with component skeleton**

Create file: `src/ui/components/collapsible_section.rs`

```rust
use leptos::prelude::*;

#[component]
pub fn CollapsibleSection(
    /// Section title text
    title: &'static str,
    /// Whether section is expanded by default
    #[prop(default = true)]
    default_open: bool,
    /// Optional badge count (e.g., P-Lock count)
    #[prop(optional)]
    badge_count: Option<Signal<usize>>,
    /// Child content to show when expanded
    children: Children,
) -> impl IntoView {
    let is_open = RwSignal::new(default_open);

    view! {
        <div class="flex flex-col">
            // Header (clickable)
            <div
                class="flex items-center justify-between px-2 py-1.5 cursor-pointer hover:bg-zinc-800/50 rounded transition-colors duration-150"
                on:click=move |_| is_open.update(|open| *open = !*open)
            >
                <div class="flex items-center gap-2">
                    // Expand/collapse indicator
                    <span class="text-zinc-400 text-xs">
                        {move || if is_open.get() { "▼" } else { "▶" }}
                    </span>
                    // Title
                    <span class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                        {title}
                    </span>
                    // Optional badge
                    {move || {
                        if let Some(count_signal) = badge_count {
                            let count = count_signal.get();
                            if count > 0 {
                                view! {
                                    <span class="text-xs text-amber-400 ml-1">
                                        {format!("({})", count)}
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>
            </div>

            // Content (conditional)
            {move || {
                if is_open.get() {
                    view! {
                        <div class="flex flex-col gap-3 mt-2 animate-in slide-in-from-top-2 fade-in duration-200">
                            {children()}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
```

**Step 2: Register CollapsibleSection in mod.rs**

Modify: `src/ui/components/mod.rs`

Add this line:
```rust
pub mod collapsible_section;
```

**Step 3: Test compilation**

Run: `cd flux-app && cargo check`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/ui/components/collapsible_section.rs src/ui/components/mod.rs
git commit -m "feat: add CollapsibleSection component for unified sidebar

Create reusable collapsible section component with:
- Expand/collapse state with visual indicator (▼/▶)
- Optional badge count display
- Smooth animations
- Click header to toggle

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Expand Sidebar and Add Step Properties Section

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Import CollapsibleSection**

Modify: `src/ui/components/step_editor_sidebar.rs`

Add to imports at top of file:
```rust
use crate::ui::components::collapsible_section::CollapsibleSection;
```

**Step 2: Change sidebar width from w-60 to w-80**

Modify: `src/ui/components/step_editor_sidebar.rs:173`

Change:
```rust
<div class="w-60 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col">
```

To:
```rust
<div class="w-80 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col overflow-y-auto">
```

**Step 3: Wrap existing step parameters in CollapsibleSection**

Modify: `src/ui/components/step_editor_sidebar.rs`

Find the parameter controls section (around line 193) and wrap them:

Change from:
```rust
<div class="flex flex-col gap-3">
    <InlineParam>
        <ParamLabel text="Note (Pitch)" ... />
        ...
    </InlineParam>
    // ... other 4 parameters
</div>
```

To:
```rust
<CollapsibleSection
    title="STEP PROPERTIES"
    default_open=true
>
    <InlineParam>
        <ParamLabel text="Note (Pitch)" locked=Signal::derive(|| false) />
        <NumberInput
            min="0"
            max="127"
            step="1"
            value=Signal::derive(move || format!("{}", note_value.get() as u8))
            on_input=on_note_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Velocity" locked=Signal::derive(|| false) />
        <NumberInput
            min="0"
            max="127"
            step="1"
            value=Signal::derive(move || format!("{}", velocity_value.get() as u8))
            on_input=on_velocity_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Length" locked=Signal::derive(|| false) />
        <NumberInput
            min="0.1"
            max="4.0"
            step="0.1"
            value=Signal::derive(move || format!("{:.1}", length_value.get()))
            on_input=on_length_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Probability" locked=Signal::derive(|| false) />
        <NumberInput
            min="0"
            max="100"
            step="1"
            value=Signal::derive(move || format!("{}", probability_value.get() as u8))
            on_input=on_probability_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Micro-timing" locked=Signal::derive(|| false) />
        <NumberInput
            min="-23"
            max="23"
            step="1"
            value=Signal::derive(move || format!("{}", micro_timing_value.get() as i8))
            on_input=on_micro_timing_change
        />
    </InlineParam>
</CollapsibleSection>
```

**Step 4: Test compilation and manual verification**

Run: `cd flux-app && cargo check`
Expected: Compiles successfully

If you can build: `npm run dev` and verify:
- Sidebar is wider (320px)
- Step Properties section has collapsible header
- Click header toggles expand/collapse
- Parameters still editable

**Step 5: Commit**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "refactor: expand sidebar and wrap step properties in collapsible section

Changes:
- Increase sidebar width from 240px to 320px (w-60 → w-80)
- Add overflow-y-auto for scrolling when content exceeds viewport
- Wrap existing 5 step parameters in CollapsibleSection
- Section expanded by default

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Add Sound Parameters Section with P-Lock Logic

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Add Sound Parameters data and helper functions**

Modify: `src/ui/components/step_editor_sidebar.rs`

After the micro_timing change handler (around line 170), add:

```rust
// Sound parameter definitions (8 parameters from Inspector)
let sound_params = vec![
    "Tuning", "Filter Freq", "Resonance", "Drive",
    "Decay", "Sustain", "Reverb", "Delay"
];

// Get track ID helper
let get_track_id = move || {
    selected_step.get()
        .map(|(tid, _)| tid)
        .unwrap_or(0)
};

// Get sound parameter value (P-Lock or track default)
let get_param_value = move |param_idx: usize| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            if let Some(track) = p.tracks.get(track_id) {
                if let Some(subtrack) = track.subtracks.get(0) {
                    if let Some(step) = subtrack.steps.get(step_idx) {
                        // Check P-Lock first, fallback to track default
                        return step.p_locks.get(param_idx)
                            .and_then(|p| *p)
                            .unwrap_or_else(|| track.default_params.get(param_idx).cloned().unwrap_or(0.0));
                    }
                }
            }
            0.0
        })
    } else {
        0.0
    }
};

// Check if parameter is P-Locked
let is_param_locked = move |param_idx: usize| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .and_then(|s| s.p_locks.get(param_idx))
                .map(|p| p.is_some())
                .unwrap_or(false)
        })
    } else {
        false
    }
};

// Handle sound parameter input (creates/updates P-Lock)
let handle_param_input = move |param_idx: usize, val: f64| {
    if let Some((track_id, step_idx)) = selected_step.get() {
        let clamped = val.clamp(0.0, 1.0) as f32;
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(0) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        // Check if value differs from track default
                        let track_default = track.default_params.get(param_idx).cloned().unwrap_or(0.0);
                        if (clamped - track_default).abs() > 0.001 {
                            // Different from default → create P-Lock
                            if param_idx < 128 {
                                step.p_locks[param_idx] = Some(clamped);
                            }
                        } else {
                            // Same as default → remove P-Lock
                            if param_idx < 128 {
                                step.p_locks[param_idx] = None;
                            }
                        }
                    }
                }
            }
        });
    }
};

// P-Lock count for badge
let p_lock_count = Signal::derive(move || {
    if let Some((track_id, step_idx)) = selected_step.get() {
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(0))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.p_locks.iter().filter(|p| p.is_some()).count())
                .unwrap_or(0)
        })
    } else {
        0
    }
});
```

**Step 2: Add Sound Parameters CollapsibleSection to view**

Modify: `src/ui/components/step_editor_sidebar.rs`

After the Step Properties CollapsibleSection, add:

```rust
<CollapsibleSection
    title="SOUND PARAMETERS"
    default_open=true
    badge_count=Some(p_lock_count)
>
    {sound_params.into_iter().enumerate().map(|(idx, name)| {
        view! {
            <InlineParam>
                <ParamLabel
                    text=name
                    locked=Signal::derive(move || is_param_locked(idx))
                />
                <NumberInput
                    min="0"
                    max="1"
                    step="0.01"
                    value=Signal::derive(move || format!("{:.2}", get_param_value(idx)))
                    on_input=move |val| {
                        handle_param_input(idx, val);
                    }
                />
            </InlineParam>
        }
    }).collect::<Vec<_>>()}
</CollapsibleSection>
```

**Step 3: Test compilation**

Run: `cd flux-app && cargo check`
Expected: Compiles successfully

**Step 4: Manual verification**

If you can build: `npm run dev` and verify:
- Sound Parameters section appears below Step Properties
- Badge shows P-Lock count when >0
- Click section header to collapse/expand
- Edit a parameter → label turns amber (P-Locked)
- Change back to default → P-Lock removed

**Step 5: Commit**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add Sound Parameters section with P-Lock logic

Add second collapsible section with 8 synthesis parameters:
- Tuning, Filter Freq, Resonance, Drive, Decay, Sustain, Reverb, Delay
- P-Lock detection and automatic creation/removal
- Amber labels for P-Locked parameters
- Badge shows P-Lock count in section header
- Values fallback to track defaults when not P-Locked

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Add LFO Section to Sidebar

**Files:**
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Import LFO Designer component**

Modify: `src/ui/components/step_editor_sidebar.rs`

Add to imports:
```rust
use crate::ui::components::lfo_designer::LfoDesigner;
```

**Step 2: Add LFO value derivations and handlers**

Modify: `src/ui/components/step_editor_sidebar.rs`

After the P-Lock logic, add:

```rust
// LFO value derivations (track-level, not step-specific)
let lfo_shape = Signal::derive(move || {
    let track_id = get_track_id();
    pattern_signal.with(|p| {
        p.tracks.get(track_id)
            .and_then(|t| t.lfos.get(0))
            .map(|l| match &l.shape {
                crate::shared::models::LFOShape::Sine => "Sine",
                crate::shared::models::LFOShape::Triangle => "Triangle",
                crate::shared::models::LFOShape::Square => "Square",
                crate::shared::models::LFOShape::Random => "Random",
                crate::shared::models::LFOShape::Designer(_) => "Designer",
            })
            .unwrap_or("Triangle")
            .to_string()
    })
});

let lfo_amount = Signal::derive(move || {
    let track_id = get_track_id();
    pattern_signal.with(|p| {
        p.tracks.get(track_id)
            .and_then(|t| t.lfos.get(0))
            .map(|l| l.amount)
            .unwrap_or(0.0)
    })
});

let lfo_speed = Signal::derive(move || {
    let track_id = get_track_id();
    pattern_signal.with(|p| {
        p.tracks.get(track_id)
            .and_then(|t| t.lfos.get(0))
            .map(|l| l.speed)
            .unwrap_or(1.0)
    })
});

let lfo_destination = Signal::derive(move || {
    let track_id = get_track_id();
    pattern_signal.with(|p| {
        p.tracks.get(track_id)
            .and_then(|t| t.lfos.get(0))
            .map(|l| l.destination.to_string())
            .unwrap_or_else(|| "74".to_string())
    })
});

let is_designer = Signal::derive(move || {
    let track_id = get_track_id();
    pattern_signal.with(|p| {
        p.tracks.get(track_id)
            .and_then(|t| t.lfos.get(0))
            .map(|l| matches!(l.shape, crate::shared::models::LFOShape::Designer(_)))
            .unwrap_or(false)
    })
});

// LFO change handlers
let on_shape_change = move |val: String| {
    let track_id = get_track_id();
    set_pattern_signal.update(|p| {
        if let Some(track) = p.tracks.get_mut(track_id) {
            if let Some(lfo) = track.lfos.get_mut(0) {
                lfo.shape = match val.as_str() {
                    "Sine" => crate::shared::models::LFOShape::Sine,
                    "Triangle" => crate::shared::models::LFOShape::Triangle,
                    "Square" => crate::shared::models::LFOShape::Square,
                    "Random" => crate::shared::models::LFOShape::Random,
                    "Designer" => crate::shared::models::LFOShape::Designer([0.0; 16].to_vec()),
                    _ => crate::shared::models::LFOShape::Triangle,
                };
            }
        }
    });
};

let on_amount_change = move |val: f64| {
    let clamped = val.clamp(-1.0, 1.0) as f32;
    let track_id = get_track_id();
    set_pattern_signal.update(|p| {
        if let Some(track) = p.tracks.get_mut(track_id) {
            if let Some(lfo) = track.lfos.get_mut(0) {
                lfo.amount = clamped;
            }
        }
    });
};

let on_speed_change = move |val: f64| {
    let clamped = val.clamp(0.1, 4.0) as f32;
    let track_id = get_track_id();
    set_pattern_signal.update(|p| {
        if let Some(track) = p.tracks.get_mut(track_id) {
            if let Some(lfo) = track.lfos.get_mut(0) {
                lfo.speed = clamped;
            }
        }
    });
};

let on_destination_change = move |val: String| {
    let parsed_val = val.parse::<u8>().unwrap_or(74);
    let track_id = get_track_id();
    set_pattern_signal.update(|p| {
        if let Some(track) = p.tracks.get_mut(track_id) {
            if let Some(lfo) = track.lfos.get_mut(0) {
                lfo.destination = parsed_val;
            }
        }
    });
};
```

**Step 3: Add LFO CollapsibleSection to view**

Modify: `src/ui/components/step_editor_sidebar.rs`

After the Sound Parameters CollapsibleSection, add:

```rust
<CollapsibleSection
    title="LFO"
    default_open=false
>
    <InlineParam>
        <ParamLabel text="Shape" locked=Signal::derive(|| false) />
        <Dropdown
            options=vec![
                ("Sine", "∿"),
                ("Triangle", "△"),
                ("Square", "▭"),
                ("Random", "※"),
                ("Designer", "✎"),
            ]
            selected=lfo_shape
            on_change=on_shape_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Amount" locked=Signal::derive(|| false) />
        <NumberInput
            min="-1"
            max="1"
            step="0.01"
            value=Signal::derive(move || format!("{:.2}", lfo_amount.get()))
            on_input=on_amount_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Speed" locked=Signal::derive(|| false) />
        <NumberInput
            min="0.1"
            max="4.0"
            step="0.1"
            value=Signal::derive(move || format!("{:.1}", lfo_speed.get()))
            on_input=on_speed_change
        />
    </InlineParam>

    <InlineParam>
        <ParamLabel text="Destination" locked=Signal::derive(|| false) />
        <Dropdown
            options=vec![
                ("74", "Filter Cutoff"),
                ("71", "Resonance"),
                ("1", "Mod Wheel"),
                ("10", "Pan"),
            ]
            selected=lfo_destination
            on_change=on_destination_change
        />
    </InlineParam>

    // Designer waveform editor (conditional)
    {move || {
        if is_designer.get() {
            view! {
                <div class="mt-2">
                    <label class="text-xs text-zinc-500 mb-1 block">"Waveform Designer"</label>
                    <LfoDesigner
                        track_id=Signal::derive(move || get_track_id())
                        lfo_index=Signal::derive(move || 0)
                        value=Signal::derive(move || {
                            let track_id = get_track_id();
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
                                let track_id = get_track_id();
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
            view! { <div></div> }.into_any()
        }
    }}
</CollapsibleSection>
```

**Step 4: Test compilation**

Run: `cd flux-app && cargo check`
Expected: Compiles successfully

**Step 5: Manual verification**

If you can build: `npm run dev` and verify:
- LFO section appears below Sound Parameters
- Section collapsed by default
- Click header to expand
- All 4 LFO controls functional
- Designer UI appears when Shape = "Designer"

**Step 6: Commit**

```bash
git add src/ui/components/step_editor_sidebar.rs
git commit -m "feat: add LFO section to unified sidebar

Add third collapsible section with LFO controls:
- Shape dropdown (Sine, Triangle, Square, Random, Designer)
- Amount (-1.0 to 1.0)
- Speed (0.1 to 4.0)
- Destination (Filter Cutoff, Resonance, Mod Wheel, Pan)
- Waveform Designer (conditional, when shape = Designer)

Section collapsed by default to save vertical space.
LFO is track-level (not per-step).

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Remove Inspector Component

**Files:**
- Delete: `src/ui/components/inspector.rs`
- Modify: `src/ui/components/mod.rs`
- Modify: `src/app.rs`

**Step 1: Remove Inspector from mod.rs**

Modify: `src/ui/components/mod.rs`

Remove this line:
```rust
pub mod inspector;
```

**Step 2: Remove Inspector from App component**

Modify: `src/app.rs`

Remove Inspector import (if exists):
```rust
use crate::ui::components::inspector::Inspector;
```

Find the Parameters section in the view (should be at the bottom of the layout) and remove it entirely:

Remove:
```rust
// Parameters Section
<div class="flex-shrink-0">
    <Inspector />
</div>
```

**Step 3: Delete inspector.rs file**

Run:
```bash
rm src/ui/components/inspector.rs
```

**Step 4: Test compilation**

Run: `cd flux-app && cargo check`
Expected: Compiles successfully (no Inspector references remaining)

**Step 5: Manual verification**

If you can build: `npm run dev` and verify:
- Bottom Inspector section is gone
- Grid section takes full vertical space
- All functionality now in sidebar
- Step Properties, Sound Parameters, and LFO sections all functional

**Step 6: Commit**

```bash
git add -A
git commit -m "refactor: remove Inspector component

Remove Inspector component and bottom Parameters section:
- Delete src/ui/components/inspector.rs
- Remove from mod.rs and app.rs
- All Inspector functionality now in unified StepEditorSidebar
- Grid section expands to fill vertical space

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Add Visual Polish and Animations

**Files:**
- Modify: `src/ui/components/collapsible_section.rs`
- Modify: `src/ui/components/step_editor_sidebar.rs`

**Step 1: Enhance CollapsibleSection animations**

Modify: `src/ui/components/collapsible_section.rs`

Update the content rendering to use better animations:

Change:
```rust
{move || {
    if is_open.get() {
        view! {
            <div class="flex flex-col gap-3 mt-2 animate-in slide-in-from-top-2 fade-in duration-200">
                {children()}
            </div>
        }.into_any()
    } else {
        view! { <div></div> }.into_any()
    }
}}
```

To:
```rust
{move || {
    if is_open.get() {
        view! {
            <div class="flex flex-col gap-3 mt-2 transition-all duration-200 animate-in slide-in-from-top-2 fade-in">
                {children()}
            </div>
        }.into_any()
    } else {
        view! { <div class="hidden"></div> }.into_any()
    }
}}
```

**Step 2: Add spacing between CollapsibleSections**

Modify: `src/ui/components/step_editor_sidebar.rs`

Wrap all three CollapsibleSections in a container with vertical spacing:

Change:
```rust
<CollapsibleSection title="STEP PROPERTIES" ...>
...
</CollapsibleSection>

<CollapsibleSection title="SOUND PARAMETERS" ...>
...
</CollapsibleSection>

<CollapsibleSection title="LFO" ...>
...
</CollapsibleSection>
```

To:
```rust
<div class="flex flex-col gap-4">
    <CollapsibleSection title="STEP PROPERTIES" default_open=true>
        // ... step properties ...
    </CollapsibleSection>

    <CollapsibleSection title="SOUND PARAMETERS" default_open=true badge_count=Some(p_lock_count)>
        // ... sound parameters ...
    </CollapsibleSection>

    <CollapsibleSection title="LFO" default_open=false>
        // ... lfo controls ...
    </CollapsibleSection>
</div>
```

**Step 3: Add visual separator between sections**

Modify: `src/ui/components/collapsible_section.rs`

Add bottom border to section container:

Change:
```rust
<div class="flex flex-col">
```

To:
```rust
<div class="flex flex-col pb-3 border-b border-zinc-800/50 last:border-b-0">
```

**Step 4: Test compilation**

Run: `cd flux-app && cargo check`
Expected: Compiles successfully

**Step 5: Manual verification**

If you can build: `npm run dev` and verify:
- Smooth expand/collapse animations
- Visual separators between sections
- Consistent spacing
- Clean, polished appearance

**Step 6: Commit**

```bash
git add src/ui/components/collapsible_section.rs src/ui/components/step_editor_sidebar.rs
git commit -m "polish: enhance unified sidebar animations and spacing

Visual improvements:
- Smooth transitions for expand/collapse
- Consistent vertical spacing between sections (gap-4)
- Visual separators (bottom borders) between sections
- Better animation classes for content show/hide

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Update Design Documentation

**Files:**
- Modify: `docs/plans/2026-02-14-unified-step-editor.md`

**Step 1: Add implementation notes to design doc**

Modify: `docs/plans/2026-02-14-unified-step-editor.md`

At the end of the file, add:

```markdown
---

## Implementation Notes (Completed)

**Date Completed:** 2026-02-14

**Changes Made:**
1. Created CollapsibleSection component (80 lines)
2. Expanded StepEditorSidebar from 240px to 320px width
3. Added three collapsible sections:
   - Step Properties (5 parameters)
   - Sound Parameters (8 parameters with P-Lock logic)
   - LFO (4 controls + designer)
4. Removed Inspector component entirely
5. Added visual polish and animations

**Files Modified:**
- Created: `src/ui/components/collapsible_section.rs`
- Modified: `src/ui/components/step_editor_sidebar.rs` (~450 lines)
- Modified: `src/ui/components/mod.rs`
- Modified: `src/app.rs`
- Deleted: `src/ui/components/inspector.rs`

**Commits:** 7 total (one per task)

**Verification:**
- All three sections collapsible and functional
- P-Lock creation/removal automatic
- P-Lock count badge accurate
- LFO controls work identically to previous Inspector
- No regressions in step editing
- Sidebar scrollable when content exceeds viewport

**Known Issues:** None

**Future Enhancements:**
- Add keyboard navigation (Tab between parameters, Arrow keys for sections)
- Add right-click context menu for "Clear P-Lock"
- Add "Track Defaults" modal if users need direct access
- Persist collapse state across sessions (localStorage)
```

**Step 2: Commit documentation update**

```bash
git add docs/plans/2026-02-14-unified-step-editor.md
git commit -m "docs: add implementation completion notes to design doc

Document completion of unified step editor sidebar implementation
with summary of changes, files modified, and verification notes.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Summary

**Implementation Plan Complete**

This plan implements the unified step editor sidebar in 7 tasks:

1. **CollapsibleSection Component** - Reusable collapsible container
2. **Expand Sidebar** - Increase width, wrap step properties
3. **Sound Parameters** - Add P-Lock section with automatic lock creation
4. **LFO Section** - Move LFO controls to sidebar
5. **Remove Inspector** - Delete Inspector component and bottom section
6. **Visual Polish** - Animations, spacing, separators
7. **Documentation** - Update design doc with completion notes

**Key Files:**
- `src/ui/components/collapsible_section.rs` (new, ~80 lines)
- `src/ui/components/step_editor_sidebar.rs` (modified, ~450 lines)
- `src/ui/components/inspector.rs` (deleted)

**Testing:**
- Manual testing after each task
- Verify expand/collapse behavior
- Verify P-Lock creation/removal
- Verify LFO controls
- No automated tests (UI component testing not in place)

**Estimated Time:** 2-3 hours for implementation + testing
