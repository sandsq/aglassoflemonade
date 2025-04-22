use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SortButtonProps {
    pub on_click: Callback<MouseEvent>,
    pub content: String,
    pub css_class: String,
}

#[function_component(SortButton)]
pub fn sort_button(
    SortButtonProps {
        on_click,
        content,
        css_class,
    }: &SortButtonProps,
) -> Html {
    html! {
        <button onclick={on_click} class={css_class}>{content}</button>
    }
}
