use crate::components::toast::toaster::Toaster;
use leptos::*;

#[component]
pub fn ToastViewer(toast: RwSignal<Toaster>) -> impl IntoView {
    let t = move || {
        toast()
            .alerts
            .into_iter()
            .map(|a| a.get())
            .collect::<Vec<_>>()
    };
    view! { <>{t}</> }
}
