use leptos::task::spawn_local;
use leptos::prelude::*;

#[component]

pub fn Inspector() -> impl IntoView {
    let sequencer_state = use_context::<crate::app::SequencerState>().expect("SequencerState context not found");
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");

    // Hardcode to Track 0, Subtrack 0 for this milestone
    let track_id = 0;
    let subtrack_id = 0;

    // Mock parameters
    let params = vec![
        "Tuning", "Filter Freq", "Resonance", "Drive",
        "Decay", "Sustain", "Reverb", "Delay"
    ];

    let handle_input = move |idx: usize, val: f64, param_name: String| {
        let current_step = sequencer_state.selected_step.get();

        set_pattern_signal.update(|p| {
             if let Some(track) = p.tracks.get_mut(track_id) {
                if let Some(step_idx) = current_step {
                     // P-Lock Mode
                     if let Some(subtrack) = track.subtracks.get_mut(subtrack_id) {
                         if let Some(step) = subtrack.steps.get_mut(step_idx) {
                             // Assuming params maps 1:1 to index 0..7 of p_locks
                             // p_locks is [Option<f32>; 128]
                             if idx < 128 {
                                 step.p_locks[idx] = Some(val as f32);
                             }
                        }
                     }
                      spawn_local(async move {
                        use crate::ui::tauri::push_midi_command;
                        push_midi_command("param_lock", Some(step_idx), Some(param_name), Some(val)).await;
                    });
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


    let get_value = move |idx: usize| {
        // Use with() to avoid cloning the heavy structure
        let current_step = sequencer_state.selected_step.get();
        
        pattern_signal.with(|p| {
            if let Some(track) = p.tracks.get(track_id) {
                if let Some(step_idx) = current_step {
                    // Check P-Lock
                     track.subtracks.get(subtrack_id)
                        .and_then(|st| st.steps.get(step_idx))
                        .and_then(|s| s.p_locks.get(idx).cloned().flatten())
                        .unwrap_or_else(|| track.default_params.get(idx).cloned().unwrap_or(0.0) as f32) as f64
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
         
         if let Some(step_idx) = current_step {
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
    };

    view! {
        <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800 shadow-xl mt-4">
            <div class="grid grid-cols-4 gap-x-6 gap-y-4">
                {params.into_iter().enumerate().map(|(idx, name)| {
                    let handle_input = handle_input.clone();
                    let name_str = name.to_string();
                    view! {
                        <div class="flex flex-col gap-2">
                            <label class=move || {
                                let base = "text-xs font-medium uppercase tracking-wide";
                                let color = if sequencer_state.selected_step.get().is_some() && is_locked(idx) {
                                    "text-amber-400"
                                } else {
                                    "text-zinc-400"
                                };
                                format!("{} {}", base, color)
                            }>
                                {name}
                            </label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.01"
                                prop:value=move || get_value(idx)
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                    handle_input(idx, val, name_str.clone());
                                }
                                class=move || {
                                    let base = "w-full h-2 bg-zinc-800 rounded-full appearance-none cursor-pointer transition-all";
                                    let track_color = if sequencer_state.selected_step.get().is_some() {
                                        "accent-amber-500"
                                    } else {
                                        "accent-amber-500"
                                    };
                                    format!("{} {}", base, track_color)
                                }
                            />
                             <div class="text-right text-xs font-mono text-zinc-400">
                                {move || format!("{:.2}", get_value(idx))}
                             </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>


            // LFO Section
            <div class="mt-4 pt-4 border-t border-zinc-800">
                <h3 class="text-sm font-bold text-zinc-400 mb-2">LFO 1</h3>
                <div class="grid grid-cols-2 gap-4">
                     // LFO Controls
                     <div class="flex flex-col gap-2">
                        <label class="text-xs text-zinc-500">Shape</label>
                        <select
                            class="bg-zinc-800 text-zinc-300 text-xs rounded p-1 border border-zinc-700"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
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
                        >
                            <option value="Sine">Sine</option>
                            <option value="Triangle" selected>Triangle</option>
                            <option value="Square">Square</option>
                            <option value="Random">Random</option>
                            <option value="Designer">Designer</option>
                        </select>
                        
                        <label class="text-xs text-zinc-500 mt-2">Destination</label>
                         <select
                            class="bg-zinc-800 text-zinc-300 text-xs rounded p-1 border border-zinc-700"
                            on:change=move |ev| {
                                let val = event_target_value(&ev).parse::<u8>().unwrap_or(74);
                                set_pattern_signal.update(|p| {
                                   if let Some(track) = p.tracks.get_mut(track_id) {
                                       if let Some(lfo) = track.lfos.get_mut(0) {
                                           lfo.destination = val;
                                       }
                                   }
                                });
                            }
                        >
                            <option value="74" selected>Filter Cutoff</option>
                            <option value="71">Resonance</option>
                            <option value="1">Mod Wheel</option>
                            <option value="10">Pan</option>
                        </select>
                         
                        <label class="text-xs text-zinc-500 mt-2">Amount</label>
                        <input type="range" min="-1" max="1" step="0.01" 
                            class="w-full h-2 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-yellow-500"
                            prop:value=move || {
                                pattern_signal.with(|p| p.tracks.get(track_id).and_then(|t| t.lfos.get(0)).map(|l| l.amount).unwrap_or(0.0))
                            }
                            on:input=move |ev| {
                                let val = event_target_value(&ev).parse::<f32>().unwrap_or(0.0);
                                set_pattern_signal.update(|p| {
                                    if let Some(track) = p.tracks.get_mut(track_id) {
                                         if let Some(lfo) = track.lfos.get_mut(0) {
                                             lfo.amount = val;
                                         }
                                    }
                                });
                            }
                        />
                        
                        <label class="text-xs text-zinc-500 mt-2">Speed</label>
                        <input type="range" min="0.1" max="4.0" step="0.1" 
                             class="w-full h-2 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-yellow-500"
                              prop:value=move || {
                                pattern_signal.with(|p| p.tracks.get(track_id).and_then(|t| t.lfos.get(0)).map(|l| l.speed).unwrap_or(1.0))
                            }
                            on:input=move |ev| {
                                let val = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                                set_pattern_signal.update(|p| {
                                    if let Some(track) = p.tracks.get_mut(track_id) {
                                         if let Some(lfo) = track.lfos.get_mut(0) {
                                             lfo.speed = val;
                                         }
                                    }
                                });
                            }
                        />
                     </div>
                     
                     // Designer View
                     <div class="flex flex-col gap-2">
                        {move || {
                             let is_designer = pattern_signal.with(|p| {
                                 p.tracks.get(track_id)
                                    .and_then(|t| t.lfos.get(0))
                                    .map(|l| matches!(l.shape, crate::shared::models::LFOShape::Designer(_)))
                                    .unwrap_or(false)
                             });
                             
                             if is_designer {
                                 view! {
                                     <label class="text-xs text-zinc-500">Waveform Designer</label>
                                     <label class="text-xs text-zinc-500">Waveform Designer</label>
                                     <crate::ui::components::lfo_designer::LfoDesigner
                                        track_id=Signal::derive(move || track_id)
                                        lfo_index=Signal::derive(move || 0)
                                        value=Signal::derive(move || {
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
        </div>
    }
}
