use std::time::Duration;

use crate::{
    components::stepper::{Step, Stepper},
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    views::recipe::new_recipe::{
        recipe_info::RecipeInfo, recipe_ingredients::RecipeIngredients, recipe_steps::RecipeSteps,
    },
};

use common::recipe::CreateRecipe;
use leptos::{logging::log, *};

use crate::components::form::Form;

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let recipe = create_rw_signal(common::recipe::CreateRecipe::default());

    let toast = use_toast().unwrap();

    let on_submit = move |create_recipe: CreateRecipe| {
        log!("{:?}", create_recipe);
        toast.add(Toast {
            ty: ToastType::Success,
            body: "Successfully created recipe".to_string(),
            timeout: Some(Duration::from_secs(5)),
        })
    };

    view! {
        <Form values=recipe on_submit=on_submit>
            <Stepper>
                <Step label="Basics" child=move || view! { <RecipeInfo/> }/>
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
