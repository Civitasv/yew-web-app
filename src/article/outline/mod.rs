use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct _Outline {
    outline: String,
}

impl _Outline {
    pub fn new(outline: String) -> Self {
        Self { outline }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub outline: _Outline,
}

#[function_component(Outline)]
pub fn outline(Props { outline }: &Props) -> Html {
    html! {
        <p class="text-sm text-slate-500">
        {&outline.outline}
        </p>
    }
}
