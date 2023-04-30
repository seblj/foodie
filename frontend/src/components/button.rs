use leptos::*;

#[component]
pub fn Button(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            {children(cx)}
        </button>
    }
}
