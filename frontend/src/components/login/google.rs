use leptos::*;

use crate::request::BASE_URL;

#[component]
pub fn Google(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <a
                href=format!("{}api/google-login", BASE_URL)
                class="btn btn-primary"
                style:background-color="#dd4b39"
                type="submit"
            >
                <i class="bi-google"></i>
                " Sign in with google"
            </a>
        </div>
    }
}