use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlphaProps {
    pub on_sort_word: Callback<bool>,
}

#[function_component(WordsListHeader)]
pub fn words_list_header(AlphaProps { on_sort_word }: &AlphaProps) -> Html {
    let on_sort_word = on_sort_word.clone();
    let on_sort_abc = {
        let on_sort_word = on_sort_word.clone();
        Callback::from(move |_| {
            on_sort_word.emit(true);
        })
    };

    html! {
        <tr>
            <th></th>
                <th>
                {"word"}
                <button onclick={on_sort_abc}>{"abc"}</button>
                // <button onclick={on_sort_zyx}>{"zyx"}</button>
                </th>
            <th>{"sounds good"}</th>
            <th>{"looks good"}</th>
            <th>{"means good"}</th>
            <th>{"overall good"}</th>
        </tr>
    }
}
