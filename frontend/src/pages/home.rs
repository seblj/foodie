use std::time::Duration;

use leptos::*;

use crate::{
    components::dropdown::{DropDown, DropDownItem},
    context::toast::{Toast, ToastType, Toaster},
};

#[component]
pub fn Home() -> impl IntoView {
    let toasts = use_context::<WriteSignal<Toaster>>().unwrap();

    let error_toast = move |_| {
        toasts.update(|t| {
            t.add(Toast {
                ty: ToastType::Error,
                body: "Error message".to_string(),
                timeout: Some(Duration::from_secs(3)),
            })
        });
    };

    let warning_toast = move |_| {
        toasts.update(|t| {
            t.add(Toast {
                ty: ToastType::Warning,
                body: "Warning message".to_string(),
                timeout: Some(Duration::from_secs(2)),
            })
        });
    };

    let success_toast = move |_| {
        toasts.update(|t| {
            t.add(Toast {
                ty: ToastType::Success,
                body: "Success message".to_string(),
                timeout: Some(Duration::from_secs(1)),
            })
        });
    };

    let items = (0..10)
        .map(|i| DropDownItem {
            key: i,
            label: format!("Item {}", i),
            value: i,
        })
        .collect::<Vec<_>>();

    view! {
        <div>
            <DropDown placeholder="Items" multiple=true items=items/>

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
        </div>
    }
}
