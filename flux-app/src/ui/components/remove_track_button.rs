use leptos::prelude::*;
use crate::shared::models::{Pattern, TrigType};

#[component]
pub fn RemoveTrackButton(
    /// Track index to remove
    track_idx: usize,
    /// Signal to trigger confirmation dialog
    show_confirm: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Check if this track has any data (active steps)
    let has_data = move || {
        pattern_signal.with(|pattern| {
            pattern.tracks.get(track_idx)
                .and_then(|t| t.subtracks.get(0))
                .map(|st| st.steps.iter().any(|s| s.trig_type != TrigType::None))
                .unwrap_or(false)
        })
    };

    // Check if button should be disabled (only 1 track left)
    let is_disabled = move || {
        pattern_signal.with(|p| p.tracks.len() <= 1)
    };

    let handle_click = move |_| {
        // Don't do anything if disabled
        if is_disabled() {
            return;
        }

        if has_data() {
            // Show confirmation dialog
            show_confirm.set(Some(track_idx));
        } else {
            // Remove immediately (no confirmation needed)
            do_remove_track(track_idx, set_pattern_signal);
        }
    };

    view! {
        <button
            class="w-4 h-4 text-zinc-600 hover:text-red-500 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            disabled=is_disabled
            on:click=handle_click
            title=move || if is_disabled() { "Cannot remove last track" } else { "Remove track" }
        >
            "×"
        </button>
    }
}

// Helper function to remove track and re-index
fn do_remove_track(track_idx: usize, set_pattern_signal: WriteSignal<Pattern>) {
    set_pattern_signal.update(|pattern| {
        if pattern.tracks.len() <= 1 {
            return; // Safety check
        }
        pattern.tracks.remove(track_idx);
        // Re-index remaining tracks
        for (i, track) in pattern.tracks.iter_mut().enumerate() {
            track.id = i;
        }
    });

    // Clear selected step if it became invalid
    if let Some(selected_step) = use_context::<RwSignal<Option<(usize, usize)>>>() {
        if let Some((selected_track, _)) = selected_step.get() {
            let track_count = use_context::<ReadSignal<Pattern>>()
                .expect("Pattern context not found")
                .with(|p| p.tracks.len());
            if selected_track >= track_count {
                selected_step.set(None);
            }
        }
    }
}
