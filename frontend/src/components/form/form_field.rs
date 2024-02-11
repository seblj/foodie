use std::{fmt::Display, marker::PhantomData, str::FromStr};

use form_derive::FormFieldValues;
use leptos::{html::Option_, *};
use serde::Serialize;
use serde_json::Number;
use web_sys::SubmitEvent;

pub enum FormFieldType {
    Text,
    TextArea,
    Number,
    Checkbox,
    Select(Vec<HtmlElement<Option_>>),
    // Max number of hours for duration component
    Duration(usize),
}

pub enum FormFieldRules {
    Required,
    MaxLength,
}

fn assign_to_field_by_name<T, U, V>(data: &mut T, field: V, value: U) -> T
where
    T: Serialize + for<'de> serde::Deserialize<'de> + form_derive::Form,
    U: Serialize,
    V: FormFieldValues<T> + Display + Copy + 'static,
{
    let mut map = match serde_json::to_value(data) {
        Ok(serde_json::Value::Object(map)) => map,
        _ => panic!("Failed to convert to value"),
    };

    map.insert(field.to_string(), serde_json::to_value(value).unwrap());

    serde_json::from_value(serde_json::Value::Object(map)).unwrap()
}

#[component]
pub fn FormField<T, U>(
    ty: FormFieldType,
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
        FormFieldType::Text => view! {
            <input
                type="text"
                class="input input-bordered"
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
        FormFieldType::TextArea => view! {
            <textarea
                class="textarea textarea-bordered"
                placeholder=placeholder
                on:input=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        *c = assign_to_field_by_name(c, name, value);
                    })
                }
            >
            </textarea>
        }
        .into_view(),
        FormFieldType::Number => view! {
            <input
                type="number"
                class="input input-bordered"
                placeholder=placeholder
                on:input=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        let num = Number::from_str(&value).unwrap();
                        *c = assign_to_field_by_name(c, name, num);
                    })
                }
            />
        }
        .into_view(),
        FormFieldType::Checkbox => view! {
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
        .into_view(),
        FormFieldType::Select(items) => view! {
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
                {items}
            </select>
        }
        .into_view(),
        FormFieldType::Duration(max_hours) => {
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
                                let time = chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0)
                                    .unwrap();
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
                                let time = chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0)
                                    .unwrap();
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
        .into_view(),
    }
}

#[component]
pub fn Form<T, U>(values: T, children: Children, on_submit: U) -> impl IntoView
where
    T: 'static + Clone + form_derive::Form,
    U: Fn(T) + 'static,
{
    let signal = create_rw_signal(values);
    provide_context(signal);

    let internal_on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        on_submit(signal());
    };

    view! {
        <form on:submit=internal_on_submit class="flex flex-col justify-center items-center">
            {children()}
        </form>
    }
}
