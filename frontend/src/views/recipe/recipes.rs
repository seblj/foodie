use leptos::*;

use crate::views::recipe::recipe_card::RecipeCard;

#[component]
pub fn Recipes() -> impl IntoView {
    view! {
        <div class="p-4 w-full justify-center flex flex-col items-center">
            <div class="grid grid-cols-12 gap-8">
                {(0..31)
                    .map(|_| {
                        view! {
                            <div class="col-span-12 sm:col-span-6 lg:col-span-4">
                                <RecipeCard/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
