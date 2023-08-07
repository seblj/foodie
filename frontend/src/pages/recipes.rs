use leptos::*;

use crate::components::recipes::recipe_card::RecipeCard;

#[component]
pub fn Recipes(cx: Scope) -> impl IntoView {
    // TODO: Load recipes here
    let recipes = vec!["foo"; 30];
    view! { cx,
        <div class="container">
            <div class="row row-cols-1 row-cols-md-2 row-cols-lg-3 row-cols-xl-4 g-3">
                {recipes
                    .into_iter()
                    .map(|recipe| {
                        view! { cx,
                            <div class="col">
                                <RecipeCard/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
