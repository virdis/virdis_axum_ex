use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::utils::common::{HtmlTemplate, Store};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
