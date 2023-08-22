use leptos::*;

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub Resource<(), bool>);

impl AuthContext {
    pub fn setup() -> Self {
        let auth = create_resource(|| (), |_| async move { get::<()>("api/me").await.is_ok() });
        Self(auth)
    }
}
