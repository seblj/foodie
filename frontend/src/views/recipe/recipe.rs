use crate::components::icons::plus_icon::PlusIcon;
use crate::components::icons::shopping_cart_icon::ShoppingCartIcon;
use crate::components::icons::{clock_icon::ClockIcon, minus_icon::MinusIcon};
use crate::components::loading::Loading;
use crate::components::not_found::NotFound;
use chrono::{NaiveTime, Timelike};
use common::recipe::{Recipe, RecipeIngredient};
use leptos::{logging::log, *};
use leptos_router::use_params_map;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::request::get;

#[component]
pub fn Recipe() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let recipe = create_resource(id, move |id| async move {
        get(&format!("/api/recipe/{}", id))
            .send()
            .await
            .ok()?
            .json::<Recipe>()
            .await
            .ok()
    });

    view! {
        <Transition fallback=Loading>
            {move || {
                recipe
                    .get()
                    .map(|data| match data {
                        None => NotFound.into_view(),
                        Some(r) => {
                            view! {
                                <RecipeCard recipe=r.clone()/>
                                // TODO: Should not only show ingredients if we have steps
                                <div class="flex gap-x-12 justify-center mt-8">
                                    <RecipeIngredients recipe=r.clone() ingredients=r.ingredients/>
                                    {if let Some(steps) = r.instructions {
                                        view! { <RecipeSteps steps=steps/> }.into_view()
                                    } else {
                                        ().into_view()
                                    }}

                                </div>
                            }
                                .into_view()
                        }
                    })
            }}

        </Transition>
    }
}

#[component]
fn RecipeCard(recipe: Recipe) -> impl IntoView {
    let _recipe = recipe.clone();
    let time = move || {
        let total_time = match (_recipe.prep_time, _recipe.baking_time) {
            (Some(prep_time), Some(baking_time)) => NaiveTime::from_hms_opt(
                prep_time.hour() + baking_time.hour(),
                prep_time.minute() + baking_time.minute(),
                0,
            ),
            (Some(prep_time), None) => Some(prep_time),
            (None, Some(baking_time)) => Some(baking_time),
            (None, None) => None,
        };

        total_time.map(format_time).unwrap_or_default()
    };

    view! {
        <div class="flex w-full justify-center">
            <div class="card lg:card-side bg-neutral max-w-[1408px]">
                <figure class="!block object-cover">
                    <img
                        class="rounded-lg h-[calc((100vw-32px)*.45)] max-h-[633.6px]"
                        src=recipe.img
                        alt="Recipe img"
                    />
                </figure>
                <div class="card-body w-[40%]">
                    <h1 class="card-title text-4xl">{recipe.name}</h1>
                    <div class="flex flex-row mt-4">
                        <ClockIcon/>
                        <p class="ml-1 flex-none mr-3">{time}</p>
                        <ShoppingCartIcon/>
                        <p class="ml-1">{format_ingredients(recipe.ingredients.len())}</p>
                    </div>
                    <p class="mt-4">{recipe.description}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RecipeIngredients(recipe: Recipe, ingredients: Vec<RecipeIngredient>) -> impl IntoView {
    let internal_ingredients = create_rw_signal(ingredients.clone());
    let (servings, set_servings) = create_signal(Decimal::from(recipe.servings));

    let set_ingredients = move |old_serving: Decimal, new_serving: Decimal| {
        if new_serving < Decimal::from(0) {
            return;
        }

        let new_serving = if new_serving == Decimal::from(0) {
            Decimal::from_f32(0.5).unwrap()
        } else if old_serving == Decimal::from_f32(0.5).unwrap() {
            Decimal::from(1)
        } else {
            new_serving
        };

        let new_ingredients = internal_ingredients()
            .iter()
            .map(|i| RecipeIngredient {
                ingredient_id: i.ingredient_id,
                ingredient_name: i.ingredient_name.clone(),
                unit: i.unit,
                amount: i.amount.map(|a| {
                    log!("a: {}, old: {}, new: {}", a, old_serving, new_serving);
                    a.checked_div(old_serving).unwrap_or(a) * new_serving
                }),
            })
            .collect();

        internal_ingredients.set(new_ingredients);
        set_servings(new_serving);
    };

    view! {
        <div class="flex flex-col max-w-[40%]">
            <h1 class="text-2xl">"Ingredients"</h1>
            <div class="flex justify-center content-center">
                <button
                    type="button"
                    class="btn btn-square btn-sm"
                    on:click=move |_| { set_ingredients(servings(), servings() - Decimal::from(1)) }
                >
                    <MinusIcon/>
                </button>
                <p>{move || format!("{} servings", servings())}</p>
                <button
                    type="button"
                    class="btn btn-square btn-sm"
                    on:click=move |_| { set_ingredients(servings(), servings() + Decimal::from(1)) }
                >
                    <PlusIcon/>
                </button>
            </div>
            {move || {
                internal_ingredients()
                    .into_iter()
                    .map(|ingredient| {
                        view! {
                            <p class="p-4 mb-1 bg-neutral rounded-md">
                                {format!(
                                    "{} {} {}",
                                    ingredient
                                        .amount
                                        .map(|a| {
                                            if a.is_integer() {
                                                a.to_i64().unwrap().to_string()
                                            } else {
                                                a.to_string()
                                            }
                                        })
                                        .unwrap_or_default(),
                                    ingredient.unit.map(|i| i.to_string()).unwrap_or_default(),
                                    ingredient.ingredient_name,
                                )}

                            </p>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </div>
    }
}

#[component]
fn RecipeSteps(steps: Vec<String>) -> impl IntoView {
    view! {
        <div class="flex flex-col max-w-[40%]">
            <h1 class="text-2xl">"Steps"</h1>
            {steps
                .into_iter()
                .enumerate()
                .map(|(idx, step)| {
                    view! {
                        <div class="p-4 mb-1 bg-neutral rounded-md">
                            <h1 class="text-lg">{format!("Steg {}", idx + 1)}</h1>
                            <p>{step}</p>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

fn format_time(time: NaiveTime) -> String {
    match (time.hour(), time.minute()) {
        (h, m) if h >= 1 && m >= 1 => format!("{} h {} min", h, m),
        (h, _) if h >= 1 => format!("{} h", h),
        (_, m) if m >= 1 => format!("{} min", m),
        _ => "".to_string(),
    }
}

fn format_ingredients(len: usize) -> String {
    let val = if len > 1 { "ingredients" } else { "ingredient" };
    format!("{len} {val}")
}
