use leptos::*;
use leptos_router::*;

#[component]
// TODO: Take a Recipe prop
pub fn RecipeCard(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="card" style="width: 18rem;">
            <A href="">
                <img
                    class="card-img-top"
                    src="https://www.nonnabox.com/wp-content/uploads/2018/01/pizza_napolitana.webp"
                    alt="Pizza"
                />
                <div class="card-body">
                    <h5 class="card-title">"Homemade pizza"</h5>
                    <p class="card-text">"The best homemade pizza you will ever taste"</p>
                </div>
            </A>
        </div>
    }
}