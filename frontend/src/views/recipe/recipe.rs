use crate::components::icons::clock_icon::ClockIcon;
use crate::components::icons::shopping_cart_icon::ShoppingCartIcon;
use crate::components::loading::Loading;
use crate::components::not_found::NotFound;
use chrono::{NaiveTime, Timelike};
use common::recipe::{Recipe, RecipeIngredient};
use leptos::*;
use leptos_router::use_params_map;

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
                                {if let Some(steps) = r.instructions {
                                    view! {
                                        <div class="flex justify-between">
                                            <RecipeIngredients ingredients=r.ingredients/>
                                            <RecipeSteps steps=steps/>
                                        </div>
                                    }
                                        .into_view()
                                } else {
                                    ().into_view()
                                }}
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
                <figure class="!block object-cover">
                    <img
                        class="rounded-lg h-[calc((100vw-32px)*.45)] max-h-[633.6px]"
                        src=recipe.img
                        alt="Recipe img"
                    />
                </figure>
            </div>
        </div>
    }
}

#[component]
fn RecipeIngredients(ingredients: Vec<RecipeIngredient>) -> impl IntoView {
    view! {
        <div class="flex flex-col max-w-[40%]">
            {ingredients
                .into_iter()
                .map(|ingredient| {
                    view! {
                        <p>
                            {format!(
                                "{} {} {}",
                                ingredient.amount.map(|a| a.to_string()).unwrap_or_default(),
                                ingredient.unit.map(|i| i.to_string()).unwrap_or_default(),
                                ingredient.ingredient_name,
                            )}

                        </p>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn RecipeSteps(steps: Vec<String>) -> impl IntoView {
    view! {
        <div class="flex flex-col max-w-[40%]">
            {steps
                .into_iter()
                .enumerate()
                .map(|(idx, step)| {
                    view! {
                        <h1 class="mt-6">{format!("Steg {}", idx + 1)}</h1>
                        <p>{step}</p>
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
