use crate::article::content::Content;
use crate::article::item::_Article;
use crate::article::outline::Outline;
use crate::article::title::Title;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub article: _Article,
}

#[function_component(ArticleDetails)]
pub fn article_details(Props { article }: &Props) -> Html {
    html! {
        <div class={classes!("bg-white", "md:w-1/2", "min-w-max", "sm:w-full", "mb-3", "mt-3",
                             "flex", "items-center")}>
            <div class="article">
                <div class="article-title">
                    <Title title={article.title.clone()}/>
                </div>
                <div class="article-text">
                    <Outline outline={article.outline.clone()}/>
                    <div class="text-sm text-slate-500 mt-2">
                        <span class="me-1">{&article.date}</span>
                        <span class="me-1">{"·"}</span>
                        <span class="me-1">{&article.read_time}</span>
                        <span class="me-1">{"·"}</span>
                        <span>{&article.author}</span>
                    </div>
                </div>
                <div class="article-content">
                    <Content content={article.content.clone()}/>
                </div>
            </div>
        </div>
    }
}
