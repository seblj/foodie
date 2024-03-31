use leptos::*;

#[component]
pub fn RecipeImage(#[prop(optional, into)] src: Option<AttributeValue>) -> impl IntoView {
    view! {
        <figure class="w-full">
            <img class="rounded-lg object-cover aspect-[4/3]" src=src alt="Recipe img"/>
        </figure>
    }
}
