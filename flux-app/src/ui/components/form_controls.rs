use leptos::prelude::*;

/// Consistent label styling for parameters and LFO controls
#[component]
pub fn ParamLabel(
    /// The label text to display
    text: &'static str,
    /// Whether this parameter is locked (shows amber color)
    #[prop(optional, into)]
    locked: Signal<bool>,
) -> impl IntoView {
    view! {
        <label class=move || {
            let base = "text-[10px] font-medium uppercase tracking-tight flex-shrink-0 w-24";
            let color = if locked.get() {
                "text-amber-400"
            } else {
                "text-zinc-400"
            };
            format!("{} {}", base, color)
        }>
            {text}
        </label>
    }
}

/// Consistent number input styling
#[component]
pub fn NumberInput(
    /// Minimum value
    min: &'static str,
    /// Maximum value
    max: &'static str,
    /// Step increment
    step: &'static str,
    /// Reactive value to display
    #[prop(into)]
    value: Signal<String>,
    /// Callback when value changes
    on_input: impl Fn(f64) + 'static,
) -> impl IntoView {
    view! {
        <input
            type="number"
            min=min
            max=max
            step=step
            prop:value=move || value.get()
            on:input=move |ev| {
                let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                on_input(val);
            }
            class="w-16 text-xs text-center bg-zinc-800 border border-zinc-700 rounded px-1.5 py-0.5 text-zinc-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900 transition-colors [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
        />
    }
}

/// Consistent dropdown/select styling
#[component]
pub fn Dropdown(
    /// The options to display (value, display text)
    options: Vec<(&'static str, &'static str)>,
    /// Currently selected value
    #[prop(into)]
    selected: Signal<String>,
    /// Callback when selection changes
    on_change: impl Fn(String) + 'static,
) -> impl IntoView {
    view! {
        <select
            prop:value=move || selected.get()
            on:change=move |ev| {
                on_change(event_target_value(&ev));
            }
            class="bg-zinc-800 text-zinc-50 text-xs rounded px-1.5 py-0.5 border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
        >
            {options.into_iter().map(|(value, text)| {
                view! {
                    <option value=value>{text}</option>
                }
            }).collect::<Vec<_>>()}
        </select>
    }
}

/// Container for inline parameter layout (label + control)
#[component]
pub fn InlineParam(children: Children) -> impl IntoView {
    view! {
        <div class="flex items-center gap-0.5">
            {children()}
        </div>
    }
}
