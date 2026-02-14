use leptos::prelude::*;
use leptos::ev;

#[component]
pub fn ConfirmDialog(
    /// Whether the dialog is visible
    visible: Signal<bool>,
    /// Callback when user confirms
    on_confirm: impl Fn() + Send + Sync + 'static,
    /// Callback when user cancels
    on_cancel: impl Fn() + Send + Sync + 'static,
    /// Dialog title
    title: &'static str,
    /// Dialog message
    message: Signal<String>,
) -> impl IntoView {
    // Store callbacks in StoredValue to allow sharing across closures
    let on_confirm = StoredValue::new(on_confirm);
    let on_cancel = StoredValue::new(on_cancel);

    // ESC key handler
    let handle_escape = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" && visible.get() {
            on_cancel.with_value(|f| f());
        }
    };

    window_event_listener(ev::keydown, handle_escape);

    view! {
        <Show when=move || visible.get()>
            <div
                class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
                on:click=move |_| on_cancel.with_value(|f| f())  // Click outside to close
            >
                <div
                    class="bg-zinc-900 border border-zinc-700 rounded-lg p-6 max-w-sm"
                    on:click=|e| e.stop_propagation()  // Prevent close when clicking inside
                >
                    <h3 class="text-lg font-medium mb-2 text-zinc-50">{title}</h3>
                    <p class="text-sm text-zinc-400 mb-4">{message}</p>
                    <div class="flex gap-2 justify-end">
                        <button
                            class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-zinc-300 transition-colors"
                            on:click=move |_| on_cancel.with_value(|f| f())
                        >
                            "Cancel"
                        </button>
                        <button
                            class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm text-zinc-50 transition-colors"
                            on:click=move |_| on_confirm.with_value(|f| f())
                        >
                            "Remove Track"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
