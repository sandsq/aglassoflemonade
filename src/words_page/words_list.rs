use yew::prelude::*;

use crate::words_page::words_list_header::{SortDirection, WordsListHeader};

use super::word::{Word, WordComponent};

#[derive(Properties, PartialEq)]
pub struct WordsListProps {
    pub words: Vec<Word>,
}

#[function_component(WordsList)]
pub fn thoughts_list(WordsListProps { words }: &WordsListProps) -> Html {
    let selected_column = use_state(|| None);

    let on_column_select = {
        let selected_column = selected_column.clone();
        Callback::from(move |b: SortDirection| selected_column.set(Some(b)))
    };
    let should_sort = selected_column.as_ref().map(|b| b);
    let should_sort = match should_sort {
        None => SortDirection::Unchanged,
        Some(i) => *i,
    };
    let mut words = words.clone();
    let words = match should_sort {
        SortDirection::Unchanged => words,
        SortDirection::Abc => {
            words.sort_by(|a, b| a.word.cmp(&b.word));
            words
        }
        SortDirection::Zyx => {
            words.sort_by(|a, b| b.word.cmp(&a.word));
            words
        }
    };

    html! {

        <table>
            <WordsListHeader on_click={on_column_select.clone()} indicator={should_sort}/>
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
