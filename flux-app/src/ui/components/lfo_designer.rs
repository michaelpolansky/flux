use leptos::prelude::*;
use web_sys::MouseEvent;
use crate::ui::tauri::set_lfo_designer_value;
use leptos::task::spawn_local;
use leptos::html::Div;

#[component]
pub fn LfoDesigner(
    #[prop(into)] track_id: Signal<usize>,
    #[prop(into)] lfo_index: Signal<usize>,
    #[prop(into)] value: Signal<Vec<f32>>, // Expecting 16 values
    #[prop(into)] on_change: Callback<Vec<f32>>, // For local state update
) -> impl IntoView {
    let (is_drawing, set_is_drawing) = signal::<bool>(false);
    let container_ref = NodeRef::<Div>::new();

    let update_value = move |e: MouseEvent| {
        if let Some(div) = container_ref.get() {
            let rect = div.get_bounding_client_rect();
            let x = e.client_x() as f64 - rect.left();
            let width = rect.width();
            let step_width = width / 16.0;
            
            let step_idx = (x / step_width).floor().max(0.0).min(15.0) as usize;
            
            // Calculate Y value (0.0 at bottom, 1.0 at top? Or -1.0 to 1.0?)
            // Model expects -1.0 to 1.0 usually for LFOs, but Designer is often 0-1 unipolar or -1 to 1.
            // Let's assume -1.0 to 1.0.
            // Mouse Y is from top.
            let y = e.client_y() as f64 - rect.top();
            let height = rect.height();
            
            // Normalize to 0.0 - 1.0 (inverted because Y is down)
            let normalized = 1.0 - (y / height);
            // Map to -1.0 to 1.0
            let mapped = (normalized * 2.0 - 1.0).max(-1.0_f64).min(1.0_f64);
            
            // Update local state is tricky with props. 
            // We invoke the callback to let parent update the signal, 
            // OR we just fire the command and assume parent will reflect it eventually?
            // "Instant feedback" requires local update.
            // But `value` is a Signal.
            
            // We need to clone the current values to modify one.
            let mut current_values = value.get();
            if step_idx < current_values.len() {
                current_values[step_idx] = mapped as f32;
                on_change.run(current_values);
                
                // Fire Command
                spawn_local(async move {
                    set_lfo_designer_value(track_id.get(), lfo_index.get(), step_idx, mapped as f32).await;
                });
            }
        }
    };

    let on_mousedown = move |e: MouseEvent| {
        set_is_drawing.set(true);
        update_value(e);
    };

    let on_mousemove = move |e: MouseEvent| {
        if is_drawing.get() {
            update_value(e);
        }
    };

    let on_mouseup = move |_| {
        set_is_drawing.set(false);
    };
    
    // Global mouseup to catch drag outside? 
    // For now stick to svg events.

    view! {
        <div
            node_ref=container_ref
            class="w-full h-32 bg-gray-900 border border-gray-700 cursor-crosshair rounded relative"
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:mouseleave=on_mouseup
        >
            <svg 
                class="w-full h-full pointer-events-none" // Events handled by parent div
                viewBox="0 0 160 100" 
                preserveAspectRatio="none"
            >
                // Grid lines
                { (0..16).map(|i| view! {
                    <line x1={i * 10} y1="0" x2={i * 10} y2="100" stroke="#333" stroke-width="0.5" />
                }).collect::<Vec<_>>() }
                <line x1="0" y1="50" x2="160" y2="50" stroke="#555" stroke-width="0.5" />

                // Bars
                {move || {
                    value.get().iter().enumerate().map(|(i, &val): (usize, &f32)| {
                        // Map -1.0..1.0 to 0..100 (Y coordinates, 0 is top)
                        // val=1.0 -> y=0
                        // val=-1.0 -> y=100
                        // val=0.0 -> y=50

                        // y start depends on polarity
                        // If pos: y = 50 - height
                        // If neg: y = 50
                        
                        let y = if val >= 0.0 { 50.0 - (val * 50.0) } else { 50.0 };
                        let h = val.abs() * 50.0;
                        
                        view! {
                            <rect 
                                x={i * 10} 
                                y={y} 
                                width="9" 
                                height={h.max(0.5)} 
                                fill="#FACC15" // Yellow-400
                                class="hover:fill-yellow-300"
                            />
                        }
                    }).collect::<Vec<_>>()
                }}
            </svg>
        </div>
    }
}
