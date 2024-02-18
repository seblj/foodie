use common::{
    ingredient::{CreateIngredient, Ingredient},
    recipe::{CreateRecipe, CreateRecipeIngredient, Unit},
};
use leptos::*;
use rust_decimal::Decimal;
use strum::IntoEnumIterator;

use crate::{
    components::{
        dropdown::{DropDown, DropDownItem},
        input::Input,
    },
    request::post,
};

#[component]
pub fn RecipeIngredients() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    let ingredients = move || recipe().ingredients;

    let selected = create_rw_signal(vec![]);
    let (amount, set_amount) = create_signal::<Option<Decimal>>(None);
    let (name, set_name) = create_signal("".to_string());

    let units = common::recipe::Unit::iter()
        .enumerate()
        .map(|(i, u)| DropDownItem {
            key: i,
            label: u.to_string(),
            value: u,
        })
        .collect::<Vec<_>>();

    let onclick = move |_| {
        let ingredient_name = name();
        let ingredient_amount = amount();
        let sel: Vec<DropDownItem<Unit, usize, String>> = selected();
        let ingredient_unit = sel.first().unwrap().value;

        spawn_local(async move {
            let ingredient = post(
                "/api/ingredient",
                &CreateIngredient {
                    name: ingredient_name,
                },
            )
            .await
            .unwrap();

            let ingredient: Ingredient = ingredient.json().await.unwrap();
            let new_ingredient = CreateRecipeIngredient {
                ingredient_id: ingredient.id,
                unit: Some(ingredient_unit),
                amount: ingredient_amount,
            };
            recipe.update(|r| {
                r.ingredients.push(new_ingredient);
            });
        });
    };

    view! {
        <Input
            ty="number"
            placeholder="Amount"
            on:input=move |e| {
                let value = event_target_value(&e).parse::<Decimal>().ok();
                set_amount(value);
            }
        />

        <DropDown selected=selected placeholder="Unit" items=units/>
        <Input
            placeholder="Name"
            on:input=move |e| {
                set_name(event_target_value(&e));
            }
        />

        <For
            each=ingredients
            key=|ingredient| ingredient.ingredient_id
            // TODO: Add a card or something to show the step
            children=move |ingredient: CreateRecipeIngredient| {
                view! {
                    <li>
                        {format!(
                            "{}, {:?}, {:?}",
                            ingredient.ingredient_id,
                            ingredient.unit,
                            ingredient.amount,
                        )}

                    </li>
                }
            }
        />

        <button on:click=onclick type="button" class="btn btn-primary">
            Add to ingredient list
        </button>
    }
}
