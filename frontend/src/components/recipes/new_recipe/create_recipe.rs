use leptos::{leptos_dom::logging::console_log, *};
use web_sys::SubmitEvent;

macro_rules! sel {
    ($start:tt, $end:tt, $struct:tt, $prop:tt) => {
        ($start..=$end)
            .map(|i| {
                view! {
                    <option value=i selected=move || $struct().$prop == i as i32>
                        {i}
                    </option>
                }
            })
            .collect::<Vec<_>>()
    };
}

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let (recipe, set_recipe) = create_signal(common::recipe::CreateRecipe::default());

    let f = |start: usize, end: usize| {
        (start..=end)
            .map(|i| {
                view! { <option value=i>{i}</option> }
            })
            .collect::<Vec<_>>()
    };

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        console_log(&format!("recipe: {:?}", recipe()));
    };

    // Prompt for are you sure you want to leave
    // window_event_listener(ev::beforeunload, |e| {
    //     e.set_return_value("true");
    // });

    view! {
        <form on:submit=on_submit class="flex flex-col justify-center items-center">
            <ul class="steps">
                <li class="step step-primary">"Basics"</li>
                <li class="step">"Ingredients"</li>
                <li class="step">"Steps"</li>
                <li class="step">"Extra details"</li>
            </ul>

            <div>
                <input
                    type="text"
                    class="input input-bordered"
                    placeholder="Name"
                    on:input=move |ev| set_recipe.update(|r| r.name = event_target_value(&ev))
                />
            </div>

            <div>
                <input type="file" accept="image/*" class="file-input file-input-bordered"/>
            </div>

            <div>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        set_recipe
                            .update(|r| {
                                r.servings = event_target_value(&ev).parse::<i32>().unwrap();
                            })
                    }
                >

                    <option disabled selected>
                        "Number of servings"
                    </option>
                    {sel!(0, 100, recipe, servings)}
                </select>
            </div>

            <div>
                <p>"Baking time"</p>
                <div class="d-flex">
                    <select class="select select-bordered">
                        <option disabled selected>
                            "Hours"
                        </option>
                        {f(0, 72)}
                    </select>
                    <select class="select select-bordered">
                        <option disabled selected>
                            "Minutes"
                        </option>
                        {f(0, 59)}
                    </select>
                </div>
            </div>

            <div>
                <p>"Prep time"</p>
                <div class="d-flex">
                    <select class="select select-bordered">
                        <option disabled selected>
                            "Hours"
                        </option>
                        {f(0, 72)}
                    </select>
                    <select class="select select-bordered">
                        <option disabled selected>
                            "Minutes"
                        </option>
                        {f(0, 59)}
                    </select>
                </div>
            </div>

            <div>
                <textarea class="textarea textarea-bordered" placeholder="Instructions"></textarea>
            </div>
            <div>
                <textarea class="textarea textarea-bordered" placeholder="Description"></textarea>
            </div>
        </form>
    }
}
