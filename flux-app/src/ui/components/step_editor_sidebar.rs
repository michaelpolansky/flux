use crate::app::SequencerState;
use crate::shared::models::Pattern;
use crate::ui::components::collapsible_section::CollapsibleSection;
use crate::ui::components::form_controls::*;
use crate::ui::components::lfo_designer::LfoDesigner;
use leptos::prelude::*;

/// Calculate track statistics (active steps count, P-Lock count)
/// Note: Examines only the primary subtrack (index 0) as per current single-subtrack design
fn calculate_track_stats(track: &crate::shared::models::Track) -> (usize, usize) {
    let active_steps = track
        .subtracks
        .get(0)
        .map(|st| {
            st.steps
                .iter()
                .filter(|s| s.trig_type != crate::shared::models::TrigType::None)
                .count()
        })
        .unwrap_or(0);

    let p_lock_count = track
        .subtracks
        .get(0)
        .map(|st| {
            st.steps
                .iter()
                .map(|s| s.p_locks.iter().filter(|p| p.is_some()).count())
                .sum::<usize>()
        })
        .unwrap_or(0);

    (active_steps, p_lock_count)
}

// Helper to extract track_id from selected_step
fn get_track_id_from_selection(selected_step: RwSignal<Option<(usize, usize)>>) -> usize {
    selected_step
        .get()
        .map(|(track_id, _)| track_id)
        .unwrap_or(0)
}

#[component]
pub fn StepEditorSidebar() -> impl IntoView {
    let sequencer_state =
        use_context::<SequencerState>().expect("SequencerState context not found");
    const P_LOCK_THRESHOLD: f32 = 0.001;
    let selected_step = sequencer_state.selected_step;

    let pattern_signal = use_context::<ReadSignal<Pattern>>().expect("Pattern context not found");
    let set_pattern_signal =
        use_context::<WriteSignal<Pattern>>().expect("Pattern write signal not found");

    // Get current note value
    let note_value = Signal::derive(move || {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                p.tracks
                    .get(track_id)
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
                p.tracks
                    .get(track_id)
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
                p.tracks
                    .get(track_id)
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
                p.tracks
                    .get(track_id)
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
                p.tracks
                    .get(track_id)
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
        "Tuning",
        "Filter Freq",
        "Resonance",
        "Drive",
        "Decay",
        "Sustain",
        "Reverb",
        "Delay",
    ];

    // Get sound parameter value (P-Lock or track default)
    let get_param_value = move |param_idx: usize| {
        if let Some((track_id, step_idx)) = selected_step.get() {
            pattern_signal.with(|p| {
                if let Some(track) = p.tracks.get(track_id) {
                    if let Some(subtrack) = track.subtracks.get(0) {
                        if let Some(step) = subtrack.steps.get(step_idx) {
                            // Check P-Lock first, fallback to track default
                            return step.p_locks.get(param_idx).and_then(|p| *p).unwrap_or_else(
                                || track.default_params.get(param_idx).cloned().unwrap_or(0.0),
                            );
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
                p.tracks
                    .get(track_id)
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
                            let track_default =
                                track.default_params.get(param_idx).cloned().unwrap_or(0.0);
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
                p.tracks
                    .get(track_id)
                    .and_then(|t| t.subtracks.get(0))
                    .and_then(|st| st.steps.get(step_idx))
                    .map(|s| s.p_locks.iter().filter(|p| p.is_some()).count())
                    .unwrap_or(0)
            })
        } else {
            0
        }
    });

    // LFO value derivations (track-level, not step-specific)
    let lfo_shape = Signal::derive(move || {
        let track_id = get_track_id_from_selection(selected_step);
        pattern_signal.with(|p| {
            p.tracks
                .get(track_id)
                .and_then(|t| t.lfos.get(0))
                .map(|l| match &l.shape {
                    crate::shared::models::LFOShape::Sine => "Sine",
                    crate::shared::models::LFOShape::Triangle => "Triangle",
                    crate::shared::models::LFOShape::Square => "Square",
                    crate::shared::models::LFOShape::Random => "Random",
                    crate::shared::models::LFOShape::Designer(_) => "Designer",
                })
                .unwrap_or("Triangle")
                .to_string()
        })
    });

    let lfo_amount = Signal::derive(move || {
        let track_id = get_track_id_from_selection(selected_step);
        pattern_signal.with(|p| {
            p.tracks
                .get(track_id)
                .and_then(|t| t.lfos.get(0))
                .map(|l| l.amount)
                .unwrap_or(0.0)
        })
    });

    let lfo_speed = Signal::derive(move || {
        let track_id = get_track_id_from_selection(selected_step);
        pattern_signal.with(|p| {
            p.tracks
                .get(track_id)
                .and_then(|t| t.lfos.get(0))
                .map(|l| l.speed)
                .unwrap_or(1.0)
        })
    });

    let lfo_destination = Signal::derive(move || {
        let track_id = get_track_id_from_selection(selected_step);
        pattern_signal.with(|p| {
            p.tracks
                .get(track_id)
                .and_then(|t| t.lfos.get(0))
                .map(|l| l.destination.to_string())
                .unwrap_or_else(|| "74".to_string())
        })
    });

    let is_designer = Signal::derive(move || {
        let track_id = get_track_id_from_selection(selected_step);
        pattern_signal.with(|p| {
            p.tracks
                .get(track_id)
                .and_then(|t| t.lfos.get(0))
                .map(|l| matches!(l.shape, crate::shared::models::LFOShape::Designer(_)))
                .unwrap_or(false)
        })
    });

    // LFO change handlers
    let on_shape_change = move |val: String| {
        let track_id = get_track_id_from_selection(selected_step);
        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some(lfo) = track.lfos.get_mut(0) {
                    lfo.shape = match val.as_str() {
                        "Sine" => crate::shared::models::LFOShape::Sine,
                        "Triangle" => crate::shared::models::LFOShape::Triangle,
                        "Square" => crate::shared::models::LFOShape::Square,
                        "Random" => crate::shared::models::LFOShape::Random,
                        "Designer" => crate::shared::models::LFOShape::Designer([0.0; 16].to_vec()),
                        _ => crate::shared::models::LFOShape::Triangle,
                    };
                }
            }
        });
    };

    let on_amount_change = move |val: f64| {
        let clamped = val.clamp(-1.0, 1.0) as f32;
        let track_id = get_track_id_from_selection(selected_step);
        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some(lfo) = track.lfos.get_mut(0) {
                    lfo.amount = clamped;
                }
            }
        });
    };

    let on_speed_change = move |val: f64| {
        let clamped = val.clamp(0.1, 4.0) as f32;
        let track_id = get_track_id_from_selection(selected_step);
        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some(lfo) = track.lfos.get_mut(0) {
                    lfo.speed = clamped;
                }
            }
        });
    };

    let on_destination_change = move |val: String| {
        let parsed_val = val.parse::<u8>().unwrap_or(74);
        let track_id = get_track_id_from_selection(selected_step);
        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some(lfo) = track.lfos.get_mut(0) {
                    lfo.destination = parsed_val;
                }
            }
        });
    };

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

                            <div class="flex flex-col gap-2">
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

                                <CollapsibleSection
                                    title="LFO"
                                    default_open=false
                                >
                                    <InlineParam>
                                        <ParamLabel text="Shape" locked=Signal::derive(|| false) />
                                        <Dropdown
                                            options=vec![
                                                ("Sine", "∿"),
                                                ("Triangle", "△"),
                                                ("Square", "▭"),
                                                ("Random", "※"),
                                                ("Designer", "✎"),
                                            ]
                                            selected=lfo_shape
                                            on_change=on_shape_change
                                        />
                                    </InlineParam>

                                    <InlineParam>
                                        <ParamLabel text="Amount" locked=Signal::derive(|| false) />
                                        <NumberInput
                                            min="-1"
                                            max="1"
                                            step="0.01"
                                            value=Signal::derive(move || format!("{:.2}", lfo_amount.get()))
                                            on_input=on_amount_change
                                        />
                                    </InlineParam>

                                    <InlineParam>
                                        <ParamLabel text="Speed" locked=Signal::derive(|| false) />
                                        <NumberInput
                                            min="0.1"
                                            max="4.0"
                                            step="0.1"
                                            value=Signal::derive(move || format!("{:.1}", lfo_speed.get()))
                                            on_input=on_speed_change
                                        />
                                    </InlineParam>

                                    <InlineParam>
                                        <ParamLabel text="Destination" locked=Signal::derive(|| false) />
                                        <Dropdown
                                            options=vec![
                                                ("74", "Filter Cutoff"),
                                                ("71", "Resonance"),
                                                ("1", "Mod Wheel"),
                                                ("10", "Pan"),
                                            ]
                                            selected=lfo_destination
                                            on_change=on_destination_change
                                        />
                                    </InlineParam>

                                    // Designer waveform editor (conditional)
                                    {move || {
                                        if is_designer.get() {
                                            view! {
                                                <div class="mt-2">
                                                    <label class="text-xs text-zinc-500 mb-1 block">"Waveform Designer"</label>
                                                    <LfoDesigner
                                                        track_id=Signal::derive(move || get_track_id_from_selection(selected_step))
                                                        lfo_index=Signal::derive(move || 0)
                                                        value=Signal::derive(move || {
                                                            let track_id = get_track_id_from_selection(selected_step);
                                                            pattern_signal.with(|p| {
                                                                p.tracks.get(track_id)
                                                                    .and_then(|t| t.lfos.get(0))
                                                                    .and_then(|l| {
                                                                        if let crate::shared::models::LFOShape::Designer(v) = &l.shape {
                                                                            Some(v.to_vec())
                                                                        } else {
                                                                            None
                                                                        }
                                                                    })
                                                                    .unwrap_or_else(|| vec![0.0; 16])
                                                            })
                                                        })
                                                        on_change=Callback::new(move |new_val: Vec<f32>| {
                                                            if new_val.len() == 16 {
                                                                let mut arr = [0.0; 16];
                                                                arr.copy_from_slice(&new_val);
                                                                let track_id = get_track_id_from_selection(selected_step);
                                                                set_pattern_signal.update(|p| {
                                                                    if let Some(track) = p.tracks.get_mut(track_id) {
                                                                        if let Some(lfo) = track.lfos.get_mut(0) {
                                                                            lfo.shape = crate::shared::models::LFOShape::Designer(arr.to_vec());
                                                                        }
                                                                    }
                                                                });
                                                            }
                                                        })
                                                    />
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                </CollapsibleSection>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex flex-col h-full animate-in fade-in duration-200">
                            // Header
                            <div class="mb-4">
                                <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                                    "PATTERN OVERVIEW"
                                </h3>
                            </div>

                            // Track summary table
                            <div class="flex-1 overflow-y-auto">
                                <table class="w-full text-sm">
                                    <thead>
                                        <tr class="text-xs text-zinc-500 border-b border-zinc-800">
                                            <th class="text-left pb-2 pr-2">"Track"</th>
                                            <th class="text-left pb-2 pr-2">"Machine"</th>
                                            <th class="text-right pb-2 pr-2">"Steps"</th>
                                            <th class="text-right pb-2">"P-Locks"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {move || {
                                            pattern_signal.with(|pattern| {
                                                if pattern.tracks.is_empty() {
                                                    view! {
                                                        <tr>
                                                            <td colspan="4" class="text-center py-8 text-zinc-500 text-sm italic">
                                                                "No tracks in pattern"
                                                            </td>
                                                        </tr>
                                                    }.into_any()
                                                } else {
                                                    pattern.tracks.iter().enumerate().map(|(idx, track)| {
                                                        let (active_steps, p_locks) = calculate_track_stats(track);
                                                        let machine_name = format!("{:?}", track.machine);

                                                        view! {
                                                            <tr class="border-b border-zinc-800/50 hover:bg-zinc-800/30 transition-colors">
                                                                <td class="py-2 pr-2 text-zinc-100">
                                                                    {format!("T{}", idx + 1)}
                                                                </td>
                                                                <td class="py-2 pr-2 text-zinc-100 truncate">
                                                                    {machine_name}
                                                                </td>
                                                                <td class={format!(
                                                                    "py-2 pr-2 text-right {}",
                                                                    if active_steps == 0 { "text-zinc-600" } else { "text-zinc-100" }
                                                                )}>
                                                                    {active_steps}
                                                                </td>
                                                                <td class={format!(
                                                                    "py-2 text-right {}",
                                                                    if p_locks == 0 { "text-zinc-600" } else { "text-zinc-100" }
                                                                )}>
                                                                    {p_locks}
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>().into_any()
                                                }
                                            })
                                        }}
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
