use leptos::*;

#[component]
pub fn Input(
    cx: Scope,
    #[prop(optional)] r#type: &'static str,
    placeholder: &'static str,
) -> impl IntoView {
    view! { cx, <input type=r#type placeholder=placeholder class="form-control"/> }
}
