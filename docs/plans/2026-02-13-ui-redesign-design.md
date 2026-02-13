# Flux UI Redesign - Design Document

**Date:** 2026-02-13
**Status:** Approved
**Design Approach:** Ableton-Inspired Professional Layout

## Executive Summary

Redesign the Flux sequencer UI to address visual polish and information density issues. The new design follows Ableton Live's layout principles: clean hierarchy, professional aesthetics, and purpose-built for music production workflows.

**Key Goals:**
- Improve visual polish (professional, not generic)
- Optimize information density (clear, not cluttered)
- Maintain current Tailwind setup (no new dependencies)
- Create reusable design system for future features

## Design Rationale

### Why This Approach?

**Chosen:** Improve current Tailwind setup with systematic design principles
**Rejected:** Adding component library (leptonic, shadcn ports)

**Reasoning:**
1. Specialized sequencer UI doesn't benefit from general-purpose component libraries
2. Zero risk - no breaking changes or new dependencies
3. Full control over music-specific widgets (step grids, waveform editors)
4. Faster iteration without learning new APIs

### Reference Inspiration

**Primary:** Ableton Live
- Clean visual hierarchy
- Dark theme with bright accent colors
- Everything visible, no hidden tabs
- Professional without being overwhelming

**Secondary:** Elektron hardware
- Dense, functional layouts
- Grid-focused design
- Clear state indicators

## Section 1: Layout & Structure

### Overall Layout

- **Container:** `max-w-7xl` (wider viewport usage)
- **Padding:** `p-6` (consistent, comfortable)
- **Vertical spacing:** `gap-6` between major sections

### Three Main Zones

**1. Header (Fixed: ~60px)**
- Horizontal layout: Logo left, controls center/right
- Background: `bg-zinc-900` (darker separation)
- Border: `border-b border-zinc-800`

**2. Grid Section (Flexible, Primary Focus)**
- Background: `bg-zinc-900/50` (subtle card)
- Padding: `p-6`
- Border radius: `rounded-lg`
- Takes majority of vertical space

**3. Parameters Section (Fixed: ~200px)**
- Background: `bg-zinc-900/50` (matches grid)
- Padding: `p-6`
- 2 rows × 4 columns = 8 visible controls

### Spacing Scale

Consistent rhythm throughout:
- `gap-2` (8px) - within component groups
- `gap-4` (16px) - between related elements
- `gap-6` (24px) - between major sections
- Never `gap-8` (too loose for music tools)

## Section 2: Color System

### Background Layers

- **Main BG:** `bg-zinc-950` (nearly black, like Ableton)
- **Card/Panel BG:** `bg-zinc-900/50` (subtle elevation)
- **Header BG:** `bg-zinc-900` (solid boundary)
- **Interactive BG:** `bg-zinc-800` (buttons, inputs)

### Accent Colors

High contrast against dark backgrounds:
- **Primary (Active/Playing):** `bg-amber-500` (warm, Ableton-orange inspired)
- **Secondary (Selected):** `bg-blue-500` (cool, selection indicator)
- **Success (Armed):** `bg-green-500` (ready states)
- **Danger (Stop):** `bg-red-500` (warnings, delete)

### Text Hierarchy

- **Primary:** `text-zinc-50` (labels, values)
- **Secondary:** `text-zinc-400` (descriptions)
- **Tertiary:** `text-zinc-600` (subtle hints)
- **Disabled:** `text-zinc-700` (barely visible)

### Interactive States

- **Hover:** `hover:bg-zinc-700` (subtle brighten)
- **Active/Pressed:** `bg-zinc-700`
- **Focus ring:** `ring-2 ring-blue-500 ring-offset-2 ring-offset-zinc-950`

### Grid Step States

- **Empty:** `bg-zinc-800 text-zinc-600` (recessed)
- **Has Note:** `bg-amber-500 text-black` (bright)
- **Playing:** `bg-amber-300 text-black scale-110` (highlighted)
- **Selected:** `ring-2 ring-blue-500` (blue outline)

## Section 3: Component Designs

### A) Header Component

```
┌─────────────────────────────────────────────────────────┐
│ FLUX              [SAVE] [LOAD]    120 BPM   [▶] [■] [●]│
│ Audio Engine                                            │
└─────────────────────────────────────────────────────────┘
```

**Structure:**
- Left: Title stack
  - Title: `text-xl font-bold`
  - Subtitle: `text-xs text-zinc-500`
- Center: File operations (`gap-2`)
- Right: BPM display + transport (`gap-4`)
- Height: `h-16` (64px)
- Buttons: `h-10 px-4 text-sm font-medium rounded-md`
- Play: `bg-green-600 hover:bg-green-500`
- Stop: `bg-zinc-800 hover:bg-zinc-700`

### B) Grid Component

```
┌─────────────────────────────────────────┐
│ TRACK 1 - LEAD SYNTH            PATTERN│
├─────────────────────────────────────────┤
│ [1][2][3][4][5][6][7][8]               │
│ [9][10][11][12][13][14][15][16]        │
└─────────────────────────────────────────┘
```

**Structure:**
- Header: Track name (left), pattern indicator (right)
  - Text: `text-sm font-medium`
- Grid: `grid grid-cols-8 gap-3`
  - 2 rows × 8 columns layout
- Step buttons: `w-16 h-16` (64px, large touch targets)
- Border radius: `rounded-lg`
- Step numbers: `text-xs font-mono`
- Padding: `p-6`

### C) Parameters Component

```
┌─────────────────────────────────────────┐
│ PARAMETERS              [STEP 4 LOCKED] │
├─────────────────────────────────────────┤
│ PITCH    FILTER   RES      DECAY        │
│ [━━━━━]  [━━━━]   [━━]     [━━━━━]     │
│                                         │
│ ATTACK   SUSTAIN  RELEASE  DRIVE       │
│ [━━━━]   [━━━━━]  [━━━━]   [━━]       │
└─────────────────────────────────────────┘
```

**Structure:**
- Header: "PARAMETERS" (left), lock indicator (right)
- Grid: `grid grid-cols-4 gap-x-6 gap-y-4`
  - 2 rows × 4 columns
- Labels: `text-xs font-medium uppercase tracking-wide text-zinc-400`
- Sliders: `h-2 w-full rounded-full bg-zinc-800`
  - Fill: `bg-amber-500`
- Lock indicator: Dot + text (`text-xs font-mono`)
- Padding: `p-6`

## Section 4: Interactive States & Behavior

### Grid Step Interactions

**Click:**
- Toggle note on/off
- Transition: `transition-all duration-100` (snappy)

**Right-click/Long-press:**
- Select step for parameter locking
- Blue ring appears
- Header indicator updates

**During Playback:**
- Current step: `scale-110 bg-amber-300`
- Transition: `transition-transform duration-75`

### Parameter Slider Interactions

**Default Mode (No Step Selected):**
- Label: `text-zinc-400`
- Affects: Track-level defaults
- Track: `bg-zinc-800`

**P-Lock Mode (Step Selected):**
- Label: `text-amber-400` (indicates active lock)
- Affects: Selected step only
- Track: `bg-zinc-700` (lighter, shows active)
- Locked values: Show dot on track

**Slider States:**
- Hover: Thumb `scale-110`
- Dragging: `cursor-grabbing`
- Focus: Blue ring for keyboard navigation

### Transport Controls

**Play Button:**
- Idle: `bg-green-600`
- Hover: `bg-green-500`
- Playing: `bg-green-700` + pulsing ring (`animate-pulse`)

**Stop Button:**
- Idle: `bg-zinc-800`
- Hover: `bg-zinc-700`
- Active: `bg-zinc-600`

### General Polish

- All buttons: `active:scale-95` (tactile feedback)
- Disabled: `opacity-50 cursor-not-allowed`
- Loading: `animate-pulse` on affected areas
- No tooltips initially (clear labels suffice)

## Implementation Strategy

### Phase 1: Core Layout
1. Update App.rs layout structure
2. Implement spacing scale
3. Apply background colors

### Phase 2: Header
1. Redesign header component
2. Consolidate duplicate transport controls
3. Add BPM display

### Phase 3: Grid
1. Update grid layout (2 rows × 8 cols)
2. Larger step buttons (64px)
3. Apply new color states

### Phase 4: Parameters
1. Reorganize into 2×4 grid
2. Update slider styling
3. Add lock indicator

### Phase 5: Polish
1. Add all transitions
2. Implement interactive states
3. Test accessibility (keyboard nav, focus rings)

## Success Criteria

- [ ] Visual hierarchy is clear (grid dominates)
- [ ] Information density feels balanced
- [ ] Professional appearance (Ableton-level polish)
- [ ] All interactive states provide clear feedback
- [ ] No new dependencies added
- [ ] Responsive behavior maintained
- [ ] Accessibility preserved (keyboard, focus)

## Future Considerations

**Not in this redesign:**
- Multi-track view (current: single track focus)
- Pattern management UI
- LFO designer integration
- Settings/preferences panel

These can be added later using the established design system.

## Design System Reference

### Quick Reference

**Spacing:** `gap-2 | gap-4 | gap-6`
**Radius:** `rounded-md | rounded-lg`
**Backgrounds:** `bg-zinc-950 | bg-zinc-900/50 | bg-zinc-800`
**Accents:** `bg-amber-500 | bg-blue-500 | bg-green-500`
**Text:** `text-zinc-50 | text-zinc-400 | text-zinc-600`

This design system can be extracted into Tailwind config for consistency across future components.
