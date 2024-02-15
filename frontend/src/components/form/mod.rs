use leptos::*;
use web_sys::SubmitEvent;

pub mod form_fields;

#[component]
pub fn Form<T, U>(values: T, children: Children, on_submit: U) -> impl IntoView
where
    T: 'static + Clone + form_derive::Form,
    U: Fn(T) + 'static,
{
    let signal = create_rw_signal(values);
    provide_context(signal);

    let internal_on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        on_submit(signal());
    };

    view! {
        <form on:submit=internal_on_submit class="flex flex-col justify-center items-center">
            {children()}
        </form>
    }
}
