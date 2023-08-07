use leptos::*;

use common::ingredient::Ingredient;

#[component]
pub fn IngredientList(cx: Scope, ingredients: Vec<Ingredient>) -> impl IntoView {
    view! { cx,
        <p>"foo"</p>
        <ul>
            {ingredients.into_iter().map(|n| view! { cx, <li>{n.name}</li> }).collect::<Vec<_>>()}
        </ul>
    }
}
