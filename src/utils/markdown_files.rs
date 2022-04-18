use std::fs;

use serde::Deserialize;

use crate::article::{content::_Content, item::_Article, outline::_Outline, title::_Title};

use super::markdown_yaml::YamlFormatter;

pub struct ArticleFactory;

#[derive(Deserialize)]
struct Metadata {
    title: String,
    author: String,
    outline: String,
    date_time: String,
}

impl ArticleFactory {
    pub fn from_markdown_directory(directory: &str) -> Vec<_Article> {
        let mut articles = vec![];
        log::info!("{}", directory);
        let paths = fs::read_dir(directory).unwrap();
        log::info!("{:?}", paths);
        for (index, path) in paths.enumerate() {
            let filecontent = fs::read_to_string(path.unwrap().path()).unwrap();

            let result = YamlFormatter::parse::<Metadata>(&filecontent).unwrap();
            let _article = _Article::new(
                index,
                _Title::new(result.metadata.title),
                _Content::new(result.content),
                _Outline::new(result.metadata.outline),
                result.metadata.date_time,
                result.metadata.author,
                String::from("4 min"),
            );
            articles.push(_article);
        }

        return articles;
    }

    pub fn from_markdown_file_id(id: usize) -> _Article {
        let articles = ArticleFactory::from_markdown_directory("./mdfiles");
        articles[id].clone()
    }
}
