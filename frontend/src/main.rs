use components::button::{Button, ButtonProps};
use components::navbar::{Navbar, NavbarProps};

use leptos::*;

mod components;

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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <SimpleCounter initial_value=0 />
    }
}

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}
