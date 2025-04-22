use yew::prelude::*;

use super::thought::Thought;

#[derive(Properties, PartialEq)]
pub struct ThoughtsListProps {
    pub thoughts: Vec<Thought>,
}

#[function_component(ThoughtsList)]
pub fn thoughts_list(ThoughtsListProps { thoughts }: &ThoughtsListProps) -> Html {
    let mut sorted_thoughts = thoughts.clone();
    sorted_thoughts.sort_by(|a, b| b.date.cmp(&a.date));
    html! {

        <table>
        {
            sorted_thoughts
            .iter()
            .map(|thought| {
                html! {
                <tr>
                    <td>{thought.content.clone()}</td>
                    <td>{format!("{}", thought.date.clone().format("%Y %b %e"))}</td>
                    <td>{thought.author.clone()}</td>
                </tr>
                }
            }).collect::<Html>()
        }
        </table>
    }
}
