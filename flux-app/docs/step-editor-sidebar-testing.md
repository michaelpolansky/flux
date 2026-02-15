# Step Editor Sidebar Testing Checklist

## Empty State
- [ ] Shows 'Select a step to edit parameters'
- [ ] Shows helpful tip text
- [ ] Smooth fade-in animation

## Step Selection
- [ ] Clicking step shows sidebar controls
- [ ] Header displays correct Track/Step number
- [ ] Close button visible
- [ ] Animation smooth

## Parameter Controls
- [ ] Note: 0-127 range works
- [ ] Velocity: 0-127 range works
- [ ] Length: 0.1-4.0 range works
- [ ] Probability: 0-100% range works
- [ ] Micro-timing: -23 to +23 range works

## Interactions
- [ ] Close button deselects step
- [ ] ESC key deselects step
- [ ] Click outside grid deselects step
- [ ] Parameter changes persist

## Layout
- [ ] Sidebar 240px width
- [ ] Grid shifted right appropriately
- [ ] Track labels visible
- [ ] Track controls functional

## Integration
- [ ] Inspector still works (bottom section)
- [ ] No StepInspector in bottom section
- [ ] Grid selection unchanged
- [ ] No console errors

All tests passed: _____ (Date/Time)
