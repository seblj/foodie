use components::button::{Button, ButtonProps};
use components::navbar::{Navbar, NavbarProps};
use pages::login::{Login, LoginProps};

use leptos::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::request::{get, post};

mod components;
mod pages;
mod request;

#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! { cx,
        <div>
            <Navbar />
            <Button on:click=clear>"Clear"</Button>
            <Button on:click=decrement>"-1"</Button>
            <span>"Value: " {value} "!"</span>
            <Button on:click=increment>"+1"</Button>
        </div>
    }
}

#[derive(Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

#[component]
pub fn Foo(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (firstname, set_firstname) = create_signal(cx, "".to_string());
    let (lastname, set_lastname) = create_signal(cx, "".to_string());

    let fetch = move |_| {
        spawn_local(async move {
            let user = get::<User>("api/foo").await.unwrap();
            set_email(user.email);
            set_firstname(user.firstname);
            set_lastname(user.lastname);
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
        <p>{firstname}</p>
        <p>{lastname}</p>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Foo />
        <Login />
        // <SimpleCounter initial_value=0 />
    }
}

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}
