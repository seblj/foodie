use crate::{
    components::stepper::{Step, Stepper},
    views::recipe::new_recipe::{
        recipe_info::RecipeInfo, recipe_ingredients::RecipeIngredients, recipe_steps::RecipeSteps,
    },
};

use common::recipe::CreateRecipe;
use leptos::{logging::log, *};

use crate::components::form::Form;

#[component]
pub fn CreateRecipe() -> impl IntoView {
    // Prompt for are you sure you want to leave
    // window_event_listener(ev::beforeunload, |e| {
    //     e.set_return_value("true");
    // });

    let recipe = create_rw_signal(common::recipe::CreateRecipe::default());

    let on_submit = move |create_recipe: CreateRecipe| {
        log!("{:?}", create_recipe);
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
