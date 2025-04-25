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
    // pub is_detail_expanded: bool,
}

#[function_component(WordComponent)]
pub fn word_component(
    WordProps {
        word,
        // is_detail_expanded,
    }: &WordProps,
) -> Html {
    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| {
            toggle.toggle();
        })
    };

    let word_comment_class = match *toggle {
        true => "word_comment",
        false => "hide_word_comment",
    };
    //     match is_detail_expanded {
    //     None => match *toggle {
    //         true => "word_comment",
    //         false => "hide_word_comment",
    //     },
    //     Some(i) => match i {
    //         true => "word_comment",
    //         false => "hide_word_comment",
    //     },
    // };

    let expand_button_class = match *toggle {
        true => "fa-angle-up fa-solid",
        false => "fa-angle-down fa-solid",
    };
    // let expand_button_class = match is_detail_expanded {
    //     None => match *toggle {
    //         true => "fa-angle-up fa-solid",
    //         false => "fa-angle-down fa-solid",
    //     },
    //     Some(i) => match i {
    //         true => "fa-angle-up fa-solid",
    //         false => "fa-angle-down fa-solid",
    //     },
    // };

    let entry_date = format!("{}", word.entry_date.clone().format("%Y %b %d"));

    html! {
        <>
        <tr>
            <td class="expand_toggle">
                <button class={expand_button_class} {onclick}></button>
            </td>
            <td class="word">{word.word.clone()}</td>
            <td class="date">{entry_date}</td>
            if word.sounds_good {
                <td class="affirmative border_left">{""}</td>
            } else {
                <td class="negative border_left">{""}</td>
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
                <td class="affirmative border_left">{""}</td>
            } else {
                <td class="negative border_left">{""}</td>
            }


        </tr>

        // if *toggle {
            <tr class={word_comment_class}>
                <td colspan=7>{word.comment.clone()}</td>
            </tr>
        // }

        </>
    }
}
