use leptos::prelude::*;
use crate::shared::models::{MachineType, Pattern};

/// Convert MachineType to 2-3 letter abbreviation for compact display
pub fn machine_abbreviation(machine: MachineType) -> &'static str {
    match machine {
        MachineType::OneShot => "OS",
        MachineType::Werp => "WP",
        MachineType::Slice => "SL",
        MachineType::FmTone => "FM",
        MachineType::Subtractive => "SUB",
        MachineType::TonverkBus => "TNV",
        MachineType::MidiCC => "CC",
    }
}

/// Convert MachineType to full name for dropdown options
pub fn machine_full_name(machine: MachineType) -> &'static str {
    match machine {
        MachineType::OneShot => "OneShot",
        MachineType::Werp => "Werp",
        MachineType::Slice => "Slice",
        MachineType::FmTone => "FmTone",
        MachineType::Subtractive => "Subtractive",
        MachineType::TonverkBus => "TonverkBus",
        MachineType::MidiCC => "MidiCC",
    }
}

/// Get all machine types in order for dropdown
pub fn all_machine_types() -> [MachineType; 7] {
    [
        MachineType::OneShot,
        MachineType::Werp,
        MachineType::Slice,
        MachineType::FmTone,
        MachineType::Subtractive,
        MachineType::TonverkBus,
        MachineType::MidiCC,
    ]
}

#[component]
pub fn MachineSelector(
    track_idx: usize,
) -> impl IntoView {
    // Access Pattern context for reading/writing machine type
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");
    let set_pattern_signal = use_context::<WriteSignal<Pattern>>()
        .expect("Pattern write signal not found");

    // Local state for dropdown open/closed
    let (is_open, set_is_open) = signal(false);

    // Read current machine type for this track
    let current_machine = move || {
        pattern_signal.with(|p| {
            p.tracks.get(track_idx)
                .map(|t| t.machine)
                .unwrap_or(MachineType::OneShot)
        })
    };

    // Update machine type and close dropdown
    let set_machine = move |new_machine: MachineType| {
        set_pattern_signal.update(|pattern| {
            if let Some(track) = pattern.tracks.get_mut(track_idx) {
                track.machine = new_machine;
            }
        });
        set_is_open.set(false);
    };

    // Toggle dropdown open/closed
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    view! {
        <div class="relative">
            <button
                on:click=toggle_dropdown
                class=move || {
                    let base = "px-1.5 py-0.5 text-[10px] font-mono border rounded \
                                hover:bg-zinc-600 transition-colors cursor-pointer";
                    if is_open.get() {
                        format!("{} bg-zinc-600 border-blue-500 text-zinc-300", base)
                    } else {
                        format!("{} bg-zinc-700 border-zinc-600 text-zinc-300", base)
                    }
                }
            >
                {move || format!("{} ▾", machine_abbreviation(current_machine()))}
            </button>

            // Dropdown menu - only render when open
            <Show when=move || is_open.get()>
                <div class="absolute top-full left-0 mt-1 bg-zinc-800 border border-zinc-600 rounded shadow-lg z-50 min-w-[120px]">
                    {move || {
                        all_machine_types().iter().map(|&machine_type| {
                            let is_current = current_machine() == machine_type;
                            view! {
                                <div
                                    on:click=move |_| set_machine(machine_type)
                                    class=move || {
                                        let base = "px-3 py-1.5 text-sm text-zinc-300 cursor-pointer transition-colors";
                                        if is_current {
                                            format!("{} bg-blue-900/30 hover:bg-zinc-700", base)
                                        } else {
                                            format!("{} hover:bg-zinc-700", base)
                                        }
                                    }
                                >
                                    {machine_full_name(machine_type)}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </Show>
        </div>
    }
}
