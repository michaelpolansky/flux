use crate::app::SequencerState;
use crate::shared::models::Pattern;
use crate::ui::state::GridUIState;
use leptos::prelude::*;

#[component]
pub fn GridStep(track_idx: usize, step_idx: usize) -> impl IntoView {
    // Get state from context
    let pattern_signal = use_context::<ReadSignal<Pattern>>().expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");
    let sequencer_state =
        use_context::<SequencerState>().expect("SequencerState context not found");
    let playback_state = use_context::<ReadSignal<crate::ui::state::PlaybackState>>()
        .expect("PlaybackState context not found");

    // Hardcode to Subtrack 0 for this milestone
    let subtrack_id = 0;

    // Compute derived state - check if this step has an active trigger
    let is_active = Signal::derive(move || {
        pattern_signal.with(|p| {
            p.tracks
                .get(track_idx)
                .and_then(|t| t.subtracks.get(subtrack_id))
                .and_then(|st| st.steps.get(step_idx))
                .map(|s| s.trig_type != crate::shared::models::TrigType::None)
                .unwrap_or(false)
        })
    });

    // Derive selection state signal
    let is_step_selected = Signal::derive(move || {
        sequencer_state
            .selected_step
            .get()
            .map(|(tid, sidx)| tid == track_idx && sidx == step_idx)
            .unwrap_or(false)
    });

    // Derive playing step state signal
    let is_playing_step = Signal::derive(move || {
        let playback = playback_state.get();
        playback.is_playing && playback.current_position == step_idx
    });

    // Get GridUIState context for trigger detection
    let grid_ui_state =
        use_context::<ReadSignal<GridUIState>>().expect("GridUIState context not found");

    let is_recently_triggered = Signal::derive(move || {
        grid_ui_state.with(|state| {
            state
                .recent_triggers
                .iter()
                .any(|t| t.track == track_idx && t.step == step_idx)
        })
    });

    // Derive complete class string signal
    let step_classes = Signal::derive(move || {
        let base_classes = "w-10 h-10 rounded-lg transition-all duration-100 flex items-center justify-center select-none active:scale-95 hover:scale-105 focus:outline-none border";

        let is_active_note = is_active.get();
        let is_selected = is_step_selected.get();

        let playing_overlay = if is_playing_step.get() {
            "bg-emerald-500/30"
        } else {
            ""
        };

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

        let beat_marker = if step_idx == 3 || step_idx == 7 || step_idx == 11 {
            "border-r-2 border-zinc-600"
        } else {
            ""
        };

        let trigger_animation = if is_recently_triggered.get() {
            "animate-pulse-once ring-2 ring-white/50" // Pulse + flash ring
        } else {
            ""
        };

        format!(
            "{} {} {} {} {} {}",
            base_classes,
            playing_overlay,
            state_classes,
            selection_classes,
            beat_marker,
            trigger_animation
        )
    });

    // Derive span class signal
    let span_classes = Signal::derive(move || {
        if is_active.get() {
            "text-white text-lg"
        } else {
            "text-zinc-600 text-lg"
        }
    });

    // Click handler - select this step
    let on_click = move |_| {
        sequencer_state
            .selected_step
            .set(Some((track_idx, step_idx)));
    };

    // Double-click handler - toggle step on/off
    let on_dblclick = move |_| {
        set_pattern_signal.update(|pattern| {
            if let Some(step) = pattern
                .tracks
                .get_mut(track_idx)
                .and_then(|t| t.subtracks.get_mut(subtrack_id))
                .and_then(|st| st.steps.get_mut(step_idx))
            {
                // Toggle between None (inactive) and Note (active)
                step.trig_type = if step.trig_type == crate::shared::models::TrigType::None {
                    crate::shared::models::TrigType::Note
                } else {
                    crate::shared::models::TrigType::None
                };
            }
        });
    };

    view! {
        <button
            class=move || step_classes.get()
            on:click=on_click
            on:dblclick=on_dblclick
        >
            // Visual indicator: filled circle for active, empty for inactive
            <span class=move || span_classes.get()>
                {move || if is_active.get() { "●" } else { "○" }}
            </span>
        </button>
    }
}
