use common::user::User;
use components::navbar::Navbar;

use leptos::*;
use leptos_router::*;

use crate::components::custom_route::{private_route, public_route};
use crate::context::auth::AuthContext;
use crate::context::toast::Toaster;
use crate::request::get;
use crate::views::auth::login_page::Login;
use crate::views::home::Home;
use crate::views::recipe::new_recipe::create_recipe::CreateRecipe;
use crate::views::recipe::recipes::Recipes;

mod components;
mod context;
mod request;
mod utils;
mod views;

#[component]
pub fn Foo() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (name, set_name) = create_signal("".to_string());

    let fetch = move |_| {
        spawn_local(async move {
            match get::<User>("/api/me").await {
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
        <button class="btn btn-primary" on:click=fetch>
            "Fetch foo"
        </button>
        <p>{email}</p>
        <p>{name}</p>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    provide_context(AuthContext::setup());

    mount_to_body(|| {
        view! {
            <Toaster>
                <Router>
                    <nav class="sticky top-0 z-[9999]">
                        <Navbar/>
                    </nav>
                    // Hack to subtract height of navbar for main
                    <main class="h-[calc(100vh-68px)] p-4 w-full">
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
    })
}
