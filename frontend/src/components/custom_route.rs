use leptos::*;
use leptos_router::{use_location, Redirect};

use crate::{components::loading::Loading, context::auth::AuthContext};

macro_rules! public_route {
    ($component:tt) => {
        || {
            use $crate::components::custom_route::PublicRoute;
            view! {
                <PublicRoute>
                    <$component/>
                </PublicRoute>
            }
        }
    };
}
macro_rules! private_route {
    ($component:tt) => {
        || {
            // use $crate::components::custom_route::PrivateRoute;
            use $crate::components::custom_route::PublicRoute;
            view! {
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
pub fn PrivateRoute(children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;

    view! {
        {move || {
            match auth.get() {
                Some(auth) => {
                    if auth {
                        children().into_view()
                    } else {
                        view! { <Redirect path="/login"/> }.into_view()
                    }
                }
                None => view! { <Loading/> }.into_view(),
            }
        }}
    }
}

#[component]
pub fn PublicRoute(children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;
    let location = use_location();

    view! {
        {move || {
            match auth.get() {
                Some(auth) => {
                    if auth {
                        if location.pathname.get() == "/login" {
                            return view! { <Redirect path="/"/> }.into_view();
                        }
                        children().into_view()
                    } else {
                        children().into_view()
                    }
                }
                None => view! { <Loading/> }.into_view(),
            }
        }}
    }
}
