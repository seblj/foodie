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
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2.5 px-4 rounded-lg"
        >
            {children(cx)}
        </button>
    }
}
