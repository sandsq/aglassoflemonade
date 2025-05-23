use yew::prelude::*;

use crate::words_page::words_list_header::{FilterState, SortDirection, WordsListHeader};

use super::word::{Word, WordComponent};

// #[derive(Clone, PartialEq)]
// pub enum ExpandState {
//     AllExpanded,
//     AllCollapsed,
//     Inactive,
// }

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
    let sort_direction = selected_column.as_ref().map(|b| b);
    let sort_direction = match sort_direction {
        None => SortDirection::Unchanged,
        Some(i) => *i,
    };

    let select_column_date = use_state(|| None);
    let on_date_column_select = {
        let select_column_date = select_column_date.clone();
        Callback::from(move |b: SortDirection| select_column_date.set(Some(b)))
    };
    let date_direction = select_column_date.as_ref().map(|b| b);
    let date_direction = match date_direction {
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

    let selected_column4 = use_state(|| None);
    let on_column_select_means = {
        let selected_column4 = selected_column4.clone();
        Callback::from(move |b: FilterState| selected_column4.set(Some(b)))
    };
    let means_filter = selected_column4.as_ref().map(|b| b);
    let means_filter = match means_filter {
        None => FilterState::All,
        Some(i) => *i,
    };

    let selected_column5 = use_state(|| None);
    let on_column_select_overall = {
        let selected_column5 = selected_column5.clone();
        Callback::from(move |b: FilterState| selected_column5.set(Some(b)))
    };
    let overall_filter = selected_column5.as_ref().map(|b| b);
    let overall_filter = match overall_filter {
        None => FilterState::All,
        Some(i) => *i,
    };

    // todo: I'm pretty sure this is incorrect. For example, if one filter is already pressed and I press another one, it should just apply the new filter onto the first, instead of recomputing all filters (which is what happens now)
    let mut words = words.clone();
    words.sort_by(|a, b| b.entry_date.cmp(&a.entry_date));
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
        .filter(|&a| match means_filter {
            FilterState::All => true,
            FilterState::OnlyTrue => a.means_good,
            FilterState::OnlyFalse => !a.means_good,
        })
        .filter(|&a| match overall_filter {
            FilterState::All => true,
            FilterState::OnlyTrue => a.overall_good,
            FilterState::OnlyFalse => !a.overall_good,
        })
        .cloned()
        .collect::<Vec<Word>>();

    // todo: double sorting should depend on the order I think

    let mut words = match sort_direction {
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
    let words = match date_direction {
        SortDirection::Unchanged => words,
        SortDirection::Abc => {
            words.sort_by(|a, b| a.entry_date.cmp(&b.entry_date));
            words
        }
        SortDirection::Zyx => {
            words.sort_by(|a, b| b.entry_date.cmp(&a.entry_date));
            words
        }
    };

    // let selected_expand = use_state(|| None);
    // let on_expand_select = {
    //     let selected_expand = selected_expand.clone();
    //     Callback::from(move |b: Option<bool>| selected_expand.set(Some(b)))
    // };
    // let expand_state = selected_expand.as_ref().map(|b| b);
    // let expand_state = match expand_state {
    //     None => None,
    //     Some(i) => Some(i),
    // };

    // let is_detail_expanded = use_state(|| ExpandState::Inactive);
    // let on_expand_all_click = {
    //     let is_detail_expanded = is_detail_expanded.clone();
    //     Callback::from(move |_| {
    //         is_detail_expanded.set(ExpandState::AllExpanded);
    //     })
    // };
    // let on_collapse_all_click = {
    //     let is_detail_expanded = is_detail_expanded.clone();
    //     Callback::from(move |_| {
    //         is_detail_expanded.set(ExpandState::Inactive);
    //     })
    // };

    html! {

        <>
        // <button onclick={on_expand_all_click}>{"expand all"}</button>
        // <button onclick={on_collapse_all_click}>{"collapse all"}</button>

        <table>
            <WordsListHeader on_click={on_column_select} sort_direction={sort_direction} on_date_click={on_date_column_select} date_sort_direction={date_direction} on_sound_good_click={on_column_select_sound} sound_filter={sound_filter} on_look_good_click={on_column_select_look} look_filter={look_filter} on_means_good_click={on_column_select_means} means_filter={means_filter} on_overall_good_click={on_column_select_overall} overall_filter={overall_filter} />
        {
            words
            .iter()
            .map(|word| {
                html! {

                    <WordComponent
                        word={(*word).clone()}
                        // is_detail_expanded={true}
                    />

                    }


            }).collect::<Html>()
        }
        </table>

        </>
    }
}
