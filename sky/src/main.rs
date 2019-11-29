use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {}

enum Msg {
	DoIt,
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Model {}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DoIt => {
				true
			}
		}
	}

	fn view(&self) -> Html<Self> {
		html! {
			<button onclick=|_| Msg::DoIt>{ "Clicfk me!" }</button>
		}
	}
}

fn main() {
	yew::start_app::<Model>();
}