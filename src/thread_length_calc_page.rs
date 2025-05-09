use web_sys::js_sys::Math::ceil;
use yew::prelude::*;
use yew::{Callback, Html, InputEvent, TargetCast, function_component, html, use_state};

enum Msg {
    InputValue(String),
}

#[function_component(ThreadLengthCalc)]
pub fn thread_length_calc() -> Html {
    let stitch_length = use_state(|| 0.0);
    let on_input = {
        let stitch_length = stitch_length.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                stitch_length.set(input.value_as_number())
            }
        })
    };

    let leather_thickness = use_state(|| 0.0);
    let on_input_leather_thickness = {
        let leather_thickness = leather_thickness.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                leather_thickness.set(input.value_as_number())
            }
        })
    };

    let stitch_spacing = use_state(|| 5.0);
    let on_input_stitch_spacing = {
        let stitch_spacing = stitch_spacing.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                stitch_spacing.set(input.value_as_number())
            }
        })
    };

    let buffer = use_state(|| 0.0);
    let on_input_buffer = {
        let buffer = buffer.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                buffer.set(input.value_as_number())
            }
        })
    };
    // 2 * length + (ceil(length / spacing) + 1) * thickness + buffer
    html! {
        <>
            <h1>{
                (*stitch_length).clone() * 2.0 + (ceil((*stitch_length).clone() / (*stitch_spacing).clone() + 1.0)) * (*leather_thickness).clone() + (*buffer).clone()
                }{"mm"}
            </h1>
            <p>{"stitch length:"}<input type="number" oninput={on_input} />{"mm"}</p>
                <p>{"leather thickness:"}<input type="number"  oninput={on_input_leather_thickness} />{"mm"}</p>
                <p>{"stitch spacing:"}<input type="number"  oninput={on_input_stitch_spacing} />{"mm"}</p>
                <p>{"buffer:"}<input type="number"  oninput={on_input_buffer} />{"mm"}</p>
        </>
    }
}
