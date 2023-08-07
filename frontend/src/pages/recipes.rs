use leptos::*;

use crate::components::recipes::recipe_card::RecipeCard;

#[component]
pub fn Recipes(cx: Scope) -> impl IntoView {
    // TODO: Load recipes here
    let recipes = vec!["foo"; 30];
    view! { cx,
        <div class="container">
            <div class="row row-cols-auto g-4">
                {recipes
                    .into_iter()
                    .map(|recipe| {
                        view! { cx,
                            <div class="col mx-auto">
                                <RecipeCard/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
