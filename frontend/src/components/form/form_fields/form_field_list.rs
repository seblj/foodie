use form_derive::FormFieldValues;
use leptos::{logging::log, *};
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};
use web_sys::MouseEvent;

pub fn use_form_field_list() -> Result<Callback<MouseEvent>, anyhow::Error> {
    use_context::<Callback<MouseEvent>>().ok_or_else(|| anyhow::anyhow!("Couldn't find context"))
}

#[component]
pub fn FormFieldList<T, U, V>(
    value: RwSignal<V>,
    children: Children,
    name: U,
    #[prop(optional)] _tx: PhantomData<T>,
) -> impl IntoView
where
    T: for<'de> serde::Deserialize<'de> + Serialize + Clone + form_derive::Form + 'static,
    V: 'static + Clone + form_derive::Form + Serialize + std::fmt::Debug + Default,
    U: FormFieldValues<T> + Display + Copy + 'static,
{
    let ctx = use_context::<RwSignal<T>>().unwrap();
    provide_context(value);

    let on_add = move |_: MouseEvent| {
        ctx.update(|c| {
            let mut map = match serde_json::to_value(&c) {
                Ok(serde_json::Value::Object(map)) => map,
                _ => panic!("Failed to convert to value"),
            };

            let val = serde_json::to_value(value()).unwrap();
            let arr = match map.remove(&name.to_string()) {
                Some(serde_json::Value::Array(mut arr)) => {
                    arr.push(val);
                    arr
                }
                Some(serde_json::Value::Null) => {
                    vec![val]
                }
                _ => panic!(),
            };
            map.insert(name.to_string(), serde_json::to_value(arr).unwrap());
            value.set(V::default());
            *c = serde_json::from_value(serde_json::Value::Object(map)).unwrap();
        })
    };

    provide_context(Callback::new(on_add));

    children()
}

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! { <div class="grid grid-cols-12 gap-4 justify-start">{children()}</div> }
}
