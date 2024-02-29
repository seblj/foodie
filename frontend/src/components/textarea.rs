use leptos::*;

use crate::utils::class_extender::ExtendClass;

#[component]
pub fn Textarea(
    value: RwSignal<String>,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView {
    let id = uuid::Uuid::new_v4();
    let class = class.extend_class("floating-label-textarea peer");

    view! {
        <div class="relative">
            <textarea
                prop:value=move || value.get()
                id=id.to_string()
                placeholder=placeholder
                class=class
            >
                {move || value.get_untracked()}
            </textarea>
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
