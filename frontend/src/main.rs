use common::user::User;
use components::button::Button;
use components::navbar::Navbar;
use pages::login::Login;

use leptos::*;
use leptos_router::*;

use crate::components::custom_route::{private_route, public_route};
use crate::components::recipes::new_recipe::create_recipe::CreateRecipe;
use crate::context::auth::AuthContext;
use crate::context::toast::Toaster;
use crate::pages::home::Home;
use crate::pages::recipes::Recipes;
use crate::request::get;

mod components;
mod context;
mod pages;
mod request;

#[component]
pub fn Foo() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (name, set_name) = create_signal("".to_string());

    let fetch = move |_| {
        spawn_local(async move {
            match get::<User>("api/me").await {
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

    view! {
        <Button on:click=fetch>"Fetch foo"</Button>
        <p>{email}</p>
        <p>{name}</p>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(AuthContext::setup());

    view! {
        <Toaster>
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
                        <Route path="/recipes/create" view=private_route!(CreateRecipe)/>
                    </Routes>
                </main>
            </Router>
        </Toaster>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App/> })
}
