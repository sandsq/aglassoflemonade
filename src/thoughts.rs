mod thoughts_list;

use yew::prelude::*;

use thoughts_list::{Thought, ThoughtsList, ThoughtDetails};

#[function_component(Thoughts)]
pub fn thoughts() -> Html {
	let thoughts = vec![
		Thought {
			content: "you know that feeling you get when you clap your hands together softly, not hard enough to make any sound, perfectly mirrored, and they squish against each other? ".to_string(),
			date: "2025 Apr 19".to_string(),
			num_likes: 1,
		},
		Thought {
			content: "not actually a lot of shower thoughts".to_string(),
			date: "2025 Apr 20".to_string(),
			num_likes: 0,
		},
	];
	
	let selected_thought = use_state(|| None);

	let on_thought_select = {
		let selected_thought = selected_thought.clone();
		Callback::from(move |thought: Thought| {
			selected_thought.set(Some(thought))
		})
	};

	let details = selected_thought.as_ref().map(|thought| html! {
		<ThoughtDetails thought={thought.clone()} />
	});

    html! {
        <div>
            <title>{"thoughts"}</title>
             
            <h1>{ "random selection of every thought to exist" }</h1>
			<ThoughtsList thoughts={thoughts} on_click={on_thought_select.clone()}/>
			{for details}
			

        </div>
    }
}