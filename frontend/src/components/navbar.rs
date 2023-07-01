use leptos::*;
use leptos_router::*;

use crate::{context::auth::AuthContext, request::post};

#[component]
fn Profile(cx: Scope, #[prop(optional)] mobile: bool) -> impl IntoView {
    let auth = use_context::<AuthContext>(cx).unwrap().0;
    let class = if mobile {
        "d-sm-none"
    } else {
        "d-none d-sm-block"
    };

    let logout = move |_| {
        spawn_local(async move {
            post("api/logout", &()).await.unwrap();
            // Need to navigate before setting the state, because otherwise the wrapper router will
            // navigate to login on protected routes
            let navigate = use_navigate(cx);
            navigate("/", Default::default()).unwrap();
            auth.set(false);
        });
    };

    view! { cx,
        {move || {
            match auth.read(cx) {
                Some(auth) => {
                    if auth {
                        view! { cx,
                            <div class="dropdown">
                                <i
                                    class=format!("bi bi-person-circle {}", class)
                                    type="button"
                                    data-bs-toggle="dropdown"
                                    style="font-size: 25px;"
                                ></i>
                                <div class="dropdown-menu dropdown-menu-end">
                                    <button class="dropdown-item" on:click=logout>
                                        "Log out"
                                    </button>
                                </div>
                            </div>
                        }
                            .into_view(cx)
                    } else {
                        view! { cx,
                            <A class=format!("nav-link {}", class) href="/login">
                                "Log in"
                            </A>
                        }
                            .into_view(cx)
                    }
                }
                None => ().into_view(cx),
            }
        }}
    }
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="navbar navbar-expand-sm">
            <div class="container-fluid">
                <button
                    class="navbar-toggler"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <A href="/" class="navbar-brand">
                    "Icon"
                </A>
                <Profile mobile=true/>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item">
                            <A class="nav-link" href="/">
                                "Home"
                            </A>
                        </li>
                        <li class="nav-item">
                            <A href="foo" class="nav-link">
                                "Foo"
                            </A>
                        </li>
                    </ul>
                </div>
                <Profile/>
            </div>
        </nav>
    }
}
