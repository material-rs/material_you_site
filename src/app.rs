use material_you::prelude::{Card, ThemeProvider};
use yew::prelude::*;

pub struct App;

pub enum Msg {}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, _: &Context<Self>) -> Html {
		html! { <ThemeProvider>
			<span>{"Welcome to material 3 rust implementation"}</span>
			<Card>
				<p>{"this is a card"}</p>
			</Card>
		</ThemeProvider> }
	}
}
