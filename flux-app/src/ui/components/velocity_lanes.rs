use crate::shared::models::Pattern;
use leptos::prelude::*;

/// Get velocity value for a specific step
fn get_velocity_value(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> u8 {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| step.velocity)  // Read from velocity field directly
        .unwrap_or(100)
}

/// Check if step is active (has trigger)
fn is_step_active(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> bool {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| step.trig_type != crate::shared::models::TrigType::None)
        .unwrap_or(false)
}

#[component]
pub fn VelocityLanes() -> impl IntoView {
    // Access shared context
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Drag state
    let (drag_state, set_drag_state) = signal::<Option<(usize, usize)>>(None);
    let (drag_start_y, set_drag_start_y) = signal::<Option<f64>>(None);
    let (drag_start_value, set_drag_start_value) = signal::<Option<u8>>(None);

    view! {
        <div
            class="velocity-lanes border-t border-zinc-800 mt-4"
            on:mousemove=move |ev| {
                if let Some((t_idx, s_idx)) = drag_state.get() {
                    if let (Some(start_y), Some(start_val)) = (drag_start_y.get(), drag_start_value.get()) {
                        let delta = start_y - ev.client_y() as f64;
                        let new_velocity = ((start_val as i32 + delta as i32).clamp(0, 127)) as u8;

                        // Update pattern
                        set_pattern_signal.update(|pattern| {
                            if let Some(step) = pattern
                                .tracks
                                .get_mut(t_idx)
                                .and_then(|t| t.subtracks.get_mut(0))
                                .and_then(|st| st.steps.get_mut(s_idx))
                            {
                                step.velocity = new_velocity;
                            }
                        });
                    }
                }
            }
            on:mouseup=move |_| {
                set_drag_state.set(None);
                set_drag_start_y.set(None);
                set_drag_start_value.set(None);
            }
        >
            // Section header
            <div class="py-2 px-2">
                <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                    "VELOCITY"
                </h3>
            </div>

            // Velocity grid
            <div class="flex">
                // Track labels column (must match step grid width exactly)
                <div class="flex flex-col gap-[2px] mr-2">
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="h-10 flex items-center justify-start gap-1 px-1">
                                    // Invisible spacer matching RemoveTrackButton (w-4)
                                    <div class="w-4"></div>
                                    <div class="text-xs text-zinc-400 w-6">
                                        {format!("T{}", track_idx + 1)}
                                    </div>
                                    // Invisible spacer matching MachineSelector button
                                    // Button has px-1.5 (6px) + text content ~24-32px = ~36-40px
                                    <button class="px-1.5 py-0.5 text-[10px] font-mono opacity-0 pointer-events-none">
                                        "OS ▾"
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>

                // Velocity cells grid
                <div class="flex flex-col gap-[2px]">
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="flex gap-[2px]">
                                    <For
                                        each=move || (0..16)
                                        key=|step_idx| *step_idx
                                        children=move |step_idx| {
                                            let value_signal = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    get_velocity_value(p, track_idx, step_idx)
                                                })
                                            });

                                            let is_active = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    is_step_active(p, track_idx, step_idx)
                                                })
                                            });

                                            view! {
                                                <div
                                                    class=move || {
                                                        let base = "w-10 h-10 bg-zinc-800/30 border border-zinc-700/50 flex items-center justify-center hover:bg-zinc-700/50 transition-colors";
                                                        let cursor = "cursor-ns-resize";
                                                        format!("{} {}", base, cursor)
                                                    }
                                                    on:mousedown=move |ev| {
                                                        ev.prevent_default();
                                                        set_drag_state.set(Some((track_idx, step_idx)));
                                                        set_drag_start_y.set(Some(ev.client_y() as f64));
                                                        let current_value = pattern_signal.with(|p| get_velocity_value(p, track_idx, step_idx));
                                                        set_drag_start_value.set(Some(current_value));
                                                    }
                                                >
                                                    <span class=move || {
                                                        let base = "text-center";
                                                        let active_class = if is_active.get() {
                                                            "text-zinc-100 text-sm"
                                                        } else {
                                                            "text-zinc-600 text-xs"
                                                        };
                                                        format!("{} {}", base, active_class)
                                                    }>
                                                        {move || {
                                                            if is_active.get() {
                                                                format!("{}", value_signal.get())
                                                            } else {
                                                                "--".to_string()
                                                            }
                                                        }}
                                                    </span>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
