use common::recipe::CreateRecipeFields;
use leptos::*;

use crate::components::{
    dropdown::DropDownItem,
    form::form_fields::{
        form_field_duration::FormFieldDuration,
        form_field_input::{FormFieldInput, FormFieldInputType},
        form_field_select::FormFieldSelect,
        form_field_textarea::FormFieldTextarea,
    },
};

#[component]
pub fn RecipeInfo() -> impl IntoView {
    let items = (0..72)
        .map(|i| DropDownItem {
            key: i,
            value: i,
            label: i.to_string(),
        })
        .collect::<Vec<_>>();

    view! {
        <FormFieldInput
            ty=FormFieldInputType::Text
            name=CreateRecipeFields::Name
            placeholder="Name"
        />

        <div>
            <input type="file" class="file-input file-input-bordered" accept="image/*"/>
        </div>

        <FormFieldSelect items=items name=CreateRecipeFields::Servings placeholder="Servings"/>

        <FormFieldDuration
            max_hours=72
            name=CreateRecipeFields::BakingTime
            placeholder="Baking time"
        />

        <FormFieldDuration max_hours=72 name=CreateRecipeFields::PrepTime placeholder="Prep time"/>

        <FormFieldTextarea name=CreateRecipeFields::Description placeholder="Description"/>
    }
}
