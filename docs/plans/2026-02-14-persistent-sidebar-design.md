# Persistent Sidebar with Smart Empty State

**Date:** 2026-02-14
**Status:** Approved
**Approach:** Simple Content Swap (Approach 1)

## Problem Statement

The current step editor sidebar has a very basic empty state that simply displays "Select a step to edit parameters" when no step is selected. This is a missed opportunity to provide useful information about the pattern while maintaining the persistent sidebar structure.

## Solution Overview

Replace the basic empty state with a comprehensive track summary table that shows an overview of all tracks in the pattern, including their machine types, active step counts, and P-Lock counts. This provides users with valuable pattern-level information while maintaining the existing sidebar structure and interaction model.

---

## Design Decisions

### Requirements Gathered

Through collaborative design discussion, the following requirements were established:

1. **Information Type:** Track-level overview (not quick actions, not track defaults, not help text)
2. **Track Scope:** Show all tracks in a summary view (not just one track)
3. **Detail Level:** Display track number, machine type, active steps, and P-Lock count
4. **Interactivity:** Read-only display (no click actions)
5. **Visual Layout:** Table format with structured columns
6. **Additional Content:** Just the table (no hints, shortcuts, or pattern stats)

### Approach Selection

**Chosen: Simple Content Swap**

The sidebar structure remains unchanged. When no step is selected, show the track summary table. When a step is selected, show the existing step editor content.

**Rationale:**
- Aligns with current design (sidebar already swaps content based on selection)
- Clean implementation with minimal changes to existing architecture
- Fits the use case (users either want overview OR detailed editing)
- Quick to build and iterate on

**Alternatives Considered:**
- Unified header with content panels (more complex, heavier refactoring)
- Split-view sidebar (less space per section, potentially cramped)

---

## Component Structure & Layout

### Current Structure
```rust
view! {
    <div class="w-80 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4">
        {move || {
            if let Some((track_id, step_idx)) = selected_step.get() {
                // Step editor content (existing)
            } else {
                // Simple empty state (current: "Select a step...")
            }
        }}
    </div>
}
```

### New Structure
```rust
view! {
    <div class="w-80 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col">
        {move || {
            if let Some((track_id, step_idx)) = selected_step.get() {
                // UNCHANGED: Existing step editor with collapsible sections
            } else {
                // NEW: Track summary table
                view! {
                    <div class="flex flex-col h-full">
                        <div class="mb-4">
                            <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                                "PATTERN OVERVIEW"
                            </h3>
                        </div>

                        <div class="flex-1">
                            <table class="w-full text-sm">
                                <thead>
                                    <tr class="text-xs text-zinc-500 border-b border-zinc-800">
                                        <th class="text-left pb-2">Track</th>
                                        <th class="text-left pb-2">Machine</th>
                                        <th class="text-right pb-2">Steps</th>
                                        <th class="text-right pb-2">P-Locks</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    // Dynamic rows from pattern data
                                </tbody>
                            </table>
                        </div>
                    </div>
                }
            }
        }}
    </div>
}
```

**Key Points:**
- Sidebar container unchanged (same width, background, borders)
- Header styling matches existing "EDITING STEP" header for consistency
- Table structure: 4 columns (Track, Machine, Steps, P-Locks)
- Uses existing design tokens (text-zinc-400, border-zinc-800)

---

## Data Flow & Calculations

### Data Source
All statistics derived from existing `pattern_signal`, which contains all track data.

### Track Statistics Calculated

1. **Track Number**
   Simple iteration index (T1, T2, T3, T4...)

2. **Machine Type**
   Read from `track.machine_type.to_string()`

3. **Active Steps Count**
   Count steps where `trig_type != TrigType::None`
   ```rust
   let active_steps = track.subtracks.get(0)
       .map(|st| st.steps.iter()
           .filter(|s| s.trig_type != TrigType::None)
           .count())
       .unwrap_or(0);
   ```

4. **P-Lock Count**
   Total P-Locks across all steps in track
   ```rust
   let p_lock_count = track.subtracks.get(0)
       .map(|st| st.steps.iter()
           .map(|s| s.p_locks.iter().filter(|p| p.is_some()).count())
           .sum::<usize>())
       .unwrap_or(0);
   ```

### Reactive Updates
- Table rendered inside `move || { ... }` closure that reacts to `selected_step`
- Also reacts to `pattern_signal` changes (steps toggled, P-Locks added/removed)
- Iterating over `pattern_signal.with(|p| p.tracks)` triggers automatic re-render on pattern changes

### Performance
- O(n) calculations where n = number of steps (typically 16 per track)
- With 4 tracks × 16 steps = 64 iterations maximum
- Very lightweight, no performance concerns
- Only calculated when sidebar in empty state (not when step selected)

---

## Visual Design Specifications

### Table Layout
```
┌─────────────────────────────────────────────┐
│ PATTERN OVERVIEW                             │
│                                              │
│ Track   Machine      Steps   P-Locks        │
│ ─────   ────────     ─────   ───────        │
│ T1      Lead Synth      5       12          │
│ T2      OneShot         3        0          │
│ T3      OneShot         0        0          │
│ T4      OneShot         8       24          │
└─────────────────────────────────────────────┘
```

### Header Section
- Text: "PATTERN OVERVIEW"
- Font: `text-xs font-bold uppercase tracking-wide`
- Color: `text-zinc-400` (matches "EDITING STEP" header)
- Spacing: `mb-4` (16px bottom margin)

### Table Header Row
- Font: `text-xs` (12px)
- Color: `text-zinc-500` (dimmer than data)
- Border: `border-b border-zinc-800`
- Padding: `pb-2` (8px bottom padding)
- Alignment: Left for Track/Machine, Right for Steps/P-Locks

### Table Data Rows
- Font: `text-sm` (14px) - slightly larger than header
- Color: `text-zinc-100` (primary text)
- Spacing: `py-2` (8px vertical padding per row)
- Hover: `hover:bg-zinc-800/30` (subtle highlight)
- Border: `border-b border-zinc-800/50` (row separator)

### Column Layout
- **Track:** Auto width (narrow, just "T1", "T2")
- **Machine:** Flex-1 (takes remaining space), `truncate` for long names
- **Steps:** Fixed right-aligned (~60px)
- **P-Locks:** Fixed right-aligned (~60px)

### Special States

**Zero Values:**
When track has 0 steps or 0 P-Locks, dim the color:
- Normal: `text-zinc-100`
- Zero: `text-zinc-600` (de-emphasized)

**Empty Pattern (0 tracks):**
```rust
<tr>
    <td colspan="4" class="text-center py-8 text-zinc-500 text-sm italic">
        "No tracks in pattern"
    </td>
</tr>
```

---

## Implementation Details

### Files to Modify
**Only one file:** `flux-app/src/ui/components/step_editor_sidebar.rs`

### Changes Required

#### 1. Add Helper Function
Add after imports, before component:
```rust
/// Calculate track statistics (active steps, P-Lock count)
fn calculate_track_stats(track: &crate::shared::models::Track) -> (usize, usize) {
    let active_steps = track.subtracks.get(0)
        .map(|st| st.steps.iter()
            .filter(|s| s.trig_type != crate::shared::models::TrigType::None)
            .count())
        .unwrap_or(0);

    let p_lock_count = track.subtracks.get(0)
        .map(|st| st.steps.iter()
            .map(|s| s.p_locks.iter().filter(|p| p.is_some()).count())
            .sum::<usize>())
        .unwrap_or(0);

    (active_steps, p_lock_count)
}
```

#### 2. Replace Empty State Block
Replace current empty state (around lines 592-599) with track summary table.

See implementation code in Section 4 of design presentation for full table rendering logic.

### Edge Cases Handled
- **Empty pattern (0 tracks):** Shows "No tracks in pattern" message
- **Track with 0 steps:** Shows "0" in dimmed color (`text-zinc-600`)
- **Track with 0 P-Locks:** Shows "0" in dimmed color
- **Long machine names:** Uses `truncate` class to prevent overflow

---

## Testing Strategy

### Manual Testing Scenarios
1. **Empty state display:**
   - No step selected → should show track summary table
   - Select a step → should show step editor
   - Deselect step (ESC or click outside) → should return to track summary

2. **Data accuracy:**
   - Toggle steps on/off → active step counts should update
   - Add/remove P-Locks → P-Lock counts should update
   - Verify counts match actual pattern state

3. **Dynamic track changes:**
   - Add new track → should appear in table
   - Remove track → should disappear from table
   - Change machine type → should reflect in table

4. **Visual verification:**
   - Table styling matches existing sidebar aesthetics
   - Zero values appear dimmed
   - Hover states work correctly
   - Header typography matches "EDITING STEP" header

### Edge Case Testing
- Pattern with 0 tracks
- Pattern with many tracks (10+)
- Track with very long machine name
- All tracks with 0 steps
- Mix of active/inactive tracks

---

## Success Criteria

- [x] Design approved by user
- [ ] Empty state shows comprehensive track summary
- [ ] Table displays track number, machine, steps, P-Locks
- [ ] Read-only (no click interactions)
- [ ] Table format with proper column alignment
- [ ] Zero values appear dimmed
- [ ] Reactive to pattern changes
- [ ] Visual design consistent with existing sidebar
- [ ] No layout shifts (sidebar remains w-80)
- [ ] All edge cases handled gracefully

---

## Future Enhancements

Not in this implementation, but potential improvements:

1. **Interactive table rows:** Click to select track for editing
2. **Sorting:** Click column headers to sort tracks
3. **Track filtering:** Show only active tracks, or tracks with P-Locks
4. **Visual indicators:** Mini step activity visualization (dots showing which steps active)
5. **Pattern-level stats:** Total active steps, total P-Locks above table
6. **Machine type icons:** Replace text with visual icons

---

## Notes

- This design maintains the existing sidebar structure and only enhances the empty state
- No changes to step editor functionality or interaction model
- Minimal code changes required (single file, ~50 lines of new code)
- Leverages existing reactive signals and design tokens
- Can be implemented and tested quickly
