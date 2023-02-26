use askama::Template;
use async_session::serde_json;
use axum::{extract::Path, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::utils::common::{HtmlTemplate, Store};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub title: String,
    pub date: String,
    pub published: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub meta: MetaData,
    pub body: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct PostsTemplate {
    posts: Vec<MetaData>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    pub post: Article,
}

pub async fn posts(Extension(store): Extension<Store>) -> impl IntoResponse {
    let mut articles: Vec<MetaData> = vec![];
    store.meta.iter().for_each(|key_value| {
        let (_, value) = key_value.unwrap();
        let value_str =
            std::str::from_utf8(value.as_ref()).expect("failed to convert value to &str");
        let meta: MetaData =
            serde_json::from_str(value_str).expect("failed to convert &str to MetaData");
        articles.push(meta);
    });
    let posts_template = PostsTemplate { posts: articles };
    HtmlTemplate(posts_template)
}

pub async fn post(
    Path(key): Path<String>,
    Extension(store): Extension<Store>,
) -> impl IntoResponse {
    let key = &key[..];
    let opt_body = store.body.get(key).expect("did not find content for key");
    let body = opt_body.unwrap();
    let body = String::from_utf8(body.as_ref().to_vec()).expect("failed to convert IVec to String");
    let meta_opt = store.meta.get(key).expect("did find meta data for key");
    let meta = meta_opt.unwrap();
    let meta_str = std::str::from_utf8(meta.as_ref()).expect("failed to convert u8[..] to str");
    let meta: MetaData =
        serde_json::from_str(meta_str).expect("failed to convert &str to MetaData");
    let post_template = PostTemplate {
        post: Article { meta, body },
    };
    HtmlTemplate(post_template);
}
