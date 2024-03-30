use crate::components::not_found::NotFound;
use std::time::Duration;

use common::recipe::Recipe;
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
                                    view! {
                                        <For
                                            each=move || r.clone()
                                            key=|recipe| recipe.id
                                            children=move |recipe| {
                                                view! {
                                                    <div class="col-span-12 sm:col-span-6 lg:col-span-4">
                                                        <RecipeCard recipe=recipe/>
                                                    </div>
                                                }
                                            }
                                        />
                                    }
                                }
                            })
                    }}

                </Transition>

            </div>
        </div>
    }
}
