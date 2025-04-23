use yew::prelude::*;

use crate::words_page::words_list_header::{FilterState, SortDirection, WordsListHeader};

use super::word::{Word, WordComponent};

#[derive(Properties, PartialEq)]
pub struct WordsListProps {
    pub words: Vec<Word>,
}

#[function_component(WordsList)]
pub fn thoughts_list(WordsListProps { words }: &WordsListProps) -> Html {
    let selected_column = use_state_eq(|| None);

    let on_column_select = {
        let selected_column = selected_column.clone();
        Callback::from(move |b: SortDirection| selected_column.set(Some(b)))
    };
    let sort_direction = selected_column.as_ref().map(|b| b);
    let sort_direction = match sort_direction {
        None => SortDirection::Unchanged,
        Some(i) => *i,
    };

    let selected_column2 = use_state(|| None);
    let on_column_select_sound = {
        let selected_column2 = selected_column2.clone();
        Callback::from(move |b: FilterState| selected_column2.set(Some(b)))
    };
    let sound_filter = selected_column2.as_ref().map(|b| b);
    let sound_filter = match sound_filter {
        None => FilterState::All,
        Some(i) => *i,
    };

    let selected_column3 = use_state(|| None);
    let on_column_select_look = {
        let selected_column3 = selected_column3.clone();
        Callback::from(move |b: FilterState| selected_column3.set(Some(b)))
    };
    let look_filter = selected_column3.as_ref().map(|b| b);
    let look_filter = match look_filter {
        None => FilterState::All,
        Some(i) => *i,
    };

    let words = words.clone();
    let mut words = words
        .iter()
        .filter(|&a| match sound_filter {
            FilterState::All => true,
            FilterState::OnlyTrue => a.sounds_good,
            FilterState::OnlyFalse => !a.sounds_good,
        })
        .filter(|&a| match look_filter {
            FilterState::All => true,
            FilterState::OnlyTrue => a.looks_good,
            FilterState::OnlyFalse => !a.looks_good,
        })
        .cloned()
        .collect::<Vec<Word>>();
    let words = match sort_direction {
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
            <WordsListHeader on_click={on_column_select.clone()} sort_direction={sort_direction} on_sound_good_click={on_column_select_sound} sound_filter={sound_filter} on_look_good_click={on_column_select_look} look_filter={look_filter} />
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
