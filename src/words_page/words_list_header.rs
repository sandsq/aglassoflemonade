use yew::prelude::*;

use super::sort_button::SortButton;

#[derive(Copy, Clone, PartialEq)]
pub enum SortDirection {
    Abc,
    Zyx,
    Unchanged,
}
#[derive(Copy, Clone, PartialEq)]
pub enum FilterState {
    OnlyTrue,
    OnlyFalse,
    All,
}

#[derive(Properties, PartialEq)]
pub struct AlphaProps {
    pub on_click: Callback<SortDirection>,
    pub sort_direction: SortDirection,
    pub on_click_sound: Callback<FilterState>,
    pub sound_filter: FilterState,
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
    let on_click_sound = on_click_sound.clone();
    let on_filter_sounds_good_click = {
        let on_click_sound = on_click_sound.clone();
        Callback::from(move |_| match sound_filter {
            FilterState::OnlyTrue => on_click_sound.emit(FilterState::All),
            _ => on_click_sound.emit(FilterState::OnlyTrue),
        })
    };

    html! {
        <tr>
            <th></th>
                <th>
                    {"word"}
                    <SortButton on_click={on_sort_abc_click} content={"abc"} css_class={abc_sort_class} />
                    <SortButton on_click={on_sort_zyx_click} content={"zyx"} css_class={zyx_sort_class} />
                </th>
            <th>
                {"sounds good"}
                <SortButton on_click={on_filter_sounds_good_click} content={"true"} css_class={sounds_good_class} />
            </th>
            <th>{"looks good"}</th>
            <th>{"means good"}</th>
            <th>{"overall good"}</th>
        </tr>
    }
}
