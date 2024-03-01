use leptos::*;
use wasm_bindgen::JsCast;

use crate::{components::input::Input, utils::class_extender::ExtendClass};

#[component]
pub fn DropDown<T, U, V, F>(
    items: Vec<DropDownItem<T, U, V>>,
    value: Signal<U>,
    on_change: F,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView
where
    T: Clone + 'static,
    U: Eq + PartialEq + Clone + std::hash::Hash + 'static,
    V: std::fmt::Display + Clone + 'static,
    F: Fn(U) + 'static + Clone,
{
    let internal_items = items
        .iter()
        .map(|it| InternalDropDownItem {
            key: it.key.clone(),
            label: it.label.clone(),
            value: it.value.clone(),
            checked: false.into(),
        })
        .collect::<Vec<_>>();

    let value = move || {
        items
            .iter()
            .find(|it| it.key == value())
            .map(|it| it.label.to_string())
            .unwrap_or_default()
    };

    let class = class.extend_class("dropdown select-bordered");

    view! {
        <div class=class>
            <Input class="w-full" value=value readonly=true placeholder=placeholder/>
            <ul
                tabindex="0"
                class="w-full dropdown-content z-[1] menu p-2 bg-base-200 rounded-box h-52 flex-nowrap overflow-y-scroll"
            >

                {internal_items
                    .into_iter()
                    .map(|item| {
                        let _on_change = on_change.clone();
                        view! {
                            <ListItem
                                on:click=move |_| {
                                    _on_change(item.key.clone());
                                    document()
                                        .active_element()
                                        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                                        .and_then(|el| el.blur().ok());
                                }

                                label=item.label.clone()
                                checkable=false
                                checked=item.checked.into()
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn ListItem<T>(
    label: T,
    #[prop(optional)] checked: Signal<bool>,
    #[prop(optional)] checkable: bool,
) -> impl IntoView
where
    T: std::fmt::Display,
{
    let label = format!("{}", label);
    let id = uuid::Uuid::new_v4();
    view! {
        <li>
            <a>
                <Show when=move || { checkable }>
                    <input id=id.to_string() checked=checked type="checkbox"/>
                </Show>

                {label}
            </a>
        </li>
    }
}

#[derive(Clone)]
pub struct DropDownItem<T, U, V>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
    V: std::fmt::Display + Clone,
{
    pub key: U,
    pub label: V,
    pub value: T,
}

#[derive(Clone)]
struct InternalDropDownItem<T, U, V>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
    V: std::fmt::Display + Clone,
{
    key: U,
    label: V,
    value: T,
    checked: RwSignal<bool>,
}

impl<T, U, V> From<InternalDropDownItem<T, U, V>> for DropDownItem<T, U, V>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
    V: std::fmt::Display + Clone,
{
    fn from(value: InternalDropDownItem<T, U, V>) -> Self {
        Self {
            key: value.key,
            label: value.label,
            value: value.value,
        }
    }
}
