use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    // TODO: Make this pretty
    view! { cx,
        <A href="/">"Home"</A>
        <A href="foo">"Foo"</A>
    }
}
