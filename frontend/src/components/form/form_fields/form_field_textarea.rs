use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

use crate::components::{
    form::form_fields::{assign_to_field_by_name, get_span},
    textarea::Textarea,
};

#[component]
pub fn FormFieldTextarea<T, U>(
    name: U,
    placeholder: &'static str,
    #[prop(optional)] span: &'static str,
    #[prop(optional)] _ty: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();

    let class = get_span(span);

    view! {
        <div class=class>
            <Textarea
                class="w-full"
                placeholder=placeholder
                on:input=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        *c = assign_to_field_by_name(c, name, value);
                    })
                }
            />

        </div>
    }
}
