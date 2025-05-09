mod about;
mod home;
mod nav_bar;
mod thoughts_page;
mod thread_length_calc_page;
mod words_page;

use yew::prelude::*;
use yew_router::prelude::*;

use log::info;

use about::About;
use home::Home;
use nav_bar::Navbar;
use thoughts_page::Thoughts;
use thread_length_calc_page::ThreadLengthCalc;
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
    #[at("/thread-length-calc")]
    ThreadLengthCalc,
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
        Route::ThreadLengthCalc => html! {
            <ThreadLengthCalc />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {

            <HashRouter>
            // <BrowserRouter>
                <Navbar />
                <div class="main_container">

                    // <- must be child of <BrowserRouter>
                    <Switch<Route> render={switch} />
                </div>
            // </BrowserRouter>
            </HashRouter>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting app...");

    yew::Renderer::<App>::new().render();
}
