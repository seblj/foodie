use crate::components::button::{Button, ButtonProps};
use crate::components::input::{Input, InputProps};
use crate::request::post;
use leptos::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};

// TODO: Should share the structs with the backend in some way
#[derive(Serialize, Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (password, set_password) = create_signal(cx, "".to_string());

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            post(
                "api/login",
                &LoginForm {
                    email: email(),
                    password: password(),
                },
            )
            .await
            .unwrap();
        });
    };

    view! (cx,
        <form on:submit=submit>
            <Input r#type="email" placeholder="Email" on:input=move |ev| {
                set_email(event_target_value(&ev));
            } />
            <Input r#type="password" placeholder="Password" on:input=move |ev| {
                set_password(event_target_value(&ev));
            }/>
            <Button r#type="submit">"Log in"</Button>
        </form>
    )
}
