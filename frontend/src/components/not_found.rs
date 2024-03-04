use leptos::*;
use leptos_router::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="absolute right-1/2 bottom-1/2 transform translate-x-1/2 translate-y-1/2 ">
            <div class="text-center">
                <h1 class="text-9xl font-black text-gray-200">404</h1>

                <p class="text-2xl font-bold tracking-tight text-gray-900 sm:text-4xl">Uh-oh!</p>

                <p class="mt-4">"We can't find that page."</p>

                <A href="/" class="mt-6 btn btn-primary">
                    "Go Back Home"
                </A>
            </div>
        </div>
    }
}
