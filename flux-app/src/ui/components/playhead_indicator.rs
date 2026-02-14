use leptos::prelude::*;

#[component]
pub fn PlayheadIndicator(
    #[prop(into)] position: Signal<usize>,
    #[prop(into)] is_playing: Signal<bool>,
) -> impl IntoView {
    // Compute horizontal offset based on position (0-15)
    // Each step is w-10 (2.5rem = 40px) + gap-1 (0.25rem = 4px)
    // Total: position * 44px
    let transform = Signal::derive(move || {
        let pos = position.get();
        format!("translateX({}px)", pos * 44)
    });

    // Visibility based on is_playing
    let visible_classes = Signal::derive(move || {
        if is_playing.get() {
            "opacity-100"
        } else {
            "opacity-0 pointer-events-none"
        }
    });

    view! {
        <div
            class=move || {
                format!(
                    "absolute w-10 h-full bg-emerald-500/20 transition-transform duration-100 transition-opacity pointer-events-none {}",
                    visible_classes.get()
                )
            }
            style=move || format!("transform: {}", transform.get())
        >
        </div>
    }
}
