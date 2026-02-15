use leptos::prelude::*;
use crate::ui::components::grid_step::GridStep;
use super::step_badge::StepBadge;
use super::playhead_indicator::PlayheadIndicator;
use crate::ui::state::GridUIState;
use super::remove_track_button::RemoveTrackButton;
use super::track_controls::TrackControls;
use super::confirm_dialog::ConfirmDialog;
use super::machine_selector::MachineSelector;

#[component]
pub fn Grid() -> impl IntoView {
    let sequencer_state = use_context::<crate::app::SequencerState>().expect("SequencerState context not found");
    let playback_state = use_context::<ReadSignal<crate::ui::state::PlaybackState>>()
        .expect("PlaybackState context not found");
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<crate::shared::models::Pattern>>()
        .expect("Pattern write signal not found");

    // Create GridUIState signal and provide context
    let grid_ui_state = signal(GridUIState::default());
    provide_context(grid_ui_state.0);  // Provide read signal
    provide_context(grid_ui_state.1);  // Provide write signal

    // State for confirmation dialog
    let (show_confirm_dialog, set_show_confirm_dialog) = signal::<Option<usize>>(None);

    // Confirmation dialog message
    let confirm_message = Signal::derive(move || {
        if let Some(track_idx) = show_confirm_dialog.get() {
            format!("Track {} has active steps. Remove anyway?", track_idx + 1)
        } else {
            String::new()
        }
    });

    // Confirmation callback
    let on_confirm_remove = move || {
        if let Some(track_idx) = show_confirm_dialog.get() {
            // Call the remove function
            crate::ui::components::remove_track_button::do_remove_track(
                track_idx,
                set_pattern_signal
            );
            set_show_confirm_dialog.set(None);
        }
    };

    let on_cancel_remove = move || {
        set_show_confirm_dialog.set(None);
    };

    // Helper for timestamp
    fn current_timestamp() -> f64 {
        js_sys::Date::now()
    }

    // Create effect to detect triggers
    Effect::new(move |_| {
        let playback = playback_state.get();  // Single call to avoid race condition
        let current_time = current_timestamp();  // Capture timestamp once per effect
        let pos = playback.current_position;
        let is_playing = playback.is_playing;

        if is_playing {
            // Check each track for active steps at current position
            pattern_signal.with(|pattern| {
                for (track_idx, track) in pattern.tracks.iter().enumerate() {
                    if let Some(subtrack) = track.subtracks.get(0) {
                        if let Some(step) = subtrack.steps.get(pos) {
                            if step.trig_type != crate::shared::models::TrigType::None {
                                // Step triggered! Add to GridUIState
                                grid_ui_state.1.update(|state| {
                                    state.add_trigger(track_idx, pos, current_time);
                                });
                            }
                        }
                    }
                }
            });

            // Clean up old triggers (older than 150ms)
            grid_ui_state.1.update(|state| {
                state.cleanup_old_triggers(current_time, 150.0);
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
        <div class="sequencer-grid">
            // Grid portion with horizontal flex layout
            <div class="flex">
                // Track labels on the left (dynamic)
                <div class="flex flex-col gap-[2px] mr-2">
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="h-10 flex items-center justify-start gap-1 px-1">
                                    <div class="text-xs text-zinc-400 w-6">
                                        {format!("T{}", track_idx + 1)}
                                    </div>
                                    <MachineSelector track_idx=track_idx />
                                    <RemoveTrackButton
                                        track_idx=track_idx
                                        show_confirm=set_show_confirm_dialog
                                    />
                                </div>
                            }
                        }
                    />
                </div>

                // Grid of 4 tracks × 16 steps
                <div class="flex flex-col gap-[2px] relative">
                    <PlayheadIndicator
                        position=playback_position
                        is_playing=is_playing
                    />
                    <For
                        each=move || {
                            pattern_signal.with(|p| (0..p.tracks.len()).collect::<Vec<_>>())
                        }
                        key=|track_idx| *track_idx
                        children=move |track_idx| {
                            view! {
                                <div class="flex gap-[2px]">
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

            // Track controls below grid
            <TrackControls />
        </div>

        // Confirmation dialog (modal overlay outside grid container)
        <ConfirmDialog
            visible=Signal::derive(move || show_confirm_dialog.get().is_some())
            on_confirm=on_confirm_remove
            on_cancel=on_cancel_remove
            title="Confirm Removal"
            message=confirm_message
        />
    }
}

