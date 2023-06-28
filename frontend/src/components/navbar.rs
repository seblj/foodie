use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="navbar navbar-expand-lg">
          <div class="container-fluid">
            // TODO: Icon here
            <A href="/" class="navbar-brand">"Icon"</A>
            <div class="collapse navbar-collapse" id="navbarSupportedContent">
              <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                  <A class="nav-link" href="/">"Home"</A>
                </li>
                <li class="nav-item">
                  <A href="foo" class="nav-link">"Foo"</A>
                </li>
                // <li class="nav-item dropdown">
                //   <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                //     "Dropdown"
                //   </a>
                //   <ul class="dropdown-menu" aria-labelledby="navbarDropdown">
                //     <li><a class="dropdown-item" href="#">"Action"</a></li>
                //     <li><a class="dropdown-item" href="#">"Another action"</a></li>
                //     <li><hr class="dropdown-divider"/></li>
                //     <li><a class="dropdown-item" href="#">"Something else here"</a></li>
                //   </ul>
                // </li>
              </ul>
              <form class="d-flex">
                <i class="bi bi-person-circle" style="font-size: 25px;"></i>
              </form>
            </div>
          </div>
        </nav>
    }
}
