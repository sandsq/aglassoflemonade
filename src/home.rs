use yew::prelude::*;
// use yew_hooks::use_toggle;
// use yew_router::prelude::*;

// use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "home" }</h1>
            <p>{"i rulez, u droolz"}</p>
        </div>
    }
}
