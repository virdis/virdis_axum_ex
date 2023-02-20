use askama::Template;
use axum::{response::{IntoResponse, Html}, http::StatusCode};
use tokio::signal::{unix::signal, self};

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
            .into_response()
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

    //#[cfg(not(unix))]
    //let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

   // tracing::debug!("signal received, starting graceful shutdown");
}