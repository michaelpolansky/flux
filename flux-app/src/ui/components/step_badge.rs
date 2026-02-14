use leptos::prelude::*;

#[component]
pub fn StepBadge(
    #[prop(into)] track: Signal<usize>,
    #[prop(into)] step: Signal<usize>,
    #[prop(into)] visible: Signal<bool>,
) -> impl IntoView {
    // Format: "T{track}・S{step}" (1-indexed for display)
    let badge_text = Signal::derive(move || {
        format!("T{}・S{}", track.get() + 1, step.get() + 1)
    });

    view! {
        <div
            class=move || {
                let base = "bg-zinc-900/90 backdrop-blur text-amber-400 text-xs px-2 py-0.5 rounded transition-opacity duration-200";
                let visibility = if visible.get() { "opacity-100" } else { "opacity-0 pointer-events-none" };
                format!("{} {}", base, visibility)
            }
        >
            {move || badge_text.get()}
        </div>
    }
}
