use axum::{Router, routing::get};

mod index;

pub static INDEX: &str = "/index" ;

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}