use leptos::*;

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional)] r#type: &'static str,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <button
            type={r#type}
            class="btn btn-primary"
        >
            {children(cx)}
        </button>
    }
}
