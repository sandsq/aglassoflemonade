mod thought;
mod thoughts_list;

use gloo_net::http::Request;
use yew::prelude::*;

use thought::Thought;
use thoughts_list::ThoughtsList;

#[function_component(Thoughts)]
pub fn thoughts() -> Html {
    let thoughts = use_state(|| vec![]);
    {
        let thoughts = thoughts.clone();
        use_effect_with((), move |_| {
            let thoughts = thoughts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_thoughts: Vec<Thought> = Request::get("./assets/thoughts.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                thoughts.set(fetched_thoughts);
            });
            || ()
        });
    }

    html! {
        <div>
            <title>{"thoughts"}</title>

            <h1>{ "random selection of every thought to exist" }</h1>
            <ThoughtsList thoughts={(*thoughts).clone()} />
            // on_click={on_thought_select.clone()}/>
            // {for details}


        </div>
    }
}
