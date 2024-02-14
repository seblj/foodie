use crate::components::{form::form::assign_to_field_by_name, input::Input};
use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

pub enum FormFieldInputType {
    Text,
    Number,
}

#[component]
pub fn FormFieldInput<T, U>(
    ty: FormFieldInputType,
    name: U,
    placeholder: &'static str,
    #[prop(optional)] _ty: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();

    match ty {
        FormFieldInputType::Text => view! {
            <Input
                placeholder=placeholder
                on:input=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        *c = assign_to_field_by_name(c, name, value);
                    })
                }
            />
        }
        .into_view(),
        FormFieldInputType::Number => view! {
            <Input
                ty="number"
                placeholder=placeholder
                on:input=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        let num = serde_json::Number::from_str(&value).unwrap();
                        *c = assign_to_field_by_name(c, name, num);
                    })
                }
            />
        }
        .into_view(),
    }
}
