use leptos_router::A;
use std::time::Duration;

use crate::components::icons::more_vertical_icon::MoreVerticalIcon;
use crate::components::icons::plus_icon::PlusIcon;
use crate::components::icons::shopping_cart_icon::ShoppingCartIcon;
use crate::components::icons::{clock_icon::ClockIcon, minus_icon::MinusIcon};
use crate::components::loading::Loading;
use crate::components::modal::Modal;
use crate::components::not_found::NotFound;
use crate::context::toast::{use_toast, Toast, ToastType, ToasterTrait};
use chrono::{NaiveTime, Timelike};
use common::recipe::{Recipe, RecipeIngredient};
use leptos::{logging::log, *};
use leptos_router::{use_navigate, use_params_map, NavigateOptions};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::request::{delete, get};

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
                                <div class="flex justify-center w-full">
                                    <div class="flex flex-col max-w-[1408px]">
                                        <RecipeCard recipe=r.clone()/>
                                        <div class="flex flex-wrap gap-x-12 justify-center mt-8">
                                            <RecipeIngredients
                                                recipe=r.clone()
                                                ingredients=r.ingredients
                                            />
                                            {if let Some(steps) = r.instructions {
                                                view! { <RecipeSteps steps=steps/> }.into_view()
                                            } else {
                                                ().into_view()
                                            }}

                                        </div>
                                    </div>
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
fn VideoOptions(recipe: Recipe) -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let toast = use_toast().unwrap();

    let on_delete = move |_| {
        spawn_local(async move {
            match delete(&format!("/api/recipe/{}", recipe.id)).send().await {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully deleted recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                    let navigate = use_navigate();
                    navigate("/recipes", NavigateOptions::default());
                }
                _ => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to delete recipe".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
            };
        });
    };

    view! {
        <div class="dropdown dropdown-end">
            <div
                tabindex="0"
                role="button"
                class="btn btn-xs btn-circle bg-neutral border-none"
                on:click=move |e| {
                    e.stop_propagation();
                    e.prevent_default();
                }
            >

                <MoreVerticalIcon/>
            </div>
            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
            >
                <li>
                    <A href=format!("/recipes/{}/edit", recipe.id)>"Edit recipe"</A>
                </li>
                <li>
                    <button on:click=move |_| {
                        log!("Add modal with confirmation about deleting video here: {}", open());
                        set_open(true);
                    }>"Delete"</button>
                </li>
            </ul>

            <Modal open=open set_open=set_open>
                <h3 class="font-bold text-lg">"Delete recipe"</h3>
                <p class="py-4">"Are you sure you want to delete the recipe?"</p>
                <div class="modal-action">
                    <form method="dialog">
                        <button on:click=on_delete class="btn">
                            "Yes"
                        </button>
                        <button class="btn">"No"</button>
                    </form>
                </div>
            </Modal>
        </div>
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

    let recipe_footer = recipe.clone();

    view! {
        <div class="flex w-full justify-center">
            <div class="card lg:card-side bg-neutral">
                <figure class="w-full lg:w-3/5">
                    <img
                        class="rounded-lg object-cover aspect-[4/3]"
                        src=recipe.img
                        alt="Recipe img"
                    />
                </figure>
                <div class="card-body lg:w-2/5">
                    <h1 class="card-title text-4xl">{recipe.name}</h1>
                    <div class="flex flex-row mt-4">
                        <ClockIcon/>
                        <p class="ml-1 flex-none mr-3">{time}</p>
                        <ShoppingCartIcon/>
                        <p class="ml-1">{format_ingredients(recipe.ingredients.len())}</p>
                    </div>
                    <p class="mt-4">{recipe.description}</p>
                    <RecipeFooter recipe=recipe_footer/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RecipeFooter(recipe: Recipe) -> impl IntoView {
    view! {
        <div class="flex pt-16">
            <p>"Sebastian Lyng Johansen"</p>
            <VideoOptions recipe=recipe/>
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
        <div class="flex flex-col w-full md:w-1/3 mb-12">
            <h1 class="text-2xl">"Ingredients"</h1>
            <div class="flex justify-center content-center p-4 mb-1 justify-between">
                <button
                    type="button"
                    class="btn border-none btn-square btn-sm bg-base-100"
                    on:click=move |_| { set_ingredients(servings(), servings() - Decimal::from(1)) }
                >
                    <MinusIcon/>
                </button>
                <p>{move || format!("{} servings", servings())}</p>
                <button
                    type="button"
                    class="btn border-none btn-square btn-sm bg-base-100"
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
        <div class="flex flex-col w-full md:max-w-[40%]">
            <h1 class="text-2xl pb-4">"Steps"</h1>
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
