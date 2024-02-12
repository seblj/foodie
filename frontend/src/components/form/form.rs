use std::fmt::Display;

use form_derive::FormFieldValues;
use leptos::*;
use serde::Serialize;
use web_sys::SubmitEvent;

pub(super) fn assign_to_field_by_name<T, U, V>(data: &mut T, field: V, value: U) -> T
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
