use common::user::User;
use components::button::Button;
use components::navbar::Navbar;
use pages::login::Login;

use leptos::*;
use leptos_router::*;

use crate::components::create_recipe::CreateRecipe;
use crate::components::custom_route::{private_route, public_route};
use crate::context::auth::AuthContext;
use crate::pages::home::Home;
use crate::pages::recipes::Recipes;
use crate::request::get;

mod components;
mod context;
mod pages;
mod request;

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
                    <Route path="/" view=public_route!(Home)/>
                    <Route path="/login" view=public_route!(Login)/>
                    <Route path="/foo" view=private_route!(Foo)/>
                    <Route path="/recipes" view=private_route!(Recipes)/>
                    // TODO: Use an actual page for it, and not just the component
                    <Route path="/recipes/create" view=private_route!(CreateRecipe)/>
                </Routes>
            </main>
        </Router>
    }
}

pub fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
