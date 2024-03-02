use std::time::Duration;

use crate::{
    components::stepper::{Step, Stepper},
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::{get, post},
    views::recipe::new_recipe::{
        recipe_info::RecipeInfo, recipe_ingredients::RecipeIngredients, recipe_steps::RecipeSteps,
    },
};

use common::recipe::{CreateRecipe, RecipeImage};
use leptos::*;
use web_sys::File;

use crate::components::form::Form;

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let recipe = create_rw_signal(common::recipe::CreateRecipe::default());
    let toast = use_toast().unwrap();

    let file = create_rw_signal::<Option<File>>(None);

    let on_submit = move |mut create_recipe: CreateRecipe| {
        spawn_local(async move {
            if let Ok(Some(img)) = try_upload_image(file.get_untracked()).await {
                create_recipe.img = Some(img);
            }

            let body = serde_json::to_value(create_recipe).unwrap();
            let res = post("/api/recipe").body(body.to_string()).send().await;

            match res {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully uploaded recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
                _ => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to upload recipe".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
            };
        })
    };

    view! {
        <Form values=recipe on_submit=on_submit>
            <Stepper>
                <Step label="Basics" child=move || view! { <RecipeInfo file=file/> }/>
                <Step label="Ingredients" child=move || view! { <RecipeIngredients/> }/>
                <Step label="Steps" child=move || view! { <RecipeSteps/> }/>
            </Stepper>

            // TODO: Have the save button on the final page
            <button type="submit" class="btn btn-primary">
                {"Save"}
            </button>
        </Form>
    }
}

async fn try_upload_image(file: Option<File>) -> Result<Option<String>, anyhow::Error> {
    let toast = use_toast().unwrap();

    let Some(file) = file else {
        return Ok(None);
    };

    let image = match get("/api/recipe/image").send().await {
        Ok(res) if res.ok() => res.json::<RecipeImage>().await?,
        _ => {
            toast.add(Toast {
                ty: ToastType::Error,
                body: "Failed to upload image".to_string(),
                timeout: Some(Duration::from_secs(5)),
            });
            return Err(anyhow::anyhow!("Couldn't upload file"));
        }
    };

    reqwasm::http::Request::put(&image.url)
        .body(file.value_of())
        .send()
        .await?;

    Ok(Some(image.id))
}
