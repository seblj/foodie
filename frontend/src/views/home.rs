use std::time::Duration;

use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        dropdown::{DropDown, DropDownItem},
        input::Input,
        textarea::Textarea,
    },
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
};

#[derive(Deserialize, Serialize)]
struct RecipeImage {
    name: String,
}

#[component]
pub fn Home() -> impl IntoView {
    let toast = use_toast().unwrap();

    let error_toast = move |_| {
        toast.add(Toast {
            ty: ToastType::Error,
            body: "Error message".to_string(),
            timeout: Some(Duration::from_secs(3)),
        });
    };

    let warning_toast = move |_| {
        toast.add(Toast {
            ty: ToastType::Warning,
            body: "Warning message".to_string(),
            timeout: Some(Duration::from_secs(2)),
        })
    };

    let success_toast = move |_| {
        toast.add(Toast {
            ty: ToastType::Success,
            body: "Success message".to_string(),
            timeout: Some(Duration::from_secs(1)),
        })
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
            <Input placeholder="Name"/>

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
