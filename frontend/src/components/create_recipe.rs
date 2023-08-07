use leptos::*;

use crate::components::input::Input;

#[component]
pub fn CreateRecipe(cx: Scope) -> impl IntoView {
    view! { cx,
        <Input placeholder="Name"/>
        <textarea class="form-control" placeholder="Instructions"></textarea>
        <textarea class="form-control" placeholder="Description"></textarea>
        // TODO: Maybe represent time in a different way
        <Input r#type="time" placeholder="Baking time"/>
        <Input r#type="time" placeholder="Prep time"/>
        // TODO: This should be dropdown
        <Input r#type="number" placeholder="Servings"/>
        // TODO: Only allow images
        <Input r#type="file" placeholder="File upload"/>
    }
}
