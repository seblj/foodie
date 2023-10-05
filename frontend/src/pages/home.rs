use leptos::{leptos_dom::logging::console_log, *};

#[derive(Clone)]
struct Bar {
    a: i32,
}

#[derive(Clone)]
struct Foo {
    bar: Vec<Bar>,
}

impl Foo {
    fn new() -> Self {
        Self { bar: vec![] }
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (a, set_a) = create_signal::<Foo>(Foo::new());
    let (b, set_b) = create_signal(0);
    create_effect(move |_| {
        let c = a();
        console_log("updating a");
    });

    let update = move |_| {
        set_a.update(|a| a.bar.push(Bar { a: 0 }));
    };

    view! {
        <div>
            <p>"Home"</p>
            <button on:click=update>
                Update a
            </button>
        </div>
    }
}
