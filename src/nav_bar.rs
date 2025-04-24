use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Navbar)]
pub fn nav_bar() -> Html {
    // let navigator = use_navigator().unwrap();

    // let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <nav class="nav-main">
            <ul>
                <li class="hvr-underline-reveal"><Link<Route> to={Route::Home}>{ "home" }</Link<Route>></li>
                // <li class="hvr-underline-reveal"><Link<Route> to={Route::Thoughts}>{ "every thought" }</Link<Route>></li>
                <li class="hvr-underline-reveal"><Link<Route> to={Route::Words}>{ "wordex" }</Link<Route>></li>
                <li class="hvr-underline-reveal"><Link<Route> to={Route::About}>{ "about" }</Link<Route>></li>
            </ul>
            </nav>
        </div>
    }
}
