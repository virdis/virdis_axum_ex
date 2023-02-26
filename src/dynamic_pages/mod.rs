use axum::{routing::get, Router};

mod articles;

pub static INDEX: &str = "/index.html";

pub fn index_route() -> Router {
    Router::new().route(INDEX, get(articles::posts))
}

pub static POST: &str = "/post.html";

pub fn post_route() -> Router {
    Router::new().route(POST, get(articles::post))
}
