use leptos::*;

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! { <div class="grid grid-cols-12 gap-4 justify-start">{children()}</div> }
}
