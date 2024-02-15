use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

use crate::components::form::form_fields::assign_to_field_by_name;

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
            .map(|i| {
                view! { <option value=i>{i}</option> }
            })
            .collect::<Vec<_>>()
    };
    view! {
        <p>{placeholder}</p>
        <div class="d-flex">
            <select
                class="select select-bordered"
                on:change=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        if value == placeholder {
                            todo!("This should not be selectable")
                        }
                        set_hours(value.parse::<u32>().unwrap());
                        let time = chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap();
                        let val = serde_json::Value::String(time.to_string());
                        *c = assign_to_field_by_name(c, name, val);
                    })
                }
            >

                <option disabled selected>
                    "Hours"
                </option>
                {f(0, max_hours)}

            </select>
            <select
                class="select select-bordered"
                on:change=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        if value == placeholder {
                            todo!("This should not be selectable")
                        }
                        set_minutes(value.parse::<u32>().unwrap());
                        let time = chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap();
                        let val = serde_json::Value::String(time.to_string());
                        *c = assign_to_field_by_name(c, name, val);
                    })
                }
            >

                <option disabled selected>
                    "Minutes"
                </option>
                {f(0, 59)}
            </select>
        </div>
    }
}
