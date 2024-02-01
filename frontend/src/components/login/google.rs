use leptos::*;

use crate::request::BASE_URL;

#[component]
pub fn Google() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center h-screen dark:bg-gray-800">
            <a
                href=format!("{}api/oauth/google/login", BASE_URL)
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
        </div>
    }
}
