use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicU64, Ordering},
        OnceLock,
    },
    time::Duration,
};

use leptos::prelude::*;

static TOASTS: OnceLock<RwSignal<Vec<Toast>>> = OnceLock::new();
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[component]
pub fn Toasts() -> impl IntoView {
    let toasts = toasts();

    view! {
        <div class="flex fixed right-4 bottom-4 z-50 flex-col gap-2 w-96 pointer-events-none print:hidden max-w-[calc(100vw-2rem)]">
            <For
                each=move || toasts.get()
                key=|t| t.id
                children=move |t| {
                    view! {
                        <div
                            class="p-3 border shadow-lg pointer-events-auto rounded-default border-danger bg-surface-deep animate-fade-in"
                            role="alert"
                        >
                            <p class="font-semibold text-ink">
                                "A Rust panic killed part of the page — details below"
                            </p>
                            <p class="overflow-auto mt-1 max-h-64 font-mono text-xs whitespace-pre-wrap text-ink-muted">
                                {t.message}
                            </p>
                        </div>
                    }
                }
            />
        </div>
    }
}

pub fn create_toast(info: impl Display) {
    let toasts = toasts();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let message = format!("{info}");
    toasts.write().push(Toast { id, message });
    set_timeout(
        move || {
            toasts.write().retain(|toast| toast.id != id);
        },
        Duration::from_secs(5),
    );
}

fn toasts() -> RwSignal<Vec<Toast>> {
    *TOASTS.get_or_init(|| RwSignal::new(Vec::new()))
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub message: String,
}
