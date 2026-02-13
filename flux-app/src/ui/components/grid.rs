use leptos::prelude::*;

#[component]
pub fn Grid() -> impl IntoView {
    // Use Sequencer State
    let sequencer_state = use_context::<crate::app::SequencerState>().expect("SequencerState context not found");

    // Use Global Pattern State
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");

    // Hardcode to Track 0, Subtrack 0 for this milestone
    let track_id = 0;
    let subtrack_id = 0;

    view! {
        <div class="grid grid-cols-8 gap-3">
            <For
                each=move || {
                    (0..16).into_iter()
                }
                key=|idx| *idx
                children=move |idx| {
                    // Create a derived signal for this specific step's active state
                    let is_active = move || {
                        pattern_signal.with(|p| {
                             p.tracks.get(track_id)
                                .and_then(|t| t.subtracks.get(subtrack_id))
                                .and_then(|st| st.steps.get(idx))
                                .map(|s| s.trig_type != crate::shared::models::TrigType::None)
                                .unwrap_or(false)
                        })
                    };

                    view! {
                        <button
                            class=move || {
                                let base_classes = "w-16 h-16 rounded-lg transition-all duration-100 flex items-center justify-center text-xs font-mono select-none active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-950";

                                let is_current_step = sequencer_state.current_step.get() == idx;
                                let is_active_note = is_active();
                                let is_selected = sequencer_state.selected_step.get() == Some(idx);

                                let state_classes = if is_current_step {
                                    "bg-amber-300 text-black shadow-lg scale-110 transition-transform duration-75"
                                } else if is_active_note {
                                    "bg-amber-500 text-black shadow-md"
                                } else {
                                    "bg-zinc-800 text-zinc-600 hover:bg-zinc-700"
                                };

                                let selection_classes = if is_selected {
                                    "ring-2 ring-blue-500 ring-offset-2 ring-offset-zinc-900"
                                } else {
                                    ""
                                };

                                format!("{} {} {}", base_classes, state_classes, selection_classes)
                            }
                            on:click=move |_| {
                                sequencer_state.selected_step.set(Some(idx));
                            }
                        >
                            {idx + 1}
                        </button>
                    }
                }
            />
        </div>
    }
}

