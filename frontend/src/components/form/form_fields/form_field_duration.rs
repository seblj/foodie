use chrono::NaiveTime;
use leptos::*;

use crate::components::{
    dropdown::{DropDown, DropDownItem},
    form::form_fields::get_span,
};

#[component]
pub fn FormFieldDuration<T>(
    max_hours: usize,
    placeholder: &'static str,
    on_change: T,
    #[prop(optional)] span: &'static str,
) -> impl IntoView
where
    T: Fn(NaiveTime) + 'static,
{
    let (hours, set_hours) = create_signal(0);
    let (minutes, set_minutes) = create_signal(0);

    let f = |start: usize, end: usize| {
        (start..=end)
            .map(|i| DropDownItem {
                key: i,
                label: i,
                value: i,
            })
            .collect::<Vec<_>>()
    };

    let selected_minute = create_rw_signal::<Option<usize>>(None);
    let selected_hour = create_rw_signal::<Option<usize>>(None);

    create_effect(move |_| {
        if let Some(val) = selected_hour() {
            set_hours(val as u32);
            on_change(chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap());
        }

        if let Some(val) = selected_minute() {
            set_minutes(val as u32);
            on_change(chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap());
        }
    });

    let class = get_span(span);

    view! {
        <div class=class>
            <p>{placeholder}</p>
            <div class="grid grid-cols-2">
                <DropDown
                    value=(move || selected_hour().unwrap_or_default()).into_signal()
                    on_change=move |h| selected_hour.set(Some(h))
                    class="col-span-1 w-full"
                    placeholder="Hours"
                    items=f(0, max_hours)
                />
                <DropDown
                    value=(move || selected_minute().unwrap_or_default()).into_signal()
                    on_change=move |h| selected_minute.set(Some(h))
                    class="col-span-1 w-full"
                    placeholder="Minutes"
                    items=f(0, 59)
                />
            </div>
        </div>
    }
}
