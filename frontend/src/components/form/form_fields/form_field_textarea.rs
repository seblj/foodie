use form_derive::FormFieldValues;
use leptos::{logging::log, *};
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

use crate::components::{
    form::form_fields::{assign_to_field_by_name, get_span, get_value},
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
    let value = get_value(ctx.get_untracked(), name);

    let value_signal = create_rw_signal(value.as_string().unwrap_or_default());

    // TODO: The form stuff is starting to not look so good since I am serializing and
    // deserializing pretty much all the time.
    create_effect(move |_| {
        let val = get_value(ctx(), name);
        value_signal.set(val.as_string().unwrap_or_default());
    });

    view! {
        <div class=class>

            <Textarea
                value=value_signal
                class="w-full"
                placeholder=placeholder
                on:input=move |ev| {
                    ctx.update(|c| {
                        let value = event_target_value(&ev);
                        *c = assign_to_field_by_name(c, name, &value);
                        value_signal.set(value);
                    })
                }
            />

        </div>
    }
}
