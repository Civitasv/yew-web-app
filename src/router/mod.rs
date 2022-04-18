use crate::article::Articles;
use crate::details::ArticleDetails;
use crate::utils::markdown_files::ArticleFactory;
use yew::prelude::*;
use yew::Html;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/details/:article_id")]
    Details { article_id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <Articles />
            }
        }
        Route::Details { article_id } => {
            let article = ArticleFactory::from_markdown_file_id(*article_id);
            html! {
                <ArticleDetails article={article.clone()}/>
            }
        }
        Route::NotFound => html! {
            <h1>{"404"}</h1>
        },
    }
}
