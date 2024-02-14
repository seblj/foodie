use leptos::*;
use wasm_bindgen::JsCast;

use crate::components::input::Input;

#[component]
pub fn DropDown<T, U>(
    items: Vec<DropDownItem<T, U>>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView
where
    T: Clone + 'static,
    U: Eq + PartialEq + Clone + std::hash::Hash + 'static,
{
    // TODO: Be able to set a default value
    let internal_items = items
        .into_iter()
        .map(|it| InternalDropDownItem {
            key: it.key,
            label: it.label,
            value: it.value,
            checked: create_rw_signal(it.checked),
        })
        .collect::<Vec<_>>();

    let (internal_items, set_internal_items) = create_signal(internal_items);
    let (selected_items, set_selected_items) = create_signal::<Vec<DropDownItem<T, U>>>(vec![]);

    let value = move || {
        let selected = selected_items();
        match selected.len() {
            0 => "".into(),
            1 => selected.first().unwrap().label.clone(),
            _ if multiple => format!("{} different selected", selected.len()),
            _ => unreachable!(),
        }
    };

    view! {
        <div class="dropdown select-bordered">
            <Input value=value placeholder=placeholder/>
            <ul
                tabindex="0"
                class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
            >

                // TODO: I don't think I need a `<For>` here. Check if it rerenders the entire list
                // with just mapping over a vec since I am mutating `item.checked`.
                <For each=internal_items key=|it| it.key.clone() let:item>
                    <ListItem
                        on:click=move |_| {
                            let is_checked = item.checked.get();
                            item.checked.set(!is_checked);
                            if !multiple {
                                set_selected_items(vec![item.clone().into()]);
                                document()
                                    .active_element()
                                    .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                                    .and_then(|el| el.blur().ok());
                                return;
                            }
                            set_selected_items
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
                </For>
            </ul>
        </div>
    }
}

#[component]
fn ListItem(
    label: String,
    #[prop(optional)] checked: Signal<bool>,
    #[prop(optional)] checkable: bool,
) -> impl IntoView {
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

// TODO: Can I not derive clone?
#[derive(Clone)]
pub struct DropDownItem<T, U>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
{
    pub key: U,
    pub label: String,
    pub value: T,
    pub checked: bool,
}

#[derive(Clone)]
struct InternalDropDownItem<T, U>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
{
    key: U,
    label: String,
    value: T,
    checked: RwSignal<bool>,
}

impl<T, U> From<InternalDropDownItem<T, U>> for DropDownItem<T, U>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
{
    fn from(value: InternalDropDownItem<T, U>) -> Self {
        Self {
            key: value.key,
            label: value.label,
            value: value.value,
            checked: value.checked.get(),
        }
    }
}
