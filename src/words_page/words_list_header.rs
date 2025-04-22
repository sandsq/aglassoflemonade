use yew::prelude::*;

use super::sort_button::SortButton;

#[derive(Copy, Clone, PartialEq)]
pub enum SortDirection {
    Abc,
    Zyx,
    Unchanged,
}

#[derive(Properties, PartialEq)]
pub struct AlphaProps {
    pub on_click: Callback<SortDirection>,
    pub indicator: SortDirection,
}

#[function_component(WordsListHeader)]
pub fn words_list_header(
    AlphaProps {
        on_click,
        indicator,
    }: &AlphaProps,
) -> Html {
    let on_click = on_click.clone();
    let on_sort_abc_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            on_click.emit(SortDirection::Abc);
        })
    };
    let on_sort_zyx_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            on_click.emit(SortDirection::Zyx);
        })
    };
    let on_sort_default = {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            on_click.emit(SortDirection::Unchanged);
        })
    };
    let abc_sort_class = match indicator {
        SortDirection::Unchanged => "",
        SortDirection::Abc => "active_sort",
        SortDirection::Zyx => "",
    };
    html! {
        <tr>
            <th></th>
                <th>
                {"word"}
                <SortButton on_click={on_sort_abc_click} content={"abc"} css_class={abc_sort_class} />
                // <button onclick={on_sort_abc_click}>{"abc"}</button>
                <button onclick={on_sort_zyx_click}>{"zyx"}</button>
                <button onclick={on_sort_default}>{"x"}</button>
                </th>
            <th>{"sounds good"}</th>
            <th>{"looks good"}</th>
            <th>{"means good"}</th>
            <th>{"overall good"}</th>
        </tr>
    }
}
