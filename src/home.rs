use yew::prelude::*;
use yew_hooks::use_toggle;
// use yew_router::prelude::*;

// use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let toggle = use_toggle("Hello", "World");

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div>
            <h1>{ "home" }</h1>
            <button {onclick}>{ "Toggle" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *toggle }
            </p>
        </div>
    }
}
