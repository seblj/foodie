use crate::components::toast::toaster::Toaster;
use leptos::*;

#[component]
pub fn ToastViewer(cx: Scope, toast: RwSignal<Toaster>) -> impl IntoView {
    let t = move || {
        toast()
            .alerts
            .into_iter()
            .map(|a| a.get(cx))
            .collect::<Vec<_>>()
    };
    view! {cx,
     <>
        {t}
    </>
    }
}
