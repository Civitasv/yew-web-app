use yew::{html, Component, Context, Html};
pub struct Title {
    title: &'static str,
}

pub enum Msg {
    Click,
}

impl Component for Title {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self { title: "Title" }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                self.title = "Changed Title";
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Click);
        html! {
            <div>
            <button {onclick}>{"CHANGE IT"}</button>
            {self.title}
            </div>
        }
    }
}
