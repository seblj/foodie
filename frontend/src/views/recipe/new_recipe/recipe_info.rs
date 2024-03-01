use common::recipe::CreateRecipe;
use leptos::*;

use crate::components::{
    dropdown::DropDownItem,
    form::{
        form_fields::{
            form_field_duration::FormFieldDuration,
            form_field_input::{FormFieldInput, FormFieldInputType},
            form_field_select::FormFieldSelect,
            form_field_textarea::FormFieldTextarea,
        },
        form_group::FormGroup,
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

    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"General info about you recipe"</h2>
                <FormGroup>
                    <FormFieldInput
                        value=move || recipe().name
                        ty=FormFieldInputType::Text
                        placeholder="Name"
                        on_input=move |name| recipe.update(|r| r.name = name)
                    />

                    // <div>
                    // <input type="file" class="file-input file-input-bordered" accept="image/*"/>
                    // </div>

                    <FormFieldSelect
                        value=(move || recipe().servings).into_signal()
                        items=items
                        placeholder="Servings"
                        on_change=move |servings| {
                            recipe.update(|r| r.servings = servings.unwrap_or_default())
                        }
                    />

                    <FormFieldDuration
                        max_hours=72
                        placeholder="Baking time"
                        on_change=move |baking_time| {
                            recipe.update(|r| r.baking_time = Some(baking_time))
                        }
                    />

                    <FormFieldDuration
                        max_hours=72
                        placeholder="Prep time"
                        on_change=move |prep_time| {
                            recipe.update(|r| r.prep_time = Some(prep_time))
                        }
                    />

                    <FormFieldTextarea
                        value=move || recipe().description
                        on_input=move |desc| recipe.update(|r| r.description = Some(desc))
                        placeholder="Description"
                    />
                </FormGroup>
            </div>
        </div>
    }
}
