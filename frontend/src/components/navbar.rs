use leptos::*;
use leptos_router::*;

use crate::{context::auth::AuthContext, request::post};

#[component]
fn Profile() -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;
    let logout = move |_| {
        spawn_local(async move {
            post("/api/logout", &()).await.unwrap();
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
                                            src="https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg"
                                        />
                                    </div>
                                </div>
                                <ul
                                    tabindex="0"
                                    class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                                >
                                    <li>
                                        <a>"Profile"</a>
                                    </li>
                                    <li>
                                        <a>"Settings"</a>
                                    </li>
                                    <li>
                                        <button on:click=logout>"Logout"</button>
                                    </li>
                                </ul>
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

#[component]
fn links() -> impl IntoView {
    view! {
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
        <li class="nav-item">
            <A href="recipes" class="nav-link">
                "Recipes"
            </A>
        </li>
        <li class="nav-item">
            <A href="recipes/create" class="nav-link">
                "Create recipe"
            </A>
        </li>
    }
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
                    <ul
                        tabindex="0"
                        class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                    >
                        <Links/>
                    </ul>
                </div>
                <A class="btn btn-ghost text-xl" href="/">
                    "Foodie"
                </A>
            </div>
            <div class="navbar-center hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    <Links/>
                </ul>
            </div>
            <div class="navbar-end">
                <Profile/>
            </div>
        </div>
    }
}
