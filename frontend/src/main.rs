use components::button::{Button, ButtonProps};
use components::navbar::{Navbar, NavbarProps};
use pages::login::{Login, LoginProps};

use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::components::toast::toaster::{Toast, ToastType};
use crate::components::toast::toastviewer::{ToastViewer, ToastViewerProps};
use crate::request::{get, post};

mod components;
mod pages;
mod request;

// TODO: Share this with backend
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
            match get::<User>("api/foo").await {
                Ok(user) => {
                    set_email(user.email);
                    set_firstname(user.firstname);
                    set_lastname(user.lastname);
                }
                Err(_) => {
                    set_email.update(|val| (*val).clear());
                    set_firstname.update(|val| (*val).clear());
                    set_lastname.update(|val| (*val).clear());
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
        <p>{firstname}</p>
        <p>{lastname}</p>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // let (toast, set_toast) = create_signal(cx, 0);
    // provide_context(cx, set_toast);

    view! { cx,
        // <ToastViewer toast= Toast {
        //         r#type: ToastType::Success,
        //         body: "This is a test".to_string(),
        //         timeout: None,
        //     }
        // />
        <Router>
            <nav>
                <Navbar />
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
