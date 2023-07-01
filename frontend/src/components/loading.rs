use leptos::*;

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="d-flex h-100 align-items-center justify-content-center">
            <div role="status" class="spinner-border">
                <span class="sr-only"></span>
            </div>
        </div>
    }
}
