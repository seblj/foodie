use leptos::*;

#[component]
pub fn Button(#[prop(optional)] r#type: &'static str, children: Children) -> impl IntoView {
    view! {
        <button type=r#type class="btn btn-primary">
            {children()}
        </button>
    }
}
