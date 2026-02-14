use leptos::prelude::*;
use crate::ui::components::grid_step::GridStep;

#[component]
pub fn Grid() -> impl IntoView {
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
            <div class="flex flex-col gap-[2px]">
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
        </div>
    }
}

