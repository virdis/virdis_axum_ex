use std::sync::Arc;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sled::Tree;
use tokio::signal::{self};

use crate::login::{auth_user::BlogAuthor, sled_user_store::SledUserStore};

pub type MyAuthContext =
    axum_login::extractors::AuthContext<BlogAuthor, SledUserStore<BlogAuthor>, ()>;

#[derive(Clone)]
pub struct BlogStore {
    pub store: Arc<Tree>,
}

#[derive(Clone)]
pub struct AppState {
    pub blogstore: BlogStore,
    pub userstore: SledUserStore<BlogAuthor>,
}
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error Details: {:?}", error),
            )
                .into_response(),
        }
    }
}

// Graceful Shutdown //

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

}
