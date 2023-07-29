use leptos::{leptos_dom::console_log, *};

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
pub fn Home(cx: Scope) -> impl IntoView {
    let (a, set_a) = create_signal::<Foo>(cx, Foo::new());
    let (b, set_b) = create_signal(cx, 0);
    create_effect(cx, move |_| {
        let c = a();
        console_log("updating a");
    });

    let update = move |_| {
        set_a.update(|a| a.bar.push(Bar { a: 0 }));
    };

    view! { cx,
        <div>
            <p>"Home"</p>
            <button on:click=update>
                Update a
            </button>
        </div>
    }
}
