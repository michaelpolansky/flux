# Grid UI/UX Enhancements Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:writing-plans to create implementation plan from this design.

**Goal**: Enhance the FLUX sequencer grid with comprehensive visual polish, clear selection feedback, and dynamic playback visualization through a state-first architecture.

**Architecture**: State-first approach - build robust state management layer for playback, selection, and UI state, then create visual components that purely react to that state.

**Tech Stack**: Leptos 0.7 (reactive UI), Tailwind CSS v4 (styling), Rust signals (state management)

---

## 1. State-First Architecture Overview

### Core Principle
Build a comprehensive state system that models all grid interactions and playback, then create visual components that purely react to that state.

### Why This Matters for a Sequencer
- Playback state needs to be frame-accurate and independent of UI rendering
- Selection, playback position, and trigger events need to coordinate without conflicts
- Future features (undo/redo, pattern recording, automation) will need this state foundation

### State Architecture Layers

**Layer 1: Domain State** (already exists)
- `Pattern` - Contains tracks, subtracks, steps, triggers
- `SequencerState` - Current step, selected step

**Layer 2: Playback State** (new)
- `PlaybackState` signal containing:
  - `is_playing: bool` - Transport state
  - `current_position: usize` - Which step (0-15) is playing
  - `current_track_triggers: Vec<bool>` - Which tracks triggered this frame (for animations)

**Layer 3: UI State** (new)
- `GridUIState` signal containing:
  - `hovered_step: Option<(usize, usize)>` - Track hover for better feedback
  - `recently_triggered: HashMap<(usize, usize), Instant>` - Track recent triggers for pulse animations
  - `selection_mode: SelectionMode` - Future: could support multi-select, range select, etc.

### State Flow
```
Audio Engine → PlaybackState → Grid Visual Layer
User Interaction → SequencerState + GridUIState → Grid Visual Layer
```

All states are separate reactive signals that the grid component observes, making each concern independent and testable.

---

## 2. State Management Implementation

### New Signals to Create

**1. PlaybackState Signal**
```rust
#[derive(Clone, Debug)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_position: usize,        // 0-15
    pub triggered_tracks: [bool; 4],    // Which tracks fired this step
}
```
- Created in `app.rs` and provided via context
- Updated by Tauri events from audio engine (already have `playback-status` event)
- Grid component observes this reactively

**2. GridUIState Signal**
```rust
#[derive(Clone, Debug, Default)]
pub struct GridUIState {
    pub hovered_step: Option<(usize, usize)>,
    pub recent_triggers: Vec<TriggerEvent>,  // Last N trigger events for animations
}

#[derive(Clone, Debug)]
pub struct TriggerEvent {
    pub track: usize,
    pub step: usize,
    pub timestamp: f64,  // For animation timing
}
```
- Created in `grid.rs` component (local to grid)
- Updated by hover events and playback triggers
- Drives pulse animations

### State Update Flow

**Playback Updates**:
- Audio engine sends event → Tauri listener → Update `PlaybackState` signal → Grid rerenders affected cells

**Selection Updates**:
- User clicks step → Update `SequencerState.selected_step` → Grid shows selection ring + step badge

**Hover Updates**:
- Mouse enters/leaves step → Update `GridUIState.hovered_step` → Grid shows hover state

**Trigger Animations**:
- `PlaybackState.current_position` changes + `Pattern` has active step → Add to `GridUIState.recent_triggers` → Pulse animation plays → Remove after animation completes

**Key Benefit**: Each state concern is isolated. Playback doesn't interfere with selection. Animations don't block interactions. Everything is reactive and declarative.

---

## 3. Visual Specification

### Visual States per Step

Each step button can be in multiple states simultaneously:
- **Base**: Inactive step (empty)
- **Active**: Has a trigger/note
- **Selected**: User is editing this step
- **Playing**: Playback head is on this step
- **Triggered**: Just fired (pulse animation)
- **Hovered**: Mouse over

### Color Palette
- **Inactive**: `bg-zinc-800` → `bg-zinc-700` (hover)
- **Active**: `bg-blue-500` → `bg-blue-400` (hover)
- **Selected Ring**: `ring-amber-400 ring-3` (distinct from blue, clear visual priority)
- **Playing Indicator**: `bg-emerald-500/30` (subtle green underlay, doesn't override active color)
- **Trigger Pulse**: Scale animation + brief `ring-white/50` flash
- **Hover**: Slight scale `scale-105` + brightness increase

### Beat Grouping Markers
- Every 4th step: `border-r-2 border-zinc-600` (subtle vertical separator)
- Creates visual rhythm: `●●●● | ●●●● | ●●●● | ●●●●`

### Selection Enhancement
- **Stronger Ring**: `ring-3` with amber color (upgraded from `ring-2 ring-blue-400`)
- **Step Info Badge**: Floating badge above selected step showing "T2・S5" (track, step)
  - Positioned absolute, slight offset above button
  - `bg-zinc-900/90 backdrop-blur text-amber-400 text-xs px-2 py-0.5 rounded`
  - Fades in with animation

### Playback Visualization

1. **Playhead Indicator**: Vertical bar spanning all 4 tracks
   - `absolute` positioned column overlay
   - `bg-emerald-500/20 w-10 h-full` (matches button width)
   - Smooth transition as it moves: `transition-transform duration-100`

2. **Playing Step Highlight**: Step under playhead gets subtle green tint
   - Layered under active/selection states
   - `bg-emerald-500/30`

3. **Trigger Animation**: When step fires
   - Brief scale pulse: `animate-pulse-once` (custom animation)
   - Flash ring: `ring-2 ring-white/50` for 100ms

---

## 4. Component Structure

### Modified Components

**1. `app.rs`** - Add new state contexts
- Create `PlaybackState` signal (initialized with defaults)
- Provide via context alongside existing `SequencerState` and `Pattern`
- Update existing Tauri listener to populate `PlaybackState` from audio events

**2. `grid.rs`** - Enhanced grid component
- Consume all three state contexts: `SequencerState`, `Pattern`, `PlaybackState`
- Create local `GridUIState` for UI-only concerns (hover, trigger animations)
- Render step buttons with combined state classes
- Render playhead overlay component
- Render step info badge when step selected

### New Components

**1. `PlayheadIndicator` component**
```rust
#[component]
pub fn PlayheadIndicator(
    #[prop(into)] position: Signal<usize>,
    #[prop(into)] is_playing: Signal<bool>,
) -> impl IntoView
```
- Absolute positioned vertical bar
- Transforms based on position (0-15 → pixel offset)
- Only visible when `is_playing` is true
- Smooth transition between positions

**2. `StepBadge` component**
```rust
#[component]
pub fn StepBadge(
    #[prop(into)] track: Signal<usize>,
    #[prop(into)] step: Signal<usize>,
    #[prop(into)] visible: Signal<bool>,
) -> impl IntoView
```
- Shows "T{track}・S{step}" format
- Positioned absolutely above selected step
- Fade in/out animation based on `visible`
- Uses consistent styling with form controls

**3. `GridStep` component** (extract from current inline view)
```rust
#[component]
pub fn GridStep(
    track_idx: usize,
    step_idx: usize,
) -> impl IntoView
```
- Encapsulates all step state logic (active, selected, playing, triggered)
- Computes combined CSS classes from multiple states
- Handles click events
- Handles hover events (updates GridUIState)
- Cleaner than current inline closure

### Component Hierarchy
```
Grid
├── Track Labels (existing)
├── PlayheadIndicator (new)
├── Grid Tracks Container
│   └── For each track
│       └── For each step
│           └── GridStep (extracted)
└── StepBadge (new, conditionally rendered)
```

### File Organization
- Keep existing files
- Add `grid_step.rs` for extracted step component
- Add `playhead_indicator.rs` for playhead
- Add `step_badge.rs` for selection badge (or keep inline in `grid.rs` if small)

---

## 5. Technical Considerations

### Performance Optimizations

1. **Reactive Granularity**
   - Each step button observes only its own state (not entire grid)
   - Use `Signal::derive` for step-specific computations
   - Playhead movement only triggers rerender of playhead component + affected steps

2. **Animation Performance**
   - Use CSS transforms (not position changes) for playhead movement
   - Trigger pulse uses `scale` transform (GPU-accelerated)
   - Clean up old trigger events from `recent_triggers` after animation completes (prevent memory leak)

3. **Memo for Expensive Computations**
   - If checking active state becomes expensive, use `Memo` instead of closures

### State Synchronization

1. **Playback State from Audio Engine**
   - Already receiving `playback-status` events with `current_step`
   - Extend to include `triggered_tracks` information (may need backend update)
   - Handle rapid updates (audio runs at ~60fps, UI doesn't need every frame)

2. **Trigger Animation Timing**
   - When `PlaybackState.current_position` changes AND step is active → add to `recent_triggers`
   - Use `requestAnimationFrame` or Leptos `create_effect` for cleanup after 150ms
   - Edge case: Rapid tempo changes shouldn't stack animations

### Visual Edge Cases

1. **Multiple Simultaneous States**
   - Step can be: active + selected + playing + triggered simultaneously
   - Visual priority (CSS order matters):
     - Trigger flash (top layer, brief)
     - Selection ring (persistent, high visibility)
     - Playing highlight (subtle underlay)
     - Active color (base state)

2. **Playhead at Grid Edge**
   - Position 15 → 0 transition should be smooth
   - Consider wrapping animation vs instant jump

3. **Responsive Spacing**
   - Beat grouping markers need to work at different grid sizes
   - Step badges shouldn't overflow or clip

### Tailwind Configuration

Need to add custom animations to `tailwind.config.js`:
```js
{
  animation: {
    'pulse-once': 'pulse 0.15s ease-out',
  }
}
```

### Testing Approach

1. **State Testing**
   - Test state transitions independently
   - Mock audio events to verify PlaybackState updates

2. **Visual Testing**
   - Manual testing with running audio
   - Verify all state combinations render correctly
   - Check animation timing feels right

3. **Performance Testing**
   - Profile with React DevTools / browser perf tools
   - Ensure 60fps during playback

---

## Implementation Phases

The implementation will follow these phases:

**Phase 1: State Foundation**
- Create `PlaybackState` and `GridUIState` types
- Add state signals to app.rs and grid.rs
- Wire up Tauri event listener to populate PlaybackState
- Test state updates independently

**Phase 2: Visual Refinement**
- Implement color palette changes
- Add beat grouping markers
- Enhance hover states
- Add stronger selection ring (amber, ring-3)

**Phase 3: New Components**
- Extract GridStep component
- Create PlayheadIndicator component
- Create StepBadge component
- Wire up to state signals

**Phase 4: Playback Visualization**
- Implement playhead movement
- Add playing step highlight
- Implement trigger pulse animations
- Add animation cleanup logic

**Phase 5: Polish & Testing**
- Add custom Tailwind animations
- Test all state combinations
- Performance profiling
- Edge case handling

---

## Success Criteria

- Grid clearly shows which step is selected (amber ring + badge)
- Playback position is instantly visible (emerald playhead + step highlight)
- Steps pulse when triggered (white flash animation)
- Beat groupings provide visual rhythm (every 4 steps)
- All interactions feel smooth and responsive (60fps)
- State management is clean and extensible for future features
