use leptos::prelude::*;
use crate::app::SequencerState;
use crate::shared::models::Pattern;
use crate::ui::components::collapsible_section::CollapsibleSection;
use crate::ui::components::form_controls::*;

#[component]
pub fn StepEditorSidebar() -> impl IntoView {
    let sequencer_state = use_context::<SequencerState>()
        .expect("SequencerState context not found");
    const P_LOCK_THRESHOLD: f32 = 0.001;
    let selected_step = sequencer_state.selected_step;

    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Get current note value
    let note_value = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.note as f64)
                    .unwrap_or(60.0)
            })
        } else {
            60.0
        }
    });

    // Note change handler
    let on_note_change = move |val: f64| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            let clamped = (val.round() as u8).clamp(0, 127);
            set_pattern_signal.update(|pattern| {
                if let Some(track) = pattern.tracks.get_mut(track_id) {
                    if let Some(subtrack) = track.subtracks.get_mut(0) {
                        if let Some(step) = subtrack.steps.get_mut(step_idx) {
                            step.note = clamped;
                        }
                    }
                }
            });
        }
    };

    // Get current velocity value
    let velocity_value = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.velocity as f64)
                    .unwrap_or(100.0)
            })
        } else {
            100.0
        }
    });

    // Velocity change handler
    let on_velocity_change = move |val: f64| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            let clamped = (val.round() as u8).clamp(0, 127);
            set_pattern_signal.update(|pattern| {
                if let Some(track) = pattern.tracks.get_mut(track_id) {
                    if let Some(subtrack) = track.subtracks.get_mut(0) {
                        if let Some(step) = subtrack.steps.get_mut(step_idx) {
                            step.velocity = clamped;
                        }
                    }
                }
            });
        }
    };

    // Get current length value
    let length_value = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.length as f64)
                    .unwrap_or(1.0)
            })
        } else {
            1.0
        }
    });

    // Length change handler
    let on_length_change = move |val: f64| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            let clamped = (val as f32).clamp(0.1, 4.0);
            set_pattern_signal.update(|pattern| {
                if let Some(track) = pattern.tracks.get_mut(track_id) {
                    if let Some(subtrack) = track.subtracks.get_mut(0) {
                        if let Some(step) = subtrack.steps.get_mut(step_idx) {
                            step.length = clamped;
                        }
                    }
                }
            });
        }
    };

    // Get current probability value
    let probability_value = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.condition.prob as f64)
                    .unwrap_or(100.0)
            })
        } else {
            100.0
        }
    });

    // Probability change handler
    let on_probability_change = move |val: f64| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            let clamped = (val.round() as u8).clamp(0, 100);
            set_pattern_signal.update(|pattern| {
                if let Some(track) = pattern.tracks.get_mut(track_id) {
                    if let Some(subtrack) = track.subtracks.get_mut(0) {
                        if let Some(step) = subtrack.steps.get_mut(step_idx) {
                            step.condition.prob = clamped;
                        }
                    }
                }
            });
        }
    };

    // Get current micro-timing value
    let micro_timing_value = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.micro_timing as f64)
                    .unwrap_or(0.0)
            })
        } else {
            0.0
        }
    });

    // Micro-timing change handler
    let on_micro_timing_change = move |val: f64| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            let clamped = (val.round() as i8).clamp(-23, 23);
            set_pattern_signal.update(|pattern| {
                if let Some(track) = pattern.tracks.get_mut(track_id) {
                    if let Some(subtrack) = track.subtracks.get_mut(0) {
                        if let Some(step) = subtrack.steps.get_mut(step_idx) {
                            step.micro_timing = clamped;
                        }
                    }
                }
            });
        }
    };

    // Sound parameter definitions (8 parameters from Inspector)
    let sound_params = [
        "Tuning", "Filter Freq", "Resonance", "Drive",
        "Decay", "Sustain", "Reverb", "Delay"
    ];

    // Get sound parameter value (P-Lock or track default)
    let get_param_value = move |param_idx: usize| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                if let Some(track) = p.tracks.get(track_id) {
                    if let Some(subtrack) = track.subtracks.get(0) {
                        if let Some(step) = subtrack.steps.get(step_idx) {
                            // Check P-Lock first, fallback to track default
                            return step.p_locks.get(param_idx)
                                .and_then(|p| *p)
                                .unwrap_or_else(|| track.default_params.get(param_idx).cloned().unwrap_or(0.0));
                        }
                    }
                }
                0.0
            })
        } else {
            0.0
        }
    };

    // Check if parameter is P-Locked
    let is_param_locked = move |param_idx: usize| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .and_then(|s| s.p_locks.get(param_idx))
                    .map(|p| p.is_some())
                    .unwrap_or(false)
            })
        } else {
            false
        }
    };

    // Handle sound parameter input (creates/updates P-Lock)
    let handle_param_input = move |param_idx: usize, val: f64| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            let clamped = val.clamp(0.0, 1.0) as f32;
            set_pattern_signal.update(|pattern| {
                if let Some(track) = pattern.tracks.get_mut(track_id) {
                    if let Some(subtrack) = track.subtracks.get_mut(0) {
                        if let Some(step) = subtrack.steps.get_mut(step_idx) {
                            // Check if value differs from track default
                            let track_default = track.default_params.get(param_idx).cloned().unwrap_or(0.0);
                            if (clamped - track_default).abs() > P_LOCK_THRESHOLD {
                                // Different from default → create P-Lock
                                if param_idx < 128 {
                                    step.p_locks[param_idx] = Some(clamped);
                                }
                            } else {
                                // Same as default → remove P-Lock
                                if param_idx < 128 {
                                    step.p_locks[param_idx] = None;
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    // P-Lock count for badge
    let p_lock_count = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks.get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.p_locks.iter().filter(|p| p.is_some()).count())
                    .unwrap_or(0)
            })
        } else {
            0
        }
    });

    view! {
        <div class="w-80 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col overflow-y-auto">
            {move || {
                if let Some((track_id, step_idx)) = selected_step.get() {
                    view! {
                        <div class="animate-in fade-in slide-in-from-top-2 duration-200">
                            <div class="flex items-center justify-between mb-4">
                                <div class="flex flex-col">
                                    <span class="text-xs font-bold text-zinc-400 uppercase tracking-wide">"EDITING STEP"</span>
                                    <span class="text-sm text-zinc-100">
                                        {format!("Track {}, Step {}", track_id + 1, step_idx + 1)}
                                    </span>
                                </div>
                                <button
                                    class="text-xs text-zinc-500 hover:text-red-500 transition-colors duration-150 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:ring-offset-zinc-900 rounded px-1"
                                    on:click=move |_| selected_step.set(None)
                                >
                                    "×"
                                </button>
                            </div>

                            <CollapsibleSection
                                title="STEP PROPERTIES"
                                default_open=true
                            >
                                <InlineParam>
                                    <ParamLabel text="Note (Pitch)" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="0"
                                        max="127"
                                        step="1"
                                        value=Signal::derive(move || format!("{}", note_value.get() as u8))
                                        on_input=on_note_change
                                    />
                                </InlineParam>

                                <InlineParam>
                                    <ParamLabel text="Velocity" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="0"
                                        max="127"
                                        step="1"
                                        value=Signal::derive(move || format!("{}", velocity_value.get() as u8))
                                        on_input=on_velocity_change
                                    />
                                </InlineParam>

                                <InlineParam>
                                    <ParamLabel text="Length" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="0.1"
                                        max="4.0"
                                        step="0.1"
                                        value=Signal::derive(move || format!("{:.1}", length_value.get()))
                                        on_input=on_length_change
                                    />
                                </InlineParam>

                                <InlineParam>
                                    <ParamLabel text="Probability" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="0"
                                        max="100"
                                        step="1"
                                        value=Signal::derive(move || format!("{}", probability_value.get() as u8))
                                        on_input=on_probability_change
                                    />
                                </InlineParam>

                                <InlineParam>
                                    <ParamLabel text="Micro-timing" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="-23"
                                        max="23"
                                        step="1"
                                        value=Signal::derive(move || format!("{}", micro_timing_value.get() as i8))
                                        on_input=on_micro_timing_change
                                    />
                                </InlineParam>
                            </CollapsibleSection>

                            <CollapsibleSection
                                title="SOUND PARAMETERS"
                                default_open=true
                                badge_count=p_lock_count
                            >
                                {sound_params.iter().enumerate().map(|(idx, &name)| {
                                    view! {
                                        <InlineParam>
                                            <ParamLabel
                                                text=name
                                                locked=Signal::derive(move || is_param_locked(idx))
                                            />
                                            <NumberInput
                                                min="0"
                                                max="1"
                                                step="0.01"
                                                value=Signal::derive(move || format!("{:.2}", get_param_value(idx)))
                                                on_input=move |val| {
                                                    handle_param_input(idx, val);
                                                }
                                            />
                                        </InlineParam>
                                    }
                                }).collect::<Vec<_>>()}
                            </CollapsibleSection>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex flex-col items-center justify-center py-8 text-center animate-in fade-in duration-300">
                            <p class="text-zinc-500 text-sm italic mb-2">
                                "Select a step to"
                            </p>
                            <p class="text-zinc-500 text-sm italic mb-4">
                                "edit parameters"
                            </p>
                            <p class="text-zinc-600 text-xs">
                                "Tip: Click or right-click a step"
                            </p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
