# Machine Selector Manual Testing Checklist

**Date:** 2026-02-14
**Implementation Status:** ✅ Complete
**Final Code Review:** A+ (95/100) - Production ready
**Final Build:** 747286e (Component reordering - × on left)

**Test This Implementation:**
- Final layout: [×] [T1] [OS ▾]
- Flexible width layout (no cramping)
- Enhanced × button with hover states
- Proper memory cleanup (no leaks)

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
- [ ] Layout order is [×] [T1] [OS ▾] (× on left)
- [ ] Track label has fixed width (w-6) for alignment
- [ ] Remove button (×) shows hover state: red-400 text + red-500/10 background
- [ ] No layout overflow or cramping with all components visible
- [ ] Container uses justify-start alignment (not centered)

## Edge Cases

- [ ] Rapid clicking button → no visual glitches
- [ ] Click button 10x fast → state stable
- [ ] Open multiple dropdowns → all work independently
- [ ] Change machine during playback → playback continues

---

## Notes

[Add any issues, bugs, or observations here]
