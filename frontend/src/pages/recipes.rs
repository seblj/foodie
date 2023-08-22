use leptos::*;

use crate::components::recipes::recipe_card::RecipeCard;

#[component]
pub fn Recipes() -> impl IntoView {
    // TODO: Load recipes here
    let recipes = vec!["foo"; 31];
    view! {
        <div class="container">
            <div class="row g-4">
                {recipes
                    .into_iter()
                    .map(|recipe| {
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
