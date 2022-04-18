use yew::{html, Component, Properties};

use web_sys::{Event, HtmlInputElement};
struct Content {
    contents: &'static str,
}

enum Msg {}

#[derive(Properties, PartialEq)]
pub struct Props {}

impl Component for Content {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            contents: "Initial Contents",
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>{self.contents}</div>
        }
    }
}
