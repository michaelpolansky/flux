use leptos::prelude::*;

#[component]
pub fn CollapsibleSection(
    /// Section title text
    title: &'static str,
    /// Whether section is expanded by default
    #[prop(default = true)]
    default_open: bool,
    /// Optional badge count (e.g., P-Lock count)
    #[prop(optional)]
    badge_count: Option<Signal<usize>>,
    /// Child content to show when expanded
    children: Children,
) -> impl IntoView {
    let is_open = RwSignal::new(default_open);

    view! {
        <div class="flex flex-col pb-3 border-b border-zinc-800/50 last:border-b-0">
            // Header (clickable)
            <div
                class="flex items-center justify-between px-2 py-1.5 cursor-pointer hover:bg-zinc-800/50 rounded transition-colors duration-150"
                on:click=move |_| is_open.update(|open| *open = !*open)
            >
                <div class="flex items-center gap-2">
                    // Expand/collapse indicator
                    <span class="text-zinc-400 text-xs">
                        {move || if is_open.get() { "▼" } else { "▶" }}
                    </span>
                    // Title
                    <span class="text-xs font-bold text-zinc-400 uppercase tracking-wide">
                        {title}
                    </span>
                    // Optional badge
                    {move || {
                        if let Some(count_signal) = badge_count {
                            let count = count_signal.get();
                            if count > 0 {
                                view! {
                                    <span class="text-xs text-amber-400 ml-1">
                                        {format!("({})", count)}
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>
            </div>

            // Content (conditional)
            <div
                class=move || {
                    if is_open.get() {
                        "flex flex-col gap-3 mt-2 transition-all duration-200 animate-in slide-in-from-top-2 fade-in"
                    } else {
                        "hidden"
                    }
                }
            >
                {children()}
            </div>
        </div>
    }
}
