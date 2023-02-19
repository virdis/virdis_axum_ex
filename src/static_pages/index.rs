use askama::Template;
use axum::response::IntoResponse;

use crate::utils::common::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
}


pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        title: "Sandeep Virdi's Blog".to_string(),
    };
    HtmlTemplate(template)
}