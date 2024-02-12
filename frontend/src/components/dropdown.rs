use leptos::{logging::log, *};

// TODO: I don't want to derive Clone I think
#[derive(Clone)]
pub struct DropDownItem<T>
where
    T: Clone,
{
    pub key: usize,
    pub label: String,
    pub value: T,
}

#[component]
pub fn DropDown<T>(
    items: Vec<DropDownItem<T>>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView
where
    T: Clone + 'static,
{
    // TODO: Be able to set a default value
    let (selected_items, set_selected_items) = create_signal::<Vec<DropDownItem<T>>>(vec![]);
    provide_context(set_selected_items);

    // TODO: Be able to set a default value
    let (value, set_value) = create_signal::<String>("".into());

    create_effect(move |_| {
        let selected = selected_items();
        if selected.len() > 0 {
            if multiple && selected.len() > 1 {
                set_value(format!("{} different selected", selected.len()));
            } else {
                set_value(selected.first().unwrap().label.clone());
            }
        } else {
            set_value("".into());
        }
        let a = selected.iter().map(|s| s.label.clone()).collect::<Vec<_>>();
        log!("{:?}", a);
    });

    view! {
        <div class="dropdown select-bordered">
            <div class="relative">
                <input
                    placeholder=placeholder
                    value=value
                    type="text"
                    role="button"
                    class="floating-label-input peer"
                />
                <label class="floating-label">{placeholder}</label>
            </div>
            <ul
                tabindex="0"
                class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
            >

                {items
                    .into_iter()
                    .map(|it| {
                        view! { <ListItem item=it multiple=multiple/> }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}

#[component]
pub fn ListItem<T>(item: DropDownItem<T>, #[prop(default = false)] multiple: bool) -> impl IntoView
where
    T: Clone + 'static,
{
    let set_selected_items = use_context::<WriteSignal<Vec<DropDownItem<T>>>>().unwrap();

    // TODO: Should be able to set a default value
    let (checked, set_checked) = create_signal(false);

    let label = item.label.clone();

    let on_click_item = move |_| {
        log!("Clicking");
        if checked() {
            if multiple {
                set_selected_items.update(|v| {
                    v.retain(|i| i.key != item.key);
                });
            }
        } else {
            if multiple {
                set_selected_items.update(|v| {
                    v.push(item.clone());
                });
            } else {
                // TODO: Should not allocate a new vec here :(
                set_selected_items(vec![item.clone()]);
            }
        }

        if multiple {
            set_checked(!checked());
        }
    };

    view! {
        <li>
            <a on:click=on_click_item>
                <Show when=move || { multiple }>
                    <input checked=checked type="checkbox"/>
                </Show>

                {label}
            </a>
        </li>
    }
}
