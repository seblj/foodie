use leptos::*;

#[component]
pub fn Input(
    #[prop(optional, into)] value: Option<AttributeValue>,
    #[prop(default = "text")] ty: &'static str,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative">
            <input value=value placeholder=placeholder type=ty class="floating-label-input peer"/>
            <label class="floating-label">{placeholder}</label>
        </div>
    }
}
