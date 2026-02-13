use leptos::*;
use web_sys::{HtmlCanvasElement, MouseEvent};
use wasm_bindgen::{Closure, JsCast};

#[component]
pub fn LfoDraw(
    #[prop(into)] value: Signal<Vec<f32>>,
    #[prop(into)] set_value: WriteSignal<Vec<f32>>,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();
    let (is_drawing, set_is_drawing) = create_signal(false);

    let draw = move |canvas: &HtmlCanvasElement, data: &[f32]| {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let w = canvas.width() as f64;
        let h = canvas.height() as f64;

        // Clear
        ctx.set_fill_style(&"rgb(24, 24, 27)".into()); // zinc-900
        ctx.fill_rect(0.0, 0.0, w, h);

        // Grid lines
        ctx.set_stroke_style(&"rgba(63, 63, 70, 0.5)".into()); // zinc-700
        ctx.set_line_width(1.0);
        ctx.begin_path();
        
        // Horizontal center
        ctx.move_to(0.0, h / 2.0);
        ctx.line_to(w, h / 2.0);
        ctx.stroke();

        // Vertical lines for steps
        let step_w = w / 16.0;
        for i in 1..16 {
            let x = i as f64 * step_w;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, h);
        }
        ctx.stroke();

        if data.is_empty() { return; }

        // Draw Shape
        ctx.set_stroke_style(&"rgb(234, 179, 8)".into()); // yellow-500
        ctx.set_line_width(2.0);
        ctx.begin_path();

        for (i, &val) in data.iter().enumerate() {
            let x = (i as f64 + 0.5) * step_w; // Center of step
            // val is -1.0 to 1.0. 
            // -1.0 -> h
            // 1.0 -> 0
            // 0 -> h/2
            let y = h / 2.0 - (val as f64 * h / 2.0);
            
            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();

        // Draw points
        ctx.set_fill_style(&"rgb(234, 179, 8)".into());
        for (i, &val) in data.iter().enumerate() {
            let x = (i as f64 + 0.5) * step_w;
            let y = h / 2.0 - (val as f64 * h / 2.0);
            ctx.begin_path();
            ctx.arc(x, y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            ctx.fill();
        }
    };

    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
             let data = value.get();
             draw(&canvas, &data);
        }
    });

    let update_value = move |mx: f64, my: f64, canvas: &HtmlCanvasElement| {
        let rect = canvas.get_bounding_client_rect();
        let x = mx - rect.left();
        let y = my - rect.top();
        let w = rect.width();
        let h = rect.height();

        let step_idx = (x / w * 16.0).floor() as usize;
        let val = 1.0 - (y / h * 2.0); // Map 0..h to 1..-1
        let val = val.clamp(-1.0, 1.0) as f32;

        if step_idx < 16 {
            set_value.update(|v| {
                if v.len() != 16 {
                    *v = vec![0.0; 16];
                }
                v[step_idx] = val;
            });
        }
    };

    view! {
        <canvas
            _ref=canvas_ref
            width="400"
            height="150"
            class="w-full h-32 rounded bg-zinc-900 cursor-crosshair border border-zinc-700"
            on:mousedown=move |_| set_is_drawing(true)
            on:mouseup=move |_| set_is_drawing(false)
            on:mouseleave=move |_| set_is_drawing(false)
            on:mousemove=move |ev: MouseEvent| {
                if is_drawing.get() {
                    if let Some(canvas) = canvas_ref.get() {
                       update_value(ev.client_x() as f64, ev.client_y() as f64, &canvas);
                    }
                }
            }
            on:click=move |ev: MouseEvent| {
                 if let Some(canvas) = canvas_ref.get() {
                   update_value(ev.client_x() as f64, ev.client_y() as f64, &canvas);
                }
            }
        />
    }
}
