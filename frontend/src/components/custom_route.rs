use leptos::*;
use leptos_router::{use_location, Redirect};

use crate::{components::loading::Loading, context::auth::AuthContext};

macro_rules! public_route {
    ($component:tt) => {
        |cx| {
            use $crate::components::custom_route::PublicRoute;
            view! { cx,
                <PublicRoute>
                    <$component/>
                </PublicRoute>
            }
        }
    };
}
macro_rules! private_route {
    ($component:tt) => {
        |cx| {
            // use $crate::components::custom_route::PrivateRoute;
            use $crate::components::custom_route::PublicRoute;
            view! { cx,
                // TODO: Use PrivateRoute once auth is working again
                <PublicRoute>
                    <$component/>
                </PublicRoute>
            }
        }
    };
}

pub(crate) use private_route;
pub(crate) use public_route;

#[component]
pub fn PrivateRoute(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>(cx).unwrap().0;

    view! { cx,
        {move || {
            match auth.read(cx) {
                Some(auth) => {
                    if auth {
                        children(cx).into_view(cx)
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

    view! { cx,
        {move || {
            match auth.read(cx) {
                Some(auth) => {
                    if auth {
                        if location.pathname.get() == "/login" {
                            return view! { cx, <Redirect path="/"/> }.into_view(cx);
                        }
                        children(cx).into_view(cx)
                    } else {
                        children(cx).into_view(cx)
                    }
                }
                None => view! { cx, <Loading/> }.into_view(cx),
            }
        }}
    }
}
