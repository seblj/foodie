use crate::utils::class_extender::ExtendClass;
use leptos::*;

#[component]
pub fn Input(
    #[prop(optional, into)] value: Option<AttributeValue>,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional, into)] ty: Option<AttributeValue>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional, into)] readonly: Option<AttributeValue>,
) -> impl IntoView {
    let class = class.extend_class("floating-label-input peer");
    let id = uuid::Uuid::new_v4();

    view! {
        <div class="relative">
            <input
                id=id.to_string()
                value=value
                placeholder=placeholder
                type=ty
                class=class
                readonly=readonly
            />
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
