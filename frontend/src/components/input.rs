use leptos::*;

#[component]
pub fn Input(#[prop(optional)] r#type: &'static str, placeholder: &'static str) -> impl IntoView {
    view! { <input type=r#type placeholder=placeholder class="form-control"/> }
}
