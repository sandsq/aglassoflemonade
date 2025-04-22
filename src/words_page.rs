mod sort_button;
mod word;
mod word_details;
mod words_list;
mod words_list_header;

use gloo_net::http::Request;
use yew::prelude::*;

use word::Word;
use words_list::WordsList;

#[function_component(Words)]
pub fn words() -> Html {
    let words = use_state(|| vec![]);
    {
        let words = words.clone();
        use_effect_with((), move |_| {
            let words = words.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_words: Vec<Word> = Request::get("./assets/words.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                words.set(fetched_words);
            });
            || ()
        });
    }

    html! {
        <div>
            <title>{"words"}</title>

            <h1>{ "random selection from all words" }</h1>
            <WordsList words={(*words).clone()} />



        </div>
    }
}
