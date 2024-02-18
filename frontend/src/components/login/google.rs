use leptos::*;
use web_sys::window;

#[component]
pub fn Google() -> impl IntoView {
    let login = move |_| {
        spawn_local(async move {
            let res = reqwasm::http::Request::get("/api/oauth/google/login")
                .send()
                .await
                .unwrap();

            let url = res.text().await.unwrap();
            window().unwrap().location().set_href(&url).unwrap();
        });
    };
    view! {
        <a
            on:click=login
            class="px-4 py-2 border flex gap-2 border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-200 hover:border-slate-400 dark:hover:border-slate-500 hover:text-slate-900 dark:hover:text-slate-300 hover:shadow transition duration-150"
        >
            <img
                class="w-6 h-6"
                src="https://www.svgrepo.com/show/475656/google-color.svg"
                loading="lazy"
                alt="google logo"
            />
            <span>Login with Google</span>
        </a>
    }
}
