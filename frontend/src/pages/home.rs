use std::time::Duration;

use leptos::*;

use crate::context::toast::{Toast, ToastType, Toaster};

#[component]
pub fn Home() -> impl IntoView {
    let toasts = use_context::<WriteSignal<Toaster>>().unwrap();

    let error_toast = move |_| {
        toasts.update(|t| {
            t.add(Toast {
                r#type: ToastType::Error,
                body: "Error message".to_string(),
                timeout: Some(Duration::from_secs(3)),
            })
        });
    };

    let warning_toast = move |_| {
        toasts.update(|t| {
            t.add(Toast {
                r#type: ToastType::Warning,
                body: "Warning message".to_string(),
                timeout: Some(Duration::from_secs(2)),
            })
        });
    };

    let success_toast = move |_| {
        toasts.update(|t| {
            t.add(Toast {
                r#type: ToastType::Success,
                body: "Success message".to_string(),
                timeout: Some(Duration::from_secs(1)),
            })
        });
    };

    view! {
        <div>
            <p>"Home"</p>
            <button class="btn btn-primary" on:click=error_toast>
                Add error toast
            </button>
            <button class="btn btn-primary" on:click=warning_toast>
                Add warning toast
            </button>
            <button class="btn btn-primary" on:click=success_toast>
                Add success toast
            </button>
        </div>
    }
}
