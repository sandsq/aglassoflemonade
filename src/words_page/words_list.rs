use yew::prelude::*;

use super::word::{Word, WordComponent};

#[derive(Properties, PartialEq)]
pub struct WordsListProps {
    pub words: Vec<Word>,
}

#[function_component(WordsList)]
pub fn thoughts_list(WordsListProps { words }: &WordsListProps) -> Html {
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
                html! {
                <WordComponent
                    word={(*word).clone()}
                />
                }
            }).collect::<Html>()
        }
        </table>
    }
}
