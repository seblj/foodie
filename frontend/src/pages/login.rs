use leptos::*;

use crate::components::login::google::Google;

#[component]
pub fn Login() -> impl IntoView {
    view! { <Google/> }
}
