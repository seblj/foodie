use leptos::*;

use common::ingredient::Ingredient;

#[component]
pub fn IngredientList(ingredients: Vec<Ingredient>) -> impl IntoView {
    view! {
        <p>"foo"</p>
        <ul>{ingredients.into_iter().map(|n| view! { <li>{n.name}</li> }).collect::<Vec<_>>()}</ul>
    }
}
