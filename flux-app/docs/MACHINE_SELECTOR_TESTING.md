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
