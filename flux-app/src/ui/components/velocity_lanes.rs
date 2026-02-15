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

/// Check if velocity is P-locked for a specific step
fn is_velocity_locked(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> bool {
    // Velocity cannot be P-locked in current architecture
    // It uses the dedicated step.velocity field
    false
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

    view! {
        <div class="velocity-lanes border-t border-zinc-800 mt-4">
            // Section header
            <div class="py-2 px-2">
                <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                    "VELOCITY"
                </h3>
            </div>

            // Velocity grid
            <div class="flex">
                // Track labels column
                <div class="flex flex-col gap-[2px] mr-2">
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="h-10 flex items-center justify-start px-1">
                                    <div class="text-xs text-zinc-400 w-6">
                                        {format!("T{}", track_idx + 1)}
                                    </div>
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

                                            let is_locked = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    is_velocity_locked(p, track_idx, step_idx)
                                                })
                                            });

                                            let is_active = Signal::derive(move || {
                                                pattern_signal.with(|p| {
                                                    is_step_active(p, track_idx, step_idx)
                                                })
                                            });

                                            view! {
                                                <div class="w-10 h-10 bg-zinc-800/30 border border-zinc-700/50 flex items-center justify-center hover:bg-zinc-700/50 transition-colors">
                                                    <span class=move || {
                                                        let base = "text-center";
                                                        let active_class = if is_active.get() {
                                                            if is_locked.get() {
                                                                "text-amber-400 font-medium text-sm"
                                                            } else {
                                                                "text-zinc-100 text-sm"
                                                            }
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
