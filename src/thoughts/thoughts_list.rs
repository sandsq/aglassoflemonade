use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Thought {
	pub content: String,
	pub date: String,
	pub num_likes: u32,
}

#[derive(Properties, PartialEq)]
pub struct ThoughtsListProps {
	pub thoughts: Vec<Thought>,
	pub on_click: Callback<Thought>, 
}

#[derive(Properties, PartialEq)]
pub struct ThoughtsDetailsProps {
	pub thought: Thought,
}

#[function_component(ThoughtDetails)]
pub fn thought_details(ThoughtsDetailsProps {thought}: &ThoughtsDetailsProps) -> Html {
	html! {
	<div>
		<p>{thought.num_likes.clone()}</p>
	</div>
	}
}

fn format_row(thought: Thought) -> Html {
	html! {
		<>
			<td>{thought.content}</td>
			<td>{thought.date}</td>
			<td>{thought.num_likes}</td>
		</>
	}
}

#[function_component(ThoughtsList)]
pub fn thoughts_list(ThoughtsListProps { thoughts, on_click }: &ThoughtsListProps) -> Html {
	let on_click = on_click.clone();
	
	thoughts.iter().map(|thought| {
		let on_thought_select = {
			let on_click = on_click.clone();
			let thought = thought.clone();
			Callback::from(move |_| {
				on_click.emit(thought.clone())
			})
		};
		html! {
		<>
		<p onclick={on_thought_select}>{format!("{} {}", thought.content, thought.date)}</p>
		</>
			// <table>
			// <tr>
			// {format_row(thought.clone())}
			// <td onclick={on_thought_select}>{"a"}</td>
			// </tr>
			// </table>
		}
	}).collect()
}
