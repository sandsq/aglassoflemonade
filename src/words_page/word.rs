use chrono::NaiveDate;
use serde::Deserialize;
use yew::prelude::*;
use yew_hooks::use_bool_toggle;

#[derive(Properties, Clone, PartialEq, Deserialize)]
pub struct Word {
    pub id: usize,
    pub word: String,
    pub sounds_good: bool,
    pub looks_good: bool,
    pub means_good: bool,
    pub overall_good: bool,
    pub comment: String,
    pub entry_date: NaiveDate,
    pub author: String,
}

// impl Word {
//     pub fn new() -> Self {
//         Self {
//             id: 9999,
//             word: "".to_string(),
//             sounds_good: false,
//             looks_good: false,
//             means_good: false,
//             overall_good: false,
//             comment: String::new(),
//             entry_date: NaiveDate::parse_from_str("2011-11-11", "%Y-%m-%d").unwrap(),
//             author: String::new(),
//         }
//     }
// }

#[derive(Properties, PartialEq)]
pub struct WordProps {
    pub word: Word,
}

#[function_component(WordComponent)]
pub fn word_component(WordProps { word }: &WordProps) -> Html {
    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    let word_comment_class = if *toggle { "" } else { "hide_word_comment" };

    html! {
        <>
        <tr>
            <td>
                <button {onclick}>{ "Toggle" }</button>


            </td>
            <td>{word.word.clone()}</td>
            if word.sounds_good {
                <td class="affirmative">{""}</td>
            } else {
                <td class="negative">{""}</td>
            }
            if word.looks_good {
                <td class="affirmative">{""}</td>
            } else {
                <td class="negative">{""}</td>
            }
            if word.means_good {
                <td class="affirmative">{""}</td>
            } else {
                <td class="negative">{""}</td>
            }
            if word.overall_good {
                <td class="affirmative">{""}</td>
            } else {
                <td class="negative">{""}</td>
            }

        </tr>

        // if *toggle {
            <tr class={word_comment_class}>
                <td colspan=6>{word.comment.clone()}</td>
            </tr>
        // }

        </>
    }
}
