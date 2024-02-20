use leptos::*;
use web_sys::SubmitEvent;

pub mod form_fields;

#[component]
pub fn Form<T, U>(values: RwSignal<T>, children: Children, on_submit: U) -> impl IntoView
where
    T: 'static + Clone + form_derive::Form,
    U: Fn(T) + 'static,
{
    provide_context(values);

    let internal_on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        on_submit(values());
    };

    // TODO: Add on dirty to add save button or something
    view! {
        <div class="p-4 mb-4 w-full justify-center flex flex-col items-center">
            <form
                on:submit=internal_on_submit
                class="grid grid-auto-columns max-w-2xl w-full gap-4"
            >
                {children()}
            </form>
        </div>
    }
}
