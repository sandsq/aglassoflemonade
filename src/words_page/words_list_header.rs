use yew::prelude::*;

use super::sort_button::SortButton;

#[derive(Copy, Clone, PartialEq)]
pub enum SortDirection {
    Abc,
    Zyx,
    Unchanged,
}
#[derive(Copy, Clone, PartialEq, strum_macros::Display)]
pub enum FilterState {
    OnlyTrue,
    OnlyFalse,
    All,
}
impl Default for FilterState {
    fn default() -> Self {
        FilterState::All
    }
}

// pub trait Filterable: Clone + Default {}
// impl Filterable for FilterState {}

#[derive(Properties, PartialEq)]
pub struct AlphaProps {
    pub on_click: Callback<SortDirection>,
    pub sort_direction: SortDirection,
    pub on_click_sound: Callback<FilterState>,
    pub sound_filter: FilterState,
}

pub fn create_on_click(
    on_click: Callback<FilterState>,
    filter_criteria: FilterState,
    default_output: FilterState,
) -> Callback<MouseEvent> {
    let on_relevant_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            if filter_criteria == default_output {
                on_click.emit(FilterState::All)
            } else {
                on_click.emit(default_output)
            }
        })
    };
    on_relevant_click
}

#[function_component(WordsListHeader)]
pub fn words_list_header(
    AlphaProps {
        on_click,
        sort_direction,
        on_click_sound,
        sound_filter,
    }: &AlphaProps,
) -> Html {
    let abc_sort_class = match sort_direction {
        SortDirection::Unchanged => "",
        SortDirection::Abc => "active_sort",
        SortDirection::Zyx => "",
    };
    let zyx_sort_class = match sort_direction {
        SortDirection::Unchanged => "",
        SortDirection::Abc => "",
        SortDirection::Zyx => "active_sort",
    };

    let sounds_good_class = match sound_filter {
        FilterState::All => "",
        FilterState::OnlyTrue => "active_sort",
        FilterState::OnlyFalse => "",
    };
    let sounds_bad_class = match sound_filter {
        FilterState::All => "",
        FilterState::OnlyTrue => "",
        FilterState::OnlyFalse => "active_sort",
    };

    let sort_direction = sort_direction.clone();
    let on_click = on_click.clone();
    let on_sort_abc_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| match sort_direction {
            SortDirection::Abc => on_click.emit(SortDirection::Unchanged),
            _ => on_click.emit(SortDirection::Abc),
        })
    };
    let on_sort_zyx_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| match sort_direction {
            SortDirection::Zyx => on_click.emit(SortDirection::Unchanged),
            _ => on_click.emit(SortDirection::Zyx),
        })
    };

    let sound_filter = sound_filter.clone();
    let on_filter_sounds_good_click =
        create_on_click(on_click_sound.clone(), sound_filter, FilterState::OnlyTrue);

    let on_filter_sounds_bad_click =
        create_on_click(on_click_sound.clone(), sound_filter, FilterState::OnlyFalse);

    html! {
        <tr>
                <th>
                    {"word"}
                    <br />
                    <SortButton on_click={on_sort_abc_click} content={"abc"} css_class={abc_sort_class} />
                    <SortButton on_click={on_sort_zyx_click} content={"zyx"} css_class={zyx_sort_class} />
                </th>
            <th>
                {"sounds"}
                <br />
                <SortButton on_click={on_filter_sounds_good_click} content={"true"} css_class={sounds_good_class} />
                <SortButton on_click={on_filter_sounds_bad_click} content={"false"} css_class={sounds_bad_class} />
            </th>
            <th>{"looks"}</th>
            <th>{"means"}</th>
            <th>{"overall"}</th>
            <th></th>
        </tr>
    }
}
