mod about;
mod home;
mod nav_bar;
mod thoughts_page;
mod words_page;

use yew::prelude::*;
use yew_router::prelude::*;

use log::info;

use about::About;
use home::Home;
use nav_bar::Navbar;
use thoughts_page::Thoughts;
use words_page::Words;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/thoughts")]
    Thoughts,
    #[at("/words")]
    Words,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
           <Home />
        },
        Route::About => html! {
            <About />
        },
        Route::Thoughts => html! {
            <Thoughts />
        },
        Route::Words => html! {
            <Words />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="main_container">
            // <HashRouter>
            <BrowserRouter>
                <Navbar />
                // <- must be child of <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
            // </HashRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting app...");

    yew::Renderer::<App>::new().render();
}
