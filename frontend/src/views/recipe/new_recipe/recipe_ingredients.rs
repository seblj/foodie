use crate::components::form::form_fields::form_field_list::{
    use_form_field_list, FormFieldList, FormGroup,
};
use crate::components::form::form_fields::{
    form_field_input::FormFieldInputType, form_field_select::FormFieldSelect,
};
use common::recipe::{CreateRecipe, CreateRecipeFields, CreateRecipeIngredient};
use common::strum::IntoEnumIterator;
use leptos::*;
use rust_decimal::Decimal;

use crate::components::{
    dropdown::DropDownItem, form::form_fields::form_field_input::FormFieldInput,
};

#[component]
pub fn RecipeIngredients() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    let ingredients = move || recipe().ingredients;
    let recipe_ingredients = create_rw_signal(CreateRecipeIngredient::default());

    let units = common::recipe::Unit::iter()
        .map(|u| DropDownItem {
            key: u.to_string(),
            label: u.to_string(),
            value: u,
        })
        .collect::<Vec<_>>();

    // TODO: I want to migrate all this css stuff out to a/some component(s).
    // I want it to just set to 12 cols by default on the outer div.
    // Then I want to add a component that can take the `span` as an optional prop. This should
    // definetely be the case for the `FormField{}`-components, but I need to think of a way to do
    // it with these that are not a separate component.

    view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"Add ingredients to your recipe"</h2>
                <FormGroup>
                    <FormFieldList value=recipe_ingredients name=CreateRecipeFields::Ingredients>
                        <FormFieldInput
                            value=move || recipe_ingredients().amount.map(|a| a.to_string())
                            span="col-span-6 md:col-span-3"
                            ty=FormFieldInputType::Number
                            placeholder="Amount"
                            on_input=move |amount| {
                                recipe_ingredients
                                    .update(|ri| {
                                        ri.amount = amount.parse::<Decimal>().ok();
                                    })
                            }
                        />

                        // TODO: I think I maybe want this callback to return Option<Unit>
                        <FormFieldSelect
                            value=(move || {
                                recipe_ingredients().unit.map(|u| u.to_string()).unwrap_or_default()
                            })
                                .into_signal()
                            span="col-span-6 md:col-span-3"
                            items=units
                            placeholder="Unit"
                            on_change=move |unit| { recipe_ingredients.update(|ri| ri.unit = unit) }
                        />

                        <FormFieldInput
                            value=move || recipe_ingredients().name
                            span="md:col-span-6"
                            ty=FormFieldInputType::Text
                            placeholder="Name"
                            on_input=move |name| recipe_ingredients.update(|ri| ri.name = name)
                        />
                    </FormFieldList>
                </FormGroup>
                <button
                    on:click=use_form_field_list().unwrap()
                    type="button"
                    class="btn btn-primary"
                >
                    "Add to ingredient list"
                </button>
            </div>
        </div>

        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"Ingredient list"</h2>
                <For
                    each=move || ingredients().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    // TODO: Add a card or something to show the ingredients
                    children=move |(_, ingredient)| {
                        let i = format!("{:?}", ingredient);
                        view! { <li>{i}</li> }
                    }
                />

            </div>
        </div>
    }
}
