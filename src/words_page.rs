mod word;
mod word_details;
mod words_list;

use gloo_net::http::Request;
use yew::prelude::*;

use word::Word;
use words_list::WordsList;

#[function_component(Words)]
pub fn words() -> Html {
    let selected_word = use_state(|| None);
    let on_word_select = {
        let selected_word = selected_word.clone();
        Callback::from(move |word: Word| selected_word.set(Some(word)))
    };
    let details = selected_word.as_ref().map(|word| {
        word.comment.clone()
        // html! {
        //     <WordDetails word={word.clone()} />
        // }
    });
    let selected_word = selected_word.as_ref();
    let selected_word = match selected_word {
        None => &Word::new(),
        Some(i) => i,
    };
    //     .map(|word| {
    //     match word {
    //         None => Word::new(),
    //         Some(i) => i,
    //     }
    // });
    let detail: String = match details {
        None => "".to_string(),
        Some(i) => i,
    };

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

            <h1>{ "random selection of every word" }</h1>
            <WordsList words={(*words).clone()} on_click={on_word_select.clone()} comment={detail} selected_word={selected_word.clone()} />
            // {for details}


        </div>
    }
}
