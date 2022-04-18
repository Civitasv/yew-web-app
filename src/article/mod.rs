pub mod content;
pub mod item;
pub mod outline;
pub mod title;

use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::introduction::_Introduction;
use crate::utils::markdown_files::ArticleFactory;

use self::item::Article;
use crate::introduction::Introduction;

#[function_component(Articles)]
pub fn articles() -> Html {
    let intro = _Introduction::new(
        String::from("Hi there is Civitasv"),
        String::from("How you doing"),
        String::from("https://github.com/Civitasv"),
    );
    let articles = ArticleFactory::from_markdown_directory("/mdfiles");
    let all_articles: VNode = articles
        .iter()
        .map(|article| {
            html! {
                <Article article={article.clone() } />
            }
        })
        .collect();

    html! {
        <div class={classes!("flex","flex-col", "justify-center", "items-center")}>
            <Introduction introduction={intro.clone()}></Introduction>
            {all_articles}
        </div>
    }
}
