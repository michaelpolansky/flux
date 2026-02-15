# FLUX User Guide

Welcome to FLUX! This guide will help you get started with creating beats and patterns using the FLUX sequencer.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Your First Pattern](#your-first-pattern)
3. [Parameter Locking (P-Locks)](#parameter-locking-p-locks)
4. [Advanced Features](#advanced-features)
5. [Keyboard Shortcuts](#keyboard-shortcuts)
6. [Tips & Tricks](#tips--tricks)

---

## Getting Started

### What is FLUX?

FLUX is a powerful step sequencer inspired by classic Elektron hardware like the Octatrack and Digitakt. It brings the beloved workflow of hardware sequencers to your computer with a modern, intuitive interface.

**Key Features:**
- 16-step sequencer with 4 tracks
- Parameter locking for per-step automation
- Real-time pattern editing
- Professional dark interface designed for music production
- Keyboard shortcuts for fast workflow

### Launching the App

**Desktop App (Recommended):**
1. Open your terminal
2. Navigate to the FLUX directory
3. Run `npm run dev`
4. The app will launch automatically with full audio capabilities

**Browser Preview:**
- If you see a yellow banner at the top saying "Preview Mode", you're running in browser mode
- Audio features won't work in preview mode
- To enable audio, run the desktop app using `npm run dev`

### Interface Overview

When you open FLUX, you'll see three main sections:

#### 1. Top Bar (Transport Controls)
Located at the very top, this is where you control playback:
- **PLAY** button - Start the sequencer
- **STOP** button - Stop playback
- **SAVE** button - Save your pattern to a file
- **LOAD** button - Load a previously saved pattern

#### 2. Sequencer Grid (Center)
The heart of FLUX - a 4-track, 16-step grid:
- **4 Tracks** (labeled T1, T2, T3, T4) running vertically
- **16 Steps** running horizontally in two rows (steps 1-8, then 9-16)
- **Blue squares** indicate active steps that will play
- **Gray squares** are inactive/muted steps
- **Orange playhead** shows the current playback position

#### 3. Step Editor Sidebar (Right Side, appears when step selected)
Unified control panel for all step-level editing:
- **Step Properties**: Note (Pitch), Velocity, Length, Probability, Micro-timing
- **Sound Parameters**: 8 synthesis parameters with automatic P-Lock creation
  - Tuning, Filter Freq, Resonance, Drive, Decay, Sustain, Reverb, Delay
  - Badge shows number of active P-Locks for current step
  - P-Locked parameters display in amber color
- **LFO**: Track-level modulation (Shape, Amount, Speed, Destination, Designer)

---

## Your First Pattern

Let's create a simple beat to get you started!

### Step 1: Activate Some Steps

1. **Click on any square** in the grid to activate it
   - The square turns blue when active
   - Click again to deactivate (turns gray)

2. **Try this simple pattern on Track 1 (top row):**
   - Activate steps 1, 5, 9, and 13
   - This creates a basic four-on-the-floor kick pattern

3. **Add a hi-hat pattern on Track 2:**
   - Activate every other step (1, 3, 5, 7, 9, 11, 13, 15)
   - This creates a steady eighth-note hi-hat groove

### Step 2: Start Playback

1. Click the **PLAY** button in the top bar
2. Watch the orange playhead move across the grid
3. Your active steps will light up as they play

### Step 3: Edit While Playing

FLUX lets you edit patterns in real-time:
- Click steps on/off while playback is running
- Changes take effect immediately on the next loop
- This is great for jamming and experimentation!

### Step 4: Save Your Work

1. Click the **SAVE** button
2. Choose a location and filename
3. Your pattern is saved with the `.flux` extension
4. You can load it later using the **LOAD** button

**Tip:** FLUX automatically saves your work to `last_pattern.flux` so you can pick up where you left off!

---

## Parameter Locking (P-Locks)

Parameter locking (or "P-Locks") is one of FLUX's most powerful features. It lets you automate parameter changes on a per-step basis.

### What Are P-Locks?

Imagine you have a hi-hat pattern on Track 2. Normally, all steps would play at the same pitch and filter settings. With P-Locks, you can make step 5 play at a higher pitch, step 9 with more filter resonance, and step 13 with different drive - all automatically!

**Use Cases:**
- Create melody lines by changing pitch on each step
- Add movement by varying filter cutoff throughout the pattern
- Build dynamics by adjusting volume step-by-step
- Make more interesting rhythms with varying decay times

### How to Set P-Locks

#### Step 1: Select a Step
**Click or right-click** on any step in the grid
- The step gets selected (visible in sidebar header)
- The Step Editor Sidebar appears on the right showing:
  - "EDITING STEP" with track and step number
  - Three collapsible sections (Step Properties, Sound Parameters, LFO)
  - A close button (×) to deselect

#### Step 2: Adjust Parameters
With a step selected, expand the "Sound Parameters" section if needed and adjust any parameter:
- P-Locks are created **automatically** when the value differs from the track default
- P-Locks are removed **automatically** when you set the value back to the track default
- The parameter label turns **amber** when P-Locked
- The badge in the section header shows the **total count** of active P-Locks

**Example:**
1. Activate steps 1, 5, 9, 13 on Track 1
2. Right-click step 5
3. Move the "Tuning" slider to +7 semitones
4. Click outside the grid to deselect
5. Right-click step 9
6. Move the "Tuning" slider to +12 semitones
7. Press PLAY - steps 1 and 13 play at normal pitch, but steps 5 and 9 play higher!

### Viewing Locked Parameters

**How to tell if a step has P-Locks:**
- The badge in the "Sound Parameters" section header shows the count (e.g., "SOUND PARAMETERS (3)")
- P-Locked parameter labels appear in **amber color** instead of gray
- When you select the step, locked parameters show their step-specific values
- Non-locked parameters show the track default values

**Track Default Mode:**
- When no step is selected, you're in "Track Default" mode
- The status shows "TRACK DEFAULT"
- Any changes you make affect all steps that don't have P-Locks

### Clearing P-Locks

To remove parameter locks from a step:
1. Right-click the step to select it
2. Move the sliders back to match the track default values
3. The P-Lock is removed for that parameter

**Quick Tip:** To see the track default value, click outside the grid to deselect all steps, then check the slider positions.

### Deselecting Steps

There are several ways to deselect a step:
- **Click anywhere outside the grid** - clears selection
- **Press ESC key** - clears selection
- **Right-click a different step** - selects the new step instead

---

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

---

## Advanced Features

### LFO Designer (Coming Soon)

The LFO (Low Frequency Oscillator) Designer will let you draw custom modulation waveforms to automatically control parameters over time. This feature is currently in development.

### Multi-Track Patterns

FLUX supports 4 independent tracks that play simultaneously:
- **Track 1 (T1)** - Often used for kick drums
- **Track 2 (T2)** - Often used for hi-hats or snares
- **Track 3 (T3)** - Melodic elements or percussion
- **Track 4 (T4)** - Bass lines or additional sounds

Each track has its own set of 16 steps and independent parameter defaults.

### BPM Control

The tempo of your pattern is displayed in the top bar:
- Default is 120 BPM
- You can adjust the tempo to match your desired speed
- Changes take effect immediately during playback

### Pattern Length

All patterns in FLUX are currently 16 steps long:
- At 120 BPM, this equals one bar of 16th notes
- Higher BPM = faster playback
- Lower BPM = slower, more relaxed groove

---

## Keyboard Shortcuts

Master these shortcuts to speed up your workflow:

### Navigation
- **Tab** - Move focus between controls
- **Arrow Keys** - Navigate through buttons and adjust sliders
- **Shift + Tab** - Move focus backwards

### Grid Controls
- **Left Click** - Toggle step on/off
- **Right Click** - Select step for parameter locking
- **ESC** - Clear step selection

### Transport
- **Space** or **Enter** - Activate focused button (Play/Stop/Save/Load)

### Parameter Adjustment
- **Left/Right Arrow Keys** - Adjust selected slider in small increments
- **Up/Down Arrow Keys** - Navigate between sliders
- **Click and drag** - Adjust slider with mouse

### Quick Actions
- **ESC** - Close step editor sidebar / deselect step
- **Click × button** - Close sidebar and deselect step
- **Click outside grid** - Deselect current step

---

## Tips & Tricks

### Creative Workflow Tips

**1. Start Simple, Add Complexity**
- Begin with a basic kick pattern (steps 1, 5, 9, 13)
- Add hi-hats on the off-beats
- Use P-Locks to add subtle variations

**2. Use P-Locks for Melody**
- Activate every step on a track
- Right-click each step and adjust the Tuning parameter
- Create melodies by setting different pitches per step

**3. Build Dynamics**
- Use P-Locks on the Decay parameter for rhythmic variation
- Long decay on some steps, short on others
- Creates a more "human" feel

**4. Live Jamming**
- Keep playback running while you edit
- Toggle steps on/off to build up your pattern gradually
- Changes take effect on the next loop

**5. Save Often**
- Use the SAVE button regularly
- Create variations by saving different versions
- FLUX auto-saves to `last_pattern.flux` but manual saves give you more control

### Understanding the Visual Feedback

**Step Colors:**
- **Gray** - Inactive step (won't play)
- **Blue** - Active step (will trigger)
- **Blue with outline** - Selected step (editing P-Locks)
- **Orange flash** - Step currently playing

**Status Indicators:**
- **Blue dot** - Step is selected for P-Lock editing
- **Gray dot** - No step selected (Track Default mode)

**Playhead:**
- **Orange vertical line** - Shows current playback position
- Moves smoothly from left to right across the grid
- Helps you visualize timing and sync

### Common Patterns

**Four-on-the-Floor Kick:**
- Track 1: Steps 1, 5, 9, 13

**Eighth-Note Hi-Hat:**
- Track 2: Steps 1, 3, 5, 7, 9, 11, 13, 15

**Snare on 2 and 4:**
- Track 3: Steps 5, 13

**Bass Line (with P-Locks):**
- Track 4: Activate multiple steps
- Use Tuning P-Locks to create a melody

### Troubleshooting

**I don't hear any sound:**
- Check if you see the yellow "Preview Mode" banner
- If yes, you need to run the desktop app: `npm run dev`
- Browser preview mode doesn't include audio

**My changes aren't saving:**
- Make sure you click the SAVE button
- Choose a location and filename when prompted
- Check for error messages in your browser's console

**Steps won't activate:**
- Make sure you're clicking directly on the step squares
- Active steps should turn blue
- If a step stays gray, try clicking again

**P-Locks aren't working:**
- Make sure you **right-click** the step first (to select it)
- Check that the status shows "TRACK X, STEP Y LOCKED"
- The blue dot should be lit when a step is selected

**Playback is stuck:**
- Click the STOP button
- Wait a moment, then click PLAY again
- If issues persist, reload the app

---

## Next Steps

Now that you know the basics:

1. **Experiment freely** - FLUX is designed for exploration
2. **Try combining P-Locks** - Lock multiple parameters per step
3. **Build complete patterns** - Use all 4 tracks together
4. **Save your favorites** - Build a library of patterns
5. **Learn the shortcuts** - Speed up your workflow with keyboard controls

For technical details and development information, see:
- `ARCHITECTURE.md` - System design and technical architecture
- `DEVELOPER_GUIDE.md` - Developer documentation

Happy sequencing!

---

**FLUX Version:** 0.1.0
**Last Updated:** 2026-02-14
**For Support:** Check the project documentation or open an issue on GitHub
