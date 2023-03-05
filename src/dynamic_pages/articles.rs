use askama::Template;
use async_session::{async_trait, serde_json};
use axum::{
    extract::{FromRequestParts, Path, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Extension,
};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};

use crate::{
    login::auth_user::BlogAuthor,
    utils::common::{AppState, HtmlTemplate},
};

struct RequireUser(BlogAuthor);
#[async_trait]
impl<S> FromRequestParts<S> for RequireUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(user): Extension<BlogAuthor> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|_err| StatusCode::FORBIDDEN)?;

        if user.get_role().is_none() {
            Ok(RequireUser(user))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

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

pub async fn posts(
    Extension(user): Extension<BlogAuthor>,
    State(appstore): State<AppState>,
) -> impl IntoResponse {
    dbg!(user);
    let mut articles: Vec<MetaData> = vec![];
    appstore.blogstore.store.iter().for_each(|key_value| {
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

pub async fn post(Path(key): Path<String>, State(appstore): State<AppState>) -> impl IntoResponse {
    let store = appstore.blogstore;
    let key = &key[..];
    let opt_body = store.store.get(key).expect("did not find content for key");
    let body = opt_body.unwrap();
    let body = String::from_utf8(body.as_ref().to_vec()).expect("failed to convert IVec to String");
    let meta_opt = store.store.get(key).expect("did find meta data for key");
    let meta = meta_opt.unwrap();
    let meta_str = std::str::from_utf8(meta.as_ref()).expect("failed to convert u8[..] to str");
    let meta: MetaData =
        serde_json::from_str(meta_str).expect("failed to convert &str to MetaData");
    let post_template = PostTemplate {
        post: Article { meta, body },
    };
    HtmlTemplate(post_template);
}
