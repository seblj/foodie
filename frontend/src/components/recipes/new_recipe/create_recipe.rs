use std::{fmt::Display, str::FromStr};

use common::recipe::CreateRecipe;
use leptos::{leptos_dom::logging::console_log, *};

use crate::components::form::form_field::{Form, FormField, FormFieldType};

pub trait FormFieldValues<T> {}

// TODO: Create a derive macro that can derive an enum with the struct fields, and a string version
// of them. Should be used for the `FormField` component. Not optimal to deserialize and serialize
// to much inside the component, but it is also very nice to not have to think about
// `event_target_value`, and coverting it to the correct type myself everytime I create a form
#[derive(Clone, Copy)]
enum CreateRecipeFields {
    Name,
    Description,
    Instructions,
    Img,
    Servings,
    PrepTime,
    BakingTime,
    Ingredients,
}

impl FormFieldValues<CreateRecipe> for CreateRecipeFields {}

impl Display for CreateRecipeFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateRecipeFields::Name => write!(f, "name"),
            CreateRecipeFields::Description => write!(f, "description"),
            CreateRecipeFields::Instructions => write!(f, "instructions"),
            CreateRecipeFields::Img => write!(f, "img"),
            CreateRecipeFields::Servings => write!(f, "servings"),
            CreateRecipeFields::PrepTime => write!(f, "prep_time"),
            CreateRecipeFields::BakingTime => write!(f, "baking_time"),
            CreateRecipeFields::Ingredients => write!(f, "ingredients"),
        }
    }
}

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
