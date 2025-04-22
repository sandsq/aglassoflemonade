use yew::prelude::*;
use yew_hooks::use_bool_toggle;

use super::word::Word;

#[derive(Properties, PartialEq)]
pub struct WordsListProps {
    pub words: Vec<Word>,
    pub comment: String,
    pub selected_word: Word,
    pub on_click: Callback<Word>,
}

#[function_component(WordsList)]
pub fn thoughts_list(
    WordsListProps {
        words,
        comment,
        selected_word,
        on_click,
    }: &WordsListProps,
) -> Html {
    let on_click = on_click.clone();

    html! {

        <table>
            <tr>
                <th></th>
                <th>{"word"}</th>
                <th>{"sounds good"}</th>
                <th>{"looks good"}</th>
                <th>{"means good"}</th>
                <th>{"overall good"}</th>
            </tr>
        {
            words
            .iter()
            .map(|word| {
                let on_word_select = {
                    let on_click = on_click.clone();
                    let word = word.clone();
                    Callback::from(move |_| {

                        on_click.emit(word.clone())
                    })
                };


                // let toggle = use_bool_toggle(true);

                // let onclick = {
                //     let toggle = toggle.clone();
                //     Callback::from(move |_| toggle.toggle())
                // };




                html! {
                <>
                <tr>
                    <td>
                        // <button {onclick}>{ "Toggle" }</button>
                        // {*toggle}
                        <button onclick={on_word_select}>{"press me :)"}</button>
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



                    // <td>{format!("{}", word.entry_date.clone().format("%Y %b %e"))}</td>

                </tr>

                if word == selected_word {
                // if is_word_selected {
                    <tr>
                        <td colspan=6>{comment}</td>
                    </tr>
                }

                </>
                }
            }).collect::<Html>()
        }
        </table>
    }
}
