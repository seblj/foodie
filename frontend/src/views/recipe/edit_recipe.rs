use crate::views::recipe::recipe_form::try_upload_image;
use std::time::Duration;

use common::recipe::{CreateRecipe, Recipe};
use leptos::*;
use leptos_router::use_params_map;
use web_sys::File;

use crate::{
    components::{
        form::Form,
        loading::Loading,
        not_found::NotFound,
        stepper::{Step, Stepper},
    },
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::{get, put},
    views::recipe::recipe_form::{
        recipe_info::RecipeInfo, recipe_ingredients::RecipeIngredients, recipe_steps::RecipeSteps,
    },
};

#[component]
pub fn EditRecipe() -> impl IntoView {
    let params = use_params_map();
    let toast = use_toast().unwrap();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let file = create_rw_signal::<Option<File>>(None);
    let (current_file, set_current_file) = create_signal::<Option<String>>(None);

    let recipe = create_resource(id, move |id| async move {
        let r = get(&format!("/api/recipe/{}", id))
            .send()
            .await
            .ok()?
            .json::<Recipe>()
            .await
            .ok()?;

        set_current_file(r.img.clone());

        Some(create_rw_signal(CreateRecipe::from(r)))
    });

    let on_submit = move |mut submit_data: CreateRecipe| {
        let _id = id();
        spawn_local(async move {
            // TODO: Tries to upload the image if there is one. See if I want to only
            // call this when I have an image, and not with `Option<File>`
            if let Ok(Some(img)) = try_upload_image(file.get_untracked()).await {
                submit_data.img = Some(img);
            }

            let body = serde_json::to_value(submit_data).unwrap();
            let res = put(&format!("/api/recipe/{}", _id))
                .body(body.to_string())
                .send()
                .await;

            match res {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully edited recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
                _ => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to edit recipe".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
            };
        })
    };

    view! {
        <Transition fallback=Loading>
            {move || {
                recipe
                    .get()
                    .map(|data| match data {
                        None => NotFound.into_view(),
                        Some(r) => {
                            view! {
                                <Form values=r on_submit=on_submit>
                                    <Stepper>
                                        <Step
                                            label="Basics"
                                            child=move || {
                                                view! { <RecipeInfo file=file current_file=current_file/> }
                                            }
                                        />

                                        <Step
                                            label="Ingredients"
                                            child=move || view! { <RecipeIngredients/> }
                                        />
                                        <Step label="Steps" child=move || view! { <RecipeSteps/> }/>
                                    </Stepper>

                                    // TODO: Have the save button on the final page
                                    <button type="submit" class="btn btn-primary">
                                        {"Save"}
                                    </button>
                                </Form>
                            }
                                .into_view()
                        }
                    })
            }}

        </Transition>
    }
}
