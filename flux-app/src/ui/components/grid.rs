use leptos::prelude::*;
use crate::ui::components::grid_step::GridStep;
use super::step_badge::StepBadge;
use super::playhead_indicator::PlayheadIndicator;
use crate::ui::state::GridUIState;

#[component]
pub fn Grid() -> impl IntoView {
    let sequencer_state = use_context::<crate::app::SequencerState>().expect("SequencerState context not found");
    let playback_state = use_context::<ReadSignal<crate::ui::state::PlaybackState>>()
        .expect("PlaybackState context not found");
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>()
        .expect("Pattern context not found");

    // Create GridUIState signal and provide context
    let grid_ui_state = signal(GridUIState::default());
    provide_context(grid_ui_state.0);  // Provide read signal
    provide_context(grid_ui_state.1);  // Provide write signal

    // Helper for timestamp
    fn current_timestamp() -> f64 {
        js_sys::Date::now()
    }

    // Create effect to detect triggers
    Effect::new(move |_| {
        let pos = playback_state.get().current_position;
        let is_playing = playback_state.get().is_playing;

        if is_playing {
            // Check each track for active steps at current position
            pattern_signal.with(|pattern| {
                for (track_idx, track) in pattern.tracks.iter().enumerate() {
                    if let Some(subtrack) = track.subtracks.get(0) {
                        if let Some(step) = subtrack.steps.get(pos) {
                            if step.trig_type != crate::shared::models::TrigType::None {
                                // Step triggered! Add to GridUIState
                                grid_ui_state.1.update(|state| {
                                    state.add_trigger(track_idx, pos, current_timestamp());
                                });
                            }
                        }
                    }
                }
            });

            // Clean up old triggers (older than 150ms)
            grid_ui_state.1.update(|state| {
                state.cleanup_old_triggers(current_timestamp(), 150.0);
            });
        }
    });

    let selected_track = Signal::derive(move || {
        sequencer_state.selected_step.get()
            .map(|(track, _)| track)
            .unwrap_or(0)
    });

    let selected_step_idx = Signal::derive(move || {
        sequencer_state.selected_step.get()
            .map(|(_, step)| step)
            .unwrap_or(0)
    });

    let badge_visible = Signal::derive(move || {
        sequencer_state.selected_step.get().is_some()
    });

    let playback_position = Signal::derive(move || {
        playback_state.get().current_position
    });

    let is_playing = Signal::derive(move || {
        playback_state.get().is_playing
    });

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
            <div class="flex flex-col gap-[2px] relative">
                <PlayheadIndicator
                    position=playback_position
                    is_playing=is_playing
                />
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
                                        view! {
                                            <GridStep track_idx=track_idx step_idx=step_idx />
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </div>

            <StepBadge
                track=selected_track
                step=selected_step_idx
                visible=badge_visible
            />
        </div>
    }
}

