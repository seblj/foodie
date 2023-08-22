use leptos::*;
use web_sys::SubmitEvent;

use crate::components::input::Input;

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let f = |start: usize, end: usize| {
        (start..=end)
            .map(|i| {
                view! { <option value=format!("{}", i)>{format!("{}", i)}</option> }
            })
            .collect::<Vec<_>>()
    };

    let hours = f(0, 72);
    let minutes = f(0, 59);
    let servings = f(1, 100);

    let on_submit = |e: SubmitEvent| {
        e.prevent_default();
    };

    view! {
        <form on:submit=on_submit>
            <Input placeholder="Name"/>
            <textarea class="form-control" placeholder="Instructions"></textarea>
            <textarea class="form-control" placeholder="Description"></textarea>
            <div>
                <p>"Baking time"</p>
                <div class="d-flex">
                    <select class="form-select">{hours.clone()}</select>
                    <p>"Hours"</p>
                    <select class="form-select">{minutes.clone()}</select>
                    <p>"Minutes"</p>
                </div>
            </div>

            <div>
                <p>"Prep time"</p>
                <div class="d-flex">
                    <select class="form-select">{hours}</select>
                    <p>"Hours"</p>
                    <select class="form-select">{minutes}</select>
                    <p>"Minutes"</p>
                </div>
            </div>

            <div>
                <p>"Number of servings"</p>
                <div class="d-flex">
                    <select class="form-select">{servings}</select>
                    <p>"Minutes"</p>
                </div>
            </div>
            // TODO: Only allow images
            <Input r#type="file" placeholder="File upload"/>
        </form>
    }
}
