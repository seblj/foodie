use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

use crate::components::{
    dropdown::{DropDown, DropDownItem},
    form::form_fields::assign_to_field_by_name,
};

#[component]
pub fn FormFieldSelect<T, U, V, I, S>(
    items: Vec<DropDownItem<V, I, S>>,
    name: U,
    placeholder: &'static str,
    #[prop(optional)] _ty: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
    V: Clone + Serialize + 'static,
    I: Eq + PartialEq + Clone + std::hash::Hash + 'static,
    S: std::fmt::Display + Clone + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();
    let selected_items = create_rw_signal::<Vec<DropDownItem<V, I, S>>>(vec![]);

    create_effect(move |_| {
        if let Some(item) = selected_items.get().first() {
            ctx.update(|c| {
                *c = assign_to_field_by_name(c, name, &item.value);
            })
        }
    });

    view! { <DropDown selected=selected_items placeholder=placeholder items=items/> }
}
