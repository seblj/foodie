use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="navbar navbar-expand-sm">
            <div class="container-fluid">
                <button
                    class="navbar-toggler"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <A href="/" class="navbar-brand">
                    "Icon"
                </A>
                <i class="bi bi-person-circle d-sm-none" style="font-size: 25px;"></i>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item">
                            <A class="nav-link" href="/">
                                "Home"
                            </A>
                        </li>
                        <li class="nav-item">
                            <A href="foo" class="nav-link">
                                "Foo"
                            </A>
                        </li>
                    </ul>
                </div>
                <i class="bi bi-person-circle d-none d-sm-block" style="font-size: 25px;"></i>
            </div>
        </nav>
    }
}
