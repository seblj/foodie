use crate::components::form::form::assign_to_field_by_name;
use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

#[component]
pub fn FormFieldCheckbox<T, U>(
    name: U,
    placeholder: &'static str,
    #[prop(optional)] _ty: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();

    view! {
        <input
            type="checkbox"
            class="checkbox"
            placeholder=placeholder
            on:input=move |ev| {
                ctx.update(|c| {
                    let value = event_target_checked(&ev);
                    *c = assign_to_field_by_name(c, name, value);
                })
            }
        />
    }
}
