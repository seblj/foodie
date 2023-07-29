use leptos::*;

use crate::components::login::google::Google;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="h-100 d-flex align-items-center justify-content-center">
            <p>"New recipe"</p>
        </div>
    }
}
