use crate::components::stepper::{Step, Stepper};
use rust_decimal::Decimal;
use strum::IntoEnumIterator;

use common::{
    ingredient::{CreateIngredient, Ingredient},
    recipe::{CreateRecipe, CreateRecipeFields, CreateRecipeIngredient, Unit},
};
use leptos::{logging::log, *};
use web_sys::MouseEvent;

use crate::{
    components::{
        dropdown::{DropDown, DropDownItem},
        form::{
            form_fields::{
                form_field_duration::FormFieldDuration,
                form_field_input::{FormFieldInput, FormFieldInputType},
                form_field_select::FormFieldSelect,
                form_field_textarea::FormFieldTextarea,
            },
            Form,
        },
        input::Input,
    },
    request::post,
};

#[component]
fn RecipeIngredients() -> impl IntoView {
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

    let onclick = move |_: MouseEvent| {
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

#[component]
fn RecipeSteps() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();
    let steps = move || recipe().instructions.unwrap_or_default();

    let (instruction, set_instruction) = create_signal("".to_string());

    let onclick = move |_: MouseEvent| {
        recipe.update(|r| {
            let val = instruction();
            if val.is_empty() {
                return;
            }

            if let Some(ref mut instructions) = r.instructions {
                instructions.push(val);
            } else {
                r.instructions = Some(vec![val])
            };
            set_instruction("".to_string());
        })
    };

    view! {
        <textarea
            prop:value=instruction
            on:input=move |e| set_instruction(event_target_value(&e))
            class="textarea textarea-bordered"
            placeholder="Steps"
        >
            {move || instruction.get_untracked()}
        </textarea>

        <button type="button" class="btn btn-primary" on:click=onclick>
            Add to instructions
        </button>

        <ul>
            <For
                each=steps
                key=|step| step.clone()
                // TODO: Add a card or something to show the step
                children=move |step: String| {
                    view! { <li>{step}</li> }
                }
            />

        </ul>
    }
}

#[component]
fn RecipeInfo() -> impl IntoView {
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
