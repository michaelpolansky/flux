use leptos::prelude::*;
use crate::app::SequencerState;
use crate::shared::models::Pattern;
use crate::ui::components::form_controls::*;

#[component]
pub fn StepEditorSidebar() -> impl IntoView {
    let sequencer_state = use_context::<SequencerState>()
        .expect("SequencerState context not found");
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

                            <div class="flex flex-col gap-3">
                                <InlineParam>
                                    <ParamLabel text="Note (Pitch)" locked=Signal::derive(|| false) />
                                    <NumberInput
                                        min="0"
                                        max="127"
                                        step="1"
                                        value=Signal::derive(move || format!("{}", note_value.get() as u8))
                                        on_input=on_note_change
                                    />
                                    <div class="flex justify-between text-xs text-zinc-500 font-mono mt-1">
                                        <span>"0 (C-1)"</span>
                                        <span>{move || {
                                            let note = note_value.get() as u8;
                                            format!("{}", note)
                                        }}</span>
                                        <span>"127 (G9)"</span>
                                    </div>
                                </InlineParam>
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
