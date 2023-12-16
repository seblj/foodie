use leptos::*;

use crate::request::BASE_URL;

#[component]
pub fn Google() -> impl IntoView {
    view! {
        <div>
            <a
                href=format!("{}api/oauth/google/login", BASE_URL)
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
