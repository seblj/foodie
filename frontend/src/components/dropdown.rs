use leptos::*;
use wasm_bindgen::JsCast;

use crate::components::input::Input;

#[component]
pub fn DropDown<T, U, V>(
    items: Vec<DropDownItem<T, U, V>>,
    #[prop(optional)] selected: RwSignal<Vec<DropDownItem<T, U, V>>>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] multiple: bool,
) -> impl IntoView
where
    T: Clone + 'static,
    U: Eq + PartialEq + Clone + std::hash::Hash + 'static,
    V: std::fmt::Display + Clone + 'static,
{
    let internal_items = items
        .into_iter()
        .map(|it| InternalDropDownItem {
            key: it.key,
            label: it.label,
            value: it.value,
            checked: create_rw_signal(it.checked),
        })
        .collect::<Vec<_>>();

    let value = move || {
        let selected = selected();
        match selected.len() {
            0 => "".into(),
            1 => format!("{}", selected[0].label),
            len if multiple => format!("{} different selected", len),
            _ => unreachable!(),
        }
    };

    view! {
        <div class="dropdown select-bordered">
            <Input value=value readonly=true placeholder=placeholder/>
            <ul
                tabindex="0"
                class="dropdown-content z-[1] menu p-2 bg-base-200 rounded-box w-52 h-52 flex-nowrap overflow-y-scroll"
            >

                {internal_items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <ListItem
                                on:click=move |_| {
                                    let is_checked = item.checked.get();
                                    item.checked.set(!is_checked);
                                    if !multiple {
                                        selected.set(vec![item.clone().into()]);
                                        document()
                                            .active_element()
                                            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                                            .and_then(|el| el.blur().ok());
                                        return;
                                    }
                                    selected
                                        .update(|v| {
                                            if is_checked {
                                                v.retain_mut(|i| i.key != item.key);
                                            } else {
                                                v.push(item.clone().into());
                                            }
                                        });
                                }

                                label=item.label.clone()
                                checkable=multiple
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
    view! {
        <li>
            <a>
                <Show when=move || { checkable }>
                    <input checked=checked type="checkbox"/>
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
    pub checked: bool,
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
            checked: value.checked.get(),
        }
    }
}
