use leptos::prelude::*;

#[component]
pub fn Grid() -> impl IntoView {
    // Use Sequencer State
    let sequencer_state = use_context::<crate::app::SequencerState>().expect("SequencerState context not found");

    // Use Global Pattern State
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");

    // Hardcode to Subtrack 0 for this milestone
    let subtrack_id = 0;

    view! {
        <div class="sequencer-grid flex">
            // Track labels on the left
            <div class="flex flex-col gap-[2px] mr-2">
                <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T1</div>
                <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T2</div>
                <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T3</div>
                <div class="w-8 h-10 flex items-center justify-center text-xs text-zinc-400">T4</div>
            </div>

            // Grid of 4 tracks × 16 steps
            <div class="flex flex-col gap-[2px]">
                <For
                    each=move || {
                        (0..4).into_iter()
                    }
                    key=|track_idx| *track_idx
                    children=move |track_idx| {
                        view! {
                            <div class="grid grid-cols-16 gap-[2px]">
                                <For
                                    each=move || {
                                        (0..16).into_iter()
                                    }
                                    key=|step_idx| *step_idx
                                    children=move |step_idx| {
                                        // Create a derived signal for this specific step's active state
                                        let is_active = move || {
                                            pattern_signal.with(|p| {
                                                p.tracks.get(track_idx)
                                                    .and_then(|t| t.subtracks.get(subtrack_id))
                                                    .and_then(|st| st.steps.get(step_idx))
                                                    .map(|s| s.trig_type != crate::shared::models::TrigType::None)
                                                    .unwrap_or(false)
                                            })
                                        };

                                        view! {
                                            <button
                                                class=move || {
                                                    let base_classes = "w-10 h-10 rounded-lg transition-all duration-100 flex items-center justify-center select-none active:scale-95 hover:scale-105 focus:outline-none border";

                                                    let is_active_note = is_active();
                                                    let is_selected = sequencer_state.selected_step.get()
                                                        .map(|(tid, sidx)| tid == track_idx && sidx == step_idx)
                                                        .unwrap_or(false);

                                                    let state_classes = if is_active_note {
                                                        "bg-blue-500 hover:bg-blue-400 border-blue-400"
                                                    } else {
                                                        "bg-zinc-800 border-zinc-700 hover:bg-zinc-700"
                                                    };

                                                    let selection_classes = if is_selected {
                                                        "ring ring-amber-400"
                                                    } else {
                                                        ""
                                                    };

                                                    format!("{} {} {}", base_classes, state_classes, selection_classes)
                                                }
                                                on:click=move |_| {
                                                    sequencer_state.selected_step.set(Some((track_idx, step_idx)));
                                                }
                                            >
                                                // Visual indicator: filled circle for active, empty for inactive
                                                <span class=move || {
                                                    if is_active() {
                                                        "text-white text-lg"
                                                    } else {
                                                        "text-zinc-600 text-lg"
                                                    }
                                                }>
                                                    {move || if is_active() { "●" } else { "○" }}
                                                </span>
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

