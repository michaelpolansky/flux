# Error Handling & Comprehensive Documentation - Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:writing-plans to create implementation plan from this design.

**Goal:** Eliminate console errors through graceful degradation and create comprehensive documentation covering users, developers, architecture, and troubleshooting.

**Approach:** Foundation-first - fix technical issues, then build documentation from inside-out (architecture → developer → user → troubleshooting).

**Tech Stack:** Rust/WASM (Leptos 0.7), Tauri 2.x, wasm-bindgen for browser API detection

---

## Overview & Goals

### Project Context

FLUX is a Tauri-based desktop sequencer with a Leptos (Rust WASM) frontend. During development and testing, the frontend runs in browser mode (`trunk serve`) where Tauri APIs are unavailable, causing TypeErrors in the console. Additionally, the project lacks comprehensive documentation for users and developers.

### Goals

1. **Clean Console** - Eliminate Tauri TypeErrors through graceful degradation
2. **Comprehensive Documentation** - User guide, developer docs, architecture, troubleshooting
3. **Maintainability** - Foundation for future contributors

### Success Criteria

- Zero console errors in both browser and Tauri modes
- Complete documentation covering all user and developer needs
- Clear onboarding path for new users and contributors

### Non-Goals

- Full browser-mode support (audio requires Tauri backend)
- API documentation generation tools (manual docs for now)
- Video tutorials (written docs only)

---

## Problem Analysis

### Current Issues

**1. Console TypeErrors**

When running in browser mode (development or testing):
```javascript
TypeError: Cannot read properties of undefined (reading 'event')
  at window.__TAURI__.event.listen()

TypeError: Cannot read properties of undefined (reading 'core')
  at window.__TAURI__.core.invoke()
```

**Root Cause:**
- `window.__TAURI__` object only exists in Tauri desktop environment
- Frontend code assumes Tauri is always available
- No detection or error handling for browser mode

**Impact:**
- Cluttered console makes debugging harder
- Confusing for new developers
- Breaks automated testing flow

**2. Documentation Gaps**

Current state:
- ✅ Good README with overview, setup, architecture highlights
- ❌ No detailed user guide (how to actually use the sequencer)
- ❌ No architecture deep-dive (referenced but doesn't exist)
- ❌ No developer/contributor guide
- ❌ No troubleshooting documentation

**Impact:**
- Steep learning curve for users
- Hard for contributors to get started
- Support questions repeat
- Missing "why" behind architectural decisions

---

## Solution: Graceful Degradation + Documentation Suite

### Part 1: Error Handling Architecture

**Concept:** Detect Tauri availability at startup and gracefully degrade features when unavailable.

#### 1.1 Tauri Detection Module

**New file:** `src/ui/tauri_detect.rs`

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"])]
    static __TAURI__: JsValue;
}

#[derive(Clone, Copy, Debug)]
pub struct TauriCapabilities {
    pub available: bool,
    pub audio_enabled: bool,
    pub events_enabled: bool,
}

impl Default for TauriCapabilities {
    fn default() -> Self {
        Self {
            available: false,
            audio_enabled: false,
            events_enabled: false,
        }
    }
}

/// Detect if Tauri APIs are available
pub fn detect_tauri() -> TauriCapabilities {
    // Check if window.__TAURI__ exists and is an object
    let tauri_exists = !__TAURI__.is_undefined() && !__TAURI__.is_null();

    if tauri_exists {
        // Further checks could verify specific APIs
        TauriCapabilities {
            available: true,
            audio_enabled: true,
            events_enabled: true,
        }
    } else {
        TauriCapabilities::default()
    }
}
```

#### 1.2 Safe API Wrappers

**Update:** `src/ui/tauri.rs`

Add error type and safe wrappers:

```rust
#[derive(Debug, Clone)]
pub enum TauriError {
    NotAvailable,
    InvokeFailed(String),
}

/// Check if Tauri is available (cached from detection)
fn is_tauri_available() -> bool {
    use_context::<TauriCapabilities>()
        .map(|caps| caps.available)
        .unwrap_or(false)
}

/// Safe invoke - returns error if Tauri unavailable
pub async fn safe_invoke(cmd: &str, args: JsValue) -> Result<JsValue, TauriError> {
    if !is_tauri_available() {
        return Err(TauriError::NotAvailable);
    }

    invoke(cmd, args)
        .await
        .map_err(|e| TauriError::InvokeFailed(format!("{:?}", e)))
}

/// Safe event listener - no-op if Tauri unavailable
pub async fn safe_listen_event<T>(event_name: &str, callback: impl Fn(T) + 'static)
where T: for<'a> Deserialize<'a> + 'static
{
    if !is_tauri_available() {
        // Log once for debugging
        web_sys::console::log_1(
            &format!("Tauri not available - event listener '{}' disabled", event_name).into()
        );
        return;
    }

    // Existing listen_event implementation
    listen_event(event_name, callback).await
}
```

#### 1.3 App Integration

**Update:** `src/app.rs`

Detect capabilities at startup and provide via context:

```rust
use crate::ui::tauri_detect::{detect_tauri, TauriCapabilities};

#[component]
pub fn App() -> impl IntoView {
    // Detect Tauri capabilities
    let tauri_capabilities = detect_tauri();
    provide_context(tauri_capabilities);

    // Conditionally setup features
    if tauri_capabilities.events_enabled {
        // Setup Tauri event listeners
        Effect::new(move |_| {
            spawn_local(async move {
                use crate::ui::tauri::safe_listen_event;
                safe_listen_event("playback-status", move |event: AudioSnapshot| {
                    // ... existing handler
                }).await;
            });
        });
    }

    view! {
        <main>
            // Show preview mode banner if Tauri unavailable
            {move || {
                if !tauri_capabilities.available {
                    view! {
                        <div class="bg-amber-500/20 border-b border-amber-500/50 px-4 py-2 text-sm text-amber-200">
                            "⚠️ Preview Mode - Audio features require "
                            <a href="https://github.com/yourusername/flux" class="underline">
                                "desktop app"
                            </a>
                        </div>
                    }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}

            // ... rest of app
        </main>
    }
}
```

#### 1.4 User Experience

**Tauri Desktop Mode** (Production):
- All features enabled
- No banner
- Full audio, playback, MIDI

**Browser Mode** (Development/Testing):
- UI preview functional
- Banner: "Preview Mode - Audio features require desktop app"
- Audio features gracefully disabled
- Clean console (no TypeErrors)

---

### Part 2: Documentation Structure

#### 2.1 File Organization

```
docs/
├── README.md                          (update - streamline to quick start)
├── USER_GUIDE.md                      (new - comprehensive user manual)
├── ARCHITECTURE.md                    (new - system design deep-dive)
├── DEVELOPER_GUIDE.md                 (new - contributing, APIs)
├── TROUBLESHOOTING.md                 (new - common issues)
├── plans/                             (existing - design/implementation docs)
└── api/                               (new - component documentation)
    ├── components.md                  (Grid, Inspector, Toolbar, etc.)
    ├── state-management.md            (Signals, context, reactivity)
    └── tauri-integration.md           (Commands, events, IPC)
```

#### 2.2 USER_GUIDE.md Structure

**Purpose:** Help users learn and master the sequencer

**Outline:**
1. **Getting Started**
   - Installation (macOS, Windows, Linux)
   - First launch walkthrough
   - Interface overview

2. **The Grid**
   - Step buttons (active, selected, playing states)
   - Playback visualization (playhead, highlights, pulse)
   - Beat grouping markers
   - Selection and navigation

3. **Creating Patterns**
   - Toggling steps on/off
   - Playback controls (play, stop, tempo)
   - Understanding tracks and subtracks
   - Basic sequencing workflow

4. **Parameter Locking (P-Lock)**
   - What is parameter locking
   - Selecting steps for editing
   - Parameter inspector usage
   - Per-step automation examples

5. **LFO Designer**
   - Opening LFO designer
   - Drawing waveforms
   - LFO routing and modulation
   - Creative modulation techniques

6. **Advanced Features**
   - Micro-timing offset
   - Step probability
   - Trig types (Note, Lock, Trigless, OneShot)
   - Pattern chaining (future)

7. **Keyboard Shortcuts**
   - Navigation shortcuts
   - Playback controls
   - Selection and editing
   - Quick reference table

8. **Tips & Best Practices**
   - Workflow suggestions
   - Performance tips
   - Creative techniques
   - Common mistakes to avoid

**Length:** ~2000 words
**Format:** Markdown with screenshots/diagrams where helpful

#### 2.3 ARCHITECTURE.md Structure

**Purpose:** Explain how the system works for developers and contributors

**Outline:**
1. **System Overview**
   - High-level architecture diagram
   - Frontend (Leptos) ↔ Backend (Tauri) ↔ Audio Engine
   - Technology choices and rationale

2. **Lock-Free Audio Architecture**
   ```
   [UI Thread]  →  [Command Queue (rtrb)]  →  [Audio Thread]
         ↑                                            ↓
         ←─────  [State Snapshot (triple_buffer)] ←──┘
   ```
   - Why lock-free matters
   - Triple buffer pattern (Audio→UI state)
   - Ring buffer pattern (UI→Audio commands)
   - Zero allocation in audio callback

3. **State Management**
   - Leptos signals and reactivity
   - Context providers (Pattern, PlaybackState, GridUIState)
   - Signal::derive for performance
   - State update flow

4. **Frontend Architecture**
   - Component hierarchy
   - Tauri integration layer
   - Event listeners and IPC
   - Error handling strategy

5. **Data Model**
   - AtomicStep structure
   - Pattern, Track, Subtrack hierarchy
   - Parameter lock storage
   - Serialization strategy

6. **Performance Considerations**
   - Reactive granularity (64 GridStep buttons)
   - Memoization patterns
   - Animation performance (CSS transforms)
   - Memory management

7. **Design Decisions**
   - Why Rust + WASM frontend
   - Why Tauri vs Electron
   - State-first architecture rationale
   - Trade-offs and alternatives considered

**Length:** ~1500 words
**Format:** Markdown with diagrams

#### 2.4 DEVELOPER_GUIDE.md Structure

**Purpose:** Help contributors get started and follow project conventions

**Outline:**
1. **Development Setup**
   - Prerequisites (Rust, Node.js, platform tools)
   - Installation steps (detailed)
   - First build verification
   - Development server (trunk serve vs tauri dev)

2. **Project Structure Walkthrough**
   - Frontend (`src/`) organization
   - Backend (`src-tauri/`) organization
   - Shared models
   - Documentation structure

3. **Component Development**
   - Leptos component patterns
   - Props and signals
   - Context consumption
   - Styling with Tailwind

4. **Adding New Features**
   - Step-by-step guide:
     1. Design phase (brainstorming skill)
     2. State management setup
     3. Component creation
     4. Tauri integration (if needed)
     5. Testing
     6. Documentation

5. **State Management Patterns**
   - When to use Signal vs Memo
   - Context provision best practices
   - Avoiding reactive pitfalls
   - Performance optimization

6. **Tauri Integration**
   - Adding commands
   - Event emission
   - Error handling
   - Testing IPC layer

7. **Testing Strategy**
   - Manual testing approach
   - Performance profiling
   - Edge case testing
   - Browser vs Tauri testing

8. **Code Style & Conventions**
   - Rust formatting (rustfmt)
   - Component naming
   - File organization
   - Documentation standards

9. **Contributing Guidelines**
   - Git workflow (branches, commits)
   - Pull request process
   - Code review expectations
   - Communication channels

**Length:** ~1200 words
**Format:** Markdown with code examples

#### 2.5 TROUBLESHOOTING.md Structure

**Purpose:** Help users and developers solve common problems

**Outline:**
1. **Platform-Specific Setup**
   - macOS setup (Xcode, permissions)
   - Windows setup (Visual Studio, dependencies)
   - Linux setup (audio dependencies, permissions)

2. **Build Errors**
   - Rust compilation errors
   - WASM target issues
   - Dependency resolution
   - Platform-specific build failures

3. **Audio Issues**
   - No sound / audio not working
   - Audio device configuration
   - Sample rate mismatches
   - Latency problems
   - MIDI configuration (future)

4. **UI Issues**
   - UI not updating / reactive issues
   - Performance problems
   - Visual glitches
   - State synchronization errors

5. **Development Environment**
   - Trunk serve not starting
   - Hot reload not working
   - Tauri dev mode errors
   - Port conflicts

6. **Runtime Errors**
   - Console TypeErrors (should be fixed by Part 1)
   - Tauri IPC errors
   - State initialization errors
   - Pattern loading/saving errors

7. **FAQ**
   - How do I reset the app to defaults?
   - Where are patterns stored?
   - How do I export/import patterns? (future)
   - Can I run this without Tauri?

**Length:** ~800 words
**Format:** Problem/Solution pairs, Q&A format

#### 2.6 README.md Updates

**Changes:**
- Streamline to "quick start" focus
- Add prominent links to new docs:
  ```markdown
  ## Documentation

  - 📖 [User Guide](docs/USER_GUIDE.md) - Learn how to use FLUX
  - 🏗️ [Architecture](docs/ARCHITECTURE.md) - How FLUX works
  - 💻 [Developer Guide](docs/DEVELOPER_GUIDE.md) - Contributing to FLUX
  - 🔧 [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions
  ```
- Remove duplicated content (defer to detailed docs)
- Keep features list, tech stack, and quick setup

---

## Implementation Phases

### Phase 1: Error Handling (~2 hours)

**Files:**
- Create: `src/ui/tauri_detect.rs`
- Modify: `src/ui/tauri.rs`, `src/app.rs`, `src/ui/mod.rs`

**Tasks:**
1. Create TauriCapabilities detection module
2. Add safe wrappers (safe_invoke, safe_listen_event)
3. Update app.rs to detect and provide capabilities
4. Add "Preview Mode" banner for browser mode
5. Test in Tauri desktop (full features)
6. Test in browser (clean console, banner visible)
7. Commit changes

**Testing:**
- Run `trunk serve` → Should see banner, no console errors
- Run `npm run dev` → Should see no banner, full features

### Phase 2: Architecture Documentation (~3 hours)

**Files:**
- Create: `docs/ARCHITECTURE.md`

**Tasks:**
1. Write system overview with diagram
2. Document lock-free audio architecture
3. Explain state management patterns
4. Document frontend architecture
5. Explain data model
6. Add performance considerations
7. Document design decisions
8. Review and polish
9. Commit documentation

**Validation:**
- Ask a developer unfamiliar with project to read
- Verify all major systems are explained
- Ensure diagrams are clear

### Phase 3: Developer Documentation (~2 hours)

**Files:**
- Create: `docs/DEVELOPER_GUIDE.md`
- Create: `docs/api/components.md`
- Create: `docs/api/state-management.md`
- Create: `docs/api/tauri-integration.md`

**Tasks:**
1. Write development setup guide
2. Document project structure
3. Write component development patterns
4. Create "adding features" walkthrough
5. Document state management patterns
6. Write Tauri integration guide
7. Add code style and conventions
8. Write contributing guidelines
9. Create component API reference
10. Commit documentation

**Validation:**
- Follow own setup guide from scratch
- Verify all patterns are documented
- Check code examples compile

### Phase 4: User Documentation (~3 hours)

**Files:**
- Create: `docs/USER_GUIDE.md`

**Tasks:**
1. Write getting started section
2. Document grid interface
3. Write pattern creation tutorial
4. Document parameter locking
5. Write LFO designer guide
6. Document advanced features
7. Create keyboard shortcuts reference
8. Add tips and best practices
9. Review and polish
10. Commit documentation

**Validation:**
- Have non-developer read getting started
- Verify all UI features are documented
- Check screenshots/diagrams are helpful

### Phase 5: Troubleshooting & README Updates (~1 hour)

**Files:**
- Create: `docs/TROUBLESHOOTING.md`
- Modify: `README.md`

**Tasks:**
1. Write platform-specific setup guides
2. Document build errors
3. Document audio issues
4. Document UI issues
5. Document development environment problems
6. Add FAQ section
7. Update README with doc links
8. Streamline README content
9. Commit documentation

**Validation:**
- Test troubleshooting steps
- Verify README links work
- Check FAQ covers common questions

---

## Timeline & Effort

**Total:** 3 sessions (~11 hours)

**Session 1** (3-4 hours):
- Phase 1: Error Handling (2h)
- Phase 2: Architecture Docs (1-2h)

**Session 2** (3-4 hours):
- Phase 2: Architecture Docs completion (1h)
- Phase 3: Developer Docs (2-3h)

**Session 3** (4 hours):
- Phase 4: User Docs (3h)
- Phase 5: Troubleshooting & README (1h)

---

## Testing Strategy

### Error Handling Testing

**Browser Mode:**
```bash
trunk serve
# Open http://localhost:1420
# Verify: Preview mode banner visible, clean console
```

**Tauri Desktop:**
```bash
npm run dev
# Verify: No banner, full features, clean console
```

### Documentation Testing

**Completeness Check:**
- [ ] All UI features documented
- [ ] All architectural patterns explained
- [ ] Setup instructions complete
- [ ] Troubleshooting covers common issues

**Clarity Check:**
- [ ] Non-developer can follow USER_GUIDE
- [ ] New contributor can set up dev environment
- [ ] Architecture makes sense to experienced developer

**Accuracy Check:**
- [ ] Code examples compile
- [ ] Setup steps actually work
- [ ] Keyboard shortcuts correct
- [ ] Diagrams match implementation

---

## Success Criteria

### Error Handling
- ✅ Zero console errors in browser mode
- ✅ Zero console errors in Tauri mode
- ✅ Clear "Preview Mode" indication in browser
- ✅ All features work in Tauri desktop
- ✅ Graceful feature degradation in browser

### Documentation
- ✅ USER_GUIDE covers all features
- ✅ ARCHITECTURE explains all major systems
- ✅ DEVELOPER_GUIDE enables contributors
- ✅ TROUBLESHOOTING solves common problems
- ✅ README links to all docs clearly

### Maintainability
- ✅ New features documented as added
- ✅ Contributors can onboard quickly
- ✅ Users can learn sequencer efficiently
- ✅ Support questions decrease
