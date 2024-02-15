use std::fmt::Display;

use form_derive::FormFieldValues;
use serde::Serialize;

pub mod form_field_checkbox;
pub mod form_field_duration;
pub mod form_field_input;
pub mod form_field_select;
pub mod form_field_textarea;

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
