use crate::components::icons::clock_icon::ClockIcon;
use crate::components::icons::shopping_cart_icon::ShoppingCartIcon;
use chrono::{DateTime, NaiveTime, Timelike};
use common::recipe::{Recipe, RecipeIngredient, Unit};
use leptos::*;

#[component]
pub fn RecipeCard(recipe: Recipe) -> impl IntoView {
    // TODO: Do I want to include both prep time and baking time when displaying how long time it
    // takes to make the recipe
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
        <div class="card card-compact max-w-96 bg-neutral">
            <figure>
                <img src=recipe.img alt="Pizza"/>
            </figure>
            <div class="card-body">
                <div class="flex flex-row">
                    <ClockIcon/>
                    <p class="ml-1 flex-none mr-3">{time}</p>
                    <ShoppingCartIcon/>
                    <p class="ml-1">{format_ingredients(recipe.ingredients.len())}</p>
                </div>
                <h2 class="card-title">{recipe.name}</h2>
            </div>
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
