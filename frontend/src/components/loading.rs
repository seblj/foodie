use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="d-flex h-100 align-items-center justify-content-center">
            <div role="status" class="spinner-border">
                <span class="sr-only"></span>
            </div>
        </div>
    }
}
