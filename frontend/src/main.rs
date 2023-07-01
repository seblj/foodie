use components::button::Button;
use components::navbar::Navbar;
use pages::login::Login;

use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::components::custom_route::{PrivateRoute, PublicRoute};
use crate::context::auth::AuthContext;
use crate::pages::home::Home;
use crate::request::get;

mod components;
mod context;
mod pages;
mod request;

// TODO: Share this with backend
#[derive(Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

#[component]
pub fn Foo(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (name, set_name) = create_signal(cx, "".to_string());

    let fetch = move |_| {
        spawn_local(async move {
            match get::<User>("api/foo").await {
                Ok(Some(user)) => {
                    set_email(user.email);
                    set_name(user.name);
                }
                _ => {
                    set_email.update(|val| (*val).clear());
                    set_name.update(|val| (*val).clear());
                }
            };
        });
    };

    view! { cx,
        <Button on:click=fetch>"Fetch foo"</Button>
        <p>{email}</p>
        <p>{name}</p>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, AuthContext::setup(cx));

    view! { cx,
        <Router>
            <nav>
                <Navbar/>
            </nav>
            <main style="height: 100%;">
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx,
                                <PublicRoute>
                                    <Home/>
                                </PublicRoute>
                            }
                        }
                    />
                    <Route
                        path="/login"
                        view=|cx| {
                            view! { cx,
                                <PublicRoute>
                                    <Login/>
                                </PublicRoute>
                            }
                        }
                    />
                    <Route
                        path="/foo"
                        view=|cx| {
                            view! { cx,
                                <PrivateRoute>
                                    <Foo/>
                                </PrivateRoute>
                            }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

pub fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
