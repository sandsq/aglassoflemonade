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
    pub on_sound_good_click: Callback<FilterState>,
    pub sound_filter: FilterState,
    pub on_look_good_click: Callback<FilterState>,
    pub look_filter: FilterState,
    pub on_means_good_click: Callback<FilterState>,
    pub means_filter: FilterState,
    pub on_overall_good_click: Callback<FilterState>,
    pub overall_filter: FilterState,
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
        on_sound_good_click,
        sound_filter,
        on_look_good_click,
        look_filter,
        on_means_good_click,
        means_filter,
        on_overall_good_click,
        overall_filter,
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
        FilterState::All => "fa-solid fa-check",
        FilterState::OnlyTrue => "active_sort fa-solid fa-check",
        FilterState::OnlyFalse => "fa-solid fa-check",
    };
    let sounds_bad_class = match sound_filter {
        FilterState::All => "fa-solid fa-xmark",
        FilterState::OnlyTrue => "fa-solid fa-xmark",
        FilterState::OnlyFalse => "active_sort fa-solid fa-xmark",
    };

    let looks_good_class = match look_filter {
        FilterState::All => "fa-solid fa-check",
        FilterState::OnlyTrue => "active_sort fa-solid fa-check",
        FilterState::OnlyFalse => "fa-solid fa-check",
    };
    let looks_bad_class = match look_filter {
        FilterState::All => "fa-solid fa-xmark",
        FilterState::OnlyTrue => "fa-solid fa-xmark",
        FilterState::OnlyFalse => "active_sort fa-solid fa-xmark",
    };
    

    let means_good_class = match means_filter {
        FilterState::All => "fa-solid fa-check",
        FilterState::OnlyTrue => "active_sort fa-solid fa-check",
        FilterState::OnlyFalse => "fa-solid fa-check",
    };
    let means_bad_class = match means_filter {
        FilterState::All => "fa-solid fa-xmark",
        FilterState::OnlyTrue => "fa-solid fa-xmark",
        FilterState::OnlyFalse => "active_sort fa-solid fa-xmark",
    };
    
    let overall_good_class = match overall_filter {
        FilterState::All => "fa-solid fa-check",
        FilterState::OnlyTrue => "active_sort fa-solid fa-check",
        FilterState::OnlyFalse => "fa-solid fa-check",
    };
    let overall_bad_class = match overall_filter {
        FilterState::All => "fa-solid fa-xmark",
        FilterState::OnlyTrue => "fa-solid fa-xmark",
        FilterState::OnlyFalse => "active_sort fa-solid fa-xmark",
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
    let on_filter_sounds_good_click = create_on_click(
        on_sound_good_click.clone(),
        sound_filter,
        FilterState::OnlyTrue,
    );
    let on_filter_sounds_bad_click = create_on_click(
        on_sound_good_click.clone(),
        sound_filter,
        FilterState::OnlyFalse,
    );

    let look_filter = look_filter.clone();
    let on_filter_looks_good_click = create_on_click(
        on_look_good_click.clone(),
        look_filter,
        FilterState::OnlyTrue,
    );
    let on_filter_looks_bad_click = create_on_click(
        on_look_good_click.clone(),
        look_filter,
        FilterState::OnlyFalse,
    );
    let means_filter = means_filter.clone();
    let on_filter_means_good_click = create_on_click(
        on_means_good_click.clone(),
        means_filter,
        FilterState::OnlyTrue,
    );
    let on_filter_means_bad_click = create_on_click(
        on_means_good_click.clone(),
        means_filter,
        FilterState::OnlyFalse,
    );
    
    let overall_filter = overall_filter.clone();
    let on_filter_overall_good_click = create_on_click(
        on_overall_good_click.clone(),
        overall_filter,
        FilterState::OnlyTrue,
    );
    let on_filter_overall_bad_click = create_on_click(
        on_overall_good_click.clone(),
        overall_filter,
        FilterState::OnlyFalse,
    );
    
    let overall_good_class = match overall_filter {
        FilterState::All => "fa-solid fa-check",
        FilterState::OnlyTrue => "active_sort fa-solid fa-check",
        FilterState::OnlyFalse => "fa-solid fa-check",
    };
    let overall_bad_class = match overall_filter {
        FilterState::All => "fa-solid fa-xmark",
        FilterState::OnlyTrue => "fa-solid fa-xmark",
        FilterState::OnlyFalse => "active_sort fa-solid fa-xmark",
    };
    
    html! {
        <>
        <tr class="border_top_bottom">
            <th></th>
            <th>
                {"word"}
                <br />
                <SortButton on_click={on_sort_abc_click} content={"abc"} css_class={abc_sort_class} />
                <SortButton on_click={on_sort_zyx_click} content={"zyx"} css_class={zyx_sort_class} />
            </th>
            <th class="border_left">
                {"sounds"}
                <br />
                <SortButton on_click={on_filter_sounds_good_click} content={""} css_class={sounds_good_class} />
                <SortButton on_click={on_filter_sounds_bad_click} content={""} css_class={sounds_bad_class} />
            </th>
            <th>
                {"looks"}
                <br />
                <SortButton on_click={on_filter_looks_good_click} content={""} css_class={looks_good_class} />
                <SortButton on_click={on_filter_looks_bad_click} content={""} css_class={looks_bad_class} />
            </th>
            <th>
                {"means"}
                <br />
                <SortButton on_click={on_filter_means_good_click} content={""} css_class={means_good_class} />
                <SortButton on_click={on_filter_means_bad_click} content={""} css_class={means_bad_class} />
            </th>
            <th class="border_left">
                {"overall"}
                <br />
                <SortButton on_click={on_filter_overall_good_click} content={""} css_class={overall_good_class} />
                <SortButton on_click={on_filter_overall_bad_click} content={""} css_class={overall_bad_class} />
            </th>
        </tr>

        // need two dummy rows for the alternate coloring to work because the detail toggle is a hidden row
        <tr class="dummy_row">
            <td class="expand_toggle"></td>
            <td class="word"></td>
            <td class="affirmative"></td>
            <td class="affirmative"></td>
            <td class="affirmative"></td>
            <td class="affirmative"></td>
        </tr>
        <tr class="dummy_row">
            <td class="expand_toggle"></td>
            <td class="word"></td>
            <td class="affirmative"></td>
            <td class="affirmative"></td>
            <td class="affirmative"></td>
            <td class="affirmative"></td>
        </tr>
        </>
    }
}
