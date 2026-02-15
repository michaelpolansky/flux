use crate::shared::models::Pattern;
use leptos::prelude::*;

/// Get velocity value for a specific step
fn get_velocity_value(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> u8 {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| step.velocity)  // Read from velocity field directly
        .unwrap_or(100)
}

/// Check if velocity is P-locked for a specific step
fn is_velocity_locked(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> bool {
    // Velocity cannot be P-locked in current architecture
    // It uses the dedicated step.velocity field
    false
}

/// Check if step is active (has trigger)
fn is_step_active(
    pattern: &crate::shared::models::Pattern,
    track_idx: usize,
    step_idx: usize,
) -> bool {
    pattern
        .tracks
        .get(track_idx)
        .and_then(|track| track.subtracks.get(0))
        .and_then(|subtrack| subtrack.steps.get(step_idx))
        .map(|step| step.trig_type != crate::shared::models::TrigType::None)
        .unwrap_or(false)
}

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
