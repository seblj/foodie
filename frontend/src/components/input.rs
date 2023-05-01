use leptos::*;

#[component]
pub fn Input(cx: Scope, r#type: &'static str, placeholder: &'static str) -> impl IntoView {
    view! (cx,
        <input
            type={r#type}
            placeholder={placeholder}
            class="border rounded-lg p-2.5 bg-inherit border-gray-500 focus:border-[#63b3ed] focus:outline-none placeholder-gray-400"
        />
    )
}
