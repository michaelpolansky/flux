use leptos::prelude::*;
use crate::app::SequencerState;

#[component]
pub fn StepEditorSidebar() -> impl IntoView {
    let sequencer_state = use_context::<SequencerState>()
        .expect("SequencerState context not found");
    let selected_step = sequencer_state.selected_step;

    view! {
        <div class="w-60 bg-zinc-900/50 border-r border-zinc-800 rounded-l-lg p-4 flex flex-col">
            {move || {
                if let Some((track_id, step_idx)) = selected_step.get() {
                    view! {
                        <div>
                            <div class="flex items-center justify-between mb-4">
                                <div class="flex flex-col">
                                    <span class="text-xs font-bold text-zinc-400 uppercase tracking-wide">"EDITING STEP"</span>
                                    <span class="text-sm text-zinc-100">
                                        {format!("Track {}, Step {}", track_id + 1, step_idx + 1)}
                                    </span>
                                </div>
                                <button
                                    class="text-xs text-zinc-500 hover:text-red-500 transition-colors focus:outline-none"
                                    on:click=move |_| selected_step.set(None)
                                >
                                    "×"
                                </button>
                            </div>

                            <div class="text-zinc-500 text-xs italic">
                                "Parameters coming soon..."
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex flex-col items-center justify-center py-8 text-center">
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
