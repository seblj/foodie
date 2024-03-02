use leptos::*;

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub Resource<(), bool>);

impl AuthContext {
    pub fn setup() -> Self {
        Self(create_resource(
            || (),
            |_| async move { get("/api/me").send().await.is_ok_and(|r| r.ok()) },
        ))
    }
}
