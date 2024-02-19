use crate::components::icons::{
    chevron_down::ChevronDown, chevron_up::ChevronUp, close_icon::CloseIcon,
};
use common::recipe::CreateRecipe;
use leptos::*;

#[component]
pub fn RecipeSteps() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    let (instruction, set_instruction) = create_signal("".to_string());

    let onclick = move |_| {
        recipe.update(|r| {
            let val = instruction();
            if val.is_empty() {
                return;
            }

            if let Some(ref mut instructions) = r.instructions {
                instructions.push(val);
            } else {
                r.instructions = Some(vec![val])
            };
            set_instruction("".to_string());
        })
    };

    let remove_card = move |index: usize| {
        recipe.update(|r| {
            let Some(ref mut instructions) = r.instructions else {
                unreachable!("Not supposed to happen");
            };
            instructions.remove(index);
        })
    };

    let swap_card = move |index: usize, other: usize| {
        recipe.update(|r| {
            let Some(ref mut instructions) = r.instructions else {
                unreachable!("Not supposed to happen");
            };
            instructions.swap(index, other);
        })
    };

    view! {
        <textarea
            prop:value=instruction
            on:input=move |e| set_instruction(event_target_value(&e))
            class="textarea textarea-bordered"
            placeholder="Steps"
        >
            {move || instruction.get_untracked()}
        </textarea>

        <button type="button" class="btn btn-primary" on:click=onclick>
            Add to instructions
        </button>

        <ul>
            // This is not so good since it rerenders the entire list on each change. However, it was a
            // bit tricky to find a good way to do it with `<For>`, since I want to be able to remove a
            // specific element, and the index is easy to do it. This works for now
            {move || {
                let steps = recipe().instructions.unwrap_or_default();
                let num_steps = steps.len();
                steps
                    .into_iter()
                    .enumerate()
                    .map(|(index, step)| {
                        view! {
                            <li>
                                <div class="card w-96 bg-red-50">
                                    <div class="card-body">
                                        <div class="card-actions justify-end">

                                            {if index > 0 {
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| swap_card(index, index - 1)
                                                        class="btn btn-square btn-sm"
                                                    >
                                                        <ChevronUp/>
                                                    </button>
                                                }
                                                    .into_view()
                                            } else {
                                                ().into_view()
                                            }}
                                            {if index < num_steps - 1 {
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| swap_card(index, index + 1)
                                                        class="btn btn-square btn-sm"
                                                    >
                                                        <ChevronDown/>
                                                    </button>
                                                }
                                                    .into_view()
                                            } else {
                                                ().into_view()
                                            }}
                                            <button
                                                type="button"
                                                on:click=move |_| remove_card(index)
                                                class="btn btn-square btn-sm"
                                            >
                                                <CloseIcon/>
                                            </button>
                                        </div>
                                        <h2 class="card-title">Step {index + 1}</h2>
                                        {step}
                                    </div>
                                </div>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </ul>
    }
}
