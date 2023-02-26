use axum::{routing::get, Router};

mod index;

// TODO Remove this , index should be dynamic
pub static INDEX: &str = "/index";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}
