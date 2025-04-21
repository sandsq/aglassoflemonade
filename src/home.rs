use yew::prelude::*;
// use yew_router::prelude::*;

// use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    // let navigator = use_navigator().unwrap();

    // let onclick = Callback::from(move |_| navigator.push(&Route::About));
    html! {
        <div>
            <h1>{ "home" }</h1>            
        </div>
    }
}