use leptos::{leptos_dom::console_log, *};
use leptos_router::{use_location, use_navigate, Redirect};

use crate::{components::loading::Loading, context::auth::AuthContext};

#[component]
pub fn PrivateRoute(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>(cx).unwrap().0;
    let navigate = use_navigate(cx);

    let children = store_value(cx, children);
    console_log(&format!("auth: {:?}", auth.read(cx)));

    view! { cx,
        {move || {
            match auth.read(cx) {
                Some(auth) => {
                    if auth {
                        view! { cx, {children.with_value(|children| children(cx))} }.into_view(cx)
                    } else {
                        view! { cx, <Redirect path="/login"/> }.into_view(cx)
                    }
                }
                None => view! { cx, <Loading/> }.into_view(cx),
            }
        }}
    }
}

#[component]
pub fn PublicRoute(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>(cx).unwrap().0;
    let location = use_location(cx);
    let navigate = use_navigate(cx);

    let children = store_value(cx, children);

    view! { cx,
        {move || {
            match auth.read(cx) {
                Some(auth) => {
                    if auth {
                        if location.pathname.get() == "/login" {
                            return view! { cx, <Redirect path="/"/> }.into_view(cx);
                        }
                        view! { cx, {children.with_value(|children| children(cx))} }.into_view(cx)
                    } else {
                        view! { cx, {children.with_value(|children| children(cx))} }.into_view(cx)
                    }
                }
                None => view! { cx, <Loading/> }.into_view(cx),
            }
        }}
    }
}
