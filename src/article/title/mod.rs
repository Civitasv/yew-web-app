use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct _Title {
    pub title: String,
}

impl _Title {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: _Title,
}

#[function_component(Title)]
pub fn outline(Props { title }: &Props) -> Html {
    html! {
        <div class="text-lg font-bold">
        {&title.title}
        </div>
    }
}
