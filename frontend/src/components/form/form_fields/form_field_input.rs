use crate::components::input::Input;
use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

use super::assign_to_field_by_name;

pub enum FormFieldInputType {
    Text,
    Number,
    Email,
    Password,
}

impl Display for FormFieldInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormFieldInputType::Text => write!(f, "text"),
            FormFieldInputType::Password => write!(f, "password"),
            FormFieldInputType::Email => write!(f, "email"),
            FormFieldInputType::Number => write!(f, "number"),
        }
    }
}

#[component]
pub fn FormFieldInput<T, U>(
    ty: FormFieldInputType,
    name: U,
    placeholder: &'static str,
    #[prop(optional)] _tx: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();

    view! {
        <Input
            ty=ty.to_string()
            class="w-full"
            placeholder=placeholder
            on:input=move |ev| {
                ctx.update(|c| {
                    let value = event_target_value(&ev);
                    let val = if let FormFieldInputType::Number = ty {
                        serde_json::Number::from_str(&value).unwrap()
                    } else {
                        serde_json::from_str(&value).unwrap()
                    };
                    *c = assign_to_field_by_name(c, name, val);
                })
            }
        />
    }
}
