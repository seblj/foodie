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
    view! {
        <div class="relative">
            <input value=value placeholder=placeholder type=ty class=class readonly=readonly/>
            <label class="floating-label">{placeholder}</label>
        </div>
    }
}
