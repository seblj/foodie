use common::user::User;
use components::navbar::Navbar;
use components::not_found::NotFound;

use leptos::*;
use leptos_router::*;

use crate::components::custom_route::{private_route, public_route};
use crate::context::auth::AuthContext;
use crate::context::toast::Toaster;
use crate::request::get;
use crate::views::auth::login_page::Login;
use crate::views::home::Home;
use crate::views::recipe::create_recipe::CreateRecipe;
use crate::views::recipe::edit_recipe::EditRecipe;
use crate::views::recipe::recipe::Recipe;
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
            let Ok(res) = get("/api/me").send().await else {
                set_email.update(|val| (*val).clear());
                set_name.update(|val| (*val).clear());
                return;
            };

            match res.json::<User>().await {
                Ok(user) => {
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
                    <main class="px-4 pt-4 pb-16 w-full">
                        <Routes>
                            <Route path="/" view=public_route!(Home)/>
                            <Route path="/login" view=public_route!(Login)/>
                            <Route path="/foo" view=private_route!(Foo)/>
                            <Route path="/recipes/:id" view=private_route!(Recipe)/>
                            <Route path="/recipes/:id/edit" view=private_route!(EditRecipe)/>
                            <Route path="/recipes" view=private_route!(Recipes)/>
                            <Route path="/recipes/create" view=private_route!(CreateRecipe)/>
                            <Route path="/*" view=public_route!(NotFound)/>
                        </Routes>
                    </main>
                </Router>
            </Toaster>
        }
    })
}
