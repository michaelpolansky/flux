use crate::shared::models::Pattern;
use leptos::prelude::*;

#[component]
pub fn VelocityLanes() -> impl IntoView {
    // Access shared context
    let pattern_signal = use_context::<ReadSignal<Pattern>>()
        .expect("Pattern context not found");

    view! {
        <div class="velocity-lanes border-t border-zinc-800 mt-4">
            <div class="py-2 px-2">
                <h3 class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                    "VELOCITY"
                </h3>
            </div>
            <div class="velocity-grid">
                <p class="text-zinc-500 text-sm p-4">"Velocity lanes component"</p>
            </div>
        </div>
    }
}
