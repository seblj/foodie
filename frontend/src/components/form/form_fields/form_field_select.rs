use crate::components::form::form::assign_to_field_by_name;
use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

#[component]
pub fn FormFieldSelect<T, U, V>(
    items: Vec<V>,
    name: U,
    placeholder: &'static str,
    #[prop(optional)] _ty: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
    V: IntoView + Eq + PartialEq + 'static + IntoAttribute + Clone,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();

    view! {
        <select
            class="select select-bordered"
            on:change=move |ev| {
                ctx.update(|c| {
                    let value = event_target_value(&ev);
                    if value == placeholder {
                        todo!("This should not be selectable")
                    }
                    let val = if let Ok(num) = serde_json::Number::from_str(&value) {
                        serde_json::Value::Number(num)
                    } else {
                        serde_json::Value::String(value)
                    };
                    *c = assign_to_field_by_name(c, name, val);
                })
            }
        >

            <option disabled selected>
                {placeholder}
            </option>

            {items
                .into_iter()
                .map(|i| {
                    let attr = i.clone();
                    view! { <option value=attr>{i}</option> }
                })
                .collect_view()}

        </select>
    }
}
