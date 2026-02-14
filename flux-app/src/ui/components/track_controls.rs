use leptos::prelude::*;
use crate::shared::models::{Pattern, Track, MachineType};

#[component]
pub fn TrackControls() -> impl IntoView {
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    let add_track = move |_| {
        set_pattern_signal.update(|pattern| {
            let new_id = pattern.tracks.len();
            let mut new_track = Track::default();
            new_track.id = new_id;
            new_track.machine = MachineType::OneShot;  // Default to OneShot
            pattern.tracks.push(new_track);
        });
    };

    let track_count = move || pattern_signal.with(|p| p.tracks.len());

    view! {
        <div class="mt-3 flex items-center gap-3">
            <button
                class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 border border-zinc-700 rounded text-sm text-zinc-300 transition-colors"
                on:click=add_track
            >
                "+ Add Track"
            </button>
            <span class="text-xs text-zinc-500 font-mono">
                {move || format!("{} tracks", track_count())}
            </span>
        </div>
    }
}
