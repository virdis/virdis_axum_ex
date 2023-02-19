mod utils;
mod static_pages;

use std::net::SocketAddr;

use axum::{Router};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(static_pages::routes());
    
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));

// TODO - add graceful shutdown
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await 
        .unwrap();

}
