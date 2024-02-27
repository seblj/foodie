use leptos::*;

use crate::utils::class_extender::ExtendClass;

#[component]
pub fn Textarea(
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView {
    let id = uuid::Uuid::new_v4();
    let class = class.extend_class("floating-label-textarea peer");
    view! {
        <div class="relative">
            <textarea id=id.to_string() placeholder=placeholder class=class></textarea>
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
