use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    // let navigator = use_navigator().unwrap();

    // let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <title>{"about"}</title>
             
            <h1>{ "about" }</h1>

        </div>
    }
}