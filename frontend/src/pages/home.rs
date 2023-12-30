use std::time::Duration;

use leptos::*;

use crate::{
    components::button::Button,
    context::toast::{Toast, ToastType, Toaster},
};

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
            <Button on:click=error_toast>Add error toast</Button>
            <Button on:click=warning_toast>Add warning toast</Button>
            <Button on:click=success_toast>Add success toast</Button>
        </div>
    }
}
