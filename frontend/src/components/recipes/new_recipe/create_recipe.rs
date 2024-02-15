use common::recipe::{CreateRecipe, CreateRecipeFields};
use leptos::{logging::log, *};

use crate::components::form::{
    form_fields::{
        form_field_duration::FormFieldDuration,
        form_field_input::{FormFieldInput, FormFieldInputType},
        form_field_select::FormFieldSelect,
        form_field_textarea::FormFieldTextarea,
    },
    Form,
};

#[component]
pub fn CreateRecipe() -> impl IntoView {
    // TODO: I don't know if I want `<Form>` to take in a `T` or `RwSignal<T>`. If I use
    // `RwSignal<T>`, then I am able to for example use `create_effect`. However, I don't know if I
    // want it to be possible to write to the form outside of the form component.
    let recipe = common::recipe::CreateRecipe::default();

    let on_submit = move |create_recipe: CreateRecipe| {
        log!("{:?}", create_recipe);
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

            <FormFieldInput
                ty=FormFieldInputType::Text
                name=CreateRecipeFields::Name
                placeholder="Name"
            />

            <div>
                <input type="file" accept="image/*" class="file-input file-input-bordered"/>
            </div>

            <FormFieldSelect
                items=(0..72).collect()
                name=CreateRecipeFields::Servings
                placeholder="Servings"
            />

            <FormFieldDuration
                max_hours=72
                name=CreateRecipeFields::BakingTime
                placeholder="Baking time"
            />

            <FormFieldDuration
                max_hours=72
                name=CreateRecipeFields::PrepTime
                placeholder="Prep time"
            />

            <FormFieldTextarea name=CreateRecipeFields::Instructions placeholder="Instructions"/>

            <FormFieldTextarea name=CreateRecipeFields::Description placeholder="Description"/>

            <button type="submit" class="btn btn-primary">
                {"Save"}
            </button>
        </Form>
    }
}
