use chrono::NaiveDate;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Thought {
    pub id: usize,
    pub content: String,
    pub date: NaiveDate,
    pub author: String,
}

#[derive(Properties, PartialEq)]
pub struct ThoughtsListProps {
    pub thoughts: Vec<Thought>,
    // pub on_click: Callback<Thought>,
}

// #[derive(Properties, PartialEq)]
// pub struct ThoughtsDetailsProps {
//     pub thought: Thought,
// }

// #[function_component(ThoughtDetails)]
// pub fn thought_details(ThoughtsDetailsProps { thought }: &ThoughtsDetailsProps) -> Html {
//     html! {
//     <div>
//         // <p>{thought.num_likes.clone()}</p>
//     </div>
//     }
// }

// fn format_row(thought: Thought) -> Html {
//     html! {
//         <>
//             <td>{thought.content}</td>
//             <td>{thought.date}</td>
//             <td>{thought.num_likes}</td>
//         </>
//     }
// }

#[function_component(ThoughtsList)]
pub fn thoughts_list(ThoughtsListProps { thoughts }: &ThoughtsListProps) -> Html {
    // let on_click = on_click.clone();
    html! {

        <table>
        {
            thoughts
            .iter()
            .map(|thought| {
                // let on_thought_select = {
                //     let on_click = on_click.clone();
                //     let thought = thought.clone();
                //     // Callback::from(move |_| on_click.emit(thought.clone()))
                // };
                html! {
                <tr>
                <td>{thought.content.clone()}</td>
                <td>{format!("{}", thought.date.clone().format("%Y %b %e"))}</td>
                // <td>{thought.num_likes.clone()}</td>
                // <td><button onclick={on_thought_select}>{"hmm"}</button></td>
                <td>{thought.author.clone()}</td>
                </tr>
                    // <table>
                    // <tr>
                    // {format_row(thought.clone())}
                    // <td onclick={on_thought_select}>{"a"}</td>
                    // </tr>
                    // </table>
                }
            }).collect::<Html>()
        }
        </table>
    }
}
