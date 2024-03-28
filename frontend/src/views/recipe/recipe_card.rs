use crate::components::icons::shopping_cart_icon::ShoppingCartIcon;
use crate::components::icons::{clock_icon::ClockIcon, more_vertical_icon::MoreVerticalIcon};
use chrono::{NaiveTime, Timelike};
use common::recipe::Recipe;
use leptos::{logging::log, *};
use leptos_router::{use_navigate, NavigateOptions, A};

#[component]
pub fn RecipeCard<T>(recipe: Recipe, on_delete: T) -> impl IntoView
where
    T: Fn() + 'static,
{
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
        <div class="card card-compact max-w-96 h-96 bg-neutral cursor-pointer">
            <figure class="h-full object-cover">
                <A class="h-full w-full" href=format!("/recipes/{}", recipe.id)>
                    <img class="h-full w-full object-cover" src=recipe.img alt="Recipe img"/>
                </A>
            </figure>
            <div
                on:click=move |_| {
                    let navigate = use_navigate();
                    navigate(&format!("/recipes/{}", recipe.id), NavigateOptions::default());
                }

                class="card-body flex flex-row"
            >
                <div class="flex flex-col">
                    <A class="card-title" href=format!("/recipes/{}", recipe.id)>
                        {recipe.name}
                    </A>
                    <div class="flex flex-row h-5">
                        <ClockIcon/>
                        <p class="ml-1 mr-3 grow-0">{time}</p>
                        <ShoppingCartIcon/>
                        <p class="ml-1">{format_ingredients(recipe.ingredients.len())}</p>
                    </div>
                </div>

                <VideoOptions on_delete=on_delete/>
            </div>
        </div>
    }
}

#[component]
fn VideoOptions<T>(on_delete: T) -> impl IntoView
where
    T: Fn() + 'static,
{
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
                    <button on:click=move |e| {
                        e.stop_propagation();
                        e.prevent_default();
                        log!("edit recipe");
                    }>"Edit recipe"</button>
                </li>
                <li>
                    <button on:click=move |e| {
                        e.stop_propagation();
                        e.prevent_default();
                        log!("Add modal with confirmation about deleting video here");
                        on_delete();
                    }>"Delete"</button>
                </li>
            </ul>
        </div>
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
