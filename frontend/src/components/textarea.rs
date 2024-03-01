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

    let inner_value = create_rw_signal("".to_string());

    // Run an effect in case we are given a reactive value that we can subscribe to
    create_effect(move |_| {
        let v = match value.into_property() {
            Property::Value(v) => v,
            Property::Fn(f) => f(),
        };
        inner_value.set(v.as_string().unwrap_or_default());
    });

    view! {
        <div class="relative">
            <textarea
                prop:value=move || inner_value.get()
                id=id.to_string()
                placeholder=placeholder
                class=class
            >
                {move || inner_value.get_untracked()}
            </textarea>
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
