use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

use crate::components::{
    dropdown::{DropDown, DropDownItem},
    form::form_fields::assign_to_field_by_name,
};

#[component]
pub fn FormFieldDuration<T, U>(
    max_hours: usize,
    name: U,
    placeholder: &'static str,
    #[prop(optional)] _ty: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    U: FormFieldValues<T> + Display + Copy + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();

    let (hours, set_hours) = create_signal(0);
    let (minutes, set_minutes) = create_signal(0);

    let f = |start: usize, end: usize| {
        (start..=end)
            .map(|i| DropDownItem {
                key: i,
                label: i,
                value: i,
                checked: false,
            })
            .collect::<Vec<_>>()
    };

    let selected_minute = create_rw_signal::<Vec<DropDownItem<usize, usize, usize>>>(vec![]);
    let selected_hour = create_rw_signal::<Vec<DropDownItem<usize, usize, usize>>>(vec![]);

    create_effect(move |_| {
        if let Some(val) = selected_hour.get().first() {
            ctx.update(|c| {
                set_hours(val.value as u32);
                let time = chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap();
                let val = serde_json::Value::String(time.to_string());
                *c = assign_to_field_by_name(c, name, val);
            })
        }

        if let Some(val) = selected_minute.get().first() {
            ctx.update(|c| {
                set_minutes(val.value as u32);
                let time = chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap();
                let val = serde_json::Value::String(time.to_string());
                *c = assign_to_field_by_name(c, name, val);
            })
        }
    });

    view! {
        <p>{placeholder}</p>
        <div class="d-flex">
            <DropDown placeholder="Hours" items=f(0, max_hours) selected=selected_hour/>
            <DropDown placeholder="Minutes" items=f(0, 59) selected=selected_minute/>
        </div>
    }
}
