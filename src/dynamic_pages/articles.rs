use askama::Template;
use async_session::serde_json;
use axum::{response::IntoResponse, Extension};
use serde::{Serialize, Deserialize};

use crate::utils::common::{Store, HtmlTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub title: String,
    pub date: String,
    pub published: bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Article {
    pub meta: MetaData,
    pub body: String,
}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate {
    posts: Vec<Article>,
}

pub async fn posts(Extension(store): Extension<Store>) -> impl IntoResponse {
    let mut articles: Vec<Article> = vec![];
    store.meta.iter()
        .for_each(|key_value| {
            let (key, value) = key_value.unwrap();
            let value_str = std::str::from_utf8(value.as_ref()).expect("failed to convert value to &str");
            let meta: MetaData = serde_json::from_str(value_str).expect("failed to convert &str to MetaData");
            let opt_body = store.body.get(key).expect("failed to find body with key");
            if let Some(body) = opt_body {
                let body = std::str::from_utf8(body.as_ref()).expect("failed to convert IVec body to &str");
                let article = Article { meta, body: body.to_string() };
                articles.push(article);
            } 
        });
        // TODO - remove title 
        let posts_template = PostsTemplate { posts: articles };
        HtmlTemplate(posts_template)

}   