use crate::components::icons::clock_icon::ClockIcon;
use crate::components::icons::shopping_cart_icon::ShoppingCartIcon;
use chrono::{NaiveTime, Timelike};
use common::recipe::Recipe;
use leptos::*;
use leptos_router::A;

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

    // TODO: Need to fix it so all images take the same height, and the text span different
    view! {
        <A
            href=format!("/recipes/{}", recipe.id)
            class="card card-compact max-w-96 h-96 bg-neutral cursor-pointer"
        >
            <figure class="h-full object-cover">
                <img class="h-full w-full object-cover" src=recipe.img alt="Recipe img"/>
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
        </A>
    }
}

// TODO: Probably move these to some common place like a mod.rs file or do something else
// Right now they are duplicated in both here and in the single recipe component
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
