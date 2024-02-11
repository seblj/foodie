use std::{fmt::Display, str::FromStr};

use common::recipe::{CreateRecipe, CreateRecipeFields};
use leptos::{leptos_dom::logging::console_log, *};

use crate::components::form::form_field::{Form, FormField, FormFieldType};

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let recipe = common::recipe::CreateRecipe::default();

    let f = |start: usize, end: usize| {
        (start..=end)
            .map(|i| {
                view! { <option value=i>{i}</option> }
            })
            .collect::<Vec<_>>()
    };

    let on_submit = move |create_recipe: CreateRecipe| {
        console_log(&format!("recipe: {:?}", create_recipe));
    };

    // Prompt for are you sure you want to leave
    // window_event_listener(ev::beforeunload, |e| {
    //     e.set_return_value("true");
    // });
    view! {
        <Form values=recipe on_submit=on_submit>
            <ul class="steps">
                <li class="step step-primary">"Basics"</li>
                <li class="step">"Ingredients"</li>
                <li class="step">"Steps"</li>
                <li class="step">"Extra details"</li>
            </ul>

            <FormField ty=FormFieldType::Text name=CreateRecipeFields::Name placeholder="Name"/>

            <div>
                <input type="file" accept="image/*" class="file-input file-input-bordered"/>
            </div>

            <FormField
                ty=FormFieldType::Select(f(0, 72))
                name=CreateRecipeFields::Servings
                placeholder="Servings"
            />

            // Time component

            <FormField
                ty=FormFieldType::Duration(72)
                name=CreateRecipeFields::BakingTime
                placeholder="Baking time"
            />

            <FormField
                ty=FormFieldType::Duration(72)
                name=CreateRecipeFields::PrepTime
                placeholder="Prep time"
            />

            <FormField
                ty=FormFieldType::TextArea
                name=CreateRecipeFields::Instructions
                placeholder="Instructions"
            />

            <FormField
                ty=FormFieldType::TextArea
                name=CreateRecipeFields::Description
                placeholder="Description"
            />

            <button type="submit" class="btn btn-primary">
                {"Save"}
            </button>
        </Form>
    }
}
