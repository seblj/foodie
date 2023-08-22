use leptos::*;

use crate::components::login::google::Google;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="h-100 d-flex align-items-center justify-content-center">
            <Google/>
        </div>
    }
}
