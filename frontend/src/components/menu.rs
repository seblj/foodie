use leptos::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Menu(items: Vec<View>) -> impl IntoView {
    view! {
        <ul
            tabindex="0"
            class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-300 rounded-box w-52"
        >
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <li on:click=move |_| {
                            document()
                                .active_element()
                                .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                                .and_then(|el| el.blur().ok());
                        }>

                            {item}
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}
