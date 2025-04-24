mod sort_button;
mod word;
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
            <title>{"wordex"}</title>

            <h1>{ "wordex" }</h1>
            <h2>{"what is"}</h2>
            <p>{"judgin words based on some criteria"}</p>

            <h2>{"the dex"}</h2>
            <WordsList words={(*words).clone()} />



        </div>
    }
}
