use leptos::*;
use leptos_router::*;

use crate::{components::menu::Menu, context::auth::AuthContext, request::post};

#[component]
fn Profile() -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;
    let logout = move |_| {
        spawn_local(async move {
            post("/api/logout").send().await.unwrap();
            // Need to navigate before setting the state, because otherwise the wrapper router will
            // navigate to login on protected routes
            let navigate = use_navigate();
            navigate("/", Default::default());
            auth.set(false);
        });
    };

    view! {
        {move || {
            match auth.get() {
                Some(auth) => {
                    if auth {
                        view! {
                            <div class="dropdown dropdown-end">
                                <div
                                    tabindex="0"
                                    role="button"
                                    class="btn btn-ghost btn-circle avatar"
                                >
                                    <div class="w-10 rounded-full">
                                        <img
                                            alt="Tailwind CSS Navbar component"
                                            src="img/user-profile.svg"
                                        />
                                    </div>
                                </div>

                                <Menu items=vec![
                                    view! { <a>"Profile"</a> }.into_view(),
                                    view! { <a>"Settings"</a> }.into_view(),
                                    view! { <button on:click=logout>"Logout"</button> }.into_view(),
                                ]/>
                            </div>
                        }
                            .into_view()
                    } else {
                        view! {
                            <A class="nav-link" href="/login">
                                "Log in"
                            </A>
                        }
                            .into_view()
                    }
                }
                None => ().into_view(),
            }
        }}
    }
}

fn get_links() -> Vec<View> {
    vec![
        view! { <A class="nav-link" href="/"> "Home" </A> }.into_view(),
        view! { <A class="nav-link" href="foo"> "Foo" </A> }.into_view(),
        view! { <A class="nav-link" href="recipes"> "Recipes" </A> }.into_view(),
        view! { <A class="nav-link" href="recipes/create"> "Create recipe" </A> }.into_view(),
    ]
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="navbar bg-neutral">
            <div class="navbar-start">
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h8m-8 6h16"
                            ></path>
                        </svg>
                    </div>

                    <Menu items=get_links()/>
                </div>
                <A class="btn btn-ghost text-xl" href="/">
                    "Foodie"
                </A>
            </div>
            <div class="navbar-center hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    {get_links()
                        .into_iter()
                        .map(|item| {
                            view! { <li class="nav-item">{item}</li> }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
            <div class="navbar-end">
                <Profile/>
            </div>
        </div>
    }
}
