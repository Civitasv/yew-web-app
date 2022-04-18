use crate::router::Route;

use super::content::_Content;
use super::outline::{Outline, _Outline};
use super::title::{Title, _Title};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct _Article {
    pub id: usize,
    pub title: _Title,
    pub content: _Content,
    pub outline: _Outline,
    pub date: String,
    pub author: String,
    pub read_time: String,
}

impl _Article {
    pub fn new(
        id: usize,
        title: _Title,
        content: _Content,
        outline: _Outline,
        date: String,
        author: String,
        read_time: String,
    ) -> Self {
        Self {
            id,
            title,
            content,
            outline,
            date,
            author,
            read_time,
        }
    }
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub article: _Article,
}

#[function_component(Article)]
pub fn article(Props { article }: &Props) -> Html {
    html! {
        <div class={classes!("cursor-pointer", "bg-white", "rounded-md", "h-28",
                             "md:w-1/2", "min-w-max", "sm:w-full", "mb-3", "mt-3",
                             "flex", "items-center", "pl-6", "pr-6", "hover:scale-95")}>
            <Link<Route> to={Route::Details {article_id: article.id}}>
                <div class="article">
                    <div class="article-title">
                        <Title title={article.title.clone()}/>
                    </div>
                    <div class="article-text">
                        <Outline outline={article.outline.clone()}/>
                        <div class="text-sm text-slate-500 mt-2">
                            <span class="me-1">{article.date.clone()}</span>
                            <span class="me-1">{"·"}</span>
                            <span class="me-1">{article.read_time.clone()}</span>
                            <span class="me-1">{"·"}</span>
                            <span>{article.author.clone()}</span>
                        </div>
                    </div>
                </div>
            </Link<Route>>
        </div>
    }
}
