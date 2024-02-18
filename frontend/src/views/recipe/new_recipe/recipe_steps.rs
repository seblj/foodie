use common::recipe::CreateRecipe;
use leptos::*;

#[component]
pub fn RecipeSteps() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();
    let steps = move || recipe().instructions.unwrap_or_default();

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
            <For
                each=steps
                key=|step| step.clone()
                // TODO: Add a card or something to show the step
                children=move |step: String| {
                    view! { <li>{step}</li> }
                }
            />

        </ul>
    }
}
