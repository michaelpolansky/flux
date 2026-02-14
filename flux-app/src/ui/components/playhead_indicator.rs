use leptos::prelude::*;

// Grid layout constants for positioning
const STEP_WIDTH_PX: usize = 40;       // w-10 = 2.5rem
const STEP_GAP_PX: usize = 2;          // Custom gap-[2px]
const STEP_TOTAL_WIDTH: usize = STEP_WIDTH_PX + STEP_GAP_PX;

// Track label offset (w-8 + mr-2 = 32px + 8px)
const TRACK_LABEL_OFFSET_PX: usize = 40;

#[component]
pub fn PlayheadIndicator(
    #[prop(into)] position: Signal<usize>,
    #[prop(into)] is_playing: Signal<bool>,
) -> impl IntoView {
    // Compute horizontal offset based on position (0-15)
    // PlayheadIndicator is inside grid container (already after track labels)
    // so we only need to multiply position by step width, no extra offset
    let transform = Signal::derive(move || {
        let pos = position.get().min(15); // Clamp to 0-15 range
        let offset = pos * STEP_TOTAL_WIDTH;
        format!("translateX({}px)", offset)
    });

    // Visibility based on is_playing
    let visible_classes = Signal::derive(move || {
        if is_playing.get() {
            "opacity-100"
        } else {
            "opacity-0"
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
