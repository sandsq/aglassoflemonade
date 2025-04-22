use super::Word;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WordsDetailsProps {
    pub word: Word,
}

#[function_component(WordDetails)]
pub fn word_details(WordsDetailsProps { word }: &WordsDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ word.comment.clone() }</h3>
        </div>
    }
}
