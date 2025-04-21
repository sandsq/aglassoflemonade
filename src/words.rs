use yew::prelude::*;
// use yew_router::prelude::*;

// use crate::Route;

#[function_component(Words)]
pub fn words() -> Html {
    // let navigator = use_navigator().unwrap();

    // let onclick = Callback::from(move |_| navigator.push(&Route::About));
    html! {
        <div>
            <h1>{ "random selection of every word" }</h1>            
        </div>
    }
}