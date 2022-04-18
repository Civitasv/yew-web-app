use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct _Introduction {
    title: String,
    introduction: String,
    github: String,
}

impl _Introduction {
    pub fn new(title: String, introduction: String, github: String) -> Self {
        Self {
            title,
            introduction,
            github,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub introduction: _Introduction,
}

#[function_component(Introduction)]
pub fn header(Props { introduction }: &Props) -> Html {
    html! {
        <div class={classes!("md:w-1/2", "min-w-max", "sm:w-full", "mb-40", "mt-40")}>
            <div class="introduction">
                <div class="introduction-title text-4xl font-semibold">
                    {introduction.title.clone()}
                </div>
                <div class="introduction-text">
                    <div class="text-sm text-slate-500 mt-5">
                    {introduction.introduction.clone()}
                    </div>
                    <div class="mt-5 w-6 h-6">
                        <a href={introduction.github.clone()} title="github" target="_blank">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22">
                                </path>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
