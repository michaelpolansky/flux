use leptos::task::spawn_local;
use leptos::prelude::*;
use crate::ui::components::form_controls::*;

#[component]

pub fn Inspector() -> impl IntoView {
    let sequencer_state = use_context::<crate::app::SequencerState>().expect("SequencerState context not found");
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");

    // Get track_id from selected step, default to 0 when no selection
    let get_track_id = move || {
        sequencer_state.selected_step.get()
            .map(|(tid, _)| tid)
            .unwrap_or(0)
    };
    let subtrack_id = 0;

    // Mock parameters
    let params = vec![
        "Tuning", "Filter Freq", "Resonance", "Drive",
        "Decay", "Sustain", "Reverb", "Delay"
    ];

    let handle_input = move |idx: usize, val: f64, param_name: String| {
        let current_step = sequencer_state.selected_step.get();
        let track_id = get_track_id();

        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some((sel_track_id, step_idx)) = current_step {
                    // P-Lock Mode
                    if sel_track_id == track_id {
                        if let Some(subtrack) = track.subtracks.get_mut(subtrack_id) {
                            if let Some(step) = subtrack.steps.get_mut(step_idx) {
                                if idx < 128 {
                                    step.p_locks[idx] = Some(val as f32);
                                }
                            }
                        }
                        spawn_local(async move {
                            use crate::ui::tauri::push_midi_command;
                            push_midi_command("param_lock", Some(step_idx), Some(param_name), Some(val)).await;
                        });
                    }
                } else {
                    // Track Default Mode
                    if idx < 128 {
                        track.default_params[idx] = val as f32;
                    }
                    spawn_local(async move {
                        use crate::ui::tauri::push_midi_command;
                        push_midi_command("param_change", None, Some(param_name), Some(val)).await;
                    });
                }
            }
        });
    };

    let toggle_step = move |step_idx: usize| {
        // Currently toggles between Note (active) and None (inactive)
        // Other TrigType variants (Lock, SynthTrigger, OneShot) not yet implemented
        let track_id = get_track_id();
        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some(subtrack) = track.subtracks.get_mut(subtrack_id) {
                    if let Some(step) = subtrack.steps.get_mut(step_idx) {
                        use crate::shared::models::TrigType;
                        if step.trig_type == TrigType::None {
                            step.trig_type = TrigType::Note;
                        } else {
                            step.trig_type = TrigType::None;
                        }

                        spawn_local(async move {
                            use crate::ui::tauri::toggle_step;
                            toggle_step(track_id, step_idx).await;
                        });
                    }
                }
            }
        });
    };

    let is_step_active = move |step_idx: usize| {
        let track_id = get_track_id();
        pattern_signal.with(|p| {
            p.tracks.get(track_id)
                .and_then(|t| t.subtracks.get(subtrack_id))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.trig_type != crate::shared::models::TrigType::None)
                .unwrap_or(false)
        })
    };

    let get_value = move |idx: usize| {
        // Use with() to avoid cloning the heavy structure
        let current_step = sequencer_state.selected_step.get();
        let track_id = get_track_id();

        pattern_signal.with(|p| {
            if let Some(track) = p.tracks.get(track_id) {
                if let Some((sel_track_id, step_idx)) = current_step {
                    if sel_track_id == track_id {
                        // Check P-Lock
                        track.subtracks.get(subtrack_id)
                            .and_then(|st| st.steps.get(step_idx))
                            .and_then(|s| s.p_locks.get(idx).cloned().flatten())
                            .unwrap_or_else(|| track.default_params.get(idx).cloned().unwrap_or(0.0) as f32) as f64
                    } else {
                        // Different track selected, show default
                        track.default_params.get(idx).cloned().unwrap_or(0.0) as f64
                    }
                } else {
                    // Track Default
                    track.default_params.get(idx).cloned().unwrap_or(0.0) as f64
                }
            } else {
                0.0
            }
        })
    };

    let is_locked = move |idx: usize| {
        let current_step = sequencer_state.selected_step.get();
        let track_id = get_track_id();

        if let Some((sel_track_id, step_idx)) = current_step {
            if sel_track_id == track_id {
                pattern_signal.with(|p| {
                    p.tracks.get(track_id)
                        .and_then(|t| t.subtracks.get(subtrack_id))
                        .and_then(|st| st.steps.get(step_idx))
                        .and_then(|s| s.p_locks.get(idx))
                        .map(|l| l.is_some())
                        .unwrap_or(false)
                })
            } else {
                false
            }
        } else {
            false
        }
    };

    view! {
        <>
        <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800 shadow-xl mt-4">
            // Header section
            <div class="flex items-center justify-between mb-4 pb-3 border-b border-zinc-800 bg-zinc-800/50 -mx-4 -mt-4 px-4 pt-4 rounded-t-xl">
                <div class="text-sm text-zinc-300">
                    {move || {
                        if let Some((track_id, step_idx)) = sequencer_state.selected_step.get() {
                            format!("Editing: Track {}, Step {}", track_id + 1, step_idx + 1)
                        } else {
                            "Editing: Track Defaults".to_string()
                        }
                    }}
                </div>

                // Active toggle button (only when step selected)
                {move || {
                    let track_id = get_track_id();
                    if let Some((sel_track_id, step_idx)) = sequencer_state.selected_step.get() {
                        if sel_track_id == track_id {
                            view! {
                                <button
                                    class=move || {
                                        let base = "px-3 py-1 rounded-lg text-xs font-medium transition-all duration-150 flex items-center gap-2";
                                        let state = if is_step_active(step_idx) {
                                            "bg-amber-500 text-black hover:bg-amber-400"
                                        } else {
                                            "bg-zinc-700 text-zinc-400 hover:bg-zinc-600"
                                        };
                                        format!("{} {}", base, state)
                                    }
                                    on:click=move |_| toggle_step(step_idx)
                                >
                                    <span class="text-base">{move || if is_step_active(step_idx) { "●" } else { "○" }}</span>
                                    "Active"
                                </button>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            // Parameter grid
            <div class="grid grid-cols-4 gap-x-4 gap-y-1">
                {params.into_iter().enumerate().map(|(idx, name)| {
                    let handle_input = handle_input.clone();
                    let name_str = name.to_string();
                    view! {
                        <InlineParam>
                            <ParamLabel
                                text=name
                                locked=Signal::derive(move || is_locked(idx))
                            />
                            <NumberInput
                                min="0"
                                max="1"
                                step="0.01"
                                value=Signal::derive(move || format!("{:.2}", get_value(idx)))
                                on_input=move |val| {
                                    let clamped = val.clamp(0.0, 1.0);
                                    handle_input(idx, clamped, name_str.clone());
                                }
                            />
                        </InlineParam>
                    }
                }).collect::<Vec<_>>()}
            </div>


        </div>

        // LFO Section - Always visible separate container
        <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800 shadow-xl mt-4">
            <div class="transition-all duration-200">
                            <h3 class="text-sm font-bold text-zinc-400 mb-3">LFO 1</h3>

                            // 4-column inline controls
                            <div class="grid grid-cols-4 gap-4 mb-3">
                                // Shape dropdown
                                <InlineParam>
                                    <ParamLabel text="Shape" locked=Signal::derive(|| false) />
                                    <Dropdown
                                        options=vec![
                                            ("Sine", "Sine"),
                                            ("Triangle", "Triangle"),
                                            ("Square", "Square"),
                                            ("Random", "Random"),
                                            ("Designer", "Designer"),
                                        ]
                                        selected=Signal::derive(move || {
                                            let track_id = get_track_id();
                                            pattern_signal.with(|p| {
                                                p.tracks.get(track_id)
                                                    .and_then(|t| t.lfos.get(0))
                                                    .map(|l| match l.shape {
                                                        crate::shared::models::LFOShape::Sine => "Sine",
                                                        crate::shared::models::LFOShape::Triangle => "Triangle",
                                                        crate::shared::models::LFOShape::Square => "Square",
                                                        crate::shared::models::LFOShape::Random => "Random",
                                                        crate::shared::models::LFOShape::Designer(_) => "Designer",
                                                    })
                                                    .unwrap_or("Triangle")
                                                    .to_string()
                                            })
                                        })
                                        on_change=move |val| {
                                            let track_id = get_track_id();
                                            set_pattern_signal.update(|p| {
                                               if let Some(track) = p.tracks.get_mut(track_id) {
                                                   if let Some(lfo) = track.lfos.get_mut(0) {
                                                        match val.as_str() {
                                                            "Sine" => lfo.shape = crate::shared::models::LFOShape::Sine,
                                                            "Triangle" => lfo.shape = crate::shared::models::LFOShape::Triangle,
                                                            "Square" => lfo.shape = crate::shared::models::LFOShape::Square,
                                                            "Random" => lfo.shape = crate::shared::models::LFOShape::Random,
                                                            "Designer" => lfo.shape = crate::shared::models::LFOShape::Designer([0.0; 16].to_vec()),
                                                            _ => {}
                                                        }
                                                    }
                                               }
                                            });
                                        }
                                    />
                                </InlineParam>

                                // Destination dropdown
                                <InlineParam>
                                    <ParamLabel text="Destination" locked=Signal::derive(|| false) />
                                    <Dropdown
                                        options=vec![
                                            ("74", "Filter Cutoff"),
                                            ("71", "Resonance"),
                                            ("1", "Mod Wheel"),
                                            ("10", "Pan"),
                                        ]
                                        selected=Signal::derive(move || {
                                            let track_id = get_track_id();
                                            pattern_signal.with(|p| {
                                                p.tracks.get(track_id)
                                                    .and_then(|t| t.lfos.get(0))
                                                    .map(|l| l.destination.to_string())
                                                    .unwrap_or_else(|| "74".to_string())
                                            })
                                        })
                                        on_change=move |val| {
                                            let parsed_val = val.parse::<u8>().unwrap_or(74);
                                            let track_id = get_track_id();
                                            set_pattern_signal.update(|p| {
                                               if let Some(track) = p.tracks.get_mut(track_id) {
                                                   if let Some(lfo) = track.lfos.get_mut(0) {
                                                       lfo.destination = parsed_val;
                                                   }
                                               }
                                            });
                                        }
                                    />
                                </InlineParam>

                                // Amount numeric input
                                <InlineParam>
                                    <ParamLabel text="Amount" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="-1"
                                        max="1"
                                        step="0.01"
                                        value=Signal::derive(move || {
                                            let track_id = get_track_id();
                                            format!("{:.2}", pattern_signal.with(|p| p.tracks.get(track_id).and_then(|t| t.lfos.get(0)).map(|l| l.amount).unwrap_or(0.0)))
                                        })
                                        on_input=move |val| {
                                            let clamped = val.clamp(-1.0, 1.0) as f32;
                                            let track_id = get_track_id();
                                            set_pattern_signal.update(|p| {
                                                if let Some(track) = p.tracks.get_mut(track_id) {
                                                     if let Some(lfo) = track.lfos.get_mut(0) {
                                                         lfo.amount = clamped;
                                                     }
                                                }
                                            });
                                        }
                                    />
                                </InlineParam>

                                // Speed numeric input
                                <InlineParam>
                                    <ParamLabel text="Speed" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="0.1"
                                        max="4.0"
                                        step="0.1"
                                        value=Signal::derive(move || {
                                            let track_id = get_track_id();
                                            format!("{:.1}", pattern_signal.with(|p| p.tracks.get(track_id).and_then(|t| t.lfos.get(0)).map(|l| l.speed).unwrap_or(1.0)))
                                        })
                                        on_input=move |val| {
                                            let clamped = val.clamp(0.1, 4.0) as f32;
                                            let track_id = get_track_id();
                                            set_pattern_signal.update(|p| {
                                                if let Some(track) = p.tracks.get_mut(track_id) {
                                                     if let Some(lfo) = track.lfos.get_mut(0) {
                                                         lfo.speed = clamped;
                                                     }
                                                }
                                            });
                                        }
                                    />
                                </InlineParam>
                            </div>

                            // Designer section
                            <div>
                                {move || {
                                     let track_id = get_track_id();
                                     let is_designer = pattern_signal.with(|p| {
                                         p.tracks.get(track_id)
                                            .and_then(|t| t.lfos.get(0))
                                            .map(|l| matches!(l.shape, crate::shared::models::LFOShape::Designer(_)))
                                            .unwrap_or(false)
                                     });

                                     if is_designer {
                                         view! {
                                             <label class="text-xs text-zinc-500">Waveform Designer</label>
                                             <crate::ui::components::lfo_designer::LfoDesigner
                                                track_id=Signal::derive(move || get_track_id())
                                                lfo_index=Signal::derive(move || 0)
                                                value=Signal::derive(move || {
                                                    let track_id = get_track_id();
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
                                                        let track_id = get_track_id();
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
                                         }.into_any()
                                     } else {
                                         view! {
                                             <div class="w-full h-32 flex items-center justify-center text-zinc-600 text-xs border border-zinc-800 rounded bg-zinc-900/50">
                                                 "Select 'Designer' shape to draw"
                                             </div>
                                         }.into_any()
                                     }
                                }}
                            </div>
                        </div>
                    </div>
        </>
    }
}
