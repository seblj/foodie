use rust_decimal::Decimal;
use std::rc::Rc;
use strum::IntoEnumIterator;

use common::{
    ingredient::{CreateIngredient, Ingredient},
    recipe::{CreateRecipe, CreateRecipeFields, CreateRecipeIngredient, Unit},
};
use leptos::{leptos_dom::Transparent, logging::log, *};
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
fn Stepper(starting_step: usize, children: Children) -> impl IntoView {
    let (step, set_step) = create_signal(starting_step);

    let children = children()
        .as_children()
        .iter()
        .map(|child| {
            child
                .as_transparent()
                .and_then(|t| t.downcast_ref::<StepStruct>())
                .expect("Child of `<Stepper />` should only be `<Step />`")
        })
        .cloned()
        .collect::<Vec<_>>();

    let internal_children = children.clone();
    let children_len = children.len();

    let current_step = move || internal_children[step()].child.clone();

    view! {
        <ul class="steps">
            {children
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    let class = move || if i <= step() { "step step-primary" } else { "step" };
                    view! {
                        <li on:click=move |_| set_step(i) class=class>
                            {s.label}
                        </li>
                    }
                })
                .collect::<Vec<_>>()}

        </ul>

        {current_step}

        <div class="btm-nav">
            <button
                type="button"
                on:click=move |_| {
                    if step() > 0 {
                        set_step(step() - 1);
                    }
                }
            >

                {move || { if step() > 0 { "Previous".into_view() } else { ().into_view() } }}
            </button>
            <button
                type="button"
                on:click=move |_| {
                    if step() < children_len - 1 {
                        set_step(step() + 1);
                    }
                }
            >

                {move || {
                    if step() < children_len - 1 { "Next".into_view() } else { ().into_view() }
                }}

            </button>
        </div>
    }
}

#[component(transparent)]
fn Step<F, E>(label: &'static str, child: F) -> impl IntoView
where
    F: Fn() -> E + 'static,
    E: IntoView,
{
    StepStruct {
        label: label.to_string(),
        child: Rc::new(move || child().into_view()),
    }
}

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
            checked: false,
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
            on:input=move |e| {
                let value = event_target_value(&e).parse::<Decimal>().ok();
                set_amount(value);
            }

            ty="number"
            placeholder="Amount"
        />
        <DropDown selected=selected placeholder="Unit" items=units/>
        <Input
            on:input=move |e| {
                set_name(event_target_value(&e));
            }

            placeholder="Name"
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
            checked: false,
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

#[derive(Clone)]
pub struct StepStruct {
    label: String,
    child: Rc<dyn Fn() -> View>,
}

impl IntoView for StepStruct {
    fn into_view(self) -> View {
        Transparent::new(self).into_view()
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
            <Stepper starting_step=0>
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
