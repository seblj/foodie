use leptos::*;

use crate::utils::class_extender::ExtendClass;

#[component]
pub fn Textarea<T>(
    value: T,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView
where
    T: IntoProperty + 'static + Copy,
{
    let id = uuid::Uuid::new_v4();
    let class = class.extend_class("floating-label-textarea peer");

    let inner_value = move || {
        let v = match value.into_property() {
            Property::Value(v) => v,
            Property::Fn(f) => f(),
        };
        v.as_string().unwrap_or_default()
    };

    view! {
        <div class="relative">
            <textarea prop:value=inner_value id=id.to_string() placeholder=placeholder class=class>
                {inner_value}
            </textarea>
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
