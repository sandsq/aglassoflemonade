mod thoughts_list;

use chrono::NaiveDate;
use gloo_net::http::Request;
use yew::prelude::*;

use thoughts_list::{Thought, ThoughtsList};

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

    //    let thoughts = vec![
    // 	Thought {
    // 	    id: 0,
    // 		content: "you know that feeling you get when you clap your hands together softly, not hard enough to make any sound, perfectly mirrored, and they squish against each other? ".to_string(),
    // 		date: NaiveDate::parse_from_str("2025-04-19", "%Y-%m-%d").unwrap(),
    // 		author: "me".to_string(),
    // 	},
    // 	Thought {
    // 	    id: 1,
    // 		content: "not actually a lot of shower thoughts".to_string(),
    // 		date: NaiveDate::parse_from_str("2025-04-20", "%Y-%m-%d").unwrap(),
    // 		author: "me".to_string(),
    // 	},
    // ];

    // let selected_thought = use_state(|| None);

    // let on_thought_select = {
    //     let selected_thought = selected_thought.clone();
    //     Callback::from(move |thought: Thought| selected_thought.set(Some(thought)))
    // };

    // let details = selected_thought.as_ref().map(|thought| {
    //     // thought.num_likes += 1
    //     html! {
    //     <ThoughtDetails thought={thought.clone()} />
    //     }
    // });

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
