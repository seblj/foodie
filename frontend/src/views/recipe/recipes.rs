use crate::components::not_found::NotFound;
use std::time::Duration;

use chrono::{DateTime, NaiveTime};
use common::recipe::{Recipe, RecipeIngredient, Unit};
use leptos::*;

use crate::{
    components::loading::Loading,
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::get,
    views::recipe::recipe_card::RecipeCard,
};

#[component]
pub fn Recipes() -> impl IntoView {
    let toast = use_toast().unwrap();
    let recipes = create_resource(
        || (),
        move |_| async move {
            let res = match get("/api/recipe").send().await {
                Ok(res) => res,
                Err(_) => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Couldn't fetch recipes".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                    return None;
                }
            };

            res.json::<Vec<Recipe>>().await.ok()
        },
    );

    // let recipe = Recipe {
    //     id: 1,
    //     user_id: 1,
    //     name: "Pizza".to_string(),
    //     description: None,
    //     instructions: None,
    //     img: Some(
    //         "https://www.nonnabox.com/wp-content/uploads/2018/01/pizza_napolitana.webp".to_string(),
    //     ),
    //     servings: 4,
    //     updated_at: DateTime::from_timestamp(1431648000, 0).unwrap().into(),
    //     prep_time: NaiveTime::from_hms_opt(1, 30, 0),
    //     baking_time: NaiveTime::from_hms_opt(0, 20, 0),
    //     ingredients: vec![RecipeIngredient {
    //         ingredient_id: 1,
    //         ingredient_name: "Flour".to_string(),
    //         amount: Some(1.into()),
    //         unit: Some(Unit::Kilogram),
    //     }],
    // };

    // view! {
    //     <div class="p-4 w-full justify-center flex flex-col items-center">
    //         <div class="grid grid-cols-12 gap-8">
    //             {(0..31)
    //                 .map(|_| {
    //                     let _recipe = recipe.clone();
    //                     view! {
    //                         <div class="col-span-12 sm:col-span-6 lg:col-span-4">
    //                             <RecipeCard recipe=_recipe/>
    //                         </div>
    //                     }
    //                 })
    //                 .collect::<Vec<_>>()}
    //         </div>
    //     </div>
    // }
    view! {
        <div class="p-4 w-full justify-center flex flex-col items-center">
            <div class="grid grid-cols-12 gap-8">
                <Transition fallback=Loading>
                    {move || {
                        recipes
                            .get()
                            .map(|data| match data {
                                None => NotFound.into_view(),
                                Some(r) => {
                                    r.into_iter()
                                        .map(|recipe| {
                                            view! {
                                                <div class="col-span-12 sm:col-span-6 lg:col-span-4">
                                                    <RecipeCard recipe=recipe/>
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .into_view()
                                }
                            })
                    }}

                </Transition>

            </div>
        </div>
    }
}
