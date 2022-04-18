use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct _Content {
    content: String,
}

impl _Content {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: _Content,
}

#[function_component(Content)]
pub fn outline(Props { content }: &Props) -> Html {
    html! {
        <div>
        {&content.content}
        </div>
    }
}
