use leptos::task::spawn_local;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveStep(pub Option<usize>);

#[component]
pub fn Grid() -> impl IntoView {
    let active_step_signal = use_context::<WriteSignal<ActiveStep>>().expect("ActiveStep context not found");
    // Use Global Pattern State
    let pattern_signal = use_context::<ReadSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<crate::shared::models::Pattern>>().expect("Pattern context not found");

    // Hardcode to Track 0, Subtrack 0 for this milestone
    let track_id = 0;
    let subtrack_id = 0;

    let toggle_step = move |idx: usize| {
        set_pattern_signal.update(|p| {
            if let Some(track) = p.tracks.get_mut(track_id) {
                 if let Some(subtrack) = track.subtracks.get_mut(subtrack_id) {
                     if let Some(step) = subtrack.steps.get_mut(idx) {
                         // Toggle logic: If None, make it Note. If Note, make it None. 
                         // Simplified for now.
                         use crate::shared::models::TrigType;
                         if step.trig_type == TrigType::None {
                             step.trig_type = TrigType::Note;
                         } else {
                             step.trig_type = TrigType::None;
                         }
                         
                         // Sync with Engine (Fire and Forget)
                         let val = if step.trig_type == TrigType::Note { 1.0 } else { 0.0 };
                         spawn_local(async move {
                            use crate::ui::tauri::push_midi_command;
                            push_midi_command("toggle_step", Some(idx), None, Some(val)).await;
                        });
                     }
                 }
            }
        });
    };


    let handle_mouse_down = move |idx: usize| {
        active_step_signal.set(ActiveStep(Some(idx)));
    };

    let handle_mouse_up = move |_| {
         active_step_signal.set(ActiveStep(None));
    };

    view! {
        <div class="grid grid-cols-8 gap-2 p-4 bg-zinc-900 rounded-xl border border-zinc-800 shadow-xl">
            <For
                each=move || {
                    // Return range 0..16
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
                                let base_classes = "h-12 w-12 rounded bg-zinc-800 transition-all duration-100 flex items-center justify-center text-xs font-bold select-none";
                                let active_classes = if is_active() { "bg-red-600 text-white shadow-[0_0_10px_rgba(220,38,38,0.5)]" } else { "text-zinc-500 hover:bg-zinc-700" };
                                format!("{} {}", base_classes, active_classes)
                            }
                            on:mousedown=move |_| handle_mouse_down(idx)
                            on:mouseup=move |e| handle_mouse_up(e)
                            on:mouseleave=move |e| handle_mouse_up(e)
                            on:click=move |_| toggle_step(idx)
                        >
                            {idx + 1}
                        </button>
                    }
                }
            />
        </div>
    }
}

