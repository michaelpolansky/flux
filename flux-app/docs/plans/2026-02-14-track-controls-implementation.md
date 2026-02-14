# Track Add/Remove Controls Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add dynamic track management controls to FLUX sequencer (add/remove tracks with confirmation dialog)

**Architecture:** Reactive signal-based mutations on Pattern.tracks Vec, with Leptos <For> components automatically re-rendering on track count changes. No new Tauri commands needed.

**Tech Stack:** Leptos 0.7 (Rust WASM), Tailwind CSS, Tauri 2.10.2 (optional backend sync)

---

## Prerequisites

- Read design document: `docs/plans/2026-02-14-track-controls-design.md`
- Current working directory: `/Users/michaelpolansky/Development/flux/flux-app`
- Desktop app running in background for testing: `npm run dev`

---

## Task 1: Update Data Models (PlaybackState to Vec)

**Files:**
- Modify: `src/ui/state.rs:4-8`
- Modify: `src/app.rs:15-20`
- Modify: `src-tauri/src/lib.rs` (find AudioSnapshot struct)
- Modify: `src-tauri/src/shared/models.rs` (Pattern default)

**Step 1: Update frontend PlaybackState**

File: `src/ui/state.rs`

Replace lines 4-8:
```rust
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,
    pub triggered_tracks: Vec<bool>,  // Changed from [bool; 4]
}
```

**Step 2: Update AudioSnapshot in app.rs**

File: `src/app.rs`

Replace lines 15-20:
```rust
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct AudioSnapshot {
    current_step: usize,
    is_playing: bool,
    triggered_tracks: Vec<bool>,  // Changed from Option<[bool; 4]>
}
```

And update line 70 to handle Vec:
```rust
state.triggered_tracks = event.triggered_tracks;  // Remove .unwrap_or([false; 4])
```

**Step 3: Find and update backend AudioSnapshot**

Search for AudioSnapshot in backend:
```bash
grep -n "struct AudioSnapshot" src-tauri/src/lib.rs
```

Update the struct to use `Vec<bool>` instead of `Option<[bool; 4]>`.

**Step 4: Update Pattern default track count**

File: `src/shared/models.rs`

Find the `impl Default for Pattern` block (around line 170) and change:
```rust
impl Default for Pattern {
    fn default() -> Self {
        let mut tracks = Vec::with_capacity(4);  // Changed from 16
        for i in 0..4 {  // Changed from 16
            let mut t = Track::default();
            t.id = i;
            tracks.push(t);
        }
        Self {
            tracks,
            bpm: 120.0,
            master_length: 16,
        }
    }
}
```

**Step 5: Verify compilation**

Run: `cargo check`
Expected: Compiles successfully (or shows errors to fix)

**Step 6: Commit**

```bash
git add src/ui/state.rs src/app.rs src/shared/models.rs src-tauri/src/lib.rs
git commit -m "refactor: change triggered_tracks from fixed array to Vec

- PlaybackState.triggered_tracks: [bool; 4] → Vec<bool>
- AudioSnapshot.triggered_tracks: Option<[bool; 4]> → Vec<bool>
- Pattern default track count: 16 → 4
- Enables dynamic track count support

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Create ConfirmDialog Component

**Files:**
- Create: `src/ui/components/confirm_dialog.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create confirm_dialog.rs**

File: `src/ui/components/confirm_dialog.rs`

```rust
use leptos::prelude::*;
use leptos::ev;

#[component]
pub fn ConfirmDialog(
    /// Whether the dialog is visible
    visible: Signal<bool>,
    /// Callback when user confirms
    on_confirm: impl Fn() + 'static,
    /// Callback when user cancels
    on_cancel: impl Fn() + 'static,
    /// Dialog title
    title: &'static str,
    /// Dialog message
    message: Signal<String>,
) -> impl IntoView {
    // ESC key handler
    let handle_escape = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" && visible.get() {
            on_cancel();
        }
    };

    window_event_listener(ev::keydown, handle_escape);

    view! {
        <Show when=move || visible.get()>
            <div
                class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
                on:click=move |_| on_cancel()  // Click outside to close
            >
                <div
                    class="bg-zinc-900 border border-zinc-700 rounded-lg p-6 max-w-sm"
                    on:click=|e| e.stop_propagation()  // Prevent close when clicking inside
                >
                    <h3 class="text-lg font-medium mb-2 text-zinc-50">{title}</h3>
                    <p class="text-sm text-zinc-400 mb-4">{message}</p>
                    <div class="flex gap-2 justify-end">
                        <button
                            class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-zinc-300 transition-colors"
                            on:click=move |_| on_cancel()
                        >
                            "Cancel"
                        </button>
                        <button
                            class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm text-zinc-50 transition-colors"
                            on:click=move |_| on_confirm()
                        >
                            "Remove Track"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
```

**Step 2: Export component in mod.rs**

File: `src/ui/components/mod.rs`

Add:
```rust
pub mod confirm_dialog;
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/ui/components/confirm_dialog.rs src/ui/components/mod.rs
git commit -m "feat: add ConfirmDialog reusable component

- Modal dialog with dark backdrop
- ESC key and click-outside-to-close support
- Styled to match FLUX design (zinc/dark theme)
- Used for track removal confirmation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Create RemoveTrackButton Component

**Files:**
- Create: `src/ui/components/remove_track_button.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create remove_track_button.rs**

File: `src/ui/components/remove_track_button.rs`

```rust
use leptos::prelude::*;
use crate::shared::models::{Pattern, TrigType};

#[component]
pub fn RemoveTrackButton(
    /// Track index to remove
    track_idx: usize,
    /// Signal to trigger confirmation dialog
    show_confirm: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Check if this track has any data (active steps)
    let has_data = move || {
        pattern_signal.with(|pattern| {
            pattern.tracks.get(track_idx)
                .and_then(|t| t.subtracks.get(0))
                .map(|st| st.steps.iter().any(|s| s.trig_type != TrigType::None))
                .unwrap_or(false)
        })
    };

    // Check if button should be disabled (only 1 track left)
    let is_disabled = move || {
        pattern_signal.with(|p| p.tracks.len() <= 1)
    };

    let handle_click = move |_| {
        // Don't do anything if disabled
        if is_disabled() {
            return;
        }

        if has_data() {
            // Show confirmation dialog
            show_confirm.set(Some(track_idx));
        } else {
            // Remove immediately (no confirmation needed)
            do_remove_track(track_idx, set_pattern_signal);
        }
    };

    view! {
        <button
            class="w-4 h-4 text-zinc-600 hover:text-red-500 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            disabled=is_disabled
            on:click=handle_click
            title=move || if is_disabled() { "Cannot remove last track" } else { "Remove track" }
        >
            "×"
        </button>
    }
}

// Helper function to remove track and re-index
fn do_remove_track(track_idx: usize, set_pattern_signal: WriteSignal<Pattern>) {
    set_pattern_signal.update(|pattern| {
        if pattern.tracks.len() <= 1 {
            return; // Safety check
        }
        pattern.tracks.remove(track_idx);
        // Re-index remaining tracks
        for (i, track) in pattern.tracks.iter_mut().enumerate() {
            track.id = i;
        }
    });

    // Clear selected step if it became invalid
    if let Some(selected_step) = use_context::<RwSignal<Option<(usize, usize)>>>() {
        if let Some((selected_track, _)) = selected_step.get() {
            let track_count = use_context::<ReadSignal<Pattern>>()
                .expect("Pattern context not found")
                .with(|p| p.tracks.len());
            if selected_track >= track_count {
                selected_step.set(None);
            }
        }
    }
}
```

**Step 2: Export component in mod.rs**

File: `src/ui/components/mod.rs`

Add:
```rust
pub mod remove_track_button;
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/ui/components/remove_track_button.rs src/ui/components/mod.rs
git commit -m "feat: add RemoveTrackButton component

- Small × button for each track label
- Checks if track has data (active steps)
- Shows confirmation dialog if needed
- Direct removal for empty tracks
- Disabled when only 1 track remains
- Clears invalid selected step after removal

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Create TrackControls Component

**Files:**
- Create: `src/ui/components/track_controls.rs`
- Modify: `src/ui/components/mod.rs`

**Step 1: Create track_controls.rs**

File: `src/ui/components/track_controls.rs`

```rust
use leptos::prelude::*;
use crate::shared::models::{Pattern, Track, MachineType};

#[component]
pub fn TrackControls() -> impl IntoView {
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    let add_track = move |_| {
        set_pattern_signal.update(|pattern| {
            let new_id = pattern.tracks.len();
            let mut new_track = Track::default();
            new_track.id = new_id;
            new_track.machine = MachineType::OneShot;  // Default to OneShot
            pattern.tracks.push(new_track);
        });
    };

    let track_count = move || pattern_signal.with(|p| p.tracks.len());

    view! {
        <div class="mt-3 flex items-center gap-3">
            <button
                class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 border border-zinc-700 rounded text-sm text-zinc-300 transition-colors"
                on:click=add_track
            >
                "+ Add Track"
            </button>
            <span class="text-xs text-zinc-500 font-mono">
                {move || format!("{} tracks", track_count())}
            </span>
        </div>
    }
}
```

**Step 2: Export component in mod.rs**

File: `src/ui/components/mod.rs`

Add:
```rust
pub mod track_controls;
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/ui/components/track_controls.rs src/ui/components/mod.rs
git commit -m "feat: add TrackControls component

- [+ Add Track] button below grid
- Shows current track count
- Instantly adds new track with OneShot machine type
- Track IDs assigned sequentially

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Update Grid Component for Dynamic Tracks

**Files:**
- Modify: `src/ui/components/grid.rs`

**Step 1: Import new components**

File: `src/ui/components/grid.rs`

Add to imports (top of file):
```rust
use super::remove_track_button::RemoveTrackButton;
use super::track_controls::TrackControls;
use super::confirm_dialog::ConfirmDialog;
```

**Step 2: Add confirmation dialog state**

File: `src/ui/components/grid.rs`

After line 18 (after providing grid_ui_state context), add:
```rust
// State for confirmation dialog
let (show_confirm_dialog, set_show_confirm_dialog) = signal::<Option<usize>>(None);

// Confirmation dialog message
let confirm_message = Signal::derive(move || {
    if let Some(track_idx) = show_confirm_dialog.get() {
        format!("Track {} has active steps. Remove anyway?", track_idx + 1)
    } else {
        String::new()
    }
});

// Confirmation callback
let on_confirm_remove = move || {
    if let Some(track_idx) = show_confirm_dialog.get() {
        // Call the remove function
        crate::ui::components::remove_track_button::do_remove_track(
            track_idx,
            set_pattern_signal
        );
        set_show_confirm_dialog.set(None);
    }
};

let on_cancel_remove = move || {
    set_show_confirm_dialog.set(None);
};
```

**Step 3: Make track iteration dynamic**

File: `src/ui/components/grid.rs`

Replace the hardcoded track labels section (lines 82-88) with:
```rust
// Track labels on the left (dynamic)
<div class="flex flex-col gap-[2px] mr-2">
    <For
        each=move || {
            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
        }
        key=|track_idx| *track_idx
        children=move |track_idx| {
            view! {
                <div class="w-8 h-10 flex items-center justify-center gap-1">
                    <div class="text-xs text-zinc-400">
                        {format!("T{}", track_idx + 1)}
                    </div>
                    <RemoveTrackButton
                        track_idx=track_idx
                        show_confirm=set_show_confirm_dialog
                    />
                </div>
            }
        }
    />
</div>
```

**Step 4: Make grid rows dynamic**

File: `src/ui/components/grid.rs`

Replace the hardcoded `(0..4)` in the grid rows section (around line 96) with:
```rust
<For
    each=move || {
        pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
    }
    key=|track_idx| *track_idx
    children=move |track_idx| {
        // ... existing step rendering code
    }
/>
```

**Step 5: Add TrackControls and ConfirmDialog to view**

File: `src/ui/components/grid.rs`

Before the closing `</div>` of the main sequencer-grid div (around line 127), add:
```rust
// Add track controls below grid
<TrackControls />

// Confirmation dialog (rendered at top level)
<ConfirmDialog
    visible=Signal::derive(move || show_confirm_dialog.get().is_some())
    on_confirm=on_confirm_remove
    on_cancel=on_cancel_remove
    title="Confirm Removal"
    message=confirm_message
/>
```

**Step 6: Make do_remove_track public**

File: `src/ui/components/remove_track_button.rs`

Change line with `fn do_remove_track` to:
```rust
pub fn do_remove_track(track_idx: usize, set_pattern_signal: WriteSignal<Pattern>) {
```

**Step 7: Verify compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 8: Commit**

```bash
git add src/ui/components/grid.rs src/ui/components/remove_track_button.rs
git commit -m "feat: make Grid component dynamic with track controls

- Track labels now render dynamically based on pattern.tracks.len()
- Grid rows iterate over actual track count
- RemoveTrackButton added to each label
- TrackControls added below grid
- ConfirmDialog integrated for safe track removal
- Tracks can now be added/removed reactively

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Manual Testing & Verification

**Step 1: Start desktop app**

Run: `npm run dev`
Expected: Tauri window opens, FLUX sequencer loads

**Step 2: Test add track**

Actions:
1. Click `[+ Add Track]` button
2. Observe new track (T5) appears
3. Click again, T6 appears
4. Verify track count updates ("6 tracks")

Expected: Tracks add instantly, grid expands smoothly

**Step 3: Test remove empty track**

Actions:
1. Click `[×]` on T6 (empty track)
2. Observe track removed immediately (no dialog)
3. Verify T6 disappears, count shows "5 tracks"

Expected: Instant removal, no confirmation

**Step 4: Test remove track with data**

Actions:
1. Click on a step in T5 to add a trigger
2. Click `[×]` on T5
3. Observe confirmation dialog appears
4. Click "Cancel"
5. Verify T5 still exists
6. Click `[×]` on T5 again
7. Click "Remove Track"
8. Verify T5 removed

Expected: Dialog shows, Cancel works, Remove works

**Step 5: Test minimum track enforcement**

Actions:
1. Remove tracks until only 1 remains
2. Observe `[×]` button is disabled (grayed out)
3. Try to click it
4. Verify nothing happens

Expected: Cannot remove last track

**Step 6: Test track ID re-indexing**

Actions:
1. Add 5 tracks (T1-T5)
2. Remove T3 (middle track)
3. Verify T4 becomes T3, T5 becomes T4
4. Verify all tracks labeled correctly

Expected: Tracks re-numbered properly

**Step 7: Test playback with dynamic tracks**

Actions:
1. Add 8 tracks
2. Add triggers to various tracks
3. Click Play
4. Observe playback works
5. Remove a track during playback
6. Verify playback continues

Expected: Playback unaffected by track changes

**Step 8: Test ESC key and click outside**

Actions:
1. Trigger confirmation dialog
2. Press ESC key
3. Verify dialog closes
4. Trigger dialog again
5. Click outside dialog (on backdrop)
6. Verify dialog closes

Expected: Both close methods work

**Step 9: Document any issues**

If bugs found:
1. Create GitHub issue or note in TODO
2. Add to known issues section in docs

**Step 10: Final verification commit**

```bash
git add -A
git commit -m "test: verify track add/remove controls functionality

Manual testing completed:
- Add track: ✅ Works instantly
- Remove empty track: ✅ No confirmation
- Remove track with data: ✅ Shows confirmation
- Minimum 1 track: ✅ Enforced
- Track re-indexing: ✅ Correct
- Playback integration: ✅ Works
- ESC key / click outside: ✅ Works

All functionality verified in desktop mode.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Update Documentation

**Files:**
- Modify: `README.md` (add track controls to features)
- Modify: `USER_GUIDE.md` (add track management section)
- Modify: `ARCHITECTURE.md` (document dynamic track system)

**Step 1: Update README.md features**

File: `flux-app/README.md`

Find the Features section and add:
```markdown
- **Dynamic Track Management**: Add/remove tracks on the fly (min 1, unlimited max)
  - `[+ Add Track]` button below grid
  - `[×]` remove buttons on track labels
  - Confirmation for tracks with active steps
```

**Step 2: Add to USER_GUIDE.md**

File: `flux-app/USER_GUIDE.md`

Add new section (after Sequencer Grid section):
```markdown
## Track Management

### Adding Tracks

1. Click the **[+ Add Track]** button below the sequencer grid
2. A new track appears instantly with 16 empty steps
3. Default machine type: OneShot (Digitakt II style)
4. Track count shown below grid (e.g., "8 tracks")

### Removing Tracks

1. Click the **[×]** button next to any track label (T1, T2, etc.)
2. If the track is empty (no active steps):
   - Track removed immediately (no confirmation)
3. If the track has data (active steps):
   - Confirmation dialog appears: "Track N has active steps. Remove anyway?"
   - Click **Cancel** (or press ESC) to abort
   - Click **Remove Track** to confirm removal
4. Track IDs automatically re-index (T4 becomes T3, etc.)

### Limitations

- **Minimum:** 1 track (cannot remove the last track)
- **Maximum:** Unlimited (performance may degrade at 50+ tracks)
- Remove button disabled when only 1 track remains
```

**Step 3: Update ARCHITECTURE.md**

File: `flux-app/ARCHITECTURE.md`

Find the "Frontend State Management" section and add:
```markdown
### Dynamic Track Management

**Pattern Mutations:**
- Pattern.tracks is a `Vec<Track>` (dynamic size)
- Add track: `pattern.tracks.push(new_track)`
- Remove track: `pattern.tracks.remove(idx)` + re-index IDs
- UI reactively updates via Leptos `<For>` components

**Track Count Limits:**
- Minimum: 1 track (enforced in UI)
- Maximum: Unlimited (practical limit ~50 tracks before performance degrades)
- Default: 4 tracks

**State Consistency:**
- Track removal clears selected step if index becomes invalid
- PlaybackState.triggered_tracks is `Vec<bool>` (matches track count)
- Backend receives updated pattern on next playback/save operation
```

**Step 4: Commit documentation**

```bash
git add README.md USER_GUIDE.md ARCHITECTURE.md
git commit -m "docs: add track management documentation

- README: Added dynamic track management to features
- USER_GUIDE: Added Track Management section with usage
- ARCHITECTURE: Documented track mutation system

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Completion Checklist

- [ ] Task 1: Data models updated (Vec<bool> for triggered_tracks)
- [ ] Task 2: ConfirmDialog component created
- [ ] Task 3: RemoveTrackButton component created
- [ ] Task 4: TrackControls component created
- [ ] Task 5: Grid component updated for dynamic tracks
- [ ] Task 6: Manual testing completed (all scenarios verified)
- [ ] Task 7: Documentation updated (README, USER_GUIDE, ARCHITECTURE)

---

## Known Issues & Future Enhancements

**Known Issues:**
- None (feature complete as designed)

**Future Enhancements:**
- Keyboard shortcuts (Ctrl/Cmd + T to add track)
- Track reordering (drag-and-drop)
- Track duplication
- Track mute/solo buttons
- Undo/redo for track operations

---

## References

- Design doc: `docs/plans/2026-02-14-track-controls-design.md`
- Leptos documentation: https://leptos.dev
- FLUX architecture: `ARCHITECTURE.md`
- User guide: `USER_GUIDE.md`
