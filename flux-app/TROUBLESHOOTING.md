# FLUX Troubleshooting Guide

This guide covers common issues, solutions, and workarounds for developers and users of the FLUX sequencer.

## Table of Contents

1. [Build & Compilation Issues](#build--compilation-issues)
2. [Audio & MIDI Issues](#audio--midi-issues)
3. [UI & Performance Issues](#ui--performance-issues)
4. [Browser vs Desktop Mode](#browser-vs-desktop-mode)
5. [Known Limitations](#known-limitations)
6. [Getting Help](#getting-help)

---

## Build & Compilation Issues

### Issue: `trunk serve` fails with "wasm32-unknown-unknown not installed"

**Symptom:**
```
error: failed to run custom build command for `flux-app-ui`
error: target `wasm32-unknown-unknown` not installed
```

**Solution:**
Install the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

---

### Issue: `npm run dev` fails with "command not found: tauri"

**Symptom:**
```
sh: tauri: command not found
```

**Solution:**
Install Node.js dependencies:
```bash
npm install
```

The Tauri CLI is installed locally via npm, not globally.

---

### Issue: Leptos compilation errors about deprecated functions

**Symptom:**
```
warning: use of deprecated function `leptos::prelude::create_signal`
warning: use of deprecated function `leptos::prelude::create_effect`
```

**Solution:**
These warnings are **non-critical**. The code still compiles and runs correctly. The Leptos team is transitioning API naming conventions, but old functions remain supported.

To silence warnings (optional):
- Replace `create_signal()` with `signal()`
- Replace `create_effect()` with `Effect::new()`
- Replace `create_node_ref()` with `NodeRef::new()`

---

### Issue: Build errors in `grid.rs` about unexpected tokens

**Symptom:**
```
error: expected one of `!`, `(`, `+`, `::`, `<`, `>`, or `as`, found `class`
  --> src/ui/components/grid.rs:78:29
```

**Solution:**
This is a Leptos **syntax error** in JSX-like view! macros. Check that:
1. All HTML attributes use Leptos syntax: `class=` not `className=`
2. Dynamic classes use `class=move ||` with proper closure syntax
3. All opening tags have matching closing tags
4. No stray characters or unclosed strings

**Example Fix:**
```rust
// WRONG
<button class=move || {
    format!("btn {}", if active { "active" } else { "" })
}

// RIGHT
<button class=move || {
    format!("btn {}", if active { "active" } else { "" })
}>
```

---

### Issue: Tailwind styles not applying

**Symptom:**
UI elements appear unstyled or use default browser styles.

**Solution:**

1. **Verify Tailwind CSS is generated:**
```bash
ls -la style/output.css
```

2. **Regenerate Tailwind CSS:**
```bash
npx @tailwindcss/cli -i style/input.css -o style/output.css --watch
```

3. **Check `index.html` includes the stylesheet:**
```html
<link data-trunk rel="css" href="style/output.css" />
```

4. **Rebuild frontend:**
```bash
trunk build --release
```

---

### Issue: "RUSTUP_TOOLCHAIN" environment variable errors on Windows

**Symptom:**
```
'RUSTUP_TOOLCHAIN' is not recognized as an internal or external command
```

**Solution:**
Windows doesn't support inline environment variables the same way. Use PowerShell or modify `package.json`:

**Option 1: PowerShell**
```powershell
$env:RUSTUP_TOOLCHAIN="stable"; npm run dev
```

**Option 2: Modify package.json scripts**
```json
"scripts": {
  "dev": "tauri dev",
  "build": "tauri build"
}
```

Then set `RUSTUP_TOOLCHAIN=stable` globally or let rustup use the default toolchain.

---

## Audio & MIDI Issues

### Issue: No audio output in desktop app

**Symptom:**
Sequencer plays (playhead moves) but no sound is heard.

**Troubleshooting Steps:**

1. **Check system audio output:**
   - Verify speakers/headphones are connected and working
   - Test with another audio application
   - Check system volume is not muted

2. **Check CPAL audio device initialization:**
   - Look for console messages in terminal (not browser console)
   - CPAL will log "Using audio device: [device name]"
   - If no device found, CPAL falls back to silent operation

3. **Check MIDI note is configured:**
   - Click a step in the grid (should turn blue)
   - Verify the step inspector shows a MIDI note (e.g., C3, A4)
   - Default note may be silent if outside your monitoring range

4. **Verify audio engine is running:**
   - Check `src-tauri/src/engine/kernel.rs` for audio callback logs
   - Audio thread should process at ~93 Hz (512 samples @ 44.1kHz)

**Note:** FLUX currently uses MIDI note triggers but does not yet have built-in synthesizers or samplers (Phase 4 feature). You may need to route MIDI to an external synth.

---

### Issue: Audio glitches, clicks, or dropouts

**Symptom:**
Crackling, popping, or intermittent audio during playback.

**Causes & Solutions:**

1. **CPU overload:**
   - Close other CPU-intensive applications
   - Reduce system load (browsers, video calls, etc.)
   - Check Activity Monitor / Task Manager for high CPU usage

2. **Buffer size too small:**
   - FLUX uses a fixed 512-sample buffer (~11.6ms latency)
   - Edit `src-tauri/src/engine/kernel.rs` to increase buffer size if needed:
   ```rust
   let buffer_size = 1024; // Increase from 512 for more stability
   ```

3. **Lock-free queue overflow:**
   - If sending too many commands from UI → Audio thread
   - Check console for "Command queue full" warnings
   - Reduce rapid parameter changes (e.g., slider dragging)

4. **System audio interference:**
   - macOS: Disable audio enhancements in System Settings > Sound
   - Windows: Disable audio enhancements in Sound Control Panel
   - Linux: Check PulseAudio/JACK configuration

---

### Issue: MIDI output not working

**Symptom:**
External MIDI devices don't receive MIDI messages from FLUX.

**Current Status:**
FLUX **does not yet support MIDI output** to external devices. MIDI processing is internal-only for triggering the audio engine. External MIDI I/O is planned for a future phase.

**Workaround:**
Use virtual MIDI routing (e.g., IAC Driver on macOS, loopMIDI on Windows) to route FLUX's future MIDI output to other software.

---

## UI & Performance Issues

### Issue: Low frame rate during playback

**Symptom:**
Grid animations are choppy or laggy (below 60 FPS).

**Diagnostics:**

1. **Check browser performance:**
   - Open DevTools > Performance tab
   - Record during playback
   - Look for frame drops (red bars) or long tasks (>50ms)

2. **Verify GPU acceleration:**
   - DevTools > Rendering > Frame Rendering Stats
   - Ensure "Composited Layers" is enabled
   - Check that animations use CSS transforms (not layout changes)

3. **Check memory leaks:**
   - DevTools > Memory tab
   - Take heap snapshots before/after playback
   - Stable memory indicates no leaks (expected: ~4 MB stable)

**Solutions:**

- **Reduce visual complexity:** Fewer active tracks/steps
- **Disable effects:** Turn off LFO visualizations if present
- **Update browser:** Ensure latest Chrome/Edge/Safari
- **Check hardware acceleration:** Enable in browser settings

**Expected Performance:**
- Target: 60+ FPS
- Typical: 120 FPS on modern hardware
- Memory: ~4 MB stable (no growth over time)

---

### Issue: Selection badge not updating

**Symptom:**
Clicking steps doesn't update the "T1・S1" badge above the grid.

**Solution:**

1. **Check step selection logic:**
   - Badge should update on every step click
   - Verify `selected_step` signal is reactive

2. **Look for console errors:**
   - Browser console may show signal update errors
   - Check for "Cannot read property" errors

3. **Clear browser cache:**
   - Hard refresh: Cmd+Shift+R (Mac) or Ctrl+F5 (Windows)
   - Clear site data in DevTools > Application > Storage

---

### Issue: Parameter inspector not closing with Escape key

**Symptom:**
Pressing Escape doesn't close the step inspector panel.

**Solution:**

1. **Check keyboard focus:**
   - Click somewhere in the app to ensure it has focus
   - Browser may intercept Escape if DevTools is focused

2. **Verify event handler:**
   - Check `src/ui/components/step_inspector.rs` for `on:keydown` handler
   - Ensure event listener is attached

3. **Workaround:**
   - Click outside the inspector panel
   - Select a different step

---

### Issue: Playhead indicator not visible

**Symptom:**
During playback, no green overlay shows which step is currently playing.

**Solution:**

1. **Check playback state:**
   - Verify "PLAY" button shows active state (orange background)
   - If not active, playback isn't running

2. **Verify state updates:**
   - Open browser console
   - Look for "Current step: X" messages (if debug logging enabled)
   - Check that `current_step` signal updates every ~500ms

3. **Check CSS rendering:**
   - Inspect element in DevTools
   - Playhead should have `bg-emerald-500/20` class
   - Verify opacity is visible against dark background

**Design Note:** Playhead uses a subtle emerald tint (`/20` opacity). If hard to see, this is a known design consideration for future adjustment.

---

## Browser vs Desktop Mode

### Issue: "Cannot read properties of undefined (reading 'core')" errors

**Symptom:**
Browser console shows TypeErrors related to Tauri APIs:
```
TypeError: Cannot read properties of undefined (reading 'core')
    at invoke (tauri.rs)
```

**Explanation:**
This is **expected behavior** when running in browser mode. Tauri APIs (`window.__TAURI__`) are only available in the desktop app built with Tauri.

**Solution:**

**Option 1: Run desktop app (recommended for audio)**
```bash
npm run dev
```
This launches the full Tauri desktop app with audio support.

**Option 2: Use browser preview mode**
```bash
trunk serve
```
Open http://localhost:1420 in browser. This mode is useful for UI development but:
- No audio functionality
- Tauri APIs unavailable
- Yellow "Preview Mode" banner appears
- Console errors are safe to ignore

**Fix (already implemented):**
FLUX uses graceful degradation via `tauri_detect.rs`:
- Detects if `window.__TAURI__` exists
- Falls back to no-op implementations in browser
- Shows "Preview Mode" banner warning users

---

### Issue: Yellow "Preview Mode" banner appears

**Symptom:**
A yellow banner at the top says "Preview Mode - Audio features disabled".

**Explanation:**
This is **intentional**. It indicates you're running in browser mode without Tauri desktop features.

**To enable full features:**
```bash
npm run dev  # Launches desktop app
```

**To hide banner (if developing UI only):**
Edit `src/app.rs` and comment out the `PreviewBanner` component. Not recommended unless you know what you're doing.

---

### Issue: Audio works in desktop mode but not browser

**Symptom:**
Desktop app plays audio correctly, but browser version is silent.

**Explanation:**
This is **expected**. Browser mode has no audio engine because:
1. Web browsers can't access low-latency audio APIs like CPAL
2. Tauri backend (which runs the audio engine) isn't available in browser
3. WebAssembly runs only the frontend UI code

**Solution:**
Use desktop mode for audio:
```bash
npm run dev
```

**Technical Background:**
FLUX uses a split architecture:
- **Frontend (WASM):** UI rendering, user interactions
- **Backend (Tauri):** Audio engine, MIDI processing, file I/O

Browser mode runs ONLY the frontend. Desktop mode runs both.

See `ARCHITECTURE.md` for details on the threading model.

---

## Known Limitations

### Current Phase Limitations

FLUX is under active development. Current limitations:

#### Audio Engine
- **No built-in synthesizers** - MIDI notes trigger internal engine but produce no sound yet
- **No sample playback** - Sample loading and playback planned for Phase 4
- **No external MIDI I/O** - Can't send MIDI to external devices or receive MIDI input
- **No audio recording** - Can't record audio output to file

#### Sequencer Features
- **16-step limitation** - Fixed pattern length (no pattern chaining yet)
- **4-track limitation** - Cannot add more tracks dynamically
- **No pattern saving** - Pattern data not persistent across sessions yet
- **No undo/redo** - Command history not implemented

#### UI Features
- **No piano roll view** - Grid view only
- **No waveform display** - Sample waveforms not visualized
- **No mixer view** - Per-track volume/pan controls not implemented
- **No effects rack** - No reverb, delay, EQ, etc.

#### LFO Designer
- **Drawing only** - LFO waveforms can be drawn but not yet routed to parameters
- **No modulation matrix** - LFO routing and depth controls planned for Phase 5
- **No preset LFO shapes** - Must draw manually (no sine/square/saw templates)

### Platform Limitations

#### macOS
- **Minimum version:** macOS 10.15 (Catalina) for Tauri 2.x
- **Audio latency:** ~11.6ms (512 samples @ 44.1kHz) - fixed, not configurable in UI

#### Windows
- **WASAPI required** - Windows XP/Vista not supported (needs Windows 7+)
- **ASIO not supported** - FLUX uses CPAL which doesn't support ASIO drivers

#### Linux
- **ALSA/PulseAudio required** - JACK support experimental
- **Audio device selection** - CPAL uses default device, can't choose in UI

### Browser Mode Limitations

When running via `trunk serve` (browser preview):
- **No audio** - Audio engine requires Tauri desktop runtime
- **No file I/O** - Can't save/load patterns
- **No system integration** - No MIDI, no audio device access
- **No OS notifications** - Desktop-only feature

**Recommendation:** Use browser mode for UI development only. Use desktop mode (`npm run dev`) for full functionality.

---

## Getting Help

### Before Reporting Issues

1. **Check this troubleshooting guide** for known issues
2. **Search existing issues** on GitHub (if applicable)
3. **Verify you're on the latest version** (`git pull` and rebuild)
4. **Try with a fresh build:**
   ```bash
   trunk clean
   cargo clean
   npm install
   npm run dev
   ```

### Reporting Bugs

When reporting bugs, include:

1. **Environment:**
   - OS version (macOS/Windows/Linux)
   - Rust version (`rustc --version`)
   - Node.js version (`node --version`)
   - Browser (if browser mode)

2. **Steps to reproduce:**
   - Exact commands run
   - User actions taken
   - Expected vs actual behavior

3. **Logs:**
   - Terminal output (for desktop app)
   - Browser console errors (for browser mode)
   - Relevant error messages

4. **Screenshots/videos:**
   - Visual bugs benefit from screenshots
   - Performance issues benefit from DevTools recordings

### Useful Debugging Commands

**Check Rust toolchain:**
```bash
rustc --version
rustup show
rustup target list | grep wasm32
```

**Check Node.js environment:**
```bash
node --version
npm --version
npm list --depth=0
```

**Clean build (nuclear option):**
```bash
# Clean all build artifacts
trunk clean
cargo clean
rm -rf target/
rm -rf dist/

# Clean dependencies
rm -rf node_modules/
npm install

# Rebuild
npm run dev
```

**Verbose build output:**
```bash
RUST_LOG=debug npm run dev
```

**Check audio device (macOS):**
```bash
system_profiler SPAudioDataType
```

### Community & Support

- **Architecture questions:** See `ARCHITECTURE.md` and `LOCK_FREE_AUDIO.md`
- **Development setup:** See `DEVELOPER_GUIDE.md`
- **User workflows:** See `USER_GUIDE.md`
- **Design decisions:** Check `docs/plans/*.md` for historical context

---

## Appendix: Common Error Messages

### "error: linking with `cc` failed"

**Full error:**
```
error: linking with `cc` failed: exit status: 1
```

**Cause:** Missing platform build tools.

**Solution:**
- **macOS:** `xcode-select --install`
- **Linux:** `sudo apt install build-essential`
- **Windows:** Install Visual Studio Build Tools

---

### "Error: Failed to fetch"

**Full error:**
```
Error: Failed to fetch
    at safe_invoke (tauri.rs)
```

**Cause:** Browser mode trying to call Tauri APIs.

**Solution:** Run desktop app (`npm run dev`) or ignore if developing UI only.

---

### "thread 'audio' panicked at 'failed to build audio stream'"

**Cause:** CPAL can't initialize audio device.

**Troubleshooting:**
1. Check audio device is connected and working
2. Close other apps using audio (DAWs, browsers, etc.)
3. Restart audio system (kill `coreaudiod` on macOS, restart audio service on Windows)
4. Check `src-tauri/src/engine/kernel.rs` for hardcoded device assumptions

---

### "error: package `tauri v2.x.x` cannot be built"

**Cause:** Rust toolchain too old.

**Solution:**
```bash
rustup update stable
rustup default stable
```

Tauri 2.x requires Rust 1.70 or later.

---

**Last Updated:** 2026-02-14
**Version:** 1.0
