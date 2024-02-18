use std::time::Duration;

use leptos::{logging::log, *};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::{
    components::dropdown::{DropDown, DropDownItem},
    context::toast::{Toast, ToastType, Toaster},
    request::{get, post},
};

#[derive(Deserialize, Serialize)]
struct RecipeImage {
    name: String,
}

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
            checked: false,
        })
        .collect::<Vec<_>>();

    let on_file = |e: Event| {
        let element = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let files = element.files().unwrap();
        let file = files.get(0).unwrap();
        spawn_local(async move {
            let res = post("api/recipe/image", &RecipeImage { name: file.name() })
                .await
                .unwrap();

            let url: RecipeImage = res.json().await.unwrap();
            let res = reqwasm::http::Request::put(&url.name)
                .body(file.value_of())
                .send()
                .await;
        })
    };

    let (has_image, set_has_image) = create_signal::<String>("".to_string());

    let fooo = move |_| {
        spawn_local(async move {
            let img: RecipeImage = get("api/recipe/image/1").await.unwrap().unwrap();

            set_has_image(img.name);
        });
    };
    view! {
        <div>
            {move || {
                if !has_image().is_empty() {
                    view! { <img src=has_image/> }.into_view()
                } else {
                    ().into_view()
                }
            }}
            <button on:click=fooo>foo</button> <div>
                <input
                    type="file"
                    on:change=on_file
                    accept="image/*"
                    class="file-input file-input-bordered"
                />
            </div> <DropDown placeholder="Items" multiple=true items=items/>
            // <Input placeholder="Name"/>

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
