use common::user::{UserLogin, UserLoginFields};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

use crate::{
    components::form::{
        form_fields::form_field_input::{FormFieldInput, FormFieldInputType},
        form_group::FormGroup,
        Form,
    },
    context::auth::AuthContext,
    views::auth::google_oauth::Google,
};

#[component]
pub fn Login() -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;
    let user = create_rw_signal(common::user::UserLogin::default());

    let on_submit = move |user: UserLogin| {
        let navigate = use_navigate();
        spawn_local(async move {
            let body = serde_json::to_value(user).unwrap();
            reqwasm::http::Request::post("/api/login")
                .header("content-type", "application/json")
                .body(body.to_string())
                .send()
                .await
                .unwrap();

            auth.refetch();
            navigate("/", NavigateOptions::default());
        });
    };

    view! {
        <div class="flex justify-center h-full">
            <Form values=user on_submit=on_submit>
                <FormGroup>
                    <FormFieldInput
                        placeholder="Email"
                        ty=FormFieldInputType::Email
                        name=UserLoginFields::Email
                    />

                    <FormFieldInput
                        placeholder="Password"
                        ty=FormFieldInputType::Password
                        name=UserLoginFields::Password
                    />
                </FormGroup>

                <button class="btn btn-primary" type="submit">
                    "Submit"
                </button>

                <Google/>
            </Form>
        </div>
    }
}
