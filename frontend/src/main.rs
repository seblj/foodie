use components::button::Button;
use components::navbar::Navbar;
use pages::login::Login;

use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::components::toast::toaster::{Toast, ToastType, Toaster};
use crate::request::{get, post};

mod components;
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
                Ok(user) => {
                    set_email(user.email);
                    set_name(user.name);
                }
                Err(_) => {
                    set_email.update(|val| (*val).clear());
                    set_name.update(|val| (*val).clear());
                }
            };
        });
    };

    let logout = move |_| {
        spawn_local(async move {
            post("api/logout", &()).await.unwrap();
        });
    };

    view! { cx,
        <Button on:click=fetch>"Fetch foo"</Button>
        <Button on:click=logout>"Log out"</Button>
        <p>{email}</p>
        <p>{name}</p>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let toast = create_rw_signal(cx, Toaster::new());
    provide_context(cx, toast);

    let add = move |_| {
        toast.update(|a| {
            a.add(Toast {
                body: "foo".to_string(),
                r#type: ToastType::Success,
                timeout: None,
            })
        });
    };

    view! { cx,
        <Router>
            <nav>
                <Navbar />
                <Button on:click=add>"Add toast"</Button>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Login/> }/>
                    <Route path="/foo" view=|cx| view! { cx, <Foo/> }/>
                </Routes>
            </main>
        </Router>
    }
}

pub fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}
