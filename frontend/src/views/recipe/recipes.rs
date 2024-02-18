use leptos::*;

use crate::views::recipe::recipe_card::RecipeCard;

#[component]
pub fn Recipes() -> impl IntoView {
    // TODO: Load recipes here
    view! {
        <div class="container">
            <div class="row g-4">
                {(0..31)
                    .map(|_| {
                        view! {
                            <div class="col-sm-6 col-md-4 col-lg-3">
                                <RecipeCard/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
