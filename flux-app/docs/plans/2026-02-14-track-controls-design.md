# Track Add/Remove Controls - Design Document

**Date:** 2026-02-14
**Status:** Approved
**Feature:** Dynamic track management with add/remove controls

---

## Overview

Add UI controls to dynamically add and remove tracks from the FLUX sequencer pattern. This enables users to expand or reduce the number of tracks in their pattern beyond the current fixed 4-track display.

---

## Requirements

### Functional Requirements

- **FR1:** Support unlimited dynamic track count (no hard maximum)
- **FR2:** Enforce minimum of 1 track (prevent empty patterns)
- **FR3:** Default new patterns to 4 tracks
- **FR4:** Add track control located below the grid
- **FR5:** Remove track control on each track label
- **FR6:** Confirmation dialog when removing tracks with active steps
- **FR7:** No confirmation for removing empty tracks
- **FR8:** New tracks default to OneShot machine type

### Non-Functional Requirements

- **NFR1:** Instant UI updates (no IPC delay for add/remove)
- **NFR2:** Graceful handling of track removal during playback
- **NFR3:** Maintain existing FLUX design language (zinc/dark theme)
- **NFR4:** Preserve selected step validity when tracks change

---

## Architecture

### Approach: Full Reactive Signals

We use Leptos reactive signal mutations to modify the Pattern in-place:

```
User Interaction
    ↓
Click [+ Add Track] or [x Remove]
    ↓
Mutation function updates Pattern signal
    ↓
Leptos reactivity detects Pattern.tracks.len() change
    ↓
Grid <For> component re-renders (adds/removes track rows)
    ↓
PlaybackState.triggered_tracks resizes (Vec instead of array)
    ↓
Backend receives updated pattern on next playback/save
```

### Key Architectural Decisions

1. **Pattern signal is source of truth** - All track operations mutate `set_pattern_signal`
2. **No new Tauri commands needed** - Existing `set_playback_state()` already sends full pattern
3. **Reactive rendering** - Grid's `<For>` loops automatically adjust to track count changes
4. **Minimal backend changes** - Only change `triggered_tracks` from `[bool; 4]` to `Vec<bool>`

---

## Data Model Changes

### Frontend (`src/ui/state.rs`)

**Before:**
```rust
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,
    pub triggered_tracks: [bool; 4],  // Fixed size array
}
```

**After:**
```rust
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,
    pub triggered_tracks: Vec<bool>,  // Dynamic vector
}
```

### Backend (`src-tauri/src/lib.rs`)

**Before:**
```rust
struct AudioSnapshot {
    current_step: usize,
    is_playing: bool,
    triggered_tracks: Option<[bool; 4]>,
}
```

**After:**
```rust
struct AudioSnapshot {
    current_step: usize,
    is_playing: bool,
    triggered_tracks: Vec<bool>,  // Dynamic vector
}
```

### Pattern Default (`src/shared/models.rs`)

**Before:**
```rust
impl Default for Pattern {
    fn default() -> Self {
        let mut tracks = Vec::with_capacity(16);
        for i in 0..16 {
            // ...
        }
    }
}
```

**After:**
```rust
impl Default for Pattern {
    fn default() -> Self {
        let mut tracks = Vec::with_capacity(4);
        for i in 0..4 {  // Default to 4 tracks
            // ...
        }
    }
}
```

### Migration Strategy

This is a minor breaking change. Since the project is in active development (not production), we can update the types directly. Existing patterns with old `triggered_tracks` format will fail to deserialize and will need to be recreated.

---

## Component Structure

### New Components

#### 1. `TrackControls` (`src/ui/components/track_controls.rs`)
- Renders `[+ Add Track]` button below the grid
- Handles add track logic
- Shows track count indicator (optional)

#### 2. `RemoveTrackButton` (`src/ui/components/remove_track_button.rs`)
- Small `[x]` button next to each track label
- Checks if track has data (any non-None trig_type)
- Shows confirmation dialog if needed
- Handles remove logic
- Disabled when only 1 track remains

#### 3. `ConfirmDialog` (`src/ui/components/confirm_dialog.rs`)
- Reusable modal dialog for confirmations
- Used for "Track has data, remove anyway?" prompt
- Clean Tailwind styling matching FLUX design
- ESC key and click-outside-to-close support

### Modified Components

#### 1. `Grid` (`src/ui/components/grid.rs`)
- Change hardcoded `(0..4)` to `(0..pattern.tracks.len())`
- Change track labels from hardcoded `T1, T2, T3, T4` to dynamic `T{i+1}`
- Add `RemoveTrackButton` next to each label
- Add `TrackControls` component below grid

### Component Hierarchy

```
Grid
├─ Track labels column
│  └─ For each track: "T{i}" + RemoveTrackButton
├─ Grid steps (existing)
│  └─ For each track × 16 steps
└─ TrackControls (new, below grid)
    └─ [+ Add Track] button

ConfirmDialog (rendered at App level, shown conditionally)
```

---

## State Management

### Add Track Flow

```rust
// In TrackControls component
let add_track = move |_| {
    set_pattern_signal.update(|pattern| {
        let new_id = pattern.tracks.len();
        let mut new_track = Track::default();
        new_track.id = new_id;
        new_track.machine = MachineType::OneShot;  // Default to OneShot
        pattern.tracks.push(new_track);
    });
};
```

**Behavior:**
- Instant UI update (no confirmation)
- New track appended to end
- Track ID = current track count
- Default machine type: OneShot
- 16 empty steps (all TrigType::None)

### Remove Track Flow

```rust
// In RemoveTrackButton component
let remove_track = move |track_idx: usize| {
    // 1. Check if track has data
    let has_data = pattern_signal.with(|pattern| {
        pattern.tracks.get(track_idx)
            .and_then(|t| t.subtracks.get(0))
            .map(|st| st.steps.iter().any(|s| s.trig_type != TrigType::None))
            .unwrap_or(false)
    });

    // 2. Check minimum tracks (must keep at least 1)
    let track_count = pattern_signal.with(|p| p.tracks.len());
    if track_count <= 1 {
        return;  // Button should be disabled, but double-check
    }

    // 3. Show confirmation if has data
    if has_data {
        show_confirm_dialog.set(Some(track_idx));  // Opens dialog
    } else {
        do_remove_track(track_idx);
    }
};

fn do_remove_track(track_idx: usize) {
    set_pattern_signal.update(|pattern| {
        pattern.tracks.remove(track_idx);
        // Re-index remaining tracks
        for (i, track) in pattern.tracks.iter_mut().enumerate() {
            track.id = i;
        }
    });

    // Clear selection if invalid
    if let Some((selected_track, _)) = selected_step.get() {
        if selected_track >= pattern_signal.with(|p| p.tracks.len()) {
            selected_step.set(None);
        }
    }
}
```

**Behavior:**
- Check if track has data (any active steps)
- If yes → show confirmation dialog
- If no → remove immediately
- Re-index all track IDs after removal
- Clear selected step if it becomes invalid

### Track ID Management

After removal, all track IDs are re-indexed to maintain a continuous 0..N sequence. This ensures:
- No gaps in track numbering
- Selected step indices remain valid
- Pattern serialization is clean

---

## User Interactions & UI/UX

### Add Track Button

**Location:** Below the grid, left-aligned
**Styling:** Matches FLUX design - zinc/dark theme, subtle border
**Text:** `+ Add Track`
**Behavior:** Click → instantly adds track, grid expands smoothly

```rust
<button class="mt-3 px-4 py-2 bg-zinc-800 hover:bg-zinc-700 border border-zinc-700
               rounded text-sm text-zinc-300 transition-colors">
    "+ Add Track"
</button>
```

### Remove Track Button

**Location:** Next to each track label (T1 [x], T2 [x], etc.)
**Styling:** Small, subtle `×` icon
**States:**
- **Normal:** Clickable, zinc-600 color
- **Disabled:** When only 1 track remains, grayed out with cursor-not-allowed
- **Hover:** Red tint to indicate destructive action

```rust
<div class="flex items-center gap-1">
    <div class="text-xs text-zinc-400">T{track_idx + 1}</div>
    <button
        class="w-4 h-4 text-zinc-600 hover:text-red-500 disabled:opacity-30
               disabled:cursor-not-allowed transition-colors"
        disabled={move || pattern_signal.with(|p| p.tracks.len() <= 1)}
    >
        "×"
    </button>
</div>
```

### Confirmation Dialog

**Trigger:** When removing a track that has active steps
**Message:** "Track {N} has active steps. Remove anyway?"
**Buttons:**
- `Cancel` (zinc, default focus) - ESC key closes
- `Remove Track` (red, destructive) - Confirm action

**Behavior:**
- Click outside dialog → closes (same as Cancel)
- ESC key → closes
- Modal overlay with dark backdrop

```rust
<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-zinc-900 border border-zinc-700 rounded-lg p-6 max-w-sm">
        <h3 class="text-lg font-medium mb-2">"Confirm Removal"</h3>
        <p class="text-sm text-zinc-400 mb-4">
            "Track " {track_num} " has active steps. Remove anyway?"
        </p>
        <div class="flex gap-2 justify-end">
            <button class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 rounded text-sm">
                "Cancel"
            </button>
            <button class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm">
                "Remove Track"
            </button>
        </div>
    </div>
</div>
```

---

## Error Handling & Edge Cases

### 1. Minimum Track Enforcement (1 track)
- Remove button disabled when `tracks.len() == 1`
- Visual feedback: grayed out, cursor-not-allowed
- No error message needed (UI makes it obvious)

### 2. Selected Step Becomes Invalid
- **Scenario:** User has step selected on Track 4, then removes Track 2
- **Solution:** Check if selected track index >= new track count, deselect if invalid

```rust
if let Some((track_idx, step_idx)) = selected_step.get() {
    if track_idx >= pattern.tracks.len() {
        selected_step.set(None);  // Deselect
    }
}
```

### 3. Playhead Position with Dynamic Tracks
- Already handled: playhead position is step-based (0-15), not track-based
- Audio engine iterates over whatever tracks exist in the pattern
- No changes needed ✅

### 4. Track Removal During Playback
- **Behavior:** Allow it! The audio engine handles it gracefully
- On next playback event, `triggered_tracks` Vec will be shorter
- Frontend adjusts automatically (reactive)

### 5. Backend Synchronization Timing
- Pattern changes are immediate in frontend
- Backend gets updated pattern on next:
  - Playback start/resume
  - Pattern save operation
- **Minor risk:** User adds tracks, plays immediately → backend might use old pattern for one cycle
- **Mitigation:** Accept this minor timing issue (adds complexity to fix)

### 6. Maximum Practical Tracks (Performance)
- No hard limit enforced
- Rendering 100+ tracks may degrade performance
- Let users discover limits organically (not implementing soft warning for MVP)

### 7. Empty Track Validation
- New tracks start with empty steps (all TrigType::None)
- This is valid and expected ✅

### 8. Graceful Degradation (Browser Mode)
- Track controls work in browser mode (no Tauri)
- Pattern mutations are pure frontend operations
- Saving won't work, but in-memory editing does

---

## Testing Strategy

### Manual Testing Checklist

#### Add Track Functionality
- [ ] Click `[+ Add Track]` → new track appears instantly
- [ ] New track labeled correctly (T5, T6, etc.)
- [ ] New track has 16 empty steps
- [ ] New track has OneShot machine type by default
- [ ] Grid expands smoothly without layout glitches
- [ ] Can add many tracks (test with 10, 20, 50)

#### Remove Track Functionality
- [ ] Click `[x]` on empty track → removes immediately (no confirmation)
- [ ] Click `[x]` on track with data → shows confirmation dialog
- [ ] Dialog Cancel button → track not removed
- [ ] Dialog Remove button → track removed
- [ ] ESC key closes dialog
- [ ] Click outside dialog → closes (same as Cancel)
- [ ] Track IDs re-indexed correctly after removal
- [ ] Remove button disabled when only 1 track remains

#### State Consistency
- [ ] Remove track while step selected → selection cleared if invalid
- [ ] Remove track during playback → continues playing, no crash
- [ ] Add track during playback → new track plays on next loop
- [ ] Pattern saves/loads with dynamic track count
- [ ] PlaybackState triggered_tracks Vec matches track count

#### Edge Cases
- [ ] Add 1 track → total 5 tracks
- [ ] Remove middle track (T3) → T4 becomes T3, IDs correct
- [ ] Remove first track (T1) → all tracks shift up
- [ ] Remove last track → no issues
- [ ] Rapid add/remove clicks → no race conditions

#### Desktop vs Browser Mode
- [ ] Desktop mode: Full functionality works
- [ ] Browser mode: UI works, playback fails gracefully (expected)

### Verification Commands

```bash
# Desktop mode (full functionality)
npm run dev

# Browser mode (UI only, no playback)
trunk serve
```

### Automated Tests (Optional)

- Unit test: Track has data detection logic
- Unit test: Track ID re-indexing after removal
- Integration test: Add/remove operations update pattern correctly

---

## Implementation Files

### Files to Create
1. `src/ui/components/track_controls.rs` - Add track button component
2. `src/ui/components/remove_track_button.rs` - Remove track button component
3. `src/ui/components/confirm_dialog.rs` - Reusable confirmation dialog

### Files to Modify
1. `src/ui/state.rs` - Change `triggered_tracks` from `[bool; 4]` to `Vec<bool>`
2. `src/ui/components/grid.rs` - Make track iteration dynamic, add controls
3. `src/ui/components/mod.rs` - Export new components
4. `src/shared/models.rs` - Change Pattern default from 16 to 4 tracks
5. `src-tauri/src/lib.rs` - Change AudioSnapshot `triggered_tracks` type
6. `src-tauri/src/commands.rs` - Update any pattern handling code (if needed)
7. `src/app.rs` - Update AudioSnapshot type in event listener

---

## Future Enhancements (Out of Scope)

- Keyboard shortcuts (Ctrl/Cmd + T to add track)
- Track reordering (drag-and-drop)
- Track duplication (clone track with all data)
- Track mute/solo buttons
- Track color coding
- Soft warning at 32+ tracks for performance
- Undo/redo for track operations

---

## Approval

**Design Status:** ✅ Approved
**Approved By:** User
**Date:** 2026-02-14

**Next Step:** Create implementation plan using `writing-plans` skill.
